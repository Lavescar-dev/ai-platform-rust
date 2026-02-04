use axum::{
    body::Body,
    extract::{ConnectInfo, State},
    http::{header, Response},
    Json,
};
use serde::Deserialize;
use std::net::SocketAddr;
use tokio::time::sleep;

use crate::error::AppError;
use crate::mock;
use crate::state::AppState;

#[derive(Deserialize)]
pub struct ImageRequest {
    pub prompt: String,
}

pub async fn handle_image_generate(
    State(state): State<AppState>,
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    Json(req): Json<ImageRequest>,
) -> Result<Response<Body>, AppError> {
    let ip = addr.ip().to_string();
    let tool = "image";

    // 1. Rate Limit Checks - No manual map_err required for clarity
    state.rate_limiter.check_global_limit(&ip).map_err(AppError::RateLimited)?;
    state.rate_limiter.check_tool_limits(&ip, tool).map_err(AppError::RateLimited)?;

    // 2. Input Validation
    let prompt = req.prompt.trim();
    if prompt.is_empty() {
        state.rate_limiter.record_error(&ip, tool);
        return Err(AppError::InvalidInput("Prompt cannot be empty".to_string()));
    }

    state.rate_limiter.increment_counters(&ip, tool);

    // 3. Binary Byte Acquisition
    let image_bytes = if state.is_demo() {
        sleep(mock::random_delay()).await;
        // Mock veriyi Base64'ten binary'ye geri çeviriyoruz (veya mock modülünü güncelleyebilirsin)
        mock::mock_image_bytes() 
    } else {
        let res = state
            .http_client
            .post("https://api-inference.huggingface.co/models/stabilityai/stable-diffusion-xl-base-1.0")
            .header(header::AUTHORIZATION, format!("Bearer {}", state.config.hf_api_token))
            .json(&serde_json::json!({ "inputs": prompt }))
            .send()
            .await
            .map_err(|e| AppError::InternalError(format!("HuggingFace request failed: {}", e)))?;

        if !res.status().is_success() {
             return Err(AppError::InternalError(format!("HF API returned error: {}", res.status())));
        }

        res.bytes()
            .await
            .map_err(|e| AppError::InternalError(format!("Failed to read image bytes: {}", e)))?
            .to_vec()
    };

    // 4. Return Direct Binary Response
    // Base64 encoding is removed to reduce payload size by ~33%.
    Ok(Response::builder()
        .status(200)
        .header(header::CONTENT_TYPE, "image/png")
        // Browser caching for 1 hour to reduce redundant generation requests
        .header(header::CACHE_CONTROL, "public, max-age=3600")
        .body(Body::from(image_bytes))
        .unwrap())
}
