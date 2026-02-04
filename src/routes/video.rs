use axum::{
    extract::{ConnectInfo, State},
    response::{sse::Event, Sse},
    Json,
};
use serde::Deserialize;
use std::convert::Infallible;
use std::net::SocketAddr;
use std::time::Duration;
use tokio::time::sleep;
use tokio_stream::wrappers::ReceiverStream;

use crate::error::AppError;
use crate::mock;
use crate::state::AppState;

#[derive(Deserialize)]
pub struct VideoRequest {
    pub topic: String,
    pub video_type: String,
    pub duration: String,
}

pub async fn handle_video_generate(
    State(state): State<AppState>,
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    Json(req): Json<VideoRequest>,
) -> Result<Sse<ReceiverStream<Result<Event, Infallible>>>, AppError> {
    let ip = addr.ip().to_string();
    let tool = "video";

    state
        .rate_limiter
        .check_global_limit(&ip)
        .map_err(|e| AppError::RateLimited(e))?;
    state
        .rate_limiter
        .check_tool_limits(&ip, tool)
        .map_err(|e| AppError::RateLimited(e))?;

    if req.topic.trim().is_empty() {
        state.rate_limiter.record_error(&ip, tool);
        return Err(AppError::InvalidInput(
            "Topic cannot be empty".to_string(),
        ));
    }

    state.rate_limiter.increment_counters(&ip, tool);

    if state.is_demo() {
        sleep(mock::random_delay()).await;
        let response = mock::mock_video_script(&req.video_type, &req.topic, &req.duration);
        let words: Vec<String> = response
            .split_whitespace()
            .map(|w| format!("{} ", w))
            .collect();

        let (tx, rx) = tokio::sync::mpsc::channel::<Result<Event, Infallible>>(32);
        tokio::spawn(async move {
            for word in words {
                let _ = tx
                    .send(Ok(Event::default().event("message").data(word)))
                    .await;
                sleep(Duration::from_millis(30)).await;
            }
            let _ = tx
                .send(Ok(Event::default().event("done").data("{}")))
                .await;
        });

        return Ok(Sse::new(ReceiverStream::new(rx)));
    }

    // Real mode: call OpenAI API
    let body = serde_json::json!({
        "model": "gpt-4o-mini",
        "messages": [
            {
                "role": "system",
                "content": format!(
                    "Write a {} video script about the given topic. Target duration: {}. \
                     Include scene descriptions, narration, and visual cues.",
                    req.video_type, req.duration
                )
            },
            { "role": "user", "content": req.topic },
        ],
        "max_tokens": 2048,
    });

    let res = state
        .http_client
        .post("https://api.openai.com/v1/chat/completions")
        .header(
            "Authorization",
            format!("Bearer {}", state.config.openai_api_key),
        )
        .json(&body)
        .send()
        .await
        .map_err(|e| AppError::InternalError(format!("OpenAI request failed: {}", e)))?;

    let data: serde_json::Value = res
        .json()
        .await
        .map_err(|e| AppError::InternalError(format!("Failed to parse response: {}", e)))?;

    let response = data["choices"][0]["message"]["content"]
        .as_str()
        .unwrap_or("No video script generated.")
        .to_string();

    let words: Vec<String> = response
        .split_whitespace()
        .map(|w| format!("{} ", w))
        .collect();

    let (tx, rx) = tokio::sync::mpsc::channel::<Result<Event, Infallible>>(32);
    tokio::spawn(async move {
        for word in words {
            let _ = tx
                .send(Ok(Event::default().event("message").data(word)))
                .await;
            sleep(Duration::from_millis(30)).await;
        }
        let _ = tx
            .send(Ok(Event::default().event("done").data("{}")))
            .await;
    });

    Ok(Sse::new(ReceiverStream::new(rx)))
}
