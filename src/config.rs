use std::env;

#[derive(Clone, Debug)]
pub struct Config {
    pub openai_api_key: String,
    pub hf_api_token: String,
    pub port: u16,
    pub domain: String,
    pub demo_mode: bool,
}

#[derive(Clone, Copy, Debug)]
pub struct RateLimitConfig {
    pub global_daily_limit: usize,
    pub tool_daily_limit: usize,
    pub tool_minute_limit: usize,
    pub error_ban_threshold: usize,
    pub error_ban_duration_secs: u64,
    pub cleanup_interval_secs: u64,
}

impl RateLimitConfig {
    pub fn for_demo() -> Self {
        Self {
            global_daily_limit: 30,
            tool_daily_limit: 15,
            tool_minute_limit: 3,
            error_ban_threshold: 5,
            error_ban_duration_secs: 3600,
            cleanup_interval_secs: 300,
        }
    }

    pub fn for_real_api() -> Self {
        Self {
            global_daily_limit: 10,
            tool_daily_limit: 5,
            tool_minute_limit: 1,
            error_ban_threshold: 3,
            error_ban_duration_secs: 7200,
            cleanup_interval_secs: 300,
        }
    }
}

impl Config {
    pub fn from_env() -> Self {
        dotenvy::dotenv().ok();

        let demo_mode = env::var("DEMO_MODE")
            .map(|v| v != "false" && v != "0")
            .unwrap_or(true);

        Self {
            openai_api_key: env::var("OPENAI_API_KEY").unwrap_or_default(),
            hf_api_token: env::var("HF_API_TOKEN").unwrap_or_default(),
            port: env::var("PORT")
                .ok()
                .and_then(|p| p.parse().ok())
                .unwrap_or(8080),
            domain: env::var("DOMAIN")
                .unwrap_or_else(|_| "localhost".to_string()),
            demo_mode,
        }
    }
}
