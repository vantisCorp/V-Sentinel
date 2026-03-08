//! Core types for MCP protocol

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// MCP Tool - AI-callable security operation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Tool {
    /// Unique tool identifier
    pub name: String,

    /// Human-readable description
    pub description: String,

    /// JSON Schema for input parameters
    pub input_schema: serde_json::Value,

    /// Whether the tool requires authentication
    pub requires_auth: bool,
}

/// Result of tool execution
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolResult {
    /// Success status
    pub success: bool,

    /// Result data
    pub data: Option<serde_json::Value>,

    /// Error message (if failed)
    pub error: Option<String>,

    /// Additional metadata
    pub metadata: HashMap<String, String>,
}

impl ToolResult {
    /// Create a successful result
    pub fn success(data: serde_json::Value) -> Self {
        Self {
            success: true,
            data: Some(data),
            error: None,
            metadata: HashMap::new(),
        }
    }

    /// Create a failed result
    pub fn error(error: String) -> Self {
        Self {
            success: false,
            data: None,
            error: Some(error),
            metadata: HashMap::new(),
        }
    }
}

/// MCP Resource - Static documentation or reference
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Resource {
    /// Unique resource identifier
    pub uri: String,

    /// Resource name
    pub name: String,

    /// Description
    pub description: String,

    /// MIME type
    pub mime_type: String,

    /// Resource content
    pub content: String,
}

/// MCP Prompt - Pre-built interaction pattern
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Prompt {
    /// Unique prompt identifier
    pub name: String,

    /// Description
    pub description: String,

    /// Prompt template
    pub template: String,

    /// Required parameters
    pub parameters: Vec<String>,
}

/// MCP Request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MCPRequest {
    /// Request ID
    pub id: String,

    /// Request method
    pub method: String,

    /// Request parameters
    pub params: serde_json::Value,
}

/// MCP Response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MCPResponse {
    /// Request ID
    pub id: String,

    /// Response data
    pub result: Option<serde_json::Value>,

    /// Error (if any)
    pub error: Option<String>,
}

impl MCPResponse {
    /// Create a successful response
    pub fn success(id: String, result: serde_json::Value) -> Self {
        Self {
            id,
            result: Some(result),
            error: None,
        }
    }

    /// Create an error response
    pub fn error(id: String, error: String) -> Self {
        Self {
            id,
            result: None,
            error: Some(error),
        }
    }
}
