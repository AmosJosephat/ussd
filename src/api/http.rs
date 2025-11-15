use crate::config::Config;
use crate::domain::{UssdRequest, UssdResponse};
use crate::error::Result;
use crate::service::UssdService;
use crate::storage::SessionStore;
use crate::middleware::{logging_middleware, metrics_middleware};
use axum::{
    extract::State,
    http::StatusCode,
    routing::{get, post},
    Json, Router,
};
use prometheus::{Encoder, TextEncoder};
use std::sync::Arc;
use tower_http::compression::CompressionLayer;
use tower_http::cors::CorsLayer;
use tower_http::timeout::TimeoutLayer;
use tracing::info;

/// Application state
#[derive(Clone)]
pub struct AppState<S: SessionStore> {
    pub service: Arc<UssdService<S>>,
    pub config: Arc<Config>,
}

/// Create HTTP router
pub fn create_router<S: SessionStore + Clone + 'static>(state: AppState<S>) -> Router {
    Router::new()
        .route("/health", get(health_check))
        .route("/metrics", get(metrics))
        .route("/api/v1/ussd", post(handle_ussd_request::<S>))
        .route("/api/v1/session/:id", get(get_session::<S>))
        .layer(axum::middleware::from_fn(logging_middleware))
        .layer(axum::middleware::from_fn(metrics_middleware))
        .layer(CompressionLayer::new())
        .layer(CorsLayer::permissive())
        .layer(TimeoutLayer::new(std::time::Duration::from_secs(30)))
        .with_state(state)
}

/// Health check endpoint
async fn health_check() -> StatusCode {
    StatusCode::OK
}

/// Metrics endpoint
async fn metrics() -> Result<String> {
    let encoder = TextEncoder::new();
    let metric_families = prometheus::gather();
    let mut buffer = Vec::new();

    encoder
        .encode(&metric_families, &mut buffer)
        .map_err(|e| crate::error::UssdError::InternalError(e.to_string()))?;

    String::from_utf8(buffer)
        .map_err(|e| crate::error::UssdError::InternalError(e.to_string()))
}

/// USSD request handler
async fn handle_ussd_request<S: SessionStore>(
    State(state): State<AppState<S>>,
    Json(request): Json<UssdRequest>,
) -> Result<Json<UssdResponse>> {
    info!(
        session_id = %request.session_id,
        phone = %request.phone_number,
        "USSD request received"
    );

    let response = state.service.process_request(request).await?;

    Ok(Json(response))
}

/// Get session by ID
async fn get_session<S: SessionStore>(
    State(state): State<AppState<S>>,
    axum::extract::Path(session_id): axum::extract::Path<String>,
) -> Result<Json<crate::domain::Session>> {
    let session = state
        .service
        .get_session(&session_id)
        .await?
        .ok_or_else(|| crate::error::UssdError::SessionNotFound(session_id))?;

    Ok(Json(session))
}
