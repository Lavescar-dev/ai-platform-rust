use axum::{
    extract::{ConnectInfo, State},
    Json,
};
use base64::Engine;
use serde::Deserialize;
use serde_json::json;
use std::net::SocketAddr;
use tokio::time::sleep;

use crate::error::AppError;
use crate::mock;
use crate::state::AppState;

#[derive(Deserialize)]
pub struct VoiceRequest {
    pub text: String,
    pub voice: String,
    pub rate: Option<f32>,
}

pub async fn handle_voice_synthesize(
    State(state): State<AppState>,
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    Json(req): Json<VoiceRequest>,
) -> Result<Json<serde_json::Value>, AppError> {
    let ip = addr.ip().to_string();
    let tool = "voice";

    state
        .rate_limiter
        .check_global_limit(&ip)
        .map_err(|e| AppError::RateLimited(e))?;
    state
        .rate_limiter
        .check_tool_limits(&ip, tool)
        .map_err(|e| AppError::RateLimited(e))?;

    if req.text.trim().is_empty() {
        state.rate_limiter.record_error(&ip, tool);
        return Err(AppError::InvalidInput(
            "Text cannot be empty".to_string(),
        ));
    }

    state.rate_limiter.increment_counters(&ip, tool);

    if state.is_demo() {
        sleep(mock::random_delay()).await;
        let audio_bytes = mock::mock_voice_audio();
        let base64_audio = base64::engine::general_purpose::STANDARD.encode(&audio_bytes);
        return Ok(Json(json!({
            "audio": format!("data:audio/mpeg;base64,{}", base64_audio),
            "demo": true,
        })));
    }

    // Real mode: call an external TTS API
    let _rate = req.rate.unwrap_or(1.0);
    let res = state
        .http_client
        .post("https://api.openai.com/v1/audio/speech")
        .header(
            "Authorization",
            format!("Bearer {}", state.config.openai_api_key),
        )
        .json(&json!({
            "model": "tts-1",
            "input": req.text,
            "voice": "alloy",
        }))
        .send()
        .await
        .map_err(|e| AppError::InternalError(format!("TTS request failed: {}", e)))?;

    let bytes = res
        .bytes()
        .await
        .map_err(|e| AppError::InternalError(format!("Failed to read audio bytes: {}", e)))?;

    let base64_audio = base64::engine::general_purpose::STANDARD.encode(&bytes);

    Ok(Json(json!({
        "audio": format!("data:audio/mpeg;base64,{}", base64_audio),
        "demo": false,
    })))
}

pub async fn handle_voice_list(
    State(_state): State<AppState>,
) -> Json<serde_json::Value> {
    let voices = &*mock::VOICE_LIST;
    Json(json!({
        "voices": voices,
        "demo": true,
    }))
}
