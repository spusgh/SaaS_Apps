// Cargo.toml dependencies needed:
// [dependencies]
// serde = { version = "1.0", features = ["derive"] }
// serde_json = "1.0"
// tokio = { version = "1.0", features = ["full"] }
// sqlx = { version = "0.7", features = ["runtime-tokio-rustls", "postgres", "chrono", "uuid"] }
// uuid = { version = "1.0", features = ["v4", "serde"] }
// chrono = { version = "0.4", features = ["serde"] }
// anyhow = "1.0"
// clap = { version = "4.0", features = ["derive"] }
// linfa = "0.7"
// linfa-clustering = "0.7"
// ndarray = "0.15"

use anyhow::Result;
use chrono::{DateTime, NaiveDate, Utc};
use clap::{Parser, Subcommand};
use serde::{Deserialize, Serialize};
use sqlx::{PgPool, Row};
use std::collections::HashMap;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct LoanRecord {
    pub loan_id: Uuid,
    pub customer_name: String,
    pub property_address: String,
    pub origination_date: NaiveDate,
    pub maturity_date: NaiveDate,
    pub loan_amount: f64,
    pub remaining_balance: f64,
    pub interest_rate: f64,
    pub monthly_payment: f64,
    pub status: String,
    pub product_name: String,
    pub product_type: String,
    pub security_name: String,
    pub servicer_name: String,
    pub current_status: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LoanAnalytics {
    pub loan_age_days: i64,
    pub remaining_term_days: i64,
    pub payment_ratio: f64,
    pub balance_ratio: f64,
    pub risk_score: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SearchFilters {
    pub status: Option<String>,
    pub product_type: Option<String>,
    pub servicer_name: Option<String>,
    pub min_loan_amount: Option<f64>,
    pub max_loan_amount: Option<f64>,
    pub min_interest_rate: Option<f64>,
    pub max_interest_rate: Option<f64>,
    pub origination_date_from: Option<NaiveDate>,
    pub origination_date_to: Option<NaiveDate>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SearchResult {
    pub loan: LoanRecord,
    pub analytics: LoanAnalytics,
    pub risk_category: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PortfolioSummary {
    pub total_loans: i64,
    pub total_loan_amount: f64,
    pub total_remaining_balance: f64,
    pub average_interest_rate: f64,
    pub status_distribution: HashMap<String, i64>,
    pub product_type_distribution: HashMap<String, i64>,
    pub risk_distribution: HashMap<String, i64>,
}

pub struct LoanSearchEngine {
    pool: PgPool,
}

impl LoanSearchEngine {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    /// Search loans with filters and AI-powered analytics
    pub async fn search_loans(&self, filters: SearchFilters) -> Result<Vec<SearchResult>> {
        let mut query = String::from(
            "SELECT loan_id, customer_name, property_address, origination_date, 
             maturity_date, loan_amount, remaining_balance, interest_rate, 
             monthly_payment, status, product_name, product_type, 
             security_name, servicer_name, current_status 
             FROM loans WHERE 1=1"
        );

        let mut params: Vec<Box<dyn sqlx::Encode<'_, sqlx::Postgres> + Send + 'static>> = Vec::new();
        let mut param_count = 0;

        // Build dynamic query based on filters
        if let Some(status) = &filters.status {
            param_count += 1;
            query.push_str(&format!(" AND status = ${}", param_count));
            params.push(Box::new(status.clone()));
        }

        if let Some(product_type) = &filters.product_type {
            param_count += 1;
            query.push_str(&format!(" AND product_type = ${}", param_count));
            params.push(Box::new(product_type.clone()));
        }

        if let Some(servicer) = &filters.servicer_name {
            param_count += 1;
            query.push_str(&format!(" AND servicer_name = ${}", param_count));
            params.push(Box::new(servicer.clone()));
        }

        if let Some(min_amount) = filters.min_loan_amount {
            param_count += 1;
            query.push_str(&format!(" AND loan_amount >= ${}", param_count));
            params.push(Box::new(min_amount));
        }

        if let Some(max_amount) = filters.max_loan_amount {
            param_count += 1;
            query.push_str(&format!(" AND loan_amount <= ${}", param_count));
            params.push(Box::new(max_amount));
        }

        query.push_str(" ORDER BY loan_amount DESC LIMIT 1000");

        // Execute query
        let rows = sqlx::query(&query).fetch_all(&self.pool).await?;

        let mut results = Vec::new();
        for row in rows {
            let loan = LoanRecord {
                loan_id: row.get("loan_id"),
                customer_name: row.get("customer_name"),
                property_address: row.get("property_address"),
                origination_date: row.get("origination_date"),
                maturity_date: row.get("maturity_date"),
                loan_amount: row.get("loan_amount"),
                remaining_balance: row.get("remaining_balance"),
                interest_rate: row.get("interest_rate"),
                monthly_payment: row.get("monthly_payment"),
                status: row.get("status"),
                product_name: row.get("product_name"),
                product_type: row.get("product_type"),
                security_name: row.get("security_name"),
                servicer_name: row.get("servicer_name"),
                current_status: row.get("current_status"),
            };

            let analytics = self.calculate_analytics(&loan);
            let risk_category = self.determine_risk_category(&loan, &analytics);

            results.push(SearchResult {
                loan,
                analytics,
                risk_category,
            });
        }

        Ok(results)
    }

    /// Calculate AI-powered analytics for a loan
    fn calculate_analytics(&self, loan: &LoanRecord) -> LoanAnalytics {
        let now = Utc::now().date_naive();
        let loan_age_days = (now - loan.origination_date).num_days();
        let remaining_term_days = (loan.maturity_date - now).num_days();
        let payment_ratio = loan.monthly_payment / loan.loan_amount * 12.0; // Annual payment ratio
        let balance_ratio = loan.remaining_balance / loan.loan_amount;
        
        // Simple risk scoring algorithm (in production, use trained ML model)
        let risk_score = self.calculate_risk_score(loan, loan_age_days, remaining_term_days, balance_ratio);

        LoanAnalytics {
            loan_age_days,
            remaining_term_days,
            payment_ratio,
            balance_ratio,
            risk_score,
        }
    }

    /// AI-powered risk scoring (simplified version)
    fn calculate_risk_score(&self, loan: &LoanRecord, age_days: i64, remaining_days: i64, balance_ratio: f64) -> f64 {
        let mut score = 50.0; // Base score

        // Interest rate factor
        if loan.interest_rate > 6.0 {
            score += (loan.interest_rate - 6.0) * 10.0;
        }

        // Balance ratio factor
        if balance_ratio > 0.9 {
            score += 20.0;
        } else if balance_ratio < 0.5 {
            score -= 10.0;
        }

        // Loan age factor
        if age_days > 365 * 3 && balance_ratio > 0.8 {
            score += 15.0;
        }

        // Status factor
        match loan.status.as_str() {
            "Current" => score -= 10.0,
            "30 Days Late" => score += 25.0,
            "60 Days Late" => score += 50.0,
            "90+ Days Late" => score += 75.0,
            "Default" => score = 100.0,
            _ => {}
        }

        score.clamp(0.0, 100.0)
    }

    /// Determine risk category based on analytics
    fn determine_risk_category(&self, _loan: &LoanRecord, analytics: &LoanAnalytics) -> String {
        match analytics.risk_score {
            score if score >= 80.0 => "High Risk".to_string(),
            score if score >= 60.0 => "Medium-High Risk".to_string(),
            score if score >= 40.0 => "Medium Risk".to_string(),
            score if score >= 20.0 => "Low-Medium Risk".to_string(),
            _ => "Low Risk".to_string(),
        }
    }

    /// Generate portfolio summary with AI insights
    pub async fn generate_portfolio_summary(&self, filters: Option<SearchFilters>) -> Result<PortfolioSummary> {
        let loans = self.search_loans(filters.unwrap_or_default()).await?;
        
        let total_loans = loans.len() as i64;
        let total_loan_amount: f64 = loans.iter().map(|r| r.loan.loan_amount).sum();
        let total_remaining_balance: f64 = loans.iter().map(|r| r.loan.remaining_balance).sum();
        let average_interest_rate: f64 = loans.iter().map(|r| r.loan.interest_rate).sum::<f64>() / total_loans as f64;

        let mut status_distribution = HashMap::new();
        let mut product_type_distribution = HashMap::new();
        let mut risk_distribution = HashMap::new();

        for result in &loans {
            *status_distribution.entry(result.loan.status.clone()).or_insert(0) += 1;
            *product_type_distribution.entry(result.loan.product_type.clone()).or_insert(0) += 1;
            *risk_distribution.entry(result.risk_category.clone()).or_insert(0) += 1;
        }

        Ok(PortfolioSummary {
            total_loans,
            total_loan_amount,
            total_remaining_balance,
            average_interest_rate,
            status_distribution,
            product_type_distribution,
            risk_distribution,
        })
    }

    /// Find similar loans using AI clustering
    pub async fn find_similar_loans(&self, loan_id: Uuid, limit: usize) -> Result<Vec<SearchResult>> {
        // Get the reference loan
        let reference_loan = self.get_loan_by_id(loan_id).await?;
        let reference_analytics = self.calculate_analytics(&reference_loan);

        // Get all loans for similarity comparison
        let all_loans = self.search_loans(SearchFilters::default()).await?;

        // Calculate similarity scores
        let mut similarities: Vec<(SearchResult, f64)> = all_loans
            .into_iter()
            .filter(|result| result.loan.loan_id != loan_id)
            .map(|result| {
                let similarity = self.calculate_similarity(&reference_analytics, &result.analytics);
                (result, similarity)
            })
            .collect();

        // Sort by similarity (highest first)
        similarities.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());

        Ok(similarities.into_iter().take(limit).map(|(result, _)| result).collect())
    }

    /// Calculate similarity between two loan analytics
    fn calculate_similarity(&self, analytics1: &LoanAnalytics, analytics2: &LoanAnalytics) -> f64 {
        let features1 = vec![
            analytics1.payment_ratio,
            analytics1.balance_ratio,
            analytics1.risk_score / 100.0,
            analytics1.loan_age_days as f64 / 3650.0, // Normalize to 10 years
        ];

        let features2 = vec![
            analytics2.payment_ratio,
            analytics2.balance_ratio,
            analytics2.risk_score / 100.0,
            analytics2.loan_age_days as f64 / 3650.0,
        ];

        // Calculate Euclidean distance and convert to similarity
        let distance: f64 = features1
            .iter()
            .zip(features2.iter())
            .map(|(a, b)| (a - b).powi(2))
            .sum::<f64>()
            .sqrt();

        1.0 / (1.0 + distance) // Convert distance to similarity
    }

    async fn get_loan_by_id(&self, loan_id: Uuid) -> Result<LoanRecord> {
        let row = sqlx::query(
            "SELECT loan_id, customer_name, property_address, origination_date, 
             maturity_date, loan_amount, remaining_balance, interest_rate, 
             monthly_payment, status, product_name, product_type, 
             security_name, servicer_name, current_status 
             FROM loans WHERE loan_id = $1"
        )
        .bind(loan_id)
        .fetch_one(&self.pool)
        .await?;

        Ok(LoanRecord {
            loan_id: row.get("loan_id"),
            customer_name: row.get("customer_name"),
            property_address: row.get("property_address"),
            origination_date: row.get("origination_date"),
            maturity_date: row.get("maturity_date"),
            loan_amount: row.get("loan_amount"),
            remaining_balance: row.get("remaining_balance"),
            interest_rate: row.get("interest_rate"),
            monthly_payment: row.get("monthly_payment"),
            status: row.get("status"),
            product_name: row.get("product_name"),
            product_type: row.get("product_type"),
            security_name: row.get("security_name"),
            servicer_name: row.get("servicer_name"),
            current_status: row.get("current_status"),
        })
    }
}

impl Default for SearchFilters {
    fn default() -> Self {
        Self {
            status: None,
            product_type: None,
            servicer_name: None,
            min_loan_amount: None,
            max_loan_amount: None,
            min_interest_rate: None,
            max_interest_rate: None,
            origination_date_from: None,
            origination_date_to: None,
        }
    }
}

#[derive(Parser)]
#[command(name = "loan-ai")]
#[command(about = "AI-powered loan portfolio search and analysis")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Search loans with filters
    Search {
        #[arg(long)]
        status: Option<String>,
        #[arg(long)]
        product_type: Option<String>,
        #[arg(long)]
        min_amount: Option<f64>,
        #[arg(long)]
        max_amount: Option<f64>,
    },
    /// Generate portfolio summary
    Summary,
    /// Find similar loans
    Similar {
        #[arg(long)]
        loan_id: String,
        #[arg(long, default_value = "10")]
        limit: usize,
    },
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();
    
    // Initialize database connection
    let database_url = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgresql://user:password@localhost/loans".to_string());
    
    let pool = PgPool::connect(&database_url).await?;
    let search_engine = LoanSearchEngine::new(pool);

    match cli.command {
        Commands::Search { status, product_type, min_amount, max_amount } => {
            let filters = SearchFilters {
                status,
                product_type,
                min_loan_amount: min_amount,
                max_loan_amount: max_amount,
                ..Default::default()
            };

            let results = search_engine.search_loans(filters).await?;
            println!("Found {} loans:", results.len());
            
            for result in results.iter().take(10) {
                println!("Loan ID: {}", result.loan.loan_id);
                println!("Customer: {}", result.loan.customer_name);
                println!("Amount: ${:.2}", result.loan.loan_amount);
                println!("Risk Score: {:.1}", result.analytics.risk_score);
                println!("Risk Category: {}", result.risk_category);
                println!("---");
            }
        }
        Commands::Summary => {
            let summary = search_engine.generate_portfolio_summary(None).await?;
            println!("Portfolio Summary:");
            println!("Total Loans: {}", summary.total_loans);
            println!("Total Amount: ${:.2}", summary.total_loan_amount);
            println!("Remaining Balance: ${:.2}", summary.total_remaining_balance);
            println!("Average Interest Rate: {:.2}%", summary.average_interest_rate);
            println!("\nStatus Distribution:");
            for (status, count) in &summary.status_distribution {
                println!("  {}: {}", status, count);
            }
            println!("\nRisk Distribution:");
            for (risk, count) in &summary.risk_distribution {
                println!("  {}: {}", risk, count);
            }
        }
        Commands::Similar { loan_id, limit } => {
            let uuid = Uuid::parse_str(&loan_id)?;
            let similar_loans = search_engine.find_similar_loans(uuid, limit).await?;
            
            println!("Found {} similar loans:", similar_loans.len());
            for loan in similar_loans {
                println!("Loan ID: {}", loan.loan.loan_id);
                println!("Customer: {}", loan.loan.customer_name);
                println!("Amount: ${:.2}", loan.loan.loan_amount);
                println!("Risk Category: {}", loan.risk_category);
                println!("---");
            }
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::NaiveDate;

    #[test]
    fn test_analytics_calculation() {
        let loan = LoanRecord {
            loan_id: Uuid::new_v4(),
            customer_name: "Test Customer".to_string(),
            property_address: "123 Test St".to_string(),
            origination_date: NaiveDate::from_ymd_opt(2020, 1, 1).unwrap(),
            maturity_date: NaiveDate::from_ymd_opt(2050, 1, 1).unwrap(),
            loan_amount: 100000.0,
            remaining_balance: 80000.0,
            interest_rate: 5.5,
            monthly_payment: 1000.0,
            status: "Current".to_string(),
            product_name: "Test Product".to_string(),
            product_type: "Fixed".to_string(),
            security_name: "Test Security".to_string(),
            servicer_name: "Test Servicer".to_string(),
            current_status: "Active".to_string(),
        };

        let pool = PgPool::connect("postgresql://test").await.unwrap();
        let engine = LoanSearchEngine::new(pool);
        let analytics = engine.calculate_analytics(&loan);

        assert!(analytics.balance_ratio > 0.0);
        assert!(analytics.payment_ratio > 0.0);
        assert!(analytics.risk_score >= 0.0 && analytics.risk_score <= 100.0);
    }
}