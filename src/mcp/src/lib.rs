//! V-Sentinel Model Context Protocol (MCP) Server
//!
//! This module implements an MCP server that allows AI agents to interact
//! with V-Sentinel's security operations through a standardized protocol.
//!
//! ## Features
//!
//! - AI-driven threat hunting
//! - Natural language security queries
//! - Automated incident response
//! - Real-time threat intelligence access
//!
//! ## Architecture
//!
//! The MCP server provides:
//! - Tools: AI-callable security operations
//! - Resources: Static documentation and references
//! - Prompts: Pre-built interaction patterns
//!
//! ## Transport Protocols
//!
//! - stdio: For local development and testing
//! - SSE (Server-Sent Events): For real-time streaming
//! - HTTP: For web-based integration
//!
//! ## Example
//!
//! ```rust,no_run
//! use v_sentinel_mcp::MCPServer;
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let server = MCPServer::new("V-Sentinel MCP Server").await?;
//!     server.run_stdio().await?;
//!     Ok(())
//! }
//! ```

pub mod error;
pub mod resources;
pub mod server;
pub mod tools;
pub mod transport;
pub mod types;

pub use error::{MCPError, MCPResult};
pub use server::MCPServer;
pub use types::{Prompt, Resource, Tool, ToolResult};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_library_creation() {
        // Basic test to ensure library compiles
        assert!(true);
    }
}
