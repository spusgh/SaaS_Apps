
# Multi-Agent Securities Management PWA
A comprehensive Progressive Web Application (PWA) for managing financial securities records with integrated agents for analytics, validation, and recommendations.

![image](https://github.com/spusgh/SaaS_Apps/blob/main/NodejsApps/AISecuritiesManagement/AISecuritiesManagement.gif)

## 🚀 Features
### 🤖 Multi-Agent AI System

Analytics Agent: Provides portfolio insights, metrics, and distribution analysis
Validation Agent: Ensures data integrity and validates security records
Recommendation Agent: Generates AI-powered investment recommendations and alerts

### 📊 Core Functionality

Securities Management: Full CRUD operations for securities records
Real-time Analytics: Portfolio value, distribution, and performance metrics
Advanced Filtering: Search and filter by type, status, and custom criteria
Responsive Design: Works seamlessly on desktop, tablet, and mobile devices
Progressive Web App: Installable, offline-capable, and fast-loading

### 🔐 Security Schema
Complete support for financial securities with the following fields:

SecurityID - Unique identifier
SecurityName - Security name/description
SecurityType - Stock, Bond, ETF, etc.
CUSIP - Committee on Uniform Securities Identification Procedures number
IssueDate - Date of security issuance
CurrentBalance - Current portfolio balance
Status - Active/Inactive status
LastTradeDate - Most recent trade date
LastTradePrice - Most recent trade price

### 🛠️ Technology Stack
#### Backend

Node.js with Express.js framework
RESTful API with comprehensive endpoints
Security middleware (Helmet, CORS, Rate Limiting)
In-memory database (easily replaceable with MongoDB, PostgreSQL, etc.)
UUID for unique identifiers

#### Frontend

React with modern hooks and functional components
Tailwind CSS for responsive styling
Lucide React for beautiful icons
Progressive Web App capabilities


## 🔧 API Endpoints
### Securities Management

GET /api/securities - Get all securities (with filtering support)
GET /api/securities/:id - Get specific security
POST /api/securities - Create new security
PUT /api/securities/:id - Update security
DELETE /api/securities/:id - Delete security

### AI Agent Endpoints

GET /api/analytics - Get portfolio analytics and AI insights
GET /api/recommendations - Get AI-powered recommendations

### Utility

GET /health - Health check endpoint

### Query Parameters

type - Filter by security type (Stock, Bond, ETF)
status - Filter by status (Active, Inactive)
search - Search by name, ID, or CUSIP

## 📱 Usage
### Dashboard

View portfolio analytics and metrics
Monitor AI-generated recommendations
Track security type distribution
Review performance insights

### Securities Management

Add new securities with validation
Search and filter existing records
View detailed security information
Edit and update security data
Delete securities as needed

### AI Recommendations
The system provides intelligent recommendations such as:

Portfolio diversification suggestions
Inactive security alerts
Liquidity monitoring
Balance distribution warnings

## 🔒 Security Features

Rate Limiting: Prevents API abuse
CORS Protection: Secure cross-origin requests
Helmet.js: Security headers and protection
Input Validation: Comprehensive data validation
Error Handling: Graceful error management

## 🎯 AI Agent Details
### Analytics Agent

Calculates portfolio metrics (total value, average balance)
Analyzes security type distribution
Generates performance insights
Tracks active vs inactive securities

### Validation Agent

Validates all security fields
Ensures CUSIP format compliance
Checks date formats and ranges
Validates numeric values and constraints

### Recommendation Agent

Monitors portfolio concentration
Tracks trading activity
Identifies potential risks
Suggests optimization strategies

## 🚀 Deployment
### Backend Deployment

Set environment variables:

bashexport PORT=3001
export NODE_ENV=production

For production, replace in-memory database with persistent storage:

javascript// Replace the securities array with database connection
const mongoose = require('mongoose');
// or
const { Pool } = require('pg');

### Frontend Deployment

Build the production version:

bashnpm run build

Deploy to your preferred hosting service (Vercel, Netlify, etc.)

### PWA Features
The app includes:

Service worker for offline functionality
Web app manifest for installation
Responsive design for all devices
Fast loading and caching

## 📊 Sample Data
The application comes with sample securities data:

Apple Inc. (Stock)
US Treasury Bond 10Y (Bond)
Microsoft Corp. (Stock)

## 🔧 Customization
### Adding New Security Types
Update the SecurityType options in both frontend and backend:
javascript// Add new types to validation
const validTypes = ['Stock', 'Bond', 'ETF', 'Mutual Fund', 'Cryptocurrency'];

### Extending AI Capabilities
Add new AI agents by creating classes similar to existing ones:
javascriptclass NewAgent {
  static performAnalysis(data) {
    // Your AI logic here
  }
}

### Database Integration
Replace the in-memory array with your preferred database:
javascript// MongoDB example
const SecuritySchema = new mongoose.Schema({
  SecurityID: String,
  SecurityName: String,
  // ... other fields
});

## 📝 License
This project is licensed under the MIT License - see the LICENSE file for details.
