//! Gateway Routes Module

use axum::{
    Router,
    routing::{get, post, put, delete},
    Json,
    http::StatusCode,
};
use serde::{Serialize, Deserialize};
use std::sync::Arc;

use super::pqc_gateway::{PqcGateway, HealthStatus};

/// Health check response
#[derive(Debug, Serialize, Deserialize)]
pub struct HealthResponse {
    pub status: String,
    pub version: String,
    pub pqc: PqcStatus,
}

/// PQC Status
#[derive(Debug, Serialize, Deserialize)]
pub struct PqcStatus {
    pub enabled: bool,
    pub kem_algorithm: String,
    pub signature_algorithm: String,
    pub security_level: u8,
}

/// Create the main router
pub fn create_router(gateway: Arc<PqcGateway>) -> Router {
    Router::new()
        .route("/health", get(health_check))
        .route("/api/v1/pqc/status", get(pqc_status))
        .route("/api/v1/pqc/handshake", post(initiate_handshake))
        .route("/api/v1/pqc/session/:id", delete(terminate_session))
        .route("/metrics", get(metrics))
        .with_state(gateway)
}

/// Health check endpoint
async fn health_check() -> Json<HealthResponse> {
    Json(HealthResponse {
        status: "healthy".to_string(),
        version: env!("CARGO_PKG_VERSION").to_string(),
        pqc: PqcStatus {
            enabled: true,
            kem_algorithm: "CrystalsKyber768".to_string(),
            signature_algorithm: "CrystalsDilithium3".to_string(),
            security_level: 3,
        },
    })
}

/// PQC status endpoint
async fn pqc_status(
    axum::extract::State(gateway): axum::extract::State<Arc<PqcGateway>>,
) -> Json<HealthStatus> {
    Json(gateway.health_check().await)
}

/// Initiate PQC handshake
async fn initiate_handshake(
    axum::extract::State(_gateway): axum::extract::State<Arc<PqcGateway>>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    // Placeholder for handshake logic
    Ok(Json(serde_json::json!({
        "status": "handshake_initiated",
        "message": "PQC handshake endpoint ready"
    })))
}

/// Terminate session
async fn terminate_session(
    axum::extract::State(_gateway): axum::extract::State<Arc<PqcGateway>>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    Ok(Json(serde_json::json!({
        "status": "session_terminated"
    })))
}

/// Metrics endpoint
async fn metrics(
    axum::extract::State(gateway): axum::extract::State<Arc<PqcGateway>>,
) -> String {
    let metrics = gateway.get_metrics();
    format!(
        "# HELP gateway_requests_total Total requests processed\n\
         # TYPE gateway_requests_total counter\n\
         gateway_requests_total {}\n\
         # HELP gateway_pqc_handshakes_total PQC handshakes completed\n\
         # TYPE gateway_pqc_handshakes_total counter\n\
         gateway_pqc_handshakes_total {}\n\
         # HELP gateway_active_sessions Active PQC sessions\n\
         # TYPE gateway_active_sessions gauge\n\
         gateway_active_sessions {}\n",
        metrics.total_requests.load(std::sync::atomic::Ordering::Relaxed),
        metrics.pqc_handshakes.load(std::sync::atomic::Ordering::Relaxed),
        metrics.active_sessions.load(std::sync::atomic::Ordering::Relaxed)
    )
}