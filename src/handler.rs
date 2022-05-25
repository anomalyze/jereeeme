use crate::articles::*;
use axum::{extract::Path, http::StatusCode, response::Html, BoxError};
use tracing::info;

pub async fn home() -> Html<String> {
    info!("home    | request");
    Html(Blog::Home.build().unwrap())
}

pub async fn about() -> Html<String> {
    info!("about   | request");
    Html(Blog::About.build().unwrap())
}

pub async fn contact() -> Html<String> {
    info!("contact | request");
    Html(Blog::Contact.build().unwrap())
}

/// Endpoint to provide a health check (/health)
pub async fn health() -> StatusCode {
    StatusCode::OK
}

/// Endpoint to return an article (/:article)
pub async fn article(Path(article): Path<String>) -> Html<String> {
    info!("article | request");
    Html(Blog::Articles(article).build().unwrap())
}

pub async fn handle_timeout_error(err: BoxError) -> (StatusCode, String) {
    if err.is::<tower::timeout::error::Elapsed>() {
        (
            StatusCode::REQUEST_TIMEOUT,
            "Request took too long".to_string(),
        )
    } else {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Unhandled internal error: {}", err),
        )
    }
}
