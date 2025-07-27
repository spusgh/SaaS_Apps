# Loan GraphQL API

A comprehensive GraphQL API for searching and filtering loan records with advanced querying capabilities. This read-only API provides powerful search functionality for mortgage and loan data management systems.

## 🚀 Features

- **Comprehensive Search**: Search loans by multiple criteria including customer name, loan ID, status, and more
- **Advanced Filtering**: Filter by loan amount ranges, interest rates, product types, and servicers
- **Sorting & Pagination**: Sort results by any field with configurable pagination
- **Statistics**: Get aggregated loan statistics and breakdowns
- **GraphQL Playground**: Interactive API explorer for testing queries
- **Type Safety**: Fully typed GraphQL schema with input validation

## 📊 Schema Overview

The API manages loan records with the following fields:

- `LoanID` - Unique loan identifier
- `CustomerName` - Borrower's full name
- `PropertyAddress` - Property location
- `OriginationDate` - Loan start date
- `MaturityDate` - Loan end date
- `LoanAmount` - Original loan amount
- `RemainingBalance` - Current outstanding balance
- `InterestRate` - Annual interest rate
- `MonthlyPayment` - Monthly payment amount
- `Status` - Current loan status
- `ProductName` - Loan product name
- `ProductType` - Product category (Conventional, FHA, etc.)
- `SecurityName` - Security instrument name
- `ServicerName` - Loan servicing company
- `CurrentStatus` - Detailed current status

## 🛠️ Installation & Setup

### Prerequisites

- Node.js 14.0 or higher
- npm or yarn package manager

### Installation

1. Clone the repository:
```bash
git clone https://github.com/yourusername/loan-graphql-api.git
cd loan-graphql-api
```

2. Install dependencies:
```bash
npm install
```

3. Start the development server:
```bash
npm run dev
```

4. Access the GraphQL Playground at `http://localhost:4000/graphql`

### Production Deployment

```bash
npm start
```

## 📝 API Usage

### Available Queries

#### 1. Get All Loans (with filtering and pagination)

```graphql
query GetLoans {
  loans(
    filter: {
      status: "CURRENT"
      productType: "CONVENTIONAL"
      minLoanAmount: 200000
      maxLoanAmount: 500000
    }
    sort: { field: "loanAmount", direction: "DESC" }
    limit: 10
    offset: 0
  ) {
    loanID
    customerName
    propertyAddress
    loanAmount
    remainingBalance
    interestRate
    status
    productType
  }
}
```

#### 2. Get Specific Loan by ID

```graphql
query GetLoan {
  loan(loanID: "L001") {
    loanID
    customerName
    propertyAddress
    originationDate
    maturityDate
    loanAmount
    remainingBalance
    interestRate
    monthlyPayment
    status
    productName
    productType
    securityName
    servicerName
    currentStatus
  }
}
```

#### 3. Search Loans by Customer Name

```graphql
query SearchByCustomer {
  searchLoansByCustomer(customerName: "John") {
    loanID
    customerName
    propertyAddress
    loanAmount
    status
  }
}
```

#### 4. Get Loans by Status

```graphql
query GetLoansByStatus {
  loansByStatus(status: "CURRENT") {
    loanID
    customerName
    loanAmount
    monthlyPayment
    currentStatus
  }
}
```

#### 5. Get Loans by Servicer

```graphql
query GetLoansByServicer {
  loansByServicer(servicerName: "ABC Mortgage Services") {
    loanID
    customerName
    loanAmount
    status
    productType
  }
}
```

#### 6. Get Loan Statistics

```graphql
query GetLoanStats {
  loanStats {
    totalLoans
    totalLoanAmount
    totalRemainingBalance
    averageInterestRate
    statusBreakdown {
      status
      count
    }
    productTypeBreakdown {
      productType
      count
    }
  }
}
```

### Filtering Options

The `LoanFilterInput` supports the following filters:

- `loanID`: String - Partial match on loan ID
- `customerName`: String - Partial match on customer name
- `status`: String - Exact match on loan status
- `productType`: String - Exact match on product type
- `servicerName`: String - Exact match on servicer name
- `currentStatus`: String - Exact match on current status
- `minLoanAmount`: Float - Minimum loan amount
- `maxLoanAmount`: Float - Maximum loan amount
- `minInterestRate`: Float - Minimum interest rate
- `maxInterestRate`: Float - Maximum interest rate

### Sorting Options

Sort by any field with `ASC` or `DESC` direction:

```graphql
sort: { field: "loanAmount", direction: "DESC" }
```

Common sort fields:
- `loanAmount`
- `remainingBalance`
- `interestRate`
- `monthlyPayment`
- `originationDate`
- `maturityDate`
- `customerName`

## 🏗️ Architecture

### Project Structure

```
loan-graphql-api/
├── index.js              # Main server file
├── package.json          # Dependencies and scripts
├── README.md            # Documentation
└── .gitignore           # Git ignore rules
```

### Technologies Used

- **Apollo Server Express**: GraphQL server implementation
- **Express.js**: Web application framework
- **GraphQL**: Query language and runtime
- **Node.js**: JavaScript runtime environment

### Data Layer

Currently uses in-memory mock data for demonstration. In production, replace with:

- **Database Integration**: PostgreSQL, MongoDB, or SQL Server
- **ORM/ODM**: Prisma, TypeORM, or Mongoose
- **Caching**: Redis for improved performance
- **Data Validation**: Additional input validation and sanitization

## 🔧 Configuration

### Environment Variables

Create a `.env` file for configuration:

```env
PORT=4000
NODE_ENV=development
DATABASE_URL=your_database_connection_string
REDIS_URL=your_redis_connection_string
```

### Database Integration Example

Replace the mock data with actual database queries:

```javascript
// Example with PostgreSQL
const { Pool } = require('pg');
const pool = new Pool({
  connectionString: process.env.DATABASE_URL
});

// In resolvers
loans: async (parent, args) => {
  const query = 'SELECT * FROM loans WHERE status = $1';
  const result = await pool.query(query, [args.filter.status]);
  return result.rows;
}
```

## 🧪 Testing

Run tests with:

```bash
npm test
```

Example test queries to verify functionality:

1. Basic loan retrieval
2. Filtering by various criteria
3. Sorting in both directions
4. Pagination with different limits
5. Statistical calculations
6. Error handling for invalid inputs

## 🚀 Deployment

### Docker Deployment

Create a `Dockerfile`:

```dockerfile
FROM node:16-alpine
WORKDIR /app
COPY package*.json ./
RUN npm ci --only=production
COPY . .
EXPOSE 4000
CMD ["npm", "start"]
```

### Cloud Deployment Options

- **AWS**: Deploy using ECS, Lambda, or Elastic Beanstalk
- **Google Cloud**: Use Cloud Run or App Engine
- **Azure**: Deploy with Container Instances or App Service
- **Heroku**: Simple deployment with git push

## 📊 Performance Considerations

- **Indexing**: Add database indexes on frequently queried fields
- **Caching**: Implement Redis caching for frequently accessed data
- **Pagination**: Always use pagination for large result sets
- **Query Complexity**: Monitor and limit query complexity
- **Rate Limiting**: Implement rate limiting for production use

## 🔒 Security Considerations

- **Authentication**: Add JWT or OAuth2 authentication
- **Authorization**: Implement role-based access control
- **Input Validation**: Validate all user inputs
- **Query Depth Limiting**: Prevent deeply nested queries
- **CORS**: Configure CORS for cross-origin requests
- **HTTPS**: Always use HTTPS in production

## 🔄 API Versioning

Current version: v1.0.0

Future versions will maintain backward compatibility. Breaking changes will be introduced in major version updates with proper migration guides.

## 📈 Monitoring & Analytics

Consider implementing:

- **Health Checks**: Endpoint for service health monitoring
- **Metrics**: Request/response time tracking
- **Logging**: Structured logging with correlation IDs
- **Error Tracking**: Integration with Sentry or similar services

---
