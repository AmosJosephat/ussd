use axum::{
    extract::Request,
    middleware::Next,
    response::Response,
};
use std::time::Instant;
use crate::metrics::{REQUEST_DURATION, REQUESTS_TOTAL};

pub async fn metrics_middleware(req: Request, next: Next) -> Response {
    let method = req.method().to_string();
    let path = req.uri().path().to_string();
    let start = Instant::now();

    let response = next.run(req).await;

    let duration = start.elapsed();
    let status = response.status().as_u16().to_string();

    // Record metrics
    REQUESTS_TOTAL
        .with_label_values(&[&method, &path, &status])
        .inc();

    REQUEST_DURATION
        .with_label_values(&[&method, &path])
        .observe(duration.as_secs_f64());

    response
}
