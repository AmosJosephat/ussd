use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;

/// Application error types
#[derive(Debug, thiserror::Error)]
pub enum UssdError {
    #[error("Session not found: {0}")]
    SessionNotFound(String),

    #[error("Session expired: {0}")]
    SessionExpired(String),

    #[error("Invalid input: {0}")]
    InvalidInput(String),

    #[error("Invalid state transition: from {from} to {to}")]
    InvalidStateTransition { from: String, to: String },

    #[error("Database error: {0}")]
    DatabaseError(#[from] sqlx::Error),

    #[error("Redis error: {0}")]
    RedisError(#[from] redis::RedisError),

    #[error("Serialization error: {0}")]
    SerializationError(#[from] serde_json::Error),

    #[error("Configuration error: {0}")]
    ConfigError(#[from] config::ConfigError),

    #[error("Plugin error: {0}")]
    PluginError(String),

    #[error("Rate limit exceeded")]
    RateLimitExceeded,

    #[error("Authentication failed: {0}")]
    AuthenticationFailed(String),

    #[error("Authorization failed: {0}")]
    AuthorizationFailed(String),

    #[error("Menu not found: {0}")]
    MenuNotFound(String),

    #[error("Internal error: {0}")]
    InternalError(String),

    #[error("Service unavailable")]
    ServiceUnavailable,
}

impl UssdError {
    fn status_code(&self) -> StatusCode {
        match self {
            Self::SessionNotFound(_) | Self::MenuNotFound(_) => StatusCode::NOT_FOUND,
            Self::InvalidInput(_) | Self::InvalidStateTransition { .. } => StatusCode::BAD_REQUEST,
            Self::SessionExpired(_) => StatusCode::GONE,
            Self::RateLimitExceeded => StatusCode::TOO_MANY_REQUESTS,
            Self::AuthenticationFailed(_) => StatusCode::UNAUTHORIZED,
            Self::AuthorizationFailed(_) => StatusCode::FORBIDDEN,
            Self::ServiceUnavailable => StatusCode::SERVICE_UNAVAILABLE,
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    fn error_code(&self) -> &str {
        match self {
            Self::SessionNotFound(_) => "SESSION_NOT_FOUND",
            Self::SessionExpired(_) => "SESSION_EXPIRED",
            Self::InvalidInput(_) => "INVALID_INPUT",
            Self::InvalidStateTransition { .. } => "INVALID_STATE_TRANSITION",
            Self::DatabaseError(_) => "DATABASE_ERROR",
            Self::RedisError(_) => "REDIS_ERROR",
            Self::SerializationError(_) => "SERIALIZATION_ERROR",
            Self::ConfigError(_) => "CONFIG_ERROR",
            Self::PluginError(_) => "PLUGIN_ERROR",
            Self::RateLimitExceeded => "RATE_LIMIT_EXCEEDED",
            Self::AuthenticationFailed(_) => "AUTHENTICATION_FAILED",
            Self::AuthorizationFailed(_) => "AUTHORIZATION_FAILED",
            Self::MenuNotFound(_) => "MENU_NOT_FOUND",
            Self::InternalError(_) => "INTERNAL_ERROR",
            Self::ServiceUnavailable => "SERVICE_UNAVAILABLE",
        }
    }
}

impl IntoResponse for UssdError {
    fn into_response(self) -> Response {
        let status = self.status_code();
        let error_code = self.error_code();
        let message = self.to_string();

        // Log error
        tracing::error!(
            error = %message,
            error_code = error_code,
            status_code = %status,
            "Request failed"
        );

        let body = Json(json!({
            "error": {
                "code": error_code,
                "message": message,
                "status": status.as_u16(),
            }
        }));

        (status, body).into_response()
    }
}

pub type Result<T> = std::result::Result<T, UssdError>;
