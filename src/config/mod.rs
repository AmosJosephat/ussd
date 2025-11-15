use crate::error::Result;
use serde::{Deserialize, Serialize};
use std::time::Duration;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub server: ServerConfig,
    pub database: DatabaseConfig,
    pub redis: RedisConfig,
    pub session: SessionConfig,
    pub security: SecurityConfig,
    pub observability: ObservabilityConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerConfig {
    #[serde(default = "default_host")]
    pub host: String,

    #[serde(default = "default_port")]
    pub port: u16,

    #[serde(default = "default_grpc_port")]
    pub grpc_port: u16,

    #[serde(default = "default_workers")]
    pub workers: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DatabaseConfig {
    pub url: String,

    #[serde(default = "default_max_connections")]
    pub max_connections: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RedisConfig {
    pub url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionConfig {
    #[serde(default = "default_session_timeout")]
    pub timeout_secs: u64,

    #[serde(default = "default_max_sessions")]
    pub max_sessions_per_user: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityConfig {
    pub jwt_secret: String,

    #[serde(default = "default_rate_limit_requests")]
    pub rate_limit_requests: u32,

    #[serde(default = "default_rate_limit_window")]
    pub rate_limit_window_secs: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ObservabilityConfig {
    #[serde(default = "default_log_level")]
    pub log_level: String,

    pub otlp_endpoint: Option<String>,

    #[serde(default = "default_enable_metrics")]
    pub enable_metrics: bool,
}

// Default values
fn default_host() -> String {
    "0.0.0.0".to_string()
}

fn default_port() -> u16 {
    8080
}

fn default_grpc_port() -> u16 {
    50051
}

fn default_workers() -> usize {
    num_cpus::get()
}

fn default_max_connections() -> u32 {
    20
}

fn default_session_timeout() -> u64 {
    300 // 5 minutes
}

fn default_max_sessions() -> usize {
    5
}

fn default_rate_limit_requests() -> u32 {
    100
}

fn default_rate_limit_window() -> u64 {
    60
}

fn default_log_level() -> String {
    "info".to_string()
}

fn default_enable_metrics() -> bool {
    true
}

impl Config {
    pub fn from_env() -> Result<Self> {
        dotenvy::dotenv().ok();

        let config = config::Config::builder()
            .add_source(config::Environment::default().separator("__"))
            .build()?;

        let settings = config.try_deserialize()?;

        Ok(settings)
    }

    pub fn session_timeout(&self) -> Duration {
        Duration::from_secs(self.session.timeout_secs)
    }

    pub fn rate_limit_window(&self) -> Duration {
        Duration::from_secs(self.security.rate_limit_window_secs)
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            server: ServerConfig {
                host: default_host(),
                port: default_port(),
                grpc_port: default_grpc_port(),
                workers: default_workers(),
            },
            database: DatabaseConfig {
                url: "postgresql://localhost/ussd".to_string(),
                max_connections: default_max_connections(),
            },
            redis: RedisConfig {
                url: "redis://localhost".to_string(),
            },
            session: SessionConfig {
                timeout_secs: default_session_timeout(),
                max_sessions_per_user: default_max_sessions(),
            },
            security: SecurityConfig {
                jwt_secret: "change-me-in-production".to_string(),
                rate_limit_requests: default_rate_limit_requests(),
                rate_limit_window_secs: default_rate_limit_window(),
            },
            observability: ObservabilityConfig {
                log_level: default_log_level(),
                otlp_endpoint: None,
                enable_metrics: default_enable_metrics(),
            },
        }
    }
}
