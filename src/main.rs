use axum::{
    error_handling::HandleErrorLayer,
    http::{header, HeaderMap, HeaderValue},
    routing::get,
    Router,
};
use tower::ServiceBuilder;
use tower_default_headers::DefaultHeadersLayer;
use tracing_subscriber;
use std::time::Duration;

pub mod articles;
mod handler;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    let mut default_headers = HeaderMap::new();
    default_headers.insert(header::X_FRAME_OPTIONS, HeaderValue::from_static("SAMEORIGIN"));
    default_headers.insert(header::STRICT_TRANSPORT_SECURITY, HeaderValue::from_static("max-age=16070400; includeSubDomains"));
    default_headers.insert(header::CONTENT_SECURITY_POLICY, HeaderValue::from_static("default-src 'self'; script-src 'nonce-20eaa6d0488e36261c88da7967d4ab0a35d7915b';"));
    default_headers.insert(header::X_XSS_PROTECTION, HeaderValue::from_static("1; mode=block"));
    default_headers.insert(header::X_CONTENT_TYPE_OPTIONS, HeaderValue::from_static("nosniff"));
            
    let app = Router::new()
        .route("/", get(handler::home))
        .route("/about", get(handler::about))
        .route("/contact", get(handler::contact))
        .route("/health", get(handler::health))
        .route("/articles/:article", get(handler::article))
        .merge(axum_extra::routing::SpaRouter::new("/assets", "./assets"))
        .layer(DefaultHeadersLayer::new(default_headers))
        .layer(
            ServiceBuilder::new()
                .layer(HandleErrorLayer::new(handler::handle_timeout_error))
                .timeout(Duration::from_secs(30)));

    axum::Server::bind(&"0.0.0.0:8080".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

