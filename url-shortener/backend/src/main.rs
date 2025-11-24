use axum::{
    Router,
    http::HeaderValue,
    response::Json,
    routing::{get, post},
};
use serde_json::{Value, json};
use std::sync::{Arc, Mutex};
use tower_http::cors::{Any, CorsLayer};

mod api_response;
mod url_api;
mod url_core;

use url_api::{create_short_url, get_short_url};
use url_core::UrlService;

#[tokio::main]
async fn main() {
    let urls = Arc::new(Mutex::new(UrlService::new()));
    tracing_subscriber::fmt().init();

    let cors = CorsLayer::new()
        .allow_origin("http://localhost:5173".parse::<HeaderValue>().unwrap())
        .allow_methods(Any)
        .allow_headers(Any);

    let app = Router::new()
        .route("/health", get(health_check))
        .route("/url", post(create_short_url))
        .route("/url", get(get_short_url))
        .layer(cors)
        .with_state(urls);

    let listrner = tokio::net::TcpListener::bind("0.0.0.0:8000")
        .await
        .expect("Failed to start listener on port 8000");
    axum::serve(listrner, app).await.unwrap();
}

async fn health_check() -> Json<Value> {
    Json(json!({
        "status": "ok",
        "message": "Server is running"
    }))
}
