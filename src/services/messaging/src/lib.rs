//! Sentinel Secure Messaging Service with Post-Quantum Cryptography Support
//!
//! This module provides a production-ready messaging service with integrated
//! PQC (Post-Quantum Cryptography) for quantum-resistant communications.

pub mod config;
pub mod pqc_messaging;
pub mod message;
pub mod encryption;

pub use config::MessagingConfig;
pub use pqc_messaging::PqcMessagingService;
pub use message::PqcMessage;
pub use encryption::PqcEncryption;

use anyhow::Result;
use tracing::info;

/// Messaging version
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Initialize the messaging module
pub fn init() -> Result<()> {
    info!("Sentinel Secure Messaging Service initialized (v{})", VERSION);
    Ok(())
}