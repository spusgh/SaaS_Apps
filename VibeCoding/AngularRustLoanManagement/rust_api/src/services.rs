// src/services.rs
use crate::models::{Loan, SearchFilters, SearchResponse, Statistics, StatusCount, ProductTypeCount};
use sqlx::PgPool;
use anyhow::Result;

pub struct LoanService;

impl LoanService {
    pub async fn search_loans(pool: &PgPool, filters: SearchFilters) -> Result<SearchResponse> {
        let page = filters.page.unwrap_or(1);
        let page_size = filters.page_size.unwrap_or(50);
        let offset = (page - 1) * page_size;

        let mut query_builder = sqlx::QueryBuilder::new(
            "SELECT loan_id, customer_name, property_address, origination_date, 
             maturity_date, loan_amount, remaining_balance, interest_rate, 
             monthly_payment, status, product_name, product_type, 
             security_name, servicer_name, current_status FROM loans WHERE 1=1"
        );

        // Add filters
        if let Some(customer_name) = &filters.customer_name {
            query_builder.push(" AND customer_name ILIKE ");
            query_builder.push_bind(format!("%{}%", customer_name));
        }

        if let Some(status) = &filters.status {
            query_builder.push(" AND status = ");
            query_builder.push_bind(status);
        }

        if let Some(product_type) = &filters.product_type {
            query_builder.push(" AND product_type = ");
            query_builder.push_bind(product_type);
        }

        if let Some(servicer_name) = &filters.servicer_name {
            query_builder.push(" AND servicer_name ILIKE ");
            query_builder.push_bind(format!("%{}%", servicer_name));
        }

        if let Some(min_amount) = filters.min_loan_amount {
            query_builder.push(" AND loan_amount >= ");
            query_builder.push_bind(min_amount);
        }

        if let Some(max_amount) = filters.max_loan_amount {
            query_builder.push(" AND loan_amount <= ");
            query_builder.push_bind(max_amount);
        }

        if let Some(date_from) = filters.origination_date_from {
            query_builder.push(" AND origination_date >= ");
            query_builder.push_bind(date_from);
        }

        if let Some(date_to) = filters.origination_date_to {
            query_builder.push(" AND origination_date <= ");
            query_builder.push_bind(date_to);
        }

        query_builder.push(" ORDER BY loan_id LIMIT ");
        query_builder.push_bind(page_size);
        query_builder.push(" OFFSET ");
        query_builder.push_bind(offset);

        let loans = query_builder
            .build_query_as::<Loan>()
            .fetch_all(pool)
            .await?;

        // Get total count
        let mut count_query = sqlx::QueryBuilder::new("SELECT COUNT(*) FROM loans WHERE 1=1");
        
        // Add same filters for count
        if let Some(customer_name) = &filters.customer_name {
            count_query.push(" AND customer_name ILIKE ");
            count_query.push_bind(format!("%{}%", customer_name));
        }

        if let Some(status) = &filters.status {
            count_query.push(" AND status = ");
            count_query.push_bind(status);
        }

        if let Some(product_type) = &filters.product_type {
            count_query.push(" AND product_type = ");
            count_query.push_bind(product_type);
        }

        if let Some(servicer_name) = &filters.servicer_name {
            count_query.push(" AND servicer_name ILIKE ");
            count_query.push_bind(format!("%{}%", servicer_name));
        }

        if let Some(min_amount) = filters.min_loan_amount {
            count_query.push(" AND loan_amount >= ");
            count_query.push_bind(min_amount);
        }

        if let Some(max_amount) = filters.max_loan_amount {
            count_query.push(" AND loan_amount <= ");
            count_query.push_bind(max_amount);
        }

        if let Some(date_from) = filters.origination_date_from {
            count_query.push(" AND origination_date >= ");
            count_query.push_bind(date_from);
        }

        if let Some(date_to) = filters.origination_date_to {
            count_query.push(" AND origination_date <= ");
            count_query.push_bind(date_to);
        }

        let total: (i64,) = count_query
            .build_query_as()
            .fetch_one(pool)
            .await?;

        Ok(SearchResponse {
            data: loans,
            total: total.0,
            page,
            page_size,
        })
    }

    pub async fn get_loan_by_id(pool: &PgPool, loan_id: &str) -> Result<Option<Loan>> {
        let loan = sqlx::query_as!(
            Loan,
            "SELECT loan_id, customer_name, property_address, origination_date, 
             maturity_date, loan_amount, remaining_balance, interest_rate, 
             monthly_payment, status, product_name, product_type, 
             security_name, servicer_name, current_status 
             FROM loans WHERE loan_id = $1",
            loan_id
        )
        .fetch_optional(pool)
        .await?;

        Ok(loan)
    }

    pub async fn get_statistics(pool: &PgPool) -> Result<Statistics> {
        let basic_stats = sqlx::query!(
            "SELECT 
                COUNT(*) as total_loans,
                COALESCE(SUM(loan_amount), 0) as total_loan_amount,
                COALESCE(SUM(remaining_balance), 0) as total_remaining_balance,
                COALESCE(AVG(interest_rate), 0) as average_interest_rate
             FROM loans"
        )
        .fetch_one(pool)
        .await?;

        let status_breakdown = sqlx::query!(
            "SELECT status, COUNT(*) as count FROM loans GROUP BY status ORDER BY count DESC"
        )
        .fetch_all(pool)
        .await?
        .into_iter()
        .map(|row| StatusCount {
            status: row.status,
            count: row.count.unwrap_or(0),
        })
        .collect();

        let product_type_breakdown = sqlx::query!(
            "SELECT product_type, COUNT(*) as count FROM loans GROUP BY product_type ORDER BY count DESC"
        )
        .fetch_all(pool)
        .await?
        .into_iter()
        .map(|row| ProductTypeCount {
            product_type: row.product_type,
            count: row.count.unwrap_or(0),
        })
        .collect();

        Ok(Statistics {
            total_loans: basic_stats.total_loans.unwrap_or(0),
            total_loan_amount: basic_stats.total_loan_amount.unwrap_or(0.0),
            total_remaining_balance: basic_stats.total_remaining_balance.unwrap_or(0.0),
            average_interest_rate: basic_stats.average_interest_rate.unwrap_or(0.0),
            status_breakdown,
            product_type_breakdown,
        })
    }
}
