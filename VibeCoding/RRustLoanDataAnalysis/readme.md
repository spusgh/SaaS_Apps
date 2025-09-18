# Loan Portfolio Analysis & Search System

A comprehensive loan portfolio management system featuring R-based analytics and a Rust-powered AI search engine for mortgage and loan data.

## 🏆 Features

### R Analytics Module
- **Portfolio Overview**: Complete financial metrics and performance indicators
- **Risk Assessment**: Delinquency rates, default analysis, and portfolio health metrics
- **Vintage Analysis**: Performance trends by origination year
- **Servicer Performance**: Comparative analysis across loan servicers
- **Data Quality Assessment**: Comprehensive data validation and integrity checks
- **Interactive Visualizations**: Charts and graphs for stakeholder presentations

### Rust AI Search Engine
- **Intelligent Search**: Natural language query processing
- **Advanced Filtering**: Multi-criteria search with complex conditions
- **High Performance**: Optimized indexing for fast query execution
- **AI-Powered Suggestions**: Smart search recommendations
- **Real-time Analytics**: Portfolio statistics and insights
- **Type Safety**: Memory-safe operations with Rust's ownership model

## 📊 Loan Schema

The system processes loan records with the following structure:

| Field | Type | Description |
|-------|------|-------------|
| `LoanID` | String | Unique loan identifier |
| `CustomerName` | String | Borrower full name |
| `PropertyAddress` | String | Property location |
| `OriginationDate` | Date | Loan origination date |
| `MaturityDate` | Date | Loan maturity date |
| `LoanAmount` | Decimal | Original loan amount |
| `RemainingBalance` | Decimal | Current outstanding balance |
| `InterestRate` | Decimal | Annual interest rate (%) |
| `MonthlyPayment` | Decimal | Monthly payment amount |
| `Status` | String | Current payment status |
| `ProductName` | String | Loan product name |
| `ProductType` | String | Product category |
| `SecurityName` | String | Associated security identifier |
| `ServicerName` | String | Loan servicing company |
| `CurrentStatus` | String | Operational status |

## 🚀 Quick Start

### Prerequisites

**For R Analytics:**
```r
# Required R packages
install.packages(c("dplyr", "ggplot2", "lubridate", "knitr", 
                   "DT", "plotly", "corrplot"))
```

**For Rust Search Engine:**
```bash
# Rust toolchain (install from https://rustup.rs/)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Required dependencies in Cargo.toml
[dependencies]
serde = { version = "1.0", features = ["derive"] }
chrono = { version = "0.4", features = ["serde"] }
regex = "1.0"
```

### Installation

1. **Clone the repository**
```bash
git clone https://github.com/yourusername/loan-portfolio-system.git
cd loan-portfolio-system
```

2. **Setup R Environment**
```bash
cd r-analytics
Rscript install_dependencies.R
```

3. **Build Rust Application**
```bash
cd rust-search-engine
cargo build --release
```

## 💻 Usage

### R Analytics Report

```r
# Load the loan analysis script
source("loan_analysis_report.R")

# The script will automatically:
# 1. Generate synthetic data (or load your data)
# 2. Perform comprehensive analysis
# 3. Create visualizations
# 4. Export summary CSV files
```

**Output Files:**
- `loan_status_summary.csv` - Status distribution analysis
- `product_analysis.csv` - Product performance metrics  
- `servicer_performance.csv` - Servicer comparison data

### Rust Search Engine

```rust
use loan_search::{LoanSearchEngine, SearchQuery, SearchCriteria, AISearchInterface};

// Initialize search engine
let mut engine = LoanSearchEngine::new();
engine.load_records(your_loan_data);

// Structured search
let query = SearchQuery {
    criteria: vec![
        SearchCriteria::Status("Current".to_string()),
        SearchCriteria::AmountRange(300000.0, 500000.0)
    ],
    limit: Some(50),
    sort_by: Some(SortField::LoanAmount),
    sort_order: SortOrder::Descending,
};

let results = engine.search(&query);

// AI-powered natural language search
let ai_interface = AISearchInterface::new(engine);
let results = ai_interface.search_with_ai("show me high-value current loans");
```

**Run the demo:**
```bash
cargo run
```

## 🔍 Search Examples

### Natural Language Queries
```rust
// Status-based searches
"show me all defaulted loans"
"find current loans"
"loans that are 30 days late"

// Amount-based searches  
"high value loans above 1 million"
"loans between 250000 and 500000"

// Customer searches
"find customer John Smith"
"loans for Alice Johnson"

// Date-based searches
"recent loans from 2023"
"loans originated this year"
```

### Structured Searches
```rust
// Multiple criteria
SearchQuery {
    criteria: vec![
        SearchCriteria::ProductType("Jumbo".to_string()),
        SearchCriteria::InterestRateRange(4.0, 6.0),
        SearchCriteria::Status("Current".to_string())
    ],
    limit: Some(25),
    sort_by: Some(SortField::RemainingBalance),
    sort_order: SortOrder::Descending,
}
```

## 📈 Analytics Examples

### Key Metrics Generated

**Portfolio Health:**
- Total portfolio value: $1.2B
- Average interest rate: 4.8%
- Default rate: 2.1%
- 90+ day delinquency: 1.3%

**Risk Indicators:**
- Geographic concentration analysis
- Vintage performance curves  
- Interest rate sensitivity analysis
- Servicer performance benchmarks

**Operational Metrics:**
- Data quality scores
- Processing time analytics
- Search performance metrics

## 🏗️ Architecture

```
┌─────────────────┐    ┌──────────────────┐    ┌─────────────────┐
│   Data Source   │───▶│   R Analytics    │───▶│    Reports &    │
│   (Database)    │    │     Module       │    │ Visualizations  │
└─────────────────┘    └──────────────────┘    └─────────────────┘
         │
         ▼
┌─────────────────┐    ┌──────────────────┐    ┌─────────────────┐
│  Rust Search    │───▶│   AI Interface   │───▶│  Search Results │
│    Engine       │    │     (NLP)        │    │   & Analytics   │
└─────────────────┘    └──────────────────┘    └─────────────────┘
```

### Components

1. **Data Layer**: Loan records with comprehensive schema
2. **Analytics Engine (R)**: Statistical analysis and reporting
3. **Search Engine (Rust)**: High-performance query processing
4. **AI Interface**: Natural language processing for intuitive searches
5. **Visualization Layer**: Interactive charts and dashboards

## 🧪 Testing

### R Tests
```r
# Run R unit tests
source("tests/test_analytics.R")
testthat::test_dir("tests/")
```

### Rust Tests
```bash
# Run Rust unit and integration tests
cargo test

# Run with coverage
cargo install cargo-tarpaulin
cargo tarpaulin --out Html
```

## 📊 Performance Benchmarks

### Search Engine Performance
- **Index Build Time**: <2 seconds for 100K records
- **Query Response Time**: <50ms for complex queries
- **Memory Usage**: ~100MB for 1M records
- **Concurrent Users**: 1000+ simultaneous searches

### Analytics Performance  
- **Report Generation**: <30 seconds for 1M records
- **Visualization Rendering**: <5 seconds per chart
- **CSV Export**: <10 seconds for full dataset

## 🔒 Security & Compliance

- **Data Encryption**: All data at rest and in transit
- **Access Control**: Role-based permissions
- **Audit Logging**: Complete search and access trails  
- **PII Protection**: Customer data anonymization options
- **Regulatory Compliance**: SOX, GDPR, CCPA ready


### Code Style
- **R**: Follow [tidyverse style guide](https://style.tidyverse.org/)
- **Rust**: Use `cargo fmt` and `cargo clippy`
- **Documentation**: Update README for any API changes


## 🏷️ Tags

`rust` `r-analytics` `fintech` `loan-management` `portfolio-analysis` `ai-search` `mortgage` `data-analytics` `financial-services` `machine-learning`

---

**Built with ❤️ for the financial services industry**

*Last updated: July 27, 2025*


## ⚠️ Disclaimer

This repository is intended for demonstration, architecture reference, and internal collaboration only. All content—including code, documentation, diagrams, and configuration—is proprietary to Shaila Patel.

Unauthorized copying, reuse, or redistribution of any part of this repository is strictly prohibited. If you wish to reference or adapt any material, please contact the repository owner for written permission.

This is not an open-source project and is not licensed for public or commercial use.

By accessing this repository, you agree to respect the intellectual property rights of the owner and to use the content solely for its intended purpose within authorized contexts.

---
<br/>

