//! Error types for MCP server

use thiserror::Error;

/// MCP server errors
#[derive(Error, Debug)]
pub enum MCPError {
    #[error("Transport error: {0}")]
    TransportError(String),

    #[error("Tool execution error: {0}")]
    ToolExecutionError(String),

    #[error("Invalid tool parameters: {0}")]
    InvalidParameters(String),

    #[error("Tool not found: {0}")]
    ToolNotFound(String),

    #[error("Resource not found: {0}")]
    ResourceNotFound(String),

    #[error("Serialization error: {0}")]
    SerializationError(#[from] serde_json::Error),

    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),

    #[error("Internal error: {0}")]
    InternalError(String),
}

/// Result type for MCP operations
pub type MCPResult<T> = Result<T, MCPError>;
