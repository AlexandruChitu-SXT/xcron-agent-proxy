mod semantic_hasher;

use axum::{
    routing::post,
    Router, Json, response::IntoResponse,
    http::{StatusCode, HeaderMap},
};
use serde_json::{Value, json};
use std::net::SocketAddr;
use semantic_hasher::SemanticHasher;

#[tokio::main]
async fn main() {
    // Cargar variables de entorno (Producción y Local)
    dotenvy::dotenv().ok();

    // Initialize tracing/logging
    tracing_subscriber::fmt::init();

    // Create the Axum async routing app
    let app = Router::new()
        .route("/v1/chat/completions", post(openai_proxy_handler));

    let addr = SocketAddr::from(([127, 0, 0, 1], 8089));
    tracing::info!("🚀 XCron Mainnet Semantic Proxy running on {}", addr);
    tracing::info!("Listening for A2A /tools/invoke API requests natively using Tokio...");

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

/// The ultra-low latency interceptor using Axum
async fn openai_proxy_handler(headers: HeaderMap, Json(mut payload): Json<Value>) -> impl IntoResponse {
    // 🛡️ WEB2 RELAYER DEFENSE: Load API key from environment to prevent secrets in git
    let valid_api_key = std::env::var("PROXY_AUTH_TOKEN")
        .unwrap_or_else(|_| "Bearer XCRON_LOCAL_AGENT_SECURE_2026".to_string());

    let is_authorized = headers
        .get("Authorization")
        .and_then(|h| h.to_str().ok())
        .map(|s| s == valid_api_key)
        .unwrap_or(false);

    if !is_authorized {
        tracing::warn!("🚨 [SECURITY BLOCK] Unauthorized payload attempt rejected! Relayer Gas protected.");
        return (
            StatusCode::UNAUTHORIZED,
            Json(json!({
                "status": "error",
                "message": "Unauthorized Access. XCron Shield Active."
            }))
        ).into_response();
    }
    let hasher = SemanticHasher::new();

    tracing::info!("--- [INCOMING] Payload intercepted ---");
    // println!("{}", serde_json::to_string_pretty(&payload).unwrap());

    // Securely transform the payload using the AST parser
    if let Some(messages) = payload.get_mut("messages") {
        if let Some(arr) = messages.as_array_mut() {
            if let Some(first_msg) = arr.get_mut(0) {
                if let Some(content) = first_msg.get("content") {
                    let hashed_ast = hasher.hash(content.clone());
                    first_msg["content"] = hashed_ast;
                }
            }
        }
    } else {
        // Fallback for direct agent wrapper tests
        payload = hasher.hash(payload);
    }

    tracing::info!("--- [OUTGOING] Compressed Payload to OpenAI API ---");
    // println!("{}", serde_json::to_string_pretty(&payload).unwrap());

    // In Mainnet production: forward the `payload` to https://api.openai.com/v1/chat/completions
    // and stream the bytes back using reqwest. Here we return the compressed JSON to verify.
    
    Json(json!({
        "status": "success",
        "message": "Proxy interception and Rust AST compression successful.",
        "compressed_sent": payload
    })).into_response()
}
