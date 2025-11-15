use once_cell::sync::Lazy;
use prometheus::{
    register_histogram_vec, register_int_counter_vec, register_int_gauge, HistogramVec,
    IntCounterVec, IntGauge,
};

/// Prometheus metrics
pub static REQUESTS_TOTAL: Lazy<IntCounterVec> = Lazy::new(|| {
    register_int_counter_vec!(
        "ussd_requests_total",
        "Total number of USSD requests",
        &["method", "endpoint", "status"]
    )
    .unwrap()
});

pub static SESSIONS_TOTAL: Lazy<IntCounterVec> = Lazy::new(|| {
    register_int_counter_vec!(
        "ussd_sessions_total",
        "Total number of USSD sessions created",
        &["service_code"]
    )
    .unwrap()
});

pub static ACTIVE_SESSIONS: Lazy<IntGauge> = Lazy::new(|| {
    register_int_gauge!(
        "ussd_active_sessions",
        "Number of currently active sessions"
    )
    .unwrap()
});

pub static REQUEST_DURATION: Lazy<HistogramVec> = Lazy::new(|| {
    register_histogram_vec!(
        "ussd_request_duration_seconds",
        "Request latency in seconds",
        &["method", "endpoint"],
        vec![0.001, 0.005, 0.01, 0.025, 0.05, 0.1, 0.25, 0.5, 1.0, 2.5, 5.0]
    )
    .unwrap()
});

pub static ERRORS_TOTAL: Lazy<IntCounterVec> = Lazy::new(|| {
    register_int_counter_vec!(
        "ussd_errors_total",
        "Total number of errors",
        &["error_type", "endpoint"]
    )
    .unwrap()
});

pub static PLUGIN_EXECUTIONS: Lazy<IntCounterVec> = Lazy::new(|| {
    register_int_counter_vec!(
        "ussd_plugin_executions_total",
        "Total number of plugin executions",
        &["plugin_name", "status"]
    )
    .unwrap()
});

pub static PLUGIN_DURATION: Lazy<HistogramVec> = Lazy::new(|| {
    register_histogram_vec!(
        "ussd_plugin_duration_seconds",
        "Plugin execution duration",
        &["plugin_name"],
        vec![0.001, 0.005, 0.01, 0.025, 0.05, 0.1, 0.25, 0.5, 1.0]
    )
    .unwrap()
});

pub static DB_OPERATIONS: Lazy<IntCounterVec> = Lazy::new(|| {
    register_int_counter_vec!(
        "ussd_db_operations_total",
        "Total number of database operations",
        &["operation", "status"]
    )
    .unwrap()
});

pub static REDIS_OPERATIONS: Lazy<IntCounterVec> = Lazy::new(|| {
    register_int_counter_vec!(
        "ussd_redis_operations_total",
        "Total number of Redis operations",
        &["operation", "status"]
    )
    .unwrap()
});
