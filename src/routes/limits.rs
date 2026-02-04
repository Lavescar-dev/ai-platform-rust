use axum::{
    extract::{ConnectInfo, Query, State},
    Json,
};
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;

use crate::state::AppState;

#[derive(Deserialize)]
pub struct LimitQuery {
    pub tool: Option<String>,
}

#[derive(Serialize)]
pub struct LimitResponse {
    pub remaining: serde_json::Value,
    pub demo: bool,
}

/// Tüm platform araçları için merkezi limit sorgu motoru.
/// Query parametresi ile her tool için dinamik telemetri sağlar.
pub async fn get_limits(
    State(state): State<AppState>,
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    Query(params): Query<LimitQuery>,
) -> Json<LimitResponse> {
    let ip = addr.ip().to_string();
    
    // Default olarak 'chat' döner, ancak query parametresi ile dinamikleşir.
    let tool = params.tool.as_deref().unwrap_or("chat");

    // 1. Data Retrieval: Redis/DashMap üzerinden ham veriyi çek
    let remaining = state.rate_limiter.get_remaining(&ip, tool);

    // 2. Type-Safe Response: Manuel JSON manipülasyonu yerine struct kullanımı.
    // 'demo' bilgisi doğrudan konfigürasyondan (Source of Truth) çekilir.
    Json(LimitResponse {
        remaining,
        demo: state.config.demo_mode,
    })
}
