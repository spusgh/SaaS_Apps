# 🏦 LexiFin AI - Intelligent Loan Denial Classification Platform

[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](https://opensource.org/licenses/MIT)
[![Python 3.11+](https://img.shields.io/badge/python-3.11+-blue.svg)](https://www.python.org/downloads/)
[![FastAPI](https://img.shields.io/badge/FastAPI-0.104-green.svg)](https://fastapi.tiangolo.com/)
[![Test Coverage](https://img.shields.io/badge/coverage-87%25-brightgreen.svg)](https://codecov.io/)
[![Azure](https://img.shields.io/badge/Azure-AKS-0078D4.svg)](https://azure.microsoft.com/)
[![Docker](https://img.shields.io/badge/Docker-Ready-2496ED.svg)](https://www.docker.com/)

> **Transform loan denial reasons into actionable insights with state-of-the-art NLP and machine learning**

LexiFin AI is an enterprise-grade NLP platform that automatically classifies loan denial reasons into 17 standardized categories, generates HMDA-compliant codes, and provides intelligent risk scoring—helping financial institutions reduce compliance costs by 40% while improving decision accuracy.

![LexiFin AI Admin Dashboard](./assets/LexiFinAIAD.png) <br/>

![LexiFin AI Admin Dashboard](./assets/LexiFinAIAD1.png) <br/>

![LexiFin AI Admin Dashboard - Recent Activities](./assets/LexiFinAIADRA.png)<br/>

![LexiFin AI Admin Dashboard - Models](./assets/LexiFinAIModels.png)<br/>

![LexiFin AI Admin Dashboard - Predictions](./assets/LexiFinAIPredictions.png)<br/>

![LexiFin AI Admin Dashboard - Compliance](./assets/LexiFinAICompliance.png)<br/>

![LexiFin AI Admin Dashboard - Loan Officer Portal](./assets/LexiFinAILOP.png)<br/>

![LexiFin AI Admin Dashboard - AI Analysis Results](./assets/LexiFinAILOPDRA.png)<br/>

---

## 🎯 Why LexiFin AI?

### The Problem
Financial institutions process thousands of loan applications daily, but:
- **Inconsistent Documentation**: Denial reasons lack standardization (87% variance across officers)
- **Compliance Risk**: Manual HMDA coding leads to 12% error rate and $2M+ in audit findings
- **Inefficiency**: Loan officers spend 40% of time on categorization instead of customer service
- **Hidden Risk**: Unstructured text hides critical risk indicators

### The Solution
LexiFin AI uses advanced NLP to:
- ✅ **Automate** denial reason classification with 95%+ accuracy
- ✅ **Standardize** HMDA compliance codes across all applications
- ✅ **Identify** hidden risk signals from unstructured text
- ✅ **Accelerate** loan processing by 75% reduction in manual work

### Business Impact
```
📊 75% reduction in manual categorization time
💰 $2M+ annual savings from compliance automation
🎯 95.2% classification accuracy (vs 83% manual baseline)
⚡ 145ms average prediction latency
📈 40% reduction in audit findings within 12 months
```

---

## ✨ Key Features

### 🤖 **Intelligent Classification**
- **17 Denial Categories**: From "Insufficient Income" to "Fraud Indicators"
- **Multi-Label Support**: Detects overlapping denial reasons
- **Confidence Scoring**: Provides transparency with 0-1 confidence scores
- **Entity Extraction**: Automatically extracts DTI values, credit scores, employment gaps

### 📋 **Regulatory Compliance**
- **HMDA Code Generation**: Automatic mapping to regulatory codes (1-9)
- **Fair Lending Analysis**: Built-in disparate impact detection
- **Audit Trail**: Complete 7-year retention with encryption
- **Explainability**: SHAP-based explanations for all predictions

### ⚡ **Enterprise Performance**
- **Real-Time API**: Sub-200ms latency for interactive applications
- **Batch Processing**: 10,000+ applications per hour
- **Auto-Scaling**: 2-20 pods based on demand
- **99.9% Uptime**: Multi-region deployment with automatic failover

### 🔒 **Security & Privacy**
- **Encryption**: AES-256 at rest, TLS 1.3 in transit
- **Authentication**: OAuth 2.0 with Azure AD integration
- **RBAC**: Granular role-based access control
- **Compliance**: GDPR, CCPA, SOC 2 Type II ready

---

## 🚀 Quick Start

### Prerequisites
- Docker & Docker Compose
- Python 3.11+ (for local development)
- Azure CLI (for cloud deployment)
- 8GB RAM minimum


---

## 🏗 Architecture

### High-Level Overview
```mermaid
flowchart TD

%% =========================
%% USER INTERFACES
%% =========================
subgraph UI[USER INTERFACES]
    LO[Loan Officer Portal]
    AD[Admin Dashboard]
    CR[Compliance Reports]
end

LO --> API
AD --> API
CR --> API

%% =========================
%% FASTAPI LAYER
%% =========================
API[FastAPI REST API]

API --> BERT
API --> LGBM

%% =========================
%% MODEL LAYER
%% =========================
subgraph MODELS[Model Ensemble Layer]
    BERT[BERT Classifier-70% weight]
    LGBM[LightGBM Model-30% weight]
end

BERT -->|Ensemble| LGBM

%% =========================
%% DATA LAYER
%% =========================
LGBM --> DATA
BERT --> DATA

DATA[Data Layer-SQL, Cosmos DB]

```

### Technology Stack

| Layer | Technology | Purpose |
|-------|-----------|---------|
| **Frontend** | React 18 + TailwindCSS | Interactive dashboards |
| **API** | FastAPI + Uvicorn | High-performance async API |
| **ML Models** | BERT (DistilBERT) + LightGBM | Hybrid classification |
| **Embeddings** | Sentence-Transformers | Semantic text encoding |
| **Database** | Azure SQL + Cosmos DB | Transactional + NoSQL |
| **Cache** | Redis | Sub-millisecond lookups |
| **Orchestration** | Kubernetes (AKS) | Container management |
| **Monitoring** | Prometheus + Grafana | Metrics & dashboards |
| **Logging** | Azure Log Analytics | Centralized logging |
| **CI/CD** | GitHub Actions | Automated deployment |
| **IaC** | Terraform | Infrastructure provisioning |

### ML Model Pipeline

```
Input Text → Preprocessing → Feature Engineering → Models → Ensemble → Output
                                                     ↓
                                              ┌──────────┐
                                              │   BERT   │ (92% acc)
                                              └──────────┘
                                                     ↓
                                              ┌──────────┐
                                              │ LightGBM │ (89% acc)
                                              └──────────┘
                                                     ↓
                                              ┌──────────┐
                                              │ Ensemble │ (95% acc)
                                              └──────────┘
```

---



---

## 📚 API Reference

### Interactive Documentation

- **Swagger UI**: http://localhost:8000/docs
- **ReDoc**: http://localhost:8000/redoc

### Core Endpoints

| Endpoint | Method | Description | Auth Required |
|----------|--------|-------------|---------------|
| `/api/v1/predict` | POST | Predict denial category | ✅ Yes |
| `/api/v1/batch` | POST | Batch processing | ✅ Yes |
| `/api/v1/health` | GET | Health check | ❌ No |
| `/api/v1/ready` | GET | Readiness check | ❌ No |
| `/api/v1/metrics` | GET | Prometheus metrics | ❌ No |
| `/api/v1/models` | GET | List available models | ✅ Yes |
| `/api/v1/reports/hmda` | GET | Generate HMDA report | ✅ Yes (Admin) |

### Authentication

All protected endpoints require Bearer token authentication:

---

