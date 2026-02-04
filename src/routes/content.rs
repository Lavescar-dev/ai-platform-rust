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
pub struct ContentRequest {
    pub prompt: String,
    pub platform: String,
    pub tone: String,
}

pub async fn handle_content_generate(
    State(state): State<AppState>,
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    Json(req): Json<ContentRequest>,
) -> Result<Sse<impl Stream<Item = Result<Event, Infallible>>>, AppError> {
    let ip = addr.ip().to_string();
    let tool = "content";

    // 1. Hardened Rate Limiting
    state.rate_limiter.check_global_limit(&ip).map_err(AppError::RateLimited)?; //
    state.rate_limiter.check_tool_limits(&ip, tool).map_err(AppError::RateLimited)?; //

    // 2. Input Validation
    let prompt = req.prompt.trim();
    if prompt.is_empty() {
        state.rate_limiter.record_error(&ip, tool); //
        return Err(AppError::InvalidInput("Prompt is missing".to_string())); //
    }

    state.rate_limiter.increment_counters(&ip, tool); //

    // 3. Optimized Stream Logic
    // Demo-only stream for stable deterministic behavior.
    let stream = mock::mock_content_stream(&req.platform, &req.tone, prompt);

    // 4. Optimized SSE Response
    Ok(Sse::new(stream)
        .keep_alive(KeepAlive::default())) //
}
