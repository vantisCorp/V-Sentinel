//! SENTINEL Security System - Main Entry Point
//! 
//! This is the main entry point for the SENTINEL security system.

use sentinel_core::SentinelCore;
use anyhow::Result;
use tracing::{info, error, Level};
use tracing_subscriber;

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize logging
    tracing_subscriber::fmt()
        .with_max_level(Level::INFO)
        .init();
    
    info!("Starting SENTINEL Security System...");
    
    // Create and start the core security system
    let mut core = SentinelCore::new()?;
    core.start().await?;
    
    info!("SENTINEL Security System is running");
    info!("Press Ctrl+C to stop");
    
    // Wait for shutdown signal
    tokio::signal::ctrl_c().await?;
    
    info!("Shutting down SENTINEL Security System...");
    
    // Stop the core security system
    core.stop().await?;
    
    info!("SENTINEL Security System stopped");
    
    Ok(())
}
