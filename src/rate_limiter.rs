use dashmap::DashMap;
use serde_json::json;
use std::net::Ipv4Addr;
use std::sync::Arc;
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Clone)]
pub struct RateLimiter {
    // String yerine u64 (IP + ToolID + TimeBucket) kullanılarak heap allocation sıfırlandı.
    daily_limits: Arc<DashMap<u64, usize>>,
    tool_daily_limits: Arc<DashMap<u64, usize>>,
    tool_minute_limits: Arc<DashMap<u64, usize>>,
    banned_ips: Arc<DashMap<u32, u64>>,
    
    // Config limitleri
    global_daily_limit: usize,
    tool_daily_limit: usize,
    tool_minute_limit: usize,
    error_ban_threshold: usize,
    error_ban_duration_secs: u64,
}

impl RateLimiter {
    pub fn new(config: &crate::config::RateLimitConfig) -> Self {
        Self {
            daily_limits: Arc::new(DashMap::new()),
            tool_daily_limits: Arc::new(DashMap::new()),
            tool_minute_limits: Arc::new(DashMap::new()),
            banned_ips: Arc::new(DashMap::new()),
            global_daily_limit: config.global_daily_limit,
            tool_daily_limit: config.tool_daily_limit, // Varsayılan değerler
            tool_minute_limit: config.tool_minute_limit,
            error_ban_threshold: config.error_ban_threshold,
            error_ban_duration_secs: config.error_ban_duration_secs,
        }
    }

    // --- Yardımcı Fonksiyonlar (Zero-Allocation) ---

    fn parse_ip(ip: &str) -> u32 {
        ip.parse::<Ipv4Addr>().map(|addr| u32::from(addr)).unwrap_or(0)
    }

    fn get_day_bucket() -> u32 {
        let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
        (now / 86400) as u32 // Günlük bucket
    }

    fn get_min_bucket() -> u32 {
        let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
        (now / 60) as u32 // Dakikalık bucket
    }

    /// Bit-packing: [IP (32b) | ToolID (8b) | Time (24b)]
    fn generate_key(ip: u32, tool_id: u8, time_bucket: u32) -> u64 {
        ((ip as u64) << 32) | ((tool_id as u64) << 24) | ((time_bucket & 0xFFFFFF) as u64)
    }

    fn get_tool_id(tool: &str) -> u8 {
        match tool {
            "chat" => 1, "content" => 2, "code" => 3, "image" => 4,
            "voice" => 5, "resume" => 6, "email" => 7, "video" => 8,
            "seo" => 9, "bot" => 10,
            _ => 0,
        }
    }

    // --- API ---

    pub fn is_ip_banned(&self, ip_str: &str) -> bool {
        let ip = Self::parse_ip(ip_str);
        if let Some(ban_until) = self.banned_ips.get(&ip) {
            let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
            if now < *ban_until {
                return true;
            }
            self.banned_ips.remove(&ip);
        }
        false
    }

    pub fn check_global_limit(&self, ip_str: &str) -> Result<(), String> {
        let ip = Self::parse_ip(ip_str);
        let key = Self::generate_key(ip, 0, Self::get_day_bucket());
        
        let used = self.daily_limits.get(&key).map(|v| *v).unwrap_or(0);
        if used >= self.global_daily_limit {
            return Err(format!("Global daily limit exceeded ({})", self.global_daily_limit));
        }
        Ok(())
    }

    pub fn check_tool_limits(&self, ip_str: &str, tool: &str) -> Result<(), String> {
        let ip = Self::parse_ip(ip_str);
        let tid = Self::get_tool_id(tool);
        let day = Self::get_day_bucket();
        let min = Self::get_min_bucket();

        // Daily Check
        let d_key = Self::generate_key(ip, tid, day);
        if self.tool_daily_limits.get(&d_key).map(|v| *v).unwrap_or(0) >= self.tool_daily_limit {
            return Err("Tool daily limit exceeded".to_string());
        }

        // Minute Check
        let m_key = Self::generate_key(ip, tid, min);
        if self.tool_minute_limits.get(&m_key).map(|v| *v).unwrap_or(0) >= self.tool_minute_limit {
            return Err("Too many requests per minute".to_string());
        }

        Ok(())
    }

    pub fn increment_counters(&self, ip_str: &str, tool: &str) {
        let ip = Self::parse_ip(ip_str);
        let tid = Self::get_tool_id(tool);

        // Global Daily Inc
        let g_key = Self::generate_key(ip, 0, Self::get_day_bucket());
        *self.daily_limits.entry(g_key).or_insert(0) += 1;

        // Tool Daily Inc
        let d_key = Self::generate_key(ip, tid, Self::get_day_bucket());
        *self.tool_daily_limits.entry(d_key).or_insert(0) += 1;

        // Tool Minute Inc
        let m_key = Self::generate_key(ip, tid, Self::get_min_bucket());
        *self.tool_minute_limits.entry(m_key).or_insert(0) += 1;
    }

    pub fn record_error(&self, ip_str: &str, _tool: &str) {
        let ip = Self::parse_ip(ip_str);
        let min = Self::get_min_bucket();
        let err_key = Self::generate_key(ip, 255, min); // 255 is error ID

        let count = {
            let mut entry = self.tool_minute_limits.entry(err_key).or_insert(0);
            *entry += 1;
            *entry
        };

        if count >= self.error_ban_threshold {
            let ban_until = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs() + self.error_ban_duration_secs;
            self.banned_ips.insert(ip, ban_until);
        }
    }

    pub fn get_remaining(&self, ip_str: &str, tool: &str) -> serde_json::Value {
        let ip = Self::parse_ip(ip_str);
        let tid = Self::get_tool_id(tool);
        let day = Self::get_day_bucket();
        let min = Self::get_min_bucket();

        let global_used = self
            .daily_limits
            .get(&Self::generate_key(ip, 0, day))
            .map(|v| *v)
            .unwrap_or(0);
        let tool_daily_used = self
            .tool_daily_limits
            .get(&Self::generate_key(ip, tid, day))
            .map(|v| *v)
            .unwrap_or(0);
        let tool_minute_used = self
            .tool_minute_limits
            .get(&Self::generate_key(ip, tid, min))
            .map(|v| *v)
            .unwrap_or(0);

        json!({
            "global_daily": {
                "used": global_used,
                "limit": self.global_daily_limit,
                "remaining": self.global_daily_limit.saturating_sub(global_used),
            },
            "tool_daily": {
                "used": tool_daily_used,
                "limit": self.tool_daily_limit,
                "remaining": self.tool_daily_limit.saturating_sub(tool_daily_used),
            },
            "tool_minute": {
                "used": tool_minute_used,
                "limit": self.tool_minute_limit,
                "remaining": self.tool_minute_limit.saturating_sub(tool_minute_used),
            },
            "banned": self.is_ip_banned(ip_str),
        })
    }
}
