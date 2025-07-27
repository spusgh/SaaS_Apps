// Rust AI Loan Search Application
// Readonly search system for loan portfolio records

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use chrono::{DateTime, Utc, NaiveDate};
use anyhow::Result;
use serde_json;
use regex::Regex;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoanRecord {
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

#[derive(Debug, Clone)]
pub struct SearchCriteria {
    pub loan_id: Option<String>,
    pub customer_name: Option<String>,
    pub status: Option<String>,
    pub product_name: Option<String>,
    pub servicer_name: Option<String>,
    pub min_loan_amount: Option<f64>,
    pub max_loan_amount: Option<f64>,
    pub min_interest_rate: Option<f64>,
    pub max_interest_rate: Option<f64>,
    pub origination_date_from: Option<NaiveDate>,
    pub origination_date_to: Option<NaiveDate>,
    pub property_state: Option<String>,
}

#[derive(Debug, Clone)]
pub struct SearchResult {
    pub total_matches: usize,
    pub records: Vec<LoanRecord>,
    pub aggregations: HashMap<String, f64>,
}

pub struct LoanSearchEngine {
    records: Vec<LoanRecord>,
    indices: HashMap<String, Vec<usize>>, // Field-based indices for faster search
}

impl LoanSearchEngine {
    pub fn new() -> Self {
        Self {
            records: Vec::new(),
            indices: HashMap::new(),
        }
    }

    /// Load loan data from JSON
    pub fn load_from_json(&mut self, json_data: &str) -> Result<()> {
        let records: Vec<LoanRecord> = serde_json::from_str(json_data)?;
        self.records = records;
        self.build_indices();
        Ok(())
    }

    /// Build search indices for faster queries
    fn build_indices(&mut self) {
        self.indices.clear();
        
        for (idx, record) in self.records.iter().enumerate() {
            // Index by status
            self.indices
                .entry(format!("status:{}", record.status.to_lowercase()))
                .or_insert_with(Vec::new)
                .push(idx);
            
            // Index by product name
            self.indices
                .entry(format!("product:{}", record.product_name.to_lowercase()))
                .or_insert_with(Vec::new)
                .push(idx);
            
            // Index by servicer
            self.indices
                .entry(format!("servicer:{}", record.servicer_name.to_lowercase()))
                .or_insert_with(Vec::new)
                .push(idx);
            
            // Index by state (extracted from property address)
            if let Some(state) = self.extract_state(&record.property_address) {
                self.indices
                    .entry(format!("state:{}", state.to_lowercase()))
                    .or_insert_with(Vec::new)
                    .push(idx);
            }
        }
    }

    /// Extract state from property address
    fn extract_state(&self, address: &str) -> Option<String> {
        let re = Regex::new(r"\b([A-Z]{2})\b").unwrap();
        re.captures(address)
            .and_then(|caps| caps.get(1))
            .map(|m| m.as_str().to_string())
    }

    /// Perform AI-enhanced search with natural language processing
    pub fn search(&self, criteria: &SearchCriteria) -> SearchResult {
        let mut matching_indices: Vec<usize> = (0..self.records.len()).collect();

        // Filter by loan ID (exact match)
        if let Some(ref loan_id) = criteria.loan_id {
            matching_indices.retain(|&idx| {
                self.records[idx].loan_id.to_lowercase().contains(&loan_id.to_lowercase())
            });
        }

        // Filter by customer name (fuzzy match)
        if let Some(ref customer_name) = criteria.customer_name {
            matching_indices.retain(|&idx| {
                self.fuzzy_match(&self.records[idx].customer_name, customer_name, 0.8)
            });
        }

        // Filter by status
        if let Some(ref status) = criteria.status {
            matching_indices.retain(|&idx| {
                self.records[idx].status.to_lowercase() == status.to_lowercase()
            });
        }

        // Filter by product name
        if let Some(ref product_name) = criteria.product_name {
            matching_indices.retain(|&idx| {
                self.records[idx].product_name.to_lowercase().contains(&product_name.to_lowercase())
            });
        }

        // Filter by servicer name
        if let Some(ref servicer_name) = criteria.servicer_name {
            matching_indices.retain(|&idx| {
                self.records[idx].servicer_name.to_lowercase().contains(&servicer_name.to_lowercase())
            });
        }

        // Filter by loan amount range
        if let Some(min_amount) = criteria.min_loan_amount {
            matching_indices.retain(|&idx| self.records[idx].loan_amount >= min_amount);
        }
        if let Some(max_amount) = criteria.max_loan_amount {
            matching_indices.retain(|&idx| self.records[idx].loan_amount <= max_amount);
        }

        // Filter by interest rate range
        if let Some(min_rate) = criteria.min_interest_rate {
            matching_indices.retain(|&idx| self.records[idx].interest_rate >= min_rate);
        }
        if let Some(max_rate) = criteria.max_interest_rate {
            matching_indices.retain(|&idx| self.records[idx].interest_rate <= max_rate);
        }

        // Filter by origination date range
        if let Some(from_date) = criteria.origination_date_from {
            matching_indices.retain(|&idx| self.records[idx].origination_date >= from_date);
        }
        if let Some(to_date) = criteria.origination_date_to {
            matching_indices.retain(|&idx| self.records[idx].origination_date <= to_date);
        }

        // Filter by property state
        if let Some(ref state) = criteria.property_state {
            matching_indices.retain(|&idx| {
                self.extract_state(&self.records[idx].property_address)
                    .map(|s| s.to_lowercase() == state.to_lowercase())
                    .unwrap_or(false)
            });
        }

        // Collect matching records
        let records: Vec<LoanRecord> = matching_indices
            .iter()
            .map(|&idx| self.records[idx].clone())
            .collect();

        // Calculate aggregations
        let aggregations = self.calculate_aggregations(&records);

        SearchResult {
            total_matches: records.len(),
            records,
            aggregations,
        }
    }

    /// Fuzzy string matching using Levenshtein distance
    fn fuzzy_match(&self, text1: &str, text2: &str, threshold: f64) -> bool {
        let distance = self.levenshtein_distance(text1, text2);
        let max_len = text1.len().max(text2.len()) as f64;
        if max_len == 0.0 {
            return true;
        }
        let similarity = 1.0 - (distance as f64 / max_len);
        similarity >= threshold
    }

    /// Calculate Levenshtein distance between two strings
    fn levenshtein_distance(&self, s1: &str, s2: &str) -> usize {
        let len1 = s1.len();
        let len2 = s2.len();
        let mut matrix = vec![vec![0; len2 + 1]; len1 + 1];

        for i in 0..=len1 {
            matrix[i][0] = i;
        }
        for j in 0..=len2 {
            matrix[0][j] = j;
        }

        for (i, c1) in s1.chars().enumerate() {
            for (j, c2) in s2.chars().enumerate() {
                let cost = if c1 == c2 { 0 } else { 1 };
                matrix[i + 1][j + 1] = (matrix[i][j + 1] + 1)
                    .min(matrix[i + 1][j] + 1)
                    .min(matrix[i][j] + cost);
            }
        }

        matrix[len1][len2]
    }

    /// Calculate aggregations for search results
    fn calculate_aggregations(&self, records: &[LoanRecord]) -> HashMap<String, f64> {
        let mut aggregations = HashMap::new();

        if records.is_empty() {
            return aggregations;
        }

        let total_loan_amount: f64 = records.iter().map(|r| r.loan_amount).sum();
        let total_remaining_balance: f64 = records.iter().map(|r| r.remaining_balance).sum();
        let avg_interest_rate: f64 = records.iter().map(|r| r.interest_rate).sum() / records.len() as f64;
        let avg_monthly_payment: f64 = records.iter().map(|r| r.monthly_payment).sum() / records.len() as f64;

        aggregations.insert("total_loan_amount".to_string(), total_loan_amount);
        aggregations.insert("total_remaining_balance".to_string(), total_remaining_balance);
        aggregations.insert("avg_interest_rate".to_string(), avg_interest_rate);
        aggregations.insert("avg_monthly_payment".to_string(), avg_monthly_payment);
        aggregations.insert("count".to_string(), records.len() as f64);

        aggregations
    }

    /// AI-powered intelligent search with natural language queries
    pub fn intelligent_search(&self, query: &str) -> SearchResult {
        let mut criteria = SearchCriteria::default();

        // Parse natural language query
        let query_lower = query.to_lowercase();

        // Extract loan amount ranges
        if let Some(captures) = Regex::new(r"amount.*?(\d+).*?to.*?(\d+)")
            .unwrap()
            .captures(&query_lower)
        {
            if let (Ok(min), Ok(max)) = (
                captures[1].parse::<f64>(),
                captures[2].parse::<f64>(),
            ) {
                criteria.min_loan_amount = Some(min);
                criteria.max_loan_amount = Some(max);
            }
        }

        // Extract status keywords
        if query_lower.contains("active") {
            criteria.status = Some("Active".to_string());
        } else if query_lower.contains("default") {
            criteria.status = Some("Default".to_string());
        } else if query_lower.contains("paid") {
            criteria.status = Some("Paid Off".to_string());
        }

        // Extract product types
        if query_lower.contains("fha") {
            criteria.product_name = Some("FHA".to_string());
        } else if query_lower.contains("va") {
            criteria.product_name = Some("VA".to_string());
        } else if query_lower.contains("jumbo") {
            criteria.product_name = Some("Jumbo".to_string());
        }

        self.search(&criteria)
    }

    /// Get portfolio statistics
    pub fn get_portfolio_stats(&self) -> HashMap<String, f64> {
        let all_criteria = SearchCriteria::default();
        let all_results = self.search(&all_criteria);
        all_results.aggregations
    }

    /// Export search results to JSON
    pub fn export_results(&self, results: &SearchResult) -> Result<String> {
        serde_json::to_string_pretty(results).map_err(Into::into)
    }
}

impl Default for SearchCriteria {
    fn default() -> Self {
        Self {
            loan_id: None,
            customer_name: None,
            status: None,
            product_name: None,
            servicer_name: None,
            min_loan_amount: None,
            max_loan_amount: None,
            min_interest_rate: None,
            max_interest_rate: None,
            origination_date_from: None,
            origination_date_to: None,
            property_state: None,
        }
    }
}

// CLI Interface
use clap::{App, Arg, SubCommand};

fn main() -> Result<()> {
    let matches = App::new("Loan Search Engine")
        .version("1.0")
        .author("AI Assistant")
        .about("AI-powered search for loan portfolio records")
        .subcommand(
            SubCommand::with_name("search")
                .about("Search loan records")
                .arg(
                    Arg::with_name("query")
                        .short("q")
                        .long("query")
                        .value_name("QUERY")
                        .help("Natural language search query")
                        .takes_value(true),
                )
                .arg(
                    Arg::with_name("loan_id")
                        .long("loan-id")
                        .value_name("ID")
                        .help("Search by loan ID")
                        .takes_value(true),
                )
                .arg(
                    Arg::with_name("status")
                        .long("status")
                        .value_name("STATUS")
                        .help("Filter by loan status")
                        .takes_value(true),
                )
                .arg(
                    Arg::with_name("amount_min")
                        .long("min-amount")
                        .value_name("AMOUNT")
                        .help("Minimum loan amount")
                        .takes_value(true),
                )
                .arg(
                    Arg::with_name("amount_max")
                        .long("max-amount")
                        .