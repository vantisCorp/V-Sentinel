//! Sentinel API Gateway with Post-Quantum Cryptography Support
//!
//! This module provides a production-ready API gateway with integrated
//! PQC (Post-Quantum Cryptography) for quantum-resistant communications.

pub mod config;
pub mod server;
pub mod routes;
pub mod middleware;
pub mod pqc_gateway;
pub mod tls;
pub mod metrics;
pub mod auth;

pub use config::GatewayConfig;
pub use server::GatewayServer;
pub use pqc_gateway::PqcGateway;
pub use auth::{AuthConfig, AuthService};
pub use metrics::GatewayMetrics;

use anyhow::Result;
use tracing::info;

/// Gateway version
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Initialize the gateway module
pub fn init() -> Result<()> {
    info!("Sentinel API Gateway initialized (v{})", VERSION);
    Ok(())
}