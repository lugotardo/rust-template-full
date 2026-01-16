//! API REST usando Axum
//!
//! Este módulo expõe endpoints HTTP para a aplicação.

use axum::{
    extract::State,
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::{get, post},
    Json, Router,
};
use serde::Serialize;
use std::sync::Arc;

pub mod handlers;
pub mod middleware;

/// Estado compartilhado da aplicação
#[derive(Clone)]
pub struct AppState {
    #[cfg(feature = "postgres")]
    pub db: Arc<crate::db::Database>,
}

/// Resposta padrão de API
#[derive(Debug, Serialize)]
pub struct ApiResponse<T> {
    pub success: bool,
    pub data: Option<T>,
    pub error: Option<String>,
}

impl<T: Serialize> ApiResponse<T> {
    pub fn success(data: T) -> Self {
        Self {
            success: true,
            data: Some(data),
            error: None,
        }
    }

    pub fn error(message: impl Into<String>) -> ApiResponse<()> {
        ApiResponse {
            success: false,
            data: None,
            error: Some(message.into()),
        }
    }
}

/// Tipo de erro da API
#[derive(Debug)]
pub enum ApiError {
    NotFound(String),
    BadRequest(String),
    InternalError(String),
    DatabaseError(String),
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let (status, message) = match self {
            ApiError::NotFound(msg) => (StatusCode::NOT_FOUND, msg),
            ApiError::BadRequest(msg) => (StatusCode::BAD_REQUEST, msg),
            ApiError::InternalError(msg) => (StatusCode::INTERNAL_SERVER_ERROR, msg),
            ApiError::DatabaseError(msg) => (StatusCode::INTERNAL_SERVER_ERROR, msg),
        };

        let body = Json(ApiResponse::<()>::error(message));
        (status, body).into_response()
    }
}

#[cfg(feature = "postgres")]
impl From<sqlx::Error> for ApiError {
    fn from(err: sqlx::Error) -> Self {
        ApiError::DatabaseError(err.to_string())
    }
}

#[cfg(feature = "api")]
impl From<validator::ValidationErrors> for ApiError {
    fn from(err: validator::ValidationErrors) -> Self {
        ApiError::BadRequest(err.to_string())
    }
}

/// Cria o router da API
pub fn create_router(state: AppState) -> Router {
    Router::new()
        // Health check
        .route("/health", get(health_check))
        .route("/ready", get(readiness_check))
        // Info
        .route("/", get(root))
        .route("/version", get(version))
        // Users API (se postgres está habilitado)
        .merge(create_users_router())
        .with_state(state)
}

/// Health check endpoint
async fn health_check() -> Json<ApiResponse<&'static str>> {
    Json(ApiResponse::success("healthy"))
}

/// Readiness check endpoint
async fn readiness_check(
    State(state): State<AppState>,
) -> Result<Json<ApiResponse<&'static str>>, ApiError> {
    #[cfg(feature = "postgres")]
    {
        // Verificar conexão com banco
        state
            .db
            .ping()
            .await
            .map_err(|e| ApiError::InternalError(format!("Database not ready: {}", e)))?;
    }

    #[cfg(not(feature = "postgres"))]
    let _ = state; // Evitar warning quando postgres não está habilitado

    Ok(Json(ApiResponse::success("ready")))
}

/// Root endpoint
async fn root() -> Json<ApiResponse<serde_json::Value>> {
    Json(ApiResponse::success(serde_json::json!({
        "name": "Rust App API",
        "version": env!("CARGO_PKG_VERSION"),
        "endpoints": [
            "/health",
            "/ready",
            "/version",
            "/api/users",
        ]
    })))
}

/// Version endpoint
async fn version() -> Json<ApiResponse<serde_json::Value>> {
    Json(ApiResponse::success(serde_json::json!({
        "version": env!("CARGO_PKG_VERSION"),
        "rust_version": env!("CARGO_PKG_RUST_VERSION"),
    })))
}

/// Router para endpoints de usuários
fn create_users_router() -> Router<AppState> {
    #[cfg(feature = "postgres")]
    {
        Router::new()
            .route("/api/users", get(handlers::list_users))
            .route("/api/users", post(handlers::create_user))
            .route("/api/users/:id", get(handlers::get_user))
            .route(
                "/api/users/:id",
                axum::routing::delete(handlers::delete_user),
            )
    }

    #[cfg(not(feature = "postgres"))]
    {
        Router::new()
    }
}
