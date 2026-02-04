use ax_sse::{Event, Sse};
use axum::{
    extract::{ConnectInfo, State},
    response::sse::KeepAlive,
    Json,
};
use futures_util::{Stream, StreamExt};
use serde::Deserialize;
use std::{convert::Infallible, net::SocketAddr};

use crate::error::AppError;
use crate::mock;
use crate::state::AppState;

#[derive(Deserialize)]
pub struct ChatRequest {
    pub message: String,
}

pub async fn handle_chat_stream(
    State(state): State<AppState>,
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    Json(req): Json<ChatRequest>,
) -> Result<Sse<impl Stream<Item = Result<Event, Infallible>>>, AppError> {
    let ip = addr.ip().to_string();
    let tool = "chat";

    // 1. Hardened Rate Limiting
    state.rate_limiter.check_global_limit(&ip).map_err(AppError::RateLimited)?;
    state.rate_limiter.check_tool_limits(&ip, tool).map_err(AppError::RateLimited)?;

    // 2. Input Validation
    let message = req.message.trim();
    if message.is_empty() {
        state.rate_limiter.record_error(&ip, tool);
        return Err(AppError::InvalidInput("Message cannot be empty".to_string()));
    }

    state.rate_limiter.increment_counters(&ip, tool);

    // 3. Unified Stream Logic
    let stream = if state.is_demo() {
        // Demo Mode: Mock asenkron akış
        mock::mock_chat_stream(message).boxed()
    } else {
        // Real Mode: Zero-latency pipe from OpenAI to Client
        let system_prompt = "You are Nexus AI, a helpful assistant.";

        state.ai_client
            .stream_chat(system_prompt, message)
            .await
            .map_err(|e| AppError::InternalError(e.to_string()))?
            .map(|token| Ok(Event::default().data(token)))
            .boxed()
    };

    // 4. Optimized SSE Response
    Ok(Sse::new(stream)
        .keep_alive(KeepAlive::default())
        // Nginx'in stream'i tamponlamasını (buffering) engellemek için kritik header
        .header("X-Accel-Buffering", "no"))
}
