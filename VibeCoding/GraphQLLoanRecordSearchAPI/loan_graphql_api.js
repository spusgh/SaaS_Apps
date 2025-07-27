const { ApolloServer } = require('apollo-server-express');
const { gql } = require('apollo-server-express');
const express = require('express');

// Mock data - Replace with your actual database connection
const loanRecords = [
  {
    loanID: "L001",
    customerName: "John Smith",
    propertyAddress: "123 Main St, New York, NY 10001",
    originationDate: "2023-01-15",
    maturityDate: "2053-01-15",
    loanAmount: 350000.00,
    remainingBalance: 340000.00,
    interestRate: 4.25,
    monthlyPayment: 1725.50,
    status: "CURRENT",
    productName: "30-Year Fixed Mortgage",
    productType: "CONVENTIONAL",
    securityName: "FNMA 4.5% 2053",
    servicerName: "ABC Mortgage Services",
    currentStatus: "ACTIVE"
  },
  {
    loanID: "L002",
    customerName: "Sarah Johnson",
    propertyAddress: "456 Oak Ave, Los Angeles, CA 90210",
    originationDate: "2022-06-20",
    maturityDate: "2037-06-20",
    loanAmount: 275000.00,
    remainingBalance: 260000.00,
    interestRate: 3.75,
    monthlyPayment: 1850.25,
    status: "CURRENT",
    productName: "15-Year Fixed Mortgage",
    productType: "FHA",
    securityName: "GNMA 3.5% 2037",
    servicerName: "XYZ Loan Servicing",
    currentStatus: "ACTIVE"
  },
  {
    loanID: "L003",
    customerName: "Michael Brown",
    propertyAddress: "789 Pine St, Chicago, IL 60601",
    originationDate: "2021-03-10",
    maturityDate: "2051-03-10",
    loanAmount: 425000.00,
    remainingBalance: 395000.00,
    interestRate: 4.75,
    monthlyPayment: 2210.80,
    status: "DELINQUENT",
    productName: "30-Year ARM",
    productType: "JUMBO",
    securityName: "FHLMC 4.0% 2051",
    servicerName: "ABC Mortgage Services",
    currentStatus: "DELINQUENT_30"
  }
];

// GraphQL Schema Definition
const typeDefs = gql`
  type Loan {
    loanID: ID!
    customerName: String!
    propertyAddress: String!
    originationDate: String!
    maturityDate: String!
    loanAmount: Float!
    remainingBalance: Float!
    interestRate: Float!
    monthlyPayment: Float!
    status: String!
    productName: String!
    productType: String!
    securityName: String!
    servicerName: String!
    currentStatus: String!
  }

  input LoanFilterInput {
    loanID: String
    customerName: String
    status: String
    productType: String
    servicerName: String
    currentStatus: String
    minLoanAmount: Float
    maxLoanAmount: Float
    minInterestRate: Float
    maxInterestRate: Float
  }

  input LoanSortInput {
    field: String!
    direction: String! # ASC or DESC
  }

  type Query {
    # Get all loans with optional filtering and sorting
    loans(
      filter: LoanFilterInput
      sort: LoanSortInput
      limit: Int = 10
      offset: Int = 0
    ): [Loan!]!
    
    # Get a specific loan by ID
    loan(loanID: ID!): Loan
    
    # Search loans by customer name (fuzzy search)
    searchLoansByCustomer(customerName: String!): [Loan!]!
    
    # Get loans by status
    loansByStatus(status: String!): [Loan!]!
    
    # Get loans by servicer
    loansByServicer(servicerName: String!): [Loan!]!
    
    # Get loan statistics
    loanStats: LoanStats!
  }

  type LoanStats {
    totalLoans: Int!
    totalLoanAmount: Float!
    totalRemainingBalance: Float!
    averageInterestRate: Float!
    statusBreakdown: [StatusCount!]!
    productTypeBreakdown: [ProductTypeCount!]!
  }

  type StatusCount {
    status: String!
    count: Int!
  }

  type ProductTypeCount {
    productType: String!
    count: Int!
  }
`;

// Resolver functions
const resolvers = {
  Query: {
    loans: (parent, args) => {
      let filteredLoans = [...loanRecords];

      // Apply filters
      if (args.filter) {
        const filter = args.filter;
        
        if (filter.loanID) {
          filteredLoans = filteredLoans.filter(loan => 
            loan.loanID.toLowerCase().includes(filter.loanID.toLowerCase())
          );
        }
        
        if (filter.customerName) {
          filteredLoans = filteredLoans.filter(loan => 
            loan.customerName.toLowerCase().includes(filter.customerName.toLowerCase())
          );
        }
        
        if (filter.status) {
          filteredLoans = filteredLoans.filter(loan => loan.status === filter.status);
        }
        
        if (filter.productType) {
          filteredLoans = filteredLoans.filter(loan => loan.productType === filter.productType);
        }
        
        if (filter.servicerName) {
          filteredLoans = filteredLoans.filter(loan => loan.servicerName === filter.servicerName);
        }
        
        if (filter.currentStatus) {
          filteredLoans = filteredLoans.filter(loan => loan.currentStatus === filter.currentStatus);
        }
        
        if (filter.minLoanAmount) {
          filteredLoans = filteredLoans.filter(loan => loan.loanAmount >= filter.minLoanAmount);
        }
        
        if (filter.maxLoanAmount) {
          filteredLoans = filteredLoans.filter(loan => loan.loanAmount <= filter.maxLoanAmount);
        }
        
        if (filter.minInterestRate) {
          filteredLoans = filteredLoans.filter(loan => loan.interestRate >= filter.minInterestRate);
        }
        
        if (filter.maxInterestRate) {
          filteredLoans = filteredLoans.filter(loan => loan.interestRate <= filter.maxInterestRate);
        }
      }

      // Apply sorting
      if (args.sort) {
        const { field, direction } = args.sort;
        filteredLoans.sort((a, b) => {
          let aVal = a[field];
          let bVal = b[field];
          
          if (typeof aVal === 'string') {
            aVal = aVal.toLowerCase();
            bVal = bVal.toLowerCase();
          }
          
          if (direction === 'DESC') {
            return bVal > aVal ? 1 : -1;
          }
          return aVal > bVal ? 1 : -1;
        });
      }

      // Apply pagination
      const start = args.offset || 0;
      const end = start + (args.limit || 10);
      return filteredLoans.slice(start, end);
    },

    loan: (parent, args) => {
      return loanRecords.find(loan => loan.loanID === args.loanID);
    },

    searchLoansByCustomer: (parent, args) => {
      return loanRecords.filter(loan => 
        loan.customerName.toLowerCase().includes(args.customerName.toLowerCase())
      );
    },

    loansByStatus: (parent, args) => {
      return loanRecords.filter(loan => loan.status === args.status);
    },

    loansByServicer: (parent, args) => {
      return loanRecords.filter(loan => loan.servicerName === args.servicerName);
    },

    loanStats: () => {
      const totalLoans = loanRecords.length;
      const totalLoanAmount = loanRecords.reduce((sum, loan) => sum + loan.loanAmount, 0);
      const totalRemainingBalance = loanRecords.reduce((sum, loan) => sum + loan.remainingBalance, 0);
      const averageInterestRate = loanRecords.reduce((sum, loan) => sum + loan.interestRate, 0) / totalLoans;

      // Status breakdown
      const statusMap = {};
      loanRecords.forEach(loan => {
        statusMap[loan.status] = (statusMap[loan.status] || 0) + 1;
      });
      const statusBreakdown = Object.entries(statusMap).map(([status, count]) => ({
        status,
        count
      }));

      // Product type breakdown
      const productTypeMap = {};
      loanRecords.forEach(loan => {
        productTypeMap[loan.productType] = (productTypeMap[loan.productType] || 0) + 1;
      });
      const productTypeBreakdown = Object.entries(productTypeMap).map(([productType, count]) => ({
        productType,
        count
      }));

      return {
        totalLoans,
        totalLoanAmount,
        totalRemainingBalance,
        averageInterestRate,
        statusBreakdown,
        productTypeBreakdown
      };
    }
  }
};

// Create Apollo Server
const server = new ApolloServer({
  typeDefs,
  resolvers,
  introspection: true,
  playground: true
});

// Create Express app
const app = express();

// Apply Apollo GraphQL middleware
server.applyMiddleware({ app, path: '/graphql' });

const PORT = process.env.PORT || 4000;

app.listen(PORT, () => {
  console.log(`🚀 Server ready at http://localhost:${PORT}${server.graphqlPath}`);
  console.log(`📊 GraphQL Playground available at http://localhost:${PORT}${server.graphqlPath}`);
});

module.exports = { app, server };