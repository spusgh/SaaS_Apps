# 🧠 TinyLlama + MS SQL Integration

This project demonstrates how to integrate the **TinyLlama 1.1B LLM** with **Microsoft SQL Server**, enabling natural language queries to be translated into SQL using a lightweight, local AI model.

---
## POC: TinyLlama-1.1B + MS SQL
 ![image](https://github.com/spusgh/SaaS_Apps/blob/main/LLM/LLM2MSSQL/LLM2MSSQL-tinyllama.png)


## 🚀 Overview

- **LLM**: [TinyLlama-1.1B](https://huggingface.co/TheBloke/TinyLlama-1.1B-Chat-v1.0-GGUF)
- **LLM Runtime**: [Ollama](https://ollama.com/)
- **Database**: Microsoft SQL Server
- **Orchestration**: PowerShell + T-SQL (`xp_cmdshell`)
- **Use Case**: Text-to-SQL generation, sentiment analysis, query explanation

---

## ⚙️ Prerequisites

### 🧠 Install TinyLlama via Ollama

```bash
ollama pull tinyllama
