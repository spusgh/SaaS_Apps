#!/usr/bin/env python3
"""
Loan Portfolio Analysis Report using Scikit-Learn
=================================================

This report analyzes loan portfolio data to provide insights on:
- Loan performance patterns
- Risk assessment
- Customer segmentation
- Predictive modeling for loan defaults

Schema:
- LoanID: Unique identifier for each loan
- CustomerName: Name of the borrower
- PropertyAddress: Address of the collateral property
- OriginationDate: Date when loan was originated
- MaturityDate: Date when loan matures
- LoanAmount: Original loan amount
- RemainingBalance: Current outstanding balance
- InterestRate: Annual interest rate
- MonthlyPayment: Monthly payment amount
- Status: Current loan status (Active, Paid Off, Default, etc.)
- ProductName: Name of the loan product
- ProductType: Type of loan product
- SecurityName: Security/collateral identifier
- ServicerName: Loan servicing company
- CurrentStatus: Real-time status of the loan
"""

import pandas as pd
import numpy as np
from sklearn.model_selection import train_test_split
from sklearn.ensemble import RandomForestClassifier, GradientBoostingRegressor
from sklearn.preprocessing import LabelEncoder, StandardScaler
from sklearn.metrics import classification_report, confusion_matrix, mean_absolute_error, r2_score
from sklearn.cluster import KMeans
import matplotlib.pyplot as plt
import seaborn as sns
from datetime import datetime, timedelta
import warnings
warnings.filterwarnings('ignore')

class LoanPortfolioAnalyzer:
    def __init__(self):
        self.le_dict = {}
        self.scaler = StandardScaler()
        self.risk_model = None
        self.balance_predictor = None
        
    def generate_sample_data(self, n_samples=1000):
        """Generate sample loan data for demonstration"""
        np.random.seed(42)
        
        # Sample data generation
        loan_ids = [f"LOAN_{str(i).zfill(6)}" for i in range(1, n_samples + 1)]
        customer_names = [f"Customer_{i}" for i in range(1, n_samples + 1)]
        
        # Property addresses
        states = ['CA', 'TX', 'FL', 'NY', 'IL', 'PA', 'OH', 'GA', 'NC', 'MI']
        property_addresses = [f"{np.random.randint(100, 9999)} {np.random.choice(['Main St', 'Oak Ave', 'Park Blvd', 'First St'])}, {np.random.choice(states)}" for _ in range(n_samples)]
        
        # Dates
        base_date = datetime(2020, 1, 1)
        origination_dates = [base_date + timedelta(days=np.random.randint(0, 1095)) for _ in range(n_samples)]
        maturity_dates = [orig_date + timedelta(days=np.random.randint(3650, 10950)) for orig_date in origination_dates]
        
        # Financial data
        loan_amounts = np.random.normal(250000, 100000, n_samples)
        loan_amounts = np.clip(loan_amounts, 50000, 1000000)
        
        remaining_balances = loan_amounts * np.random.uniform(0.1, 0.95, n_samples)
        interest_rates = np.random.uniform(2.5, 8.5, n_samples)
        monthly_payments = (loan_amounts * (interest_rates/100/12)) / (1 - (1 + interest_rates/100/12)**(-360))
        
        # Status and product information
        statuses = np.random.choice(['Active', 'Paid Off', 'Default', 'Delinquent'], n_samples, p=[0.7, 0.2, 0.05, 0.05])
        product_names = np.random.choice(['Prime Mortgage', 'Jumbo Loan', 'FHA Loan', 'VA Loan', 'Conventional'], n_samples)
        product_types = np.random.choice(['Fixed Rate', 'Adjustable Rate', 'Interest Only'], n_samples)
        security_names = [f"SEC_{str(i).zfill(4)}" for i in np.random.randint(1, 100, n_samples)]
        servicer_names = np.random.choice(['ServiceCorp A', 'ServiceCorp B', 'ServiceCorp C', 'ServiceCorp D'], n_samples)
        current_statuses = statuses  # Assuming same as Status for simplicity
        
        return pd.DataFrame({
            'LoanID': loan_ids,
            'CustomerName': customer_names,
            'PropertyAddress': property_addresses,
            'OriginationDate': origination_dates,
            'MaturityDate': maturity_dates,
            'LoanAmount': loan_amounts,
            'RemainingBalance': remaining_balances,
            'InterestRate': interest_rates,
            'MonthlyPayment': monthly_payments,
            'Status': statuses,
            'ProductName': product_names,
            'ProductType': product_types,
            'SecurityName': security_names,
            'ServicerName': servicer_names,
            'CurrentStatus': current_statuses
        })
    
    def preprocess_data(self, df):
        """Preprocess the loan data for analysis"""
        # Create derived features
        df['LoanAge'] = (datetime.now() - pd.to_datetime(df['OriginationDate'])).dt.days
        df['TimeToMaturity'] = (pd.to_datetime(df['MaturityDate']) - datetime.now()).dt.days
        df['LoanToValue'] = df['RemainingBalance'] / df['LoanAmount']
        df['PaymentToIncome'] = df['MonthlyPayment'] / (df['LoanAmount'] * 0.01)  # Estimated income proxy
        
        # Risk indicators
        df['HighRisk'] = ((df['Status'].isin(['Default', 'Delinquent'])) | 
                         (df['InterestRate'] > 6.0) | 
                         (df['LoanToValue'] > 0.8)).astype(int)
        
        return df
    
    def perform_eda(self, df):
        """Perform Exploratory Data Analysis"""
        print("LOAN PORTFOLIO ANALYSIS REPORT")
        print("=" * 50)
        print(f"Dataset Shape: {df.shape}")
        print(f"Analysis Date: {datetime.now().strftime('%Y-%m-%d %H:%M:%S')}")
        print("\n")
        
        # Basic statistics
        print("1. PORTFOLIO OVERVIEW")
        print("-" * 30)
        print(f"Total Loans: {len(df):,}")
        print(f"Total Loan Amount: ${df['LoanAmount'].sum():,.2f}")
        print(f"Total Remaining Balance: ${df['RemainingBalance'].sum():,.2f}")
        print(f"Average Interest Rate: {df['InterestRate'].mean():.2f}%")
        print(f"Average Monthly Payment: ${df['MonthlyPayment'].mean():,.2f}")
        
        # Status distribution
        print("\n2. LOAN STATUS DISTRIBUTION")
        print("-" * 30)
        status_counts = df['Status'].value_counts()
        for status, count in status_counts.items():
            percentage = (count / len(df)) * 100
            print(f"{status}: {count:,} ({percentage:.1f}%)")
        
        # Risk analysis
        print("\n3. RISK ANALYSIS")
        print("-" * 30)
        high_risk_count = df['HighRisk'].sum()
        print(f"High Risk Loans: {high_risk_count:,} ({(high_risk_count/len(df)*100):.1f}%)")
        print(f"Average Interest Rate (High Risk): {df[df['HighRisk']==1]['InterestRate'].mean():.2f}%")
        print(f"Average Interest Rate (Low Risk): {df[df['HighRisk']==0]['InterestRate'].mean():.2f}%")
        
        # Product analysis
        print("\n4. PRODUCT ANALYSIS")
        print("-" * 30)
        product_summary = df.groupby('ProductName').agg({
            'LoanAmount': ['count', 'sum', 'mean'],
            'InterestRate': 'mean',
            'HighRisk': 'mean'
        }).round(2)
        print(product_summary)
        
        return df
    
    def build_risk_model(self, df):
        """Build a risk prediction model using Random Forest"""
        # Encode categorical variables
        categorical_cols = ['ProductName', 'ProductType', 'ServicerName']
        df_encoded = df.copy()
        
        for col in categorical_cols:
            le = LabelEncoder()
            df_encoded[col + '_encoded'] = le.fit_transform(df_encoded[col])
            self.le_dict[col] = le
        
        # Select features for modeling
        feature_cols = ['LoanAmount', 'InterestRate', 'LoanAge', 'TimeToMaturity', 
                       'LoanToValue', 'PaymentToIncome'] + [col + '_encoded' for col in categorical_cols]
        
        X = df_encoded[feature_cols]
        y = df_encoded['HighRisk']
        
        # Split data
        X_train, X_test, y_train, y_test = train_test_split(X, y, test_size=0.2, random_state=42)
        
        # Train model
        self.risk_model = RandomForestClassifier(n_estimators=100, random_state=42)
        self.risk_model.fit(X_train, y_train)
        
        # Evaluate model
        y_pred = self.risk_model.predict(X_test)
        
        print("\n5. RISK PREDICTION MODEL PERFORMANCE")
        print("-" * 40)
        print("Classification Report:")
        print(classification_report(y_test, y_pred))
        
        # Feature importance
        feature_importance = pd.DataFrame({
            'feature': feature_cols,
            'importance': self.risk_model.feature_importances_
        }).sort_values('importance', ascending=False)
        
        print("\nFeature Importance:")
        for _, row in feature_importance.head(5).iterrows():
            print(f"{row['feature']}: {row['importance']:.3f}")
        
        return X_test, y_test, y_pred
    
    def build_balance_predictor(self, df):
        """Build remaining balance prediction model"""
        categorical_cols = ['ProductName', 'ProductType', 'ServicerName']
        df_encoded = df.copy()
        
        for col in categorical_cols:
            if col not in self.le_dict:
                le = LabelEncoder()
                df_encoded[col + '_encoded'] = le.fit_transform(df_encoded[col])
                self.le_dict[col] = le
            else:
                df_encoded[col + '_encoded'] = self.le_dict[col].transform(df_encoded[col])
        
        feature_cols = ['LoanAmount', 'InterestRate', 'LoanAge', 'MonthlyPayment'] + [col + '_encoded' for col in categorical_cols]
        
        X = df_encoded[feature_cols]
        y = df_encoded['RemainingBalance']
        
        X_train, X_test, y_train, y_test = train_test_split(X, y, test_size=0.2, random_state=42)
        
        self.balance_predictor = GradientBoostingRegressor(n_estimators=100, random_state=42)
        self.balance_predictor.fit(X_train, y_train)
        
        y_pred = self.balance_predictor.predict(X_test)
        
        print("\n6. REMAINING BALANCE PREDICTION MODEL")
        print("-" * 40)
        print(f"Mean Absolute Error: ${mean_absolute_error(y_test, y_pred):,.2f}")
        print(f"R² Score: {r2_score(y_test, y_pred):.3f}")
        
        return X_test, y_test, y_pred
    
    def customer_segmentation(self, df):
        """Perform customer segmentation using K-Means clustering"""
        # Prepare features for clustering
        features = ['LoanAmount', 'InterestRate', 'LoanToValue', 'PaymentToIncome']
        X = df[features].fillna(df[features].mean())
        
        # Standardize features
        X_scaled = self.scaler.fit_transform(X)
        
        # Perform K-Means clustering
        kmeans = KMeans(n_clusters=4, random_state=42)
        df['Segment'] = kmeans.fit_predict(X_scaled)
        
        print("\n7. CUSTOMER SEGMENTATION")
        print("-" * 30)
        segment_summary = df.groupby('Segment').agg({
            'LoanAmount': 'mean',
            'InterestRate': 'mean',
            'LoanToValue': 'mean',
            'HighRisk': 'mean',
            'LoanID': 'count'
        }).round(2)
        
        segment_labels = ['Conservative', 'Standard', 'Aggressive', 'High-Value']
        segment_summary.index = segment_labels
        
        print("Segment Characteristics:")
        print(segment_summary)
        
        return df
    
    def generate_recommendations(self, df):
        """Generate business recommendations based on analysis"""
        print("\n8. BUSINESS RECOMMENDATIONS")
        print("-" * 35)
        
        # Risk-based recommendations
        high_risk_rate = df['HighRisk'].mean()
        if high_risk_rate > 0.15:
            print("• HIGH PRIORITY: Risk exposure above 15%. Implement stricter underwriting criteria.")
        
        # Interest rate optimization
        avg_rate = df['InterestRate'].mean()
        market_rate = 5.5  # Assumed market rate
        if avg_rate < market_rate:
            print(f"• OPPORTUNITY: Average rate ({avg_rate:.2f}%) below market. Consider rate adjustments.")
        
        # Portfolio concentration
        top_servicer_pct = df['ServicerName'].value_counts().iloc[0] / len(df)
        if top_servicer_pct > 0.4:
            print("• RISK: High concentration with single servicer. Diversify servicing relationships.")
        
        # Product performance
        product_risk = df.groupby('ProductName')['HighRisk'].mean()
        high_risk_products = product_risk[product_risk > 0.2].index.tolist()
        if high_risk_products:
            print(f"• REVIEW: High-risk products identified: {', '.join(high_risk_products)}")
        
        print("\n9. MONITORING RECOMMENDATIONS")
        print("-" * 35)
        print("• Implement monthly risk scoring updates")
        print("• Monitor interest rate sensitivity")
        print("• Track servicer performance metrics")
        print("• Review product mix quarterly")
        
def main():
    """Main analysis function"""
    analyzer = LoanPortfolioAnalyzer()
    
    # Generate sample data (replace with actual data loading)
    print("Loading loan portfolio data...")
    df = analyzer.generate_sample_data(1000)
    
    # Preprocess data
    df = analyzer.preprocess_data(df)
    
    # Perform analysis
    df = analyzer.perform_eda(df)
    analyzer.build_risk_model(df)
    analyzer.build_balance_predictor(df)
    df = analyzer.customer_segmentation(df)
    analyzer.generate_recommendations(df)
    
    print("\n" + "="*50)
    print("ANALYSIS COMPLETE")
    print("="*50)
    
    return analyzer, df

if __name__ == "__main__":
    analyzer, df = main()
