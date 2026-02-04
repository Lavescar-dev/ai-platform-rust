use axum::{
    extract::{ConnectInfo, State},
    Json,
};
use serde::Deserialize;
use serde_json::json;
use std::net::SocketAddr;
use tokio::time::sleep;

use crate::error::AppError;
use crate::mock;
use crate::state::AppState;

#[derive(Deserialize)]
pub struct SeoRequest {
    pub content: String,
    pub analysis_type: Option<String>,
}

pub async fn handle_seo_analyze(
    State(state): State<AppState>,
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    Json(req): Json<SeoRequest>,
) -> Result<Json<serde_json::Value>, AppError> {
    let ip = addr.ip().to_string();
    let tool = "seo";

    state
        .rate_limiter
        .check_global_limit(&ip)
        .map_err(|e| AppError::RateLimited(e))?;
    state
        .rate_limiter
        .check_tool_limits(&ip, tool)
        .map_err(|e| AppError::RateLimited(e))?;

    if req.content.trim().is_empty() {
        state.rate_limiter.record_error(&ip, tool);
        return Err(AppError::InvalidInput(
            "Content cannot be empty".to_string(),
        ));
    }

    state.rate_limiter.increment_counters(&ip, tool);

    if state.is_demo() {
        sleep(mock::random_delay()).await;
        let mut report = mock::mock_seo_report(&req.content);
        if let Some(obj) = report.as_object_mut() {
            obj.insert("demo".to_string(), json!(true));
        }
        return Ok(Json(report));
    }

    // Real mode: call OpenAI API for SEO analysis
    let body = json!({
        "model": "gpt-4o-mini",
        "messages": [
            {
                "role": "system",
                "content": "You are an SEO expert. Analyze the provided content and return a JSON \
                    report with these fields: overall_score (0-100), keyword_density, readability_score, \
                    meta_suggestions, content_suggestions, technical_issues. Return valid JSON only."
            },
            { "role": "user", "content": req.content },
        ],
        "max_tokens": 1024,
        "response_format": { "type": "json_object" },
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

    let content_str = data["choices"][0]["message"]["content"]
        .as_str()
        .unwrap_or("{}");

    let mut report: serde_json::Value =
        serde_json::from_str(content_str).unwrap_or_else(|_| json!({ "error": "Failed to parse SEO report" }));

    if let Some(obj) = report.as_object_mut() {
        obj.insert("demo".to_string(), json!(false));
    }

    Ok(Json(report))
}
