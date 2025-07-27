// src/handlers.rs
use actix_web::{web, HttpResponse, Result};
use sqlx::PgPool;
use crate::models::{SearchFilters};
use crate::services::LoanService;

pub mod loan_handlers {
    use super::*;

    pub async fn search_loans(
        pool: web::Data<PgPool>,
        query: web::Query<SearchFilters>,
    ) -> Result<HttpResponse> {
        match LoanService::search_loans(&pool, query.into_inner()).await {
            Ok(response) => Ok(HttpResponse::Ok().json(response.data)),
            Err(e) => {
                log::error!("Failed to search loans: {}", e);
                Ok(HttpResponse::InternalServerError().json(serde_json::json!({
                    "error": "Failed to search loans"
                })))
            }
        }
    }

    pub async fn get_loan_by_id(
        pool: web::Data<PgPool>,
        path: web::Path<String>,
    ) -> Result<HttpResponse> {
        let loan_id = path.into_inner();
        
        match LoanService::get_loan_by_id(&pool, &loan_id).await {
            Ok(Some(loan)) => Ok(HttpResponse::Ok().json(loan)),
            Ok(None) => Ok(HttpResponse::NotFound().json(serde_json::json!({
                "error": "Loan not found"
            }))),
            Err(e) => {
                log::error!("Failed to get loan by id: {}", e);
                Ok(HttpResponse::InternalServerError().json(serde_json::json!({
                    "error": "Failed to retrieve loan"
                })))
            }
        }
    }

    pub async fn get_statistics(pool: web::Data<PgPool>) -> Result<HttpResponse> {
        match LoanService::get_statistics(&pool).await {
            Ok(stats) => Ok(HttpResponse::Ok().json(stats)),
            Err(e) => {
                log::error!("Failed to get statistics: {}", e);
                Ok(HttpResponse::InternalServerError().json(serde_json::json!({
                    "error": "Failed to retrieve statistics"
                })))
            }
        }
    }
}