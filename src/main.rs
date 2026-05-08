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
        .route("/v1/chat/completions", post(openai_proxy_handler))
        .route("/v1/agent/intent", post(agent_intent_handler))
        .layer(axum::extract::DefaultBodyLimit::max(512 * 1024)); // 🛡️ 512KB JSON Bomb Protection

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
        .expect("FATAL: PROXY_AUTH_TOKEN missing. Refusing to run insecurely.");

    let is_authorized = headers
        .get("Authorization")
        .and_then(|h| h.to_str().ok())
        .map(|s| {
            use subtle::ConstantTimeEq;
            s.as_bytes().ct_eq(valid_api_key.as_bytes()).unwrap_u8() == 1
        })
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
    tracing::info!("--- [INCOMING] Payload intercepted ---");
    // println!("{}", serde_json::to_string_pretty(&payload).unwrap());

    // 🛡️ Securely transform the payload using Tokio blocking pool to prevent thread starvation
    payload = tokio::task::spawn_blocking(move || {
        let hasher = SemanticHasher::new();
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
        payload
    }).await.unwrap();

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

/// Phase 1: API-First Escrow for Autonomous Agents (Programmable Money)
/// Allows an AI Agent (the "Brain") to inject a Pre-Cognitive Intent (PCIT) directly into the XCron network
/// without requiring a human xPortal signature. The Agent operates under the strict limits
/// of the on-chain Agent Shield to prevent AI hallucinations.
async fn agent_intent_handler(headers: HeaderMap, Json(intent_payload): Json<Value>) -> impl IntoResponse {
    // 🛡️ SECURITY 1: Verify Agent API Key
    // 🔴 XCRON-PROTECT: Removed silent fallback to dev token.
    // A missing ENV var in production would allow anyone to bypass the shield using the dev token.
    let valid_api_key = std::env::var("AGENT_AUTH_TOKEN")
        .expect("FATAL: AGENT_AUTH_TOKEN missing in environment. Refusing to start in insecure mode.");

    let is_authorized = headers
        .get("Authorization")
        .and_then(|h| h.to_str().ok())
        .map(|s| {
            // Strip "Bearer " if present
            let token = if s.starts_with("Bearer ") { &s[7..] } else { s };
            use subtle::ConstantTimeEq;
            token.as_bytes().ct_eq(valid_api_key.as_bytes()).unwrap_u8() == 1
        })
        .unwrap_or(false);

    if !is_authorized {
        tracing::warn!("🚨 [AGENT SHIELD] Unauthorized autonomous intent rejected!");
        return (
            StatusCode::UNAUTHORIZED,
            Json(json!({
                "status": "error",
                "message": "Unauthorized Agent Access. Quantum Shield Active."
            }))
        ).into_response();
    }

    tracing::info!("🤖 [AGENT ESCROW] LLM Brain Intent Received. Initiating strict validation...");

    // 🛡️ SECURITY 2: Strict AI Hallucination Validation
    // The AI (Dual LLM) sends a JSON. We MUST validate it before it reaches the Rust Muscle (XSE).
    let target_contract = match intent_payload.get("target_contract").and_then(|v| v.as_str()) {
        Some(addr) if addr.starts_with("erd1") && addr.len() == 62 => addr,
        _ => return (
            StatusCode::BAD_REQUEST,
            Json(json!({ "status": "error", "message": "Hallucination Detected: Invalid or missing erd1 target_contract." }))
        ).into_response(),
    };

    let action = match intent_payload.get("action").and_then(|v| v.as_str()) {
        Some(act) => act,
        None => return (
            StatusCode::BAD_REQUEST,
            Json(json!({ "status": "error", "message": "Hallucination Detected: Missing action field." }))
        ).into_response(),
    };

    let amount_egld = match intent_payload.get("amount_egld").and_then(|v| v.as_f64()) {
        Some(amt) if amt >= 0.0 && amt <= 1000.0 => amt, // Hard limit to prevent catastrophic AI errors
        _ => return (
            StatusCode::BAD_REQUEST,
            Json(json!({ "status": "error", "message": "Security Alert: Invalid amount or exceeds 1000 EGLD hard limit." }))
        ).into_response(),
    };

    // 🔗 BRAIN TO MUSCLE CONNECTION: Translate the valid AI Intent into the universal XSE ExecutionIntent format
    let timestamp = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_secs();
    let client_reference_id = format!("xse_intent_{}", timestamp);

    let xse_execution_intent = json!({
        "intent_type": "autonomous_ai_action",
        "venue": "multiversx_mainnet",
        "mode": "strict_execution",
        "source_asset": "EGLD",
        "orders": [
            {
                "side": "execute",
                "asset": target_contract,
                "max_quote_amount": amount_egld,
                "order_type": action
            }
        ],
        "constraints": {
            "max_slippage_pct": 0.5, // 🛡️ XCRON-PROTECT: Strict 0.5% to kill Flash Loan MEV
            "expires_at": (timestamp + 300).to_string(), // 5 minute expiry
            "allowed_assets": ["EGLD"],
            "allow_withdrawals": false // AI is NOT allowed to withdraw funds, only execute logic
        },
        "client_reference_id": client_reference_id
    });

    tracing::info!("✅ [AGENT ESCROW] Intent verified. Translated to XSE standard format.");

    Json(json!({
        "status": "success",
        "message": "Agent Intent Validated and Routed to XSE Sovereign Enclave.",
        "client_reference_id": client_reference_id,
        "xse_intent_payload": xse_execution_intent,
        "shield_status": "Active - Hallucination Checks Passed"
    })).into_response()
}
