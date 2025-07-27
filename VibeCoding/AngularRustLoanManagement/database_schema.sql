-- migrations/001_create_loans_table.sql
CREATE TABLE IF NOT EXISTS loans (
    loan_id VARCHAR(50) PRIMARY KEY,
    customer_name VARCHAR(255) NOT NULL,
    property_address TEXT NOT NULL,
    origination_date DATE NOT NULL,
    maturity_date DATE NOT NULL,
    loan_amount DECIMAL(15,2) NOT NULL,
    remaining_balance DECIMAL(15,2) NOT NULL,
    interest_rate DECIMAL(5,3) NOT NULL,
    monthly_payment DECIMAL(10,2) NOT NULL,
    status VARCHAR(50) NOT NULL,
    product_name VARCHAR(255) NOT NULL,
    product_type VARCHAR(100) NOT NULL,
    security_name VARCHAR(255) NOT NULL,
    servicer_name VARCHAR(255) NOT NULL,
    current_status VARCHAR(50) NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

-- Create indexes for better search performance
CREATE INDEX IF NOT EXISTS idx_loans_customer_name ON loans(customer_name);
CREATE INDEX IF NOT EXISTS idx_loans_status ON loans(status);
CREATE INDEX IF NOT EXISTS idx_loans_product_type ON loans(product_type);
CREATE INDEX IF NOT EXISTS idx_loans_servicer_name ON loans(servicer_name);
CREATE INDEX IF NOT EXISTS idx_loans_loan_amount ON loans(loan_amount);
CREATE INDEX IF NOT EXISTS idx_loans_origination_date ON loans(origination_date);
CREATE INDEX IF NOT EXISTS idx_loans_current_status ON loans(current_status);

-- Composite indexes for common search combinations
CREATE INDEX IF NOT EXISTS idx_loans_status_product_type ON loans(status, product_type);
CREATE INDEX IF NOT EXISTS idx_loans_servicer_status ON loans(servicer_name, status);

-- Sample data for testing
INSERT INTO loans (
    loan_id, customer_name, property_address, origination_date, maturity_date,
    loan_amount, remaining_balance, interest_rate, monthly_payment, status,
    product_name, product_type, security_name, servicer_name, current_status
) VALUES 
(
    'LN001', 'John Smith', '123 Main St, Anytown, CA 90210', '2020-01-15', '2050-01-15',
    350000.00, 325000.00, 3.250, 1520.25, 'Active',
    'Fixed Rate 30-Year Mortgage', 'Fixed Rate', 'MBS-2020-001', 'ABC Servicing', 'Current'
),
(
    'LN002', 'Jane Doe', '456 Oak Ave, Springfield, IL 62701', '2019-03-22', '2049-03-22',
    225000.00, 195000.00, 4.125, 1089.45, 'Active',
    'Fixed Rate 30-Year Mortgage', 'Fixed Rate', 'MBS-2019-045', 'XYZ Mortgage Co', 'Current'
),
(
    'LN003', 'Michael Johnson', '789 Pine Rd, Denver, CO 80202', '2018-07-10', '2025-07-10',
    180000.00, 25000.00, 5.750, 1508.32, 'Active',
    'Adjustable Rate 7/1 ARM', 'Adjustable Rate', 'ARM-2018-078', 'DEF Financial', 'Current'
),
(
    'LN004', 'Sarah Williams', '321 Elm St, Austin, TX 73301', '2021-11-05', '2051-11-05',
    425000.00, 415000.00, 2.875, 1764.89, 'Active',
    'Fixed Rate 30-Year Jumbo', 'Fixed Rate', 'JUMBO-2021-012', 'GHI Servicing', 'Current'
),
(
    'LN005', 'Robert Brown', '654 Maple Dr, Miami, FL 33101', '2017-05-18', '2024-05-18',
    95000.00, 8500.00, 6.250, 987.65, 'Active',
    'Interest Only 7-Year', 'Interest Only', 'IO-2017-023', 'JKL Mortgage', 'Current'
),
(
    'LN006', 'Emily Davis', '987 Cedar Ln, Portland, OR 97201', '2016-09-12', '2023-09-12',
    275000.00, 0.00, 4.500, 0.00, 'Paid Off',
    'Fixed Rate 15-Year Mortgage', 'Fixed Rate', 'MBS-2016-089', 'MNO Bank', 'Paid Off'
),
(
    'LN007', 'David Wilson', '147 Birch Way, Seattle, WA 98101', '2022-02-28', '2052-02-28',
    550000.00, 545000.00, 3.750, 2546.78, 'Delinquent',
    'Fixed Rate 30-Year Jumbo', 'Fixed Rate', 'JUMBO-2022-034', 'PQR Servicing', '30 Days Late'
),
(
    'LN008', 'Lisa Garcia', '258 Spruce St, Phoenix, AZ 85001', '2015-12-03', '2030-12-03',
    165000.00, 45000.00, 5.125, 1298.75, 'Active',
    'Adjustable Rate 15/1 ARM', 'Adjustable Rate', 'ARM-2015-156', 'STU Financial', 'Current'
),
(
    'LN009', 'Christopher Lee', '369 Willow Ave, Nashville, TN 37201', '2020-08-20', '2023-08-20',
    85000.00, 15000.00, 7.250, 1456.32, 'Default',
    'Interest Only 3-Year', 'Interest Only', 'IO-2020-087', 'VWX Mortgage', 'In Foreclosure'
),
(
    'LN010', 'Amanda Martinez', '741 Poplar Blvd, Atlanta, GA 30301', '2021-06-15', '2051-06-15',
    385000.00, 375000.00, 3.125, 1645.21, 'Active',
    'Fixed Rate 30-Year Mortgage', 'Fixed Rate', 'MBS-2021-067', 'YZA Servicing', 'Current'
);

-- Create a function to update the updated_at timestamp
CREATE OR REPLACE FUNCTION update_updated_at_column()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = CURRENT_TIMESTAMP;
    RETURN NEW;
END;
$$ language 'plpgsql';

-- Create trigger to automatically update updated_at
CREATE TRIGGER update_loans_updated_at BEFORE UPDATE ON loans
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();