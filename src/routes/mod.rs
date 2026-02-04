pub mod landing;
pub mod chat;
pub mod limits;
pub mod content;
pub mod code;
pub mod email;
pub mod video;
pub mod seo;
pub mod image;
pub mod voice;
pub mod resume;
pub mod bot;

use axum::{
    extract::Request,
    http::HeaderValue,
    middleware::{self, Next},
    response::Response,
    routing::{get, post},
    Router,
};
use crate::state::AppState;

/// Middleware: Platformun otonom durumunu header seviyesinde fısıldar.
async fn demo_header_middleware(state: axum::extract::State<AppState>, req: Request, next: Next) -> Response {
    let mut response = next.run(req).await;
    
    // Hardcoded yerine gerçek config durumunu yansıtıyoruz.
    let mode = if state.config.demo_mode {
        HeaderValue::from_static("true")
    } else {
        HeaderValue::from_static("false")
    };

    response.headers_mut().insert(
        "X-Demo-Mode",
        mode,
    );
    response
}

pub fn create_router(state: AppState) -> Router {
    // 1. Landing Shell: Tüm sayfa isteklerini tek bir handler'a bağla.
    // 'landing::serve_index' artık tüm path'ler için tek giriş noktası (Entry Point).
    let landing_routes = Router::new()
        .route("/", get(landing::serve_index))
        .route("/chat", get(landing::serve_index))
        .route("/content", get(landing::serve_index))
        .route("/code", get(landing::serve_index))
        .route("/email", get(landing::serve_index))
        .route("/video", get(landing::serve_index))
        .route("/seo", get(landing::serve_index))
        .route("/image", get(landing::serve_index))
        .route("/voice", get(landing::serve_index))
        .route("/resume", get(landing::serve_index))
        .route("/bot", get(landing::serve_index));

    // 2. API Aggregator: Araç bazlı asenkron handler'ların merkezi.
    let api_routes = Router::new()
        .route("/chat/api/chat", post(chat::handle_chat_stream))
        .route("/chat/api/limits", get(limits::get_limits))
        .route("/content/api/generate", post(content::handle_content_generate))
        .route("/code/api/generate", post(code::handle_code_generate))
        .route("/email/api/generate", post(email::handle_email_generate))
        .route("/video/api/generate", post(video::handle_video_generate))
        .route("/seo/api/analyze", post(seo::handle_seo_analyze))
        .route("/image/api/generate", post(image::handle_image_generate))
        .route("/voice/api/synthesize", post(voice::handle_voice_synthesize))
        .route("/voice/api/voices", get(voice::handle_voice_list))
        .route("/resume/api/generate", post(resume::handle_resume_generate))
        .route("/bot/api/create", post(bot::handle_bot_create))
        .route("/bot/api/chat/:bot_id", post(bot::handle_bot_chat));

    // 3. Construct Final Router
    Router::new()
        .merge(landing_routes)
        .merge(api_routes)
        // State-aware middleware kullanarak dinamik header ekliyoruz.
        .layer(middleware::from_fn_with_state(state.clone(), demo_header_middleware))
        .with_state(state)
}
