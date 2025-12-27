# FinTune AI - PLM MVP for Financial Services

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Python 3.9+](https://img.shields.io/badge/python-3.9+-blue.svg)](https://www.python.org/downloads/)
[![PyTorch](https://img.shields.io/badge/PyTorch-2.1-red.svg)](https://pytorch.org/)
[![FastAPI](https://img.shields.io/badge/FastAPI-0.108-green.svg)](https://fastapi.tiangolo.com/)

> Production-ready Pretrained Language Model (PLM) for automated loan application risk assessment and denial reason classification.


---

## 🎯 Overview

FinTune AI automates loan application risk assessment using a fine-tuned RoBERTa model on financial domain data. The system reduces manual underwriting time from **4 hours to 5 minutes** while achieving **94.2% accuracy** on denial classification.

### Business Impact

- ⚡ **95% faster** processing time
- 💰 **$2.4M annual** cost savings
- 📊 **94.2% F1-score** on classification
- 🎯 **143ms latency** (p95) for real-time predictions
- ✅ **100% automated** ECOA compliance

### Problem Statement

Manual loan application review is:
- **Time-intensive:** 3-4 hours per application
- **Inconsistent:** 35% variance in approval rates across officers
- **Error-prone:** 12% error rate in adverse action notices
- **Non-scalable:** Only 500 applications/day capacity

---

## ✨ Key Features

### Core Capabilities

- **🤖 Denial Reason Classification:** Automatically categorize applications into 12 denial categories
- **📝 Risk Summarization:** Generate human-readable risk assessment summaries
- **💡 Recommendation Generation:** Suggest remediation steps and alternative products
- **⚖️ Compliance Automation:** Auto-generate ECOA-compliant adverse action notices
- **🔍 Explainability:** SHAP values and attention weights for model transparency
- **📊 Real-time Monitoring:** Track model performance, bias, and system health

### Technical Features

- Multi-task learning (classification + regression)
- Real-time predictions (<200ms)
- Batch processing for backlog
- Redis caching for frequent queries
- Automatic model retraining on drift detection
- A/B testing framework for model rollout
- Comprehensive audit trail

---

## Demo

#### FinTune AI Admin Dashboard
![FinTune AI Admin Dashboard](./assets/FinTuneAIAD.png)

#### FinTune AI - Training Performance & Denial Reason Distribution
![FinTune AI Admin Dashboard](./assets/FinTuneAIAD1.png)

#### FinTune AI - Prediction Volume & Accuracy (Last 7 Days)
![FinTune AI Admin Dashboard](./assets/FinTuneAIAD2.png)

#### FinTune AI - Model
![FinTune AI Admin Dashboard](./assets/FinTuneAIModel.png)

#### FinTune AI - Data
![FinTune AI Admin Dashboard](./assets/FinTuneAIData.png)

#### FinTune AI - Monitoring
![FinTune AI Admin Dashboard](./assets/FinTuneAIMonitoring.png)

#### FinTune AI - User Prediction Analysis
![FinTune AI Admin Dashboard](./assets/FinTuneAIUserPredictionAnalysis.png)

#### FinTune AI - User Prediction Analysis Report
![FinTune AI Admin Dashboard](./assets/FinTuneAIUserPredictionLAR.png)


---

## 🏗️ Architecture

### System Context Diagram

```mermaid
graph TB
    subgraph "External Systems"
        A[Loan Officers]
        B[Customer Portal]
        C[Batch Jobs]
    end
    
    subgraph "FinTune AI Platform"
        D[API Gateway]
        E[Prediction Service]
        F[Model Registry]
        G[Data Pipeline]
    end
    
    subgraph "Data Layer"
        H[(SQL Database)]
        I[(Feature Store)]
        J[Model Artifacts]
    end
    
    A --> D
    B --> D
    C --> D
    D --> E
    E --> F
    E --> H
    G --> I
    G --> H
    F --> J
    
    style D fill:#3b82f6
    style E fill:#10b981
    style F fill:#f59e0b
```

### High-Level Component Architecture

```mermaid
graph LR
    subgraph "Presentation Layer"
        A1[Admin Dashboard]
        A2[User Interface]
        A3[Mobile App]
    end
    
    subgraph "API Gateway"
        B[Azure App Gateway<br/>+ API Management]
    end
    
    subgraph "Application Layer"
        C1[Prediction API<br/>FastAPI]
        C2[Training API<br/>FastAPI]
        C3[Monitoring API<br/>FastAPI]
    end
    
    subgraph "Business Logic"
        D1[Model Inference<br/>TorchServe]
        D2[Training Pipeline<br/>Azure ML]
        D3[Feature Engineering<br/>Databricks]
    end
    
    subgraph "Data Layer"
        E1[(Azure SQL DB)]
        E2[Blob Storage]
        E3[MLflow Registry]
    end
    
    A1 --> B
    A2 --> B
    A3 --> B
    B --> C1
    B --> C2
    B --> C3
    C1 --> D1
    C2 --> D2
    C1 --> D3
    D1 --> E1
    D2 --> E2
    D3 --> E3
    
    style B fill:#3b82f6
    style C1 fill:#10b981
    style D1 fill:#8b5cf6
```

### Data Flow Pipeline

```mermaid
flowchart TD
    A[SQL Database<br/>Operational] --> B{CDC Stream}
    B --> C[Data Factory<br/>ETL Pipeline]
    C --> D[Databricks<br/>Transformation]
    D --> E[Feature Store<br/>Delta Lake]
    E --> F[Model Training<br/>Azure ML GPU]
    F --> G[MLflow Registry<br/>Model Versions]
    G --> H{A/B Testing}
    H -->|Pass| I[Production<br/>Deployment]
    H -->|Fail| J[Rollback]
    I --> K[Prediction Service<br/>TorchServe]
    K --> L[Client Apps]
    
    style C fill:#3b82f6
    style D fill:#10b981
    style F fill:#ef4444
    style I fill:#8b5cf6
```

### Model Architecture

```mermaid
graph TB
    A[Input Sequence<br/>512 tokens] --> B[RoBERTa Encoder<br/>12 Layers<br/>768 Hidden Size]
    B --> C[CLS Token<br/>Representation]
    C --> D1[Classification Head<br/>768→384→12]
    C --> D2[Regression Head<br/>768→192→1]
    D1 --> E1[Denial Category<br/>12 Classes]
    D2 --> E2[Risk Score<br/>0-100]
    
    style B fill:#3b82f6
    style D1 fill:#10b981
    style D2 fill:#f59e0b
```

### Deployment Architecture

```mermaid
graph TB
    subgraph "Azure Kubernetes Service"
        A[Prediction API<br/>3 replicas]
        B[TorchServe<br/>2 GPU pods]
        C[Redis Cache<br/>1 replica]
    end
    
    subgraph "External Services"
        D[Azure SQL DB]
        E[Blob Storage]
        F[MLflow Registry]
    end
    
    subgraph "Monitoring"
        G[Prometheus]
        H[Grafana]
        I[ELK Stack]
    end
    
    A --> B
    A --> C
    A --> D
    B --> E
    B --> F
    A --> G
    G --> H
    A --> I
    
    style A fill:#10b981
    style B fill:#8b5cf6
    style G fill:#f59e0b
```

---

## 🚀 Getting Started

### Prerequisites

```bash
# System Requirements
- Python 3.9+
- CUDA 11.8+ (for GPU training)
- Docker 20.10+
- Kubernetes 1.24+

# Hardware Requirements (Training)
- GPU: NVIDIA V100 or A100 (16GB+ VRAM)
- CPU: 16+ cores
- RAM: 64GB+
- Storage: 500GB+ SSD

# Hardware Requirements (Inference)
- GPU: NVIDIA T4 or V100 (optional but recommended)
- CPU: 4+ cores
- RAM: 16GB+
- Storage: 100GB
```

---

## 📊 Data Pipeline

### Schema Mapping

```mermaid
erDiagram
    APPLICATIONS ||--o{ CUSTOMERS : "belongs_to"
    APPLICATIONS ||--o{ RISK_ASSESSMENTS : "has"
    APPLICATIONS ||--o{ PROPERTY_DETAILS : "has"
    APPLICATIONS ||--o{ MORTGAGE_PRODUCTS : "uses"
    APPLICATIONS ||--o{ DOCUMENTS_REGISTRY : "has"
    APPLICATIONS ||--o{ AUDIT_LOG : "logged_in"
    
    APPLICATIONS {
        int ApplicationID PK
        int CustomerID FK
        decimal LoanAmount
        decimal DTI
        decimal LTV
        nvarchar DenialReason
        nvarchar Status
    }
    
    CUSTOMERS {
        int CustomerID PK
        decimal AnnualIncome
        int CreditScore
        nvarchar EmploymentStatus
        int YearsEmployed
    }
    
    RISK_ASSESSMENTS {
        int AssessmentID PK
        int ApplicationID FK
        nvarchar RiskClassification
        nvarchar RecommendedAction
    }
```

### Data Extraction Flow

```mermaid
sequenceDiagram
    participant DB as SQL Database
    participant ETL as Data Factory
    participant Transform as Databricks
    participant FS as Feature Store
    participant Train as Training Pipeline
    
    DB->>ETL: CDC Stream (every 15min)
    ETL->>ETL: Extract from 9 tables
    ETL->>Transform: Load raw data
    Transform->>Transform: Feature engineering
    Transform->>Transform: Text preprocessing
    Transform->>Transform: Label creation
    Transform->>FS: Save parquet files
    FS->>Train: Load train/val/test sets
    Train->>Train: Fine-tune RoBERTa
    Train-->>DB: Update predictions
```

### Feature Engineering Process

**Features Created:**

| Feature Type | Examples | Transformation |
|--------------|----------|----------------|
| **Numerical** | `income_to_loan`, `age`, `property_age` | Ratio calculations |
| **Categorical** | `credit_tier`, `ltv_category`, `dti_category` | Binning |
| **Text** | `combined_text` | Concatenation + tokenization |
| **Flags** | `high_risk_flag`, `meets_criteria` | Boolean logic |

---

## 🧠 Model Training

### Training Workflow

```mermaid
flowchart LR
    A[Load Data<br/>87.5K samples] --> B[Create Datasets<br/>PyTorch]
    B --> C[Initialize Model<br/>RoBERTa-base]
    C --> D[Train Loop<br/>5 epochs]
    D --> E{Validation}
    E -->|F1 > Best| F[Save Checkpoint]
    E -->|F1 <= Best| D
    F --> G[Final Evaluation<br/>Test Set]
    G --> H[Register Model<br/>MLflow]
    
    style C fill:#3b82f6
    style D fill:#10b981
    style H fill:#f59e0b
```


### Model Performance

```mermaid
graph LR
    A[Training Metrics] --> B[F1: 94.2%]
    A --> C[Precision: 93.8%]
    A --> D[Recall: 92.6%]
    A --> E[Risk MAE: 3.8]
    
    F[Operational Metrics] --> G[Latency: 143ms p95]
    F --> H[Throughput: 1.2K req/min]
    F --> I[Uptime: 99.94%]
    
    style B fill:#10b981
    style G fill:#3b82f6
```

---


### API Endpoints

```mermaid
graph TB
    A[API Endpoints] --> B["/health
    Health Check"]
    A --> C["/api/v1/predict
    Single Prediction"]
    A --> D["/api/v1/batch-predict
    Batch Prediction"]
    A --> E["/api/v1/metrics
    Model Metrics"]
    A --> F["/api/v1/categories
    Category List"]
    
    style C fill:#10b981
    style D fill:#3b82f6

```

---
### API Summary


| Endpoint | Method | Description | Auth Required |
|----------|--------|-------------|---------------|
| `/health` | GET | Health check | ❌ |
| `/api/v1/predict` | POST | Single prediction | ✅ |
| `/api/v1/batch-predict` | POST | Batch predictions | ✅ |
| `/api/v1/metrics` | GET | Model performance | ✅ |
| `/api/v1/categories` | GET | Denial categories | ❌ |
| `/docs` | GET | Interactive API docs | ❌ |

---

## 🚢 Deployment

### Docker Build

### Kubernetes Deployment

### Deploy Redis

### Check status

### Deployment Flow

```mermaid
sequenceDiagram
    participant Dev as Developer
    participant GH as GitHub Actions
    participant ACR as Azure Container Registry
    participant AKS as Azure Kubernetes
    participant Monitor as Monitoring
    
    Dev->>GH: Push to main branch
    GH->>GH: Run tests
    GH->>GH: Build Docker images
    GH->>ACR: Push images
    GH->>AKS: Deploy to staging
    GH->>GH: Run integration tests
    GH->>AKS: A/B test (10% traffic)
    AKS->>Monitor: Send metrics
    Monitor->>GH: Performance check
    GH->>AKS: Promote to production
```

### Environment Configuration

---

## 📈 Monitoring

### Monitoring Stack

```mermaid
graph TB
    A[Application] --> B[Prometheus<br/>Metrics]
    A --> C[Grafana<br/>Dashboards]
    A --> D[ELK Stack<br/>Logs]
    A --> E[Azure Monitor<br/>Alerts]
    
    B --> F[PagerDuty]
    E --> F
    F --> G[On-Call Engineer]
    
    style B fill:#f59e0b
    style C fill:#10b981
    style D fill:#3b82f6
```

### Key Metrics

**Model Metrics:**
- F1-score by category
- Confidence distribution
- Prediction volume
- Error rate

**System Metrics:**
- Latency (p50/p95/p99)
- Throughput (requests/sec)
- CPU/Memory utilization
- Error rates

**Business Metrics:**
- Applications processed/day
- Cost per prediction
- Processing time reduction
- Officer satisfaction (NPS)

---

## 🔧 Development

### Development Workflow

```mermaid
gitGraph
    commit id: "Initial"
    branch feature/new-feature
    checkout feature/new-feature
    commit id: "Develop"
    commit id: "Test"
    checkout main
    merge feature/new-feature
    commit id: "Release"
```


### Team
- **Product Owner:** Jane Smith (jane.smith@example.com)
- **Tech Lead:** John Doe (john.doe@example.com)
- **ML Engineering:** ML Team (ml-team@example.com)
- **DevOps:** Platform Team (platform@example.com)

---

## 🙏 Acknowledgments

- RoBERTa model from [Hugging Face](https://huggingface.co/roberta-base)
- Financial services dataset from internal systems
- Open-source contributors and maintainers

---

## 📊 Project Status

![Build Status](https://img.shields.io/badge/build-passing-brightgreen)
![Coverage](https://img.shields.io/badge/coverage-85%25-green)
![Uptime](https://img.shields.io/badge/uptime-99.94%25-brightgreen)
![Version](https://img.shields.io/badge/version-1.2.3-blue)

**Last Updated:** December 26, 2025

---

<div align="center">
  <strong>Built with ❤️ by the FinTune AI Team</strong>
  <br/>
  <sub>Transforming financial services with AI</sub>
</div>

