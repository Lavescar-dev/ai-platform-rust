use axum::{
    extract::{ConnectInfo, Path, State},
    response::sse::{Event, KeepAlive, Sse},
    Json,
};
use futures::Stream;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::{convert::Infallible, net::SocketAddr};

use crate::error::AppError;
use crate::mock;
use crate::state::AppState;

#[derive(Deserialize, Serialize)]
pub struct BotCreateRequest {
    pub name: String,
    pub system_prompt: String,
    pub welcome_message: String,
}

#[derive(Deserialize)]
pub struct BotChatRequest {
    pub message: String,
}

pub async fn handle_bot_create(
    State(state): State<AppState>,
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    Json(req): Json<BotCreateRequest>,
) -> Result<Json<serde_json::Value>, AppError> {
    let ip = addr.ip().to_string();
    let tool = "bot";

    state.rate_limiter.check_global_limit(&ip).map_err(AppError::RateLimited)?;
    state.rate_limiter.check_tool_limits(&ip, tool).map_err(AppError::RateLimited)?;

    if req.name.trim().is_empty() {
        state.rate_limiter.record_error(&ip, tool);
        return Err(AppError::InvalidInput("Bot name cannot be empty".to_string()));
    }

    state.rate_limiter.increment_counters(&ip, tool);

    let bot_id = format!("bot_{}", chrono::Local::now().timestamp_millis());
    let bot_data = json!({
        "id": bot_id,
        "name": req.name,
        "system_prompt": req.system_prompt,
        "welcome_message": req.welcome_message,
        "created_at": chrono::Local::now().to_rfc3339(),
    });

    state.bots.insert(bot_id.clone(), bot_data);

    Ok(Json(json!({
        "bot_id": bot_id,
        "message": "Bot created successfully",
        "demo": state.is_demo(),
    })))
}

pub async fn handle_bot_chat(
    State(state): State<AppState>,
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    Path(bot_id): Path<String>,
    Json(req): Json<BotChatRequest>,
) -> Result<Sse<impl Stream<Item = Result<Event, Infallible>>>, AppError> {
    let ip = addr.ip().to_string();
    let tool = "bot";

    state.rate_limiter.check_global_limit(&ip).map_err(AppError::RateLimited)?;
    state.rate_limiter.check_tool_limits(&ip, tool).map_err(AppError::RateLimited)?;

    if req.message.trim().is_empty() {
        state.rate_limiter.record_error(&ip, tool);
        return Err(AppError::InvalidInput("Message cannot be empty".to_string()));
    }

    // DashMap referansını scope içinde tutarak hızlıca serbest bırakıyoruz (Deadlock önleme)
    let persona = {
        let bot = state.bots.get(&bot_id)
            .ok_or_else(|| AppError::NotFound(format!("Bot {} not found", bot_id)))?;
        bot.get("system_prompt")
            .and_then(|v| v.as_str())
            .unwrap_or("helpful assistant")
            .to_string()
    };

    state.rate_limiter.increment_counters(&ip, tool);

    // Demo-only stream for stable deterministic behavior.
    let stream = mock::mock_bot_stream(&persona, &req.message);

    Ok(Sse::new(stream)
        .keep_alive(KeepAlive::default()))
}
