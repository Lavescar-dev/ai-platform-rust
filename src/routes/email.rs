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
pub struct EmailRequest {
    pub subject: String,
    pub email_type: String,
    pub tone: String,
}

pub async fn handle_email_generate(
    State(state): State<AppState>,
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    Json(req): Json<EmailRequest>,
) -> Result<Sse<impl Stream<Item = Result<Event, Infallible>>>, AppError> {
    let ip = addr.ip().to_string();
    let tool = "email";

    // 1. Hardened Rate Limiting
    state.rate_limiter.check_global_limit(&ip).map_err(AppError::RateLimited)?;
    state.rate_limiter.check_tool_limits(&ip, tool).map_err(AppError::RateLimited)?;

    // 2. Input Validation
    let subject = req.subject.trim();
    if subject.is_empty() {
        state.rate_limiter.record_error(&ip, tool);
        return Err(AppError::InvalidInput("Email subject is mandatory".to_string()));
    }

    state.rate_limiter.increment_counters(&ip, tool);

    // 3. Optimized Stream Logic
    let stream = if state.is_demo() {
        // Demo Mode: Mock veriyi asenkron akışa çevir
        mock::mock_email_stream(&req.email_type, subject, &req.tone).boxed()
    } else {
        // Real Mode: OpenAI'dan gelen stream'i doğrudan pipe et (Zero-copy)
        let system_prompt = format!(
            "Write a {} email with a {} tone. Be professional and well-structured.",
            req.email_type, req.tone
        );
        let user_content = format!("Subject: {}", subject);

        state.ai_client
            .stream_chat(&system_prompt, &user_content)
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
