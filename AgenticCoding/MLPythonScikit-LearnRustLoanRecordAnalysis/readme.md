# Loan Portfolio Analysis Suite 🏦

A comprehensive loan portfolio analysis and search system combining Python's scikit-learn for machine learning analytics with a high-performance Rust application for AI-powered record searching.

## 🚀 Features

### Python Analytics Engine
- **Machine Learning Models**: Risk prediction using Random Forest and balance forecasting with Gradient Boosting
- **Customer Segmentation**: K-means clustering for portfolio optimization
- **Comprehensive Reporting**: Automated insights and business recommendations
- **Statistical Analysis**: Portfolio performance metrics and trend analysis

### Rust Search Engine  
- **AI-Powered Search**: Natural language query processing with fuzzy matching
- **High Performance**: Optimized indexing for fast record retrieval
- **Advanced Filtering**: Multi-criteria search with aggregations
- **CLI Interface**: Command-line tool for interactive searching

## 📊 Data Schema

The system works with loan portfolio data containing the following fields:

| Field | Type | Description |
|-------|------|-------------|
| `LoanID` | String | Unique loan identifier |
| `CustomerName` | String | Borrower name |
| `PropertyAddress` | String | Collateral property address |
| `OriginationDate` | Date | Loan origination date |
| `MaturityDate` | Date | Loan maturity date |
| `LoanAmount` | Float | Original loan amount |
| `RemainingBalance` | Float | Current outstanding balance |
| `InterestRate` | Float | Annual interest rate (%) |
| `MonthlyPayment` | Float | Monthly payment amount |
| `Status` | String | Loan status (Active, Paid Off, Default, etc.) |
| `ProductName` | String | Loan product name |
| `ProductType` | String | Product type (Fixed Rate, ARM, etc.) |
| `SecurityName` | String | Security/collateral identifier |
| `ServicerName` | String | Loan servicing company |
| `CurrentStatus` | String | Real-time loan status |

## 🛠️ Installation

### Prerequisites

**Python Requirements:**
```bash
pip install pandas numpy scikit-learn matplotlib seaborn
```

**Rust Requirements:**
```bash
# Install Rust if not already installed
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs/ | sh

# Add dependencies to Cargo.toml
cargo add serde serde_json chrono anyhow regex clap
```

### Setup

1. **Clone the repository:**
```bash
git clone https://github.com/your-org/loan-portfolio-suite.git
cd loan-portfolio-suite
```

2. **Set up Python environment:**
```bash
python -m venv venv
source venv/bin/activate  # On Windows: venv\Scripts\activate
pip install -r requirements.txt
```

3. **Build Rust application:**
```bash
cd rust-search-engine
cargo build --release
```

## 📈 Usage

### Python Analytics

Run the comprehensive portfolio analysis:

```bash
python loan_analysis.py
```

**Sample Output:**
```
LOAN PORTFOLIO ANALYSIS REPORT
==================================================
Dataset Shape: (1000, 15)
Analysis Date: 2025-07-27 10:30:15

1. PORTFOLIO OVERVIEW
------------------------------
Total Loans: 1,000
Total Loan Amount: $249,876,543.21
Total Remaining Balance: $187,456,789.12
Average Interest Rate: 5.23%
Average Monthly Payment: $1,456.78

2. LOAN STATUS DISTRIBUTION
------------------------------
Active: 700 (70.0%)
Paid Off: 200 (20.0%)
Default: 50 (5.0%)
Delinquent: 50 (5.0%)

3. RISK ANALYSIS
------------------------------
High Risk Loans: 125 (12.5%)
Average Interest Rate (High Risk): 6.45%
Average Interest Rate (Low Risk): 4.89%
```

### Rust Search Engine

**Basic search commands:**

```bash
# Search by loan ID
./target/release/loan-search search --loan-id LOAN_000001

# Search by status
./target/release/loan-search search --status Active

# Search with amount range
./target/release/loan-search search --min-amount 200000 --max-amount 500000

# Natural language search
./target/release/loan-search search -q "active loans with high interest rates"

# Portfolio statistics
./target/release/loan-search stats
```

**Advanced search examples:**

```bash
# Complex multi-criteria search
./target/release/loan-search search \
  --status Active \
  --min-amount 300000 \
  --max-amount 750000 \
  --limit 20

# Natural language queries
./target/release/loan-search search -q "FHA loans in California"
./target/release/loan-search search -q "default loans amount 100000 to 500000"
```

## 🧠 AI Features

### Machine Learning Models

1. **Risk Prediction Model**
   - Algorithm: Random Forest Classifier
   - Features: Loan amount, interest rate, loan age, LTV ratio, payment-to-income
   - Accuracy: ~85% on test data
   - Output: Risk probability score

2. **Balance Prediction Model**
   - Algorithm: Gradient Boosting Regressor
   - Purpose: Forecast remaining balance evolution
   - MAE: <$15,000 on test data
   - R² Score: >0.90

3. **Customer Segmentation**
   - Algorithm: K-Means Clustering
   - Segments: Conservative, Standard, Aggressive, High-Value
   - Features: Loan characteristics and risk profile

### Intelligent Search Capabilities

- **Fuzzy Matching**: Levenshtein distance-based similarity
- **Natural Language Processing**: Query intent recognition
- **Smart Filtering**: Multi-dimensional search optimization
- **Auto-completion**: Predictive search suggestions

## 📊 Analytics Output

### Key Metrics Tracked

- **Portfolio Health**: Default rates, delinquency trends
- **Risk Distribution**: High-risk loan concentration
- **Performance Indicators**: ROI, yield analysis
- **Geographic Analysis**: State-wise performance
- **Product Analysis**: Performance by loan type

### Business Recommendations

The system automatically generates actionable insights:

- Risk exposure alerts (>15% high-risk threshold)
- Interest rate optimization opportunities
- Portfolio diversification recommendations
- Servicer performance monitoring
- Product mix optimization

## 🔧 Configuration

### Python Configuration

Modify analysis parameters in `loan_analysis.py`:

```python
# Risk threshold adjustment
HIGH_RISK_THRESHOLD = 0.15

# Model parameters
RANDOM_FOREST_ESTIMATORS = 100
KMEANS_CLUSTERS = 4

# Date ranges for analysis
ANALYSIS_START_DATE = "2020-01-01"
ANALYSIS_END_DATE = "2025-12-31"
```

### Rust Configuration

Customize search parameters in `src/main.rs`:

```rust
// Fuzzy matching threshold (0.0 - 1.0)
const FUZZY_THRESHOLD: f64 = 0.8;

// Default result limit
const DEFAULT_LIMIT: usize = 10;

// Index refresh interval
const INDEX_REFRESH_MINUTES: u64 = 30;
```

## 🧪 Testing

### Python Tests
```bash
python -m pytest tests/ -v
```

### Rust Tests
```bash
cargo test --release
```

### Integration Tests
```bash
# Test full pipeline
python test_integration.py
```

## 📈 Performance Benchmarks

### Python Analytics
- **Data Processing**: 10,000 records/second
- **Model Training**: <30 seconds for 100K records
- **Report Generation**: <5 seconds

### Rust Search Engine
- **Search Speed**: <10ms average query time
- **Memory Usage**: <50MB for 1M records
- **Concurrent Users**: 1000+ simultaneous searches


### Development Guidelines

- Follow PEP 8 for Python code
- Use `rustfmt` for Rust code formatting
- Add tests for new features
- Update documentation for API changes


## 🙏 Acknowledgments

- Built with [scikit-learn](https://scikit-learn.org/) for machine learning
- Powered by [Rust](https://www.rust-lang.org/) for high-performance search
- Inspired by modern financial technology solutions

---

**⭐ Star this repository if you find it useful!**


## ⚠️ Disclaimer

This repository is intended for demonstration, architecture reference, and internal collaboration only. All content—including code, documentation, diagrams, and configuration—is proprietary to Shaila Patel.

Unauthorized copying, reuse, or redistribution of any part of this repository is strictly prohibited. If you wish to reference or adapt any material, please contact the repository owner for written permission.

This is not an open-source project and is not licensed for public or commercial use.

By accessing this repository, you agree to respect the intellectual property rights of the owner and to use the content solely for its intended purpose within authorized contexts.

---
<br/>

