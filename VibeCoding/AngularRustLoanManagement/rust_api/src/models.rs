// src/models.rs
use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc, NaiveDate};
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize, FromRow)]
#[serde(rename_all = "camelCase")]
pub struct Loan {
    pub loan_id: String,
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

#[derive(Debug, Deserialize)]
pub struct SearchFilters {
    pub customer_name: Option<String>,
    pub status: Option<String>,
    pub product_type: Option<String>,
    pub servicer_name: Option<String>,
    pub min_loan_amount: Option<f64>,
    pub max_loan_amount: Option<f64>,
    pub origination_date_from: Option<NaiveDate>,
    pub origination_date_to: Option<NaiveDate>,
    pub page: Option<i32>,
    pub page_size: Option<i32>,
}

#[derive(Debug, Serialize)]
pub struct SearchResponse {
    pub data: Vec<Loan>,
    pub total: i64,
    pub page: i32,
    pub page_size: i32,
}

#[derive(Debug, Serialize)]
pub struct Statistics {
    pub total_loans: i64,
    pub total_loan_amount: f64,
    pub total_remaining_balance: f64,
    pub average_interest_rate: f64,
    pub status_breakdown: Vec<StatusCount>,
    pub product_type_breakdown: Vec<ProductTypeCount>,
}

#[derive(Debug, Serialize)]
pub struct StatusCount {
    pub status: String,
    pub count: i64,
}

#[derive(Debug, Serialize)]
pub struct ProductTypeCount {
    pub product_type: String,
    pub count: i64,
}
