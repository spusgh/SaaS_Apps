// Loan Search AI Application
// Readonly search functionality for loan database records

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt;
use chrono::{DateTime, NaiveDate, Utc};
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

impl fmt::Display for LoanRecord {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, 
            "Loan ID: {} | Customer: {} | Amount: ${:.2} | Status: {} | Rate: {:.2}%",
            self.loan_id, self.customer_name, self.loan_amount, self.status, self.interest_rate
        )
    }
}

#[derive(Debug, Clone)]
pub enum SearchCriteria {
    LoanId(String),
    CustomerName(String),
    AmountRange(f64, f64),
    InterestRateRange(f64, f64),
    Status(String),
    ProductType(String),
    ServicerName(String),
    OriginationDateRange(NaiveDate, NaiveDate),
    PropertyAddress(String),
}

#[derive(Debug, Clone)]
pub struct SearchQuery {
    pub criteria: Vec<SearchCriteria>,
    pub limit: Option<usize>,
    pub sort_by: Option<SortField>,
    pub sort_order: SortOrder,
}

#[derive(Debug, Clone)]
pub enum SortField {
    LoanAmount,
    InterestRate,
    OriginationDate,
    RemainingBalance,
    CustomerName,
}

#[derive(Debug, Clone)]
pub enum SortOrder {
    Ascending,
    Descending,
}

pub struct LoanSearchEngine {
    records: Vec<LoanRecord>,
    index: HashMap<String, Vec<usize>>, // Simple indexing for faster searches
}

impl LoanSearchEngine {
    pub fn new() -> Self {
        Self {
            records: Vec::new(),
            index: HashMap::new(),
        }
    }

    // Load loan records from a data source (mock implementation)
    pub fn load_records(&mut self, records: Vec<LoanRecord>) {
        self.records = records;
        self.build_index();
    }

    // Build simple index for common search fields
    fn build_index(&mut self) {
        self.index.clear();
        
        for (idx, record) in self.records.iter().enumerate() {
            // Index by loan ID
            self.index.entry(format!("loan_id:{}", record.loan_id.to_lowercase()))
                .or_insert_with(Vec::new)
                .push(idx);
            
            // Index by customer name
            self.index.entry(format!("customer:{}", record.customer_name.to_lowercase()))
                .or_insert_with(Vec::new)
                .push(idx);
            
            // Index by status
            self.index.entry(format!("status:{}", record.status.to_lowercase()))
                .or_insert_with(Vec::new)
                .push(idx);
            
            // Index by product type
            self.index.entry(format!("product_type:{}", record.product_type.to_lowercase()))
                .or_insert_with(Vec::new)
                .push(idx);
        }
    }

    // Main search function
    pub fn search(&self, query: &SearchQuery) -> SearchResult {
        let mut matching_indices: Vec<usize> = (0..self.records.len()).collect();

        // Apply each search criterion
        for criterion in &query.criteria {
            matching_indices = self.filter_by_criterion(&matching_indices, criterion);
        }

        // Get the actual records
        let mut results: Vec<LoanRecord> = matching_indices
            .iter()
            .map(|&idx| self.records[idx].clone())
            .collect();

        // Sort results
        if let Some(sort_field) = &query.sort_by {
            self.sort_results(&mut results, sort_field, &query.sort_order);
        }

        // Apply limit
        if let Some(limit) = query.limit {
            results.truncate(limit);
        }

        SearchResult {
            records: results,
            total_matches: matching_indices.len(),
            query_time_ms: 0, // In a real implementation, this would be measured
        }
    }

    fn filter_by_criterion(&self, indices: &[usize], criterion: &SearchCriteria) -> Vec<usize> {
        indices
            .iter()
            .filter(|&&idx| self.matches_criterion(&self.records[idx], criterion))
            .copied()
            .collect()
    }

    fn matches_criterion(&self, record: &LoanRecord, criterion: &SearchCriteria) -> bool {
        match criterion {
            SearchCriteria::LoanId(id) => {
                record.loan_id.to_lowercase().contains(&id.to_lowercase())
            }
            SearchCriteria::CustomerName(name) => {
                record.customer_name.to_lowercase().contains(&name.to_lowercase())
            }
            SearchCriteria::AmountRange(min, max) => {
                record.loan_amount >= *min && record.loan_amount <= *max
            }
            SearchCriteria::InterestRateRange(min, max) => {
                record.interest_rate >= *min && record.interest_rate <= *max
            }
            SearchCriteria::Status(status) => {
                record.status.to_lowercase() == status.to_lowercase()
            }
            SearchCriteria::ProductType(product_type) => {
                record.product_type.to_lowercase() == product_type.to_lowercase()
            }
            SearchCriteria::ServicerName(servicer) => {
                record.servicer_name.to_lowercase().contains(&servicer.to_lowercase())
            }
            SearchCriteria::OriginationDateRange(start, end) => {
                record.origination_date >= *start && record.origination_date <= *end
            }
            SearchCriteria::PropertyAddress(address) => {
                record.property_address.to_lowercase().contains(&address.to_lowercase())
            }
        }
    }

    fn sort_results(&self, results: &mut [LoanRecord], sort_field: &SortField, order: &SortOrder) {
        results.sort_by(|a, b| {
            let comparison = match sort_field {
                SortField::LoanAmount => a.loan_amount.partial_cmp(&b.loan_amount).unwrap(),
                SortField::InterestRate => a.interest_rate.partial_cmp(&b.interest_rate).unwrap(),
                SortField::OriginationDate => a.origination_date.cmp(&b.origination_date),
                SortField::RemainingBalance => a.remaining_balance.partial_cmp(&b.remaining_balance).unwrap(),
                SortField::CustomerName => a.customer_name.cmp(&b.customer_name),
            };

            match order {
                SortOrder::Ascending => comparison,
                SortOrder::Descending => comparison.reverse(),
            }
        });
    }

    // AI-powered intelligent search suggestions
    pub fn suggest_searches(&self, input: &str) -> Vec<String> {
        let mut suggestions = Vec::new();
        let input_lower = input.to_lowercase();

        // Suggest based on common patterns
        if input_lower.contains("default") || input_lower.contains("delinquent") {
            suggestions.push("status:Default".to_string());
            suggestions.push("status:90+ Days Late".to_string());
        }

        if input_lower.contains("high") && input_lower.contains("amount") {
            suggestions.push("amount_range:1000000-5000000".to_string());
        }

        if input_lower.contains("recent") {
            suggestions.push("origination_date:2023-01-01 to 2024-12-31".to_string());
        }

        // Fuzzy matching for loan IDs
        if input.len() >= 3 {
            for record in &self.records {
                if record.loan_id.to_lowercase().contains(&input_lower) {
                    suggestions.push(format!("loan_id:{}", record.loan_id));
                    if suggestions.len() >= 5 { break; }
                }
            }
        }

        suggestions
    }

    // Analytics functions
    pub fn get_portfolio_stats(&self) -> PortfolioStats {
        let total_loans = self.records.len();
        let total_amount: f64 = self.records.iter().map(|r| r.loan_amount).sum();
        let total_remaining: f64 = self.records.iter().map(|r| r.remaining_balance).sum();
        let avg_interest_rate: f64 = self.records.iter().map(|r| r.interest_rate).sum() / total_loans as f64;

        let mut status_counts = HashMap::new();
        for record in &self.records {
            *status_counts.entry(record.status.clone()).or_insert(0) += 1;
        }

        PortfolioStats {
            total_loans,
            total_amount,
            total_remaining_balance: total_remaining,
            average_interest_rate: avg_interest_rate,
            status_distribution: status_counts,
        }
    }
}

#[derive(Debug)]
pub struct SearchResult {
    pub records: Vec<LoanRecord>,
    pub total_matches: usize,
    pub query_time_ms: u64,
}

#[derive(Debug)]
pub struct PortfolioStats {
    pub total_loans: usize,
    pub total_amount: f64,
    pub total_remaining_balance: f64,
    pub average_interest_rate: f64,
    pub status_distribution: HashMap<String, i32>,
}

// AI Search Interface
pub struct AISearchInterface {
    engine: LoanSearchEngine,
}

impl AISearchInterface {
    pub fn new(engine: LoanSearchEngine) -> Self {
        Self { engine }
    }

    // Natural language query processing
    pub fn process_natural_query(&self, query: &str) -> SearchQuery {
        let mut criteria = Vec::new();
        let query_lower = query.to_lowercase();

        // Simple NLP-like parsing
        if let Some(captures) = Regex::new(r"loan\s+(\w+)").unwrap().captures(&query_lower) {
            if let Some(loan_id) = captures.get(1) {
                criteria.push(SearchCriteria::LoanId(loan_id.as_str().to_string()));
            }
        }

        if query_lower.contains("default") {
            criteria.push(SearchCriteria::Status("Default".to_string()));
        }

        if query_lower.contains("current") && !query_lower.contains("current status") {
            criteria.push(SearchCriteria::Status("Current".to_string()));
        }

        if let Some(captures) = Regex::new(r"amount.*?(\d+).*?(\d+)").unwrap().captures(&query_lower) {
            if let (Some(min), Some(max)) = (captures.get(1), captures.get(2)) {
                if let (Ok(min_val), Ok(max_val)) = (min.as_str().parse::<f64>(), max.as_str().parse::<f64>()) {
                    criteria.push(SearchCriteria::AmountRange(min_val, max_val));
                }
            }
        }

        // Extract customer name patterns
        if let Some(captures) = Regex::new(r"customer\s+(.+?)(?:\s|$)").unwrap().captures(&query_lower) {
            if let Some(customer) = captures.get(1) {
                criteria.push(SearchCriteria::CustomerName(customer.as_str().to_string()));
            }
        }

        SearchQuery {
            criteria,
            limit: Some(100), // Default limit
            sort_by: Some(SortField::LoanAmount),
            sort_order: SortOrder::Descending,
        }
    }

    pub fn search_with_ai(&self, natural_query: &str) -> SearchResult {
        let structured_query = self.process_natural_query(natural_query);
        self.engine.search(&structured_query)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::NaiveDate;

    fn create_sample_data() -> Vec<LoanRecord> {
        vec![
            LoanRecord {
                loan_id: "LN000001".to_string(),
                customer_name: "John Doe".to_string(),
                property_address: "123 Main St, Springfield".to_string(),
                origination_date: NaiveDate::from_ymd_opt(2023, 1, 15).unwrap(),
                maturity_date: NaiveDate::from_ymd_opt(2053, 1, 15).unwrap(),
                loan_amount: 350000.0,
                remaining_balance: 340000.0,
                interest_rate: 4.5,
                monthly_payment: 1773.0,
                status: "Current".to_string(),
                product_name: "30-Year Fixed".to_string(),
                product_type: "Conventional".to_string(),
                security_name: "MBS-2023-A".to_string(),
                servicer_name: "Premier Servicing".to_string(),
                current_status: "Active".to_string(),
            },
            LoanRecord {
                loan_id: "LN000002".to_string(),
                customer_name: "Jane Smith".to_string(),
                property_address: "456 Oak Ave, Franklin".to_string(),
                origination_date: NaiveDate::from_ymd_opt(2022, 6, 10).unwrap(),
                maturity_date: NaiveDate::from_ymd_opt(2052, 6, 10).unwrap(),
                loan_amount: 500000.0,
                remaining_balance: 480000.0,
                interest_rate: 3.8,
                monthly_payment: 2337.0,
                status: "30 Days Late".to_string(),
                product_name: "30-Year Fixed".to_string(),
                product_type: "Jumbo".to_string(),
                security_name: "MBS-2022-B".to_string(),
                servicer_name: "National Loan Services".to_string(),
                current_status: "Active".to_string(),
            },
        ]
    }

    #[test]
    fn test_basic_search() {
        let mut engine = LoanSearchEngine::new();
        engine.load_records(create_sample_data());

        let query = SearchQuery {
            criteria: vec![SearchCriteria::Status("Current".to_string())],
            limit: None,
            sort_by: None,
            sort_order: SortOrder::Ascending,
        };

        let results = engine.search(&query);
        assert_eq!(results.records.len(), 1);
        assert_eq!(results.records[0].customer_name, "John Doe");
    }

    #[test]
    fn test_amount_range_search() {
        let mut engine = LoanSearchEngine::new();
        engine.load_records(create_sample_data());

        let query = SearchQuery {
            criteria: vec![SearchCriteria::AmountRange(400000.0, 600000.0)],
            limit: None,
            sort_by: None,
            sort_order: SortOrder::Ascending,
        };

        let results = engine.search(&query);
        assert_eq!(results.records.len(), 1);
        assert_eq!(results.records[0].customer_name, "Jane Smith");
    }

    #[test]
    fn test_ai_natural_query() {
        let mut engine = LoanSearchEngine::new();
        engine.load_records(create_sample_data());
        let ai_interface = AISearchInterface::new(engine);

        let results = ai_interface.search_with_ai("show me current loans");
        assert!(results.records.len() > 0);
    }
}

// Main function for demonstration
fn main() {
    println!("Loan Search AI Application");
    println!("==========================");

    // Initialize the search engine
    let mut engine = LoanSearchEngine::new();
    
    // In a real application, you would load data from a database
    // For demo purposes, we'll create some sample data
    let sample_data = vec![
        LoanRecord {
            loan_id: "LN000001".to_string(),
            customer_name: "Alice Johnson".to_string(),
            property_address: "789 Pine Rd, Georgetown".to_string(),
            origination_date: NaiveDate::from_ymd_opt(2023, 3, 20).unwrap(),
            maturity_date: NaiveDate::from_ymd_opt(2053, 3, 20).unwrap(),
            loan_amount: 425000.0,
            remaining_balance: 415000.0,
            interest_rate: 4.2,
            monthly_payment: 2085.0,
            status: "Current".to_string(),
            product_name: "30-Year Fixed".to_string(),
            product_type: "Conventional".to_string(),
            security_name: "MBS-2023-C".to_string(),
            servicer_name: "Metro Mortgage".to_string(),
            current_status: "Active".to_string(),
        },
    ];

    engine.load_records(sample_data);

    // Demo search
    let query = SearchQuery {
        criteria: vec![SearchCriteria::Status("Current".to_string())],
        limit: Some(10),
        sort_by: Some(SortField::LoanAmount),
        sort_order: SortOrder::Descending,
    };

    let results = engine.search(&query);
    println!("Search Results:");
    for record in results.records {
        println!("  {}", record);
    }

    // Demo AI interface
    let ai_interface = AISearchInterface::new(engine);
    let ai_results = ai_interface.search_with_ai("show me loans with current status");
    println!("\nAI Search Results:");
    for record in ai_results.records {
        println!("  {}", record);
    }

    println!("\nApplication ready for loan record searches!");
}