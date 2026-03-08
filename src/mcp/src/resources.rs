//! MCP Resources - Static documentation and references

use serde::{Deserialize, Serialize};

/// Resource type
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Resource {
    pub uri: String,
    pub name: String,
    pub description: Option<String>,
    pub mime_type: Option<String>,
}

/// Resource content
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceContent {
    pub uri: String,
    pub content: String,
    pub mime_type: Option<String>,
}

/// Get API documentation resource
pub fn get_api_docs() -> Resource {
    Resource {
        uri: "docs://api".to_string(),
        name: "API Documentation".to_string(),
        description: Some("V-Sentinel MCP API documentation".to_string()),
        mime_type: Some("text/markdown".to_string()),
    }
}

/// Get FQL guide resource
pub fn get_fql_guide() -> Resource {
    Resource {
        uri: "docs://fql-guide".to_string(),
        name: "FQL Guide".to_string(),
        description: Some("Filter Query Language guide".to_string()),
        mime_type: Some("text/markdown".to_string()),
    }
}

/// List all available resources
pub fn list_resources() -> Vec<Resource> {
    vec![get_api_docs(), get_fql_guide()]
}

/// Read resource content by URI
pub fn read_resource(uri: &str) -> Option<ResourceContent> {
    match uri {
        "docs://api" => Some(ResourceContent {
            uri: uri.to_string(),
            content: include_str!("../resources/api_docs.md").to_string(),
            mime_type: Some("text/markdown".to_string()),
        }),
        "docs://fql-guide" => Some(ResourceContent {
            uri: uri.to_string(),
            content: include_str!("../resources/fql_guide.md").to_string(),
            mime_type: Some("text/markdown".to_string()),
        }),
        _ => None,
    }
}
