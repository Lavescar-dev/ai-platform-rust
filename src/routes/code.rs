use axum::{
    extract::{ConnectInfo, State},
    response::sse::{Event, KeepAlive, Sse},
    Json,
};
use futures::Stream;
use serde::Deserialize;
use std::{convert::Infallible, net::SocketAddr};

use crate::error::AppError;
use crate::mock;
use crate::state::AppState;

#[derive(Deserialize)]
pub struct CodeRequest {
    pub description: String,
    pub language: String,
    pub mode: Option<String>,
}

pub async fn handle_code_generate(
    State(state): State<AppState>,
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    Json(req): Json<CodeRequest>,
) -> Result<Sse<impl Stream<Item = Result<Event, Infallible>>>, AppError> {
    let ip = addr.ip().to_string();
    let tool = "code";

    // 1. Hardened Rate Limiting
    state.rate_limiter.check_global_limit(&ip).map_err(AppError::RateLimited)?;
    state.rate_limiter.check_tool_limits(&ip, tool).map_err(AppError::RateLimited)?;

    // 2. Input Validation
    let description = req.description.trim();
    if description.is_empty() {
        state.rate_limiter.record_error(&ip, tool);
        return Err(AppError::InvalidInput("Code description is missing".to_string()));
    }

    state.rate_limiter.increment_counters(&ip, tool);

    // 3. Unified Stream Logic
    // Demo-only stream for stable deterministic behavior.
    let stream = mock::mock_code_stream(&req.language, description);

    // 4. Optimized SSE Response
    Ok(Sse::new(stream)
        .keep_alive(KeepAlive::default())) // Bypass Nginx proxy buffering
}
