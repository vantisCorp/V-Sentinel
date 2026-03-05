//! Sentinel VPN Service with Post-Quantum Cryptography Support
//!
//! This module provides a production-ready VPN service with integrated
//! PQC (Post-Quantum Cryptography) for quantum-resistant VPN connections.

pub mod config;
pub mod pqc_vpn;
pub mod tunnel;
pub mod client;
pub mod server;

pub use config::VpnConfig;
pub use pqc_vpn::PqcVpnService;
pub use tunnel::PqcTunnel;
pub use client::PqcVpnClient;
pub use server::PqcVpnServer;

use anyhow::Result;
use tracing::info;

/// VPN version
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Initialize the VPN module
pub fn init() -> Result<()> {
    info!("Sentinel VPN Service initialized (v{})", VERSION);
    Ok(())
}