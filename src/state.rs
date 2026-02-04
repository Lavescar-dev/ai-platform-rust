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
        let daily_limits = Arc::new(DashMap::new());
        let tool_daily_limits = Arc::new(DashMap::new());
        let tool_minute_limits = Arc::new(DashMap::new());
        let banned_ips = Arc::new(DashMap::new());

        let rate_limiter = RateLimiter::new(
            daily_limits,
            tool_daily_limits,
            tool_minute_limits,
            banned_ips,
            rate_limit_config.global_daily_limit,
            rate_limit_config.tool_daily_limit,
            rate_limit_config.tool_minute_limit,
            rate_limit_config.error_ban_threshold,
            rate_limit_config.error_ban_duration_secs,
        );

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
