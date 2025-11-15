use axum::{
    extract::Request,
    middleware::Next,
    response::Response,
};
use governor::{
    clock::DefaultClock,
    state::{InMemoryState, NotKeyed},
    Quota, RateLimiter,
};
use std::sync::Arc;
use std::num::NonZeroU32;

pub type GlobalRateLimiter = Arc<RateLimiter<NotKeyed, InMemoryState, DefaultClock>>;

pub fn create_rate_limiter(requests: u32, _window_secs: u64) -> GlobalRateLimiter {
    let quota = Quota::per_second(NonZeroU32::new(requests).unwrap())
        .allow_burst(NonZeroU32::new(requests * 2).unwrap());

    Arc::new(RateLimiter::direct(quota))
}

pub async fn rate_limit_middleware(
    req: Request,
    next: Next,
) -> Response {
    // In production, extract limiter from app state
    // For now, this is a placeholder
    next.run(req).await
}
