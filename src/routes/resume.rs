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
pub struct ResumeRequest {
    pub name: String,
    pub experience: String,
    pub skills: String,
}

pub async fn handle_resume_generate(
    State(state): State<AppState>,
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    Json(req): Json<ResumeRequest>,
) -> Result<Sse<impl Stream<Item = Result<Event, Infallible>>>, AppError> {
    let ip = addr.ip().to_string();
    let tool = "resume";

    // 1. Guard & Rate Limit
    state.rate_limiter.check_global_limit(&ip).map_err(AppError::RateLimited)?;
    state.rate_limiter.check_tool_limits(&ip, tool).map_err(AppError::RateLimited)?;

    if req.name.trim().is_empty() {
        state.rate_limiter.record_error(&ip, tool);
        return Err(AppError::InvalidInput("Name is mandatory".to_string()));
    }

    state.rate_limiter.increment_counters(&ip, tool);

    // 2. Stream Generation
    let stream = if state.is_demo() {
        // Demo modu: Mock veriyi asenkron akışa çevir
        mock::mock_resume_stream(&req.name, &req.experience, &req.skills).boxed()
    } else {
        // Real Mode: OpenAI'dan gelen stream'i doğrudan map et (Zero-copy)
        let prompt = format!("Name: {}\nExp: {}\nSkills: {}", req.name, req.experience, req.skills);
        
        state.ai_client
            .stream_chat("You are a professional resume writer. Output Markdown.", &prompt)
            .await
            .map_err(|e| AppError::InternalError(e.to_string()))?
            .map(|token| Ok(Event::default().data(token)))
            .boxed()
    };

    // 3. Optimized SSE Response
    Ok(Sse::new(stream)
        .keep_alive(KeepAlive::default())
        // Nginx'in stream'i tamponlamasını (buffering) engellemek için kritik header
        .header("X-Accel-Buffering", "no"))
}
