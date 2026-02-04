#[global_allocator]
static GLOBAL: mimalloc::MiMalloc = mimalloc::MiMalloc;

mod config;
mod state;
mod error;
mod rate_limiter;
mod mock;
mod routes;

use axum::{
    http::Method,
    middleware::Next,
    response::Response,
};
use config::Config;
use state::AppState;
use tower_http::cors::CorsLayer;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let config = Config::from_env();
    let rate_limit_config = if config.demo_mode {
        config::RateLimitConfig::for_demo()
    } else {
        config::RateLimitConfig::for_real_api()
    };

    tracing::info!(
        "Starting Nexus AI on port {} (demo_mode={})",
        config.port, config.demo_mode
    );

    let app_state = AppState::new(config.clone(), rate_limit_config);

    let cors = CorsLayer::permissive();

    let app = routes::create_router(app_state)
        .layer(cors)
        .layer(axum::middleware::from_fn(logging_middleware))
        .into_make_service_with_connect_info::<std::net::SocketAddr>();

    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", config.port))
        .await
        .expect("Failed to bind to port");

    tracing::info!("Server listening on http://0.0.0.0:{}", config.port);

    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await
        .expect("Server error");
}

async fn logging_middleware(
    method: Method,
    axum::extract::ConnectInfo(addr): axum::extract::ConnectInfo<std::net::SocketAddr>,
    req: axum::extract::Request,
    next: Next,
) -> Response {
    let path = req.uri().path().to_string();
    tracing::info!("{} {} from {}", method, path, addr);
    next.run(req).await
}

async fn shutdown_signal() {
    let ctrl_c = async {
        tokio::signal::ctrl_c()
            .await
            .expect("Failed to install CTRL+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        tokio::signal::unix::signal(tokio::signal::unix::SignalKind::terminate())
            .expect("Failed to install SIGTERM handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => tracing::info!("Shutdown signal received (CTRL+C)"),
        _ = terminate => tracing::info!("Shutdown signal received (SIGTERM)"),
    }
}
