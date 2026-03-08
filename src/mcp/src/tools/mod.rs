//! AI-callable security tools

pub mod detections;
pub mod hosts;
pub mod incidents;
pub mod threat_intel;

use crate::error::{MCPError, MCPResult};
use crate::types::{Tool, ToolResult};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

/// Registry of available tools
pub type ToolRegistry = Arc<RwLock<HashMap<String, Box<dyn ToolTrait + Send + Sync>>>>;

/// Trait that all tools must implement
#[async_trait::async_trait]
pub trait ToolTrait: Send + Sync {
    /// Get tool metadata
    fn get_tool(&self) -> Tool;

    /// Execute the tool with given parameters
    async fn execute(&self, params: serde_json::Value) -> MCPResult<ToolResult>;
}

/// Tool manager for registering and executing tools
pub struct ToolManager {
    registry: ToolRegistry,
}

impl ToolManager {
    /// Create a new tool manager
    pub fn new() -> Self {
        Self {
            registry: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Register a tool
    pub async fn register<T>(&self, tool: T)
    where
        T: ToolTrait + Send + Sync + 'static,
    {
        let tool_info = tool.get_tool();
        let name = tool_info.name.clone();
        self.registry.write().await.insert(name, Box::new(tool));
    }

    /// Get all registered tools
    pub async fn list_tools(&self) -> Vec<Tool> {
        self.registry
            .read()
            .await
            .values()
            .map(|tool| tool.get_tool())
            .collect()
    }

    /// Execute a tool by name
    pub async fn execute_tool(
        &self,
        name: &str,
        params: serde_json::Value,
    ) -> MCPResult<ToolResult> {
        let registry = self.registry.read().await;

        match registry.get(name) {
            Some(tool) => tool.execute(params).await,
            None => Err(MCPError::ToolNotFound(name.to_string())),
        }
    }

    /// Get a tool by name
    pub async fn get_tool(&self, name: &str) -> MCPResult<Tool> {
        let registry = self.registry.read().await;

        match registry.get(name) {
            Some(tool) => Ok(tool.get_tool()),
            None => Err(MCPError::ToolNotFound(name.to_string())),
        }
    }
}

impl Default for ToolManager {
    fn default() -> Self {
        Self::new()
    }
}
