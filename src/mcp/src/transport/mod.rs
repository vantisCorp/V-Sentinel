//! Transport protocols for MCP server

pub mod stdio;

use crate::error::MCPError;

/// Transport trait for different communication protocols
#[async_trait::async_trait]
pub trait Transport: Send + Sync {
    /// Start the transport
    async fn start(&self) -> Result<(), MCPError>;
    
    /// Stop the transport
    async fn stop(&self) -> Result<(), MCPError>;
    
    /// Check if transport is running
    fn is_running(&self) -> bool;
}