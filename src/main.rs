mod semantic_hasher;

use axum::{
    routing::post,
    Router, Json, response::IntoResponse,
};
use serde_json::{Value, json};
use std::net::SocketAddr;
use semantic_hasher::SemanticHasher;

#[tokio::main]
async fn main() {
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
async fn openai_proxy_handler(Json(mut payload): Json<Value>) -> impl IntoResponse {
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
    }))
}
