use crate::config::{Config, RateLimitConfig};
use crate::rate_limiter::RateLimiter;
use dashmap::DashMap;
use std::sync::Arc;

#[derive(Clone)]
pub struct AppState {
    pub config: Config,
    pub rate_limit_config: RateLimitConfig,
    pub http_client: reqwest::Client,
    pub rate_limiter: RateLimiter,
    pub bots: Arc<DashMap<String, serde_json::Value>>,
}

impl AppState {
    pub fn new(config: Config, rate_limit_config: RateLimitConfig) -> Self {
        let rate_limiter = RateLimiter::new(&rate_limit_config);

        Self {
            config,
            rate_limit_config,
            http_client: reqwest::Client::new(),
            rate_limiter,
            bots: Arc::new(DashMap::new()),
        }
    }

    pub fn is_demo(&self) -> bool {
        self.config.demo_mode
    }
}
