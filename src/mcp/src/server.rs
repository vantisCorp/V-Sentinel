//! MCP server implementation

use crate::error::MCPResult;
use crate::tools::ToolManager;
use crate::types::{MCPRequest, MCPResponse, Resource};
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::info;

/// MCP Server
pub struct MCPServer {
    /// Server name
    name: String,

    /// Server version
    version: String,

    /// Tool manager
    tool_manager: Arc<ToolManager>,

    /// Resources (static documentation)
    resources: Arc<RwLock<Vec<Resource>>>,

    /// Running state
    running: Arc<RwLock<bool>>,
}

impl MCPServer {
    /// Create a new MCP server
    pub async fn new(name: &str) -> MCPResult<Self> {
        let server = Self {
            name: name.to_string(),
            version: env!("CARGO_PKG_VERSION").to_string(),
            tool_manager: Arc::new(ToolManager::new()),
            resources: Arc::new(RwLock::new(Vec::new())),
            running: Arc::new(RwLock::new(false)),
        };

        // Register default tools
        server.register_default_tools().await?;

        // Initialize default resources
        server.initialize_resources().await?;

        info!("MCP Server '{}' initialized", name);

        Ok(server)
    }

    /// Register default tools
    async fn register_default_tools(&self) -> MCPResult<()> {
        use crate::tools::{
            detections::DetectionsTool, hosts::HostsTool, incidents::IncidentsTool,
            threat_intel::ThreatIntelTool,
        };

        self.tool_manager.register(DetectionsTool).await;
        self.tool_manager.register(HostsTool).await;
        self.tool_manager.register(IncidentsTool).await;
        self.tool_manager.register(ThreatIntelTool).await;

        info!("Registered 4 default tools");

        Ok(())
    }

    /// Initialize default resources
    async fn initialize_resources(&self) -> MCPResult<()> {
        let mut resources = self.resources.write().await;

        // Add V-Sentinel API documentation
        resources.push(Resource {
            uri: "docs://api-reference".to_string(),
            name: "V-Sentinel API Reference".to_string(),
            description: "Complete API documentation for V-Sentinel security platform".to_string(),
            mime_type: "text/markdown".to_string(),
            content: include_str!("../resources/api_docs.md").to_string(),
        });

        // Add FQL guide
        resources.push(Resource {
            uri: "docs://fql-guide".to_string(),
            name: "Falcon Query Language (FQL) Guide".to_string(),
            description: "Comprehensive guide to FQL for searching detections and events"
                .to_string(),
            mime_type: "text/markdown".to_string(),
            content: include_str!("../resources/fql_guide.md").to_string(),
        });

        info!("Initialized 2 resources");

        Ok(())
    }

    /// Handle MCP request
    pub async fn handle_request(&self, request: MCPRequest) -> MCPResponse {
        info!("Handling request: {} (id: {})", request.method, request.id);

        match request.method.as_str() {
            "tools/list" => self.handle_list_tools(request).await,
            "tools/call" => self.handle_tool_call(request).await,
            "resources/list" => self.handle_list_resources(request).await,
            "resources/read" => self.handle_read_resource(request).await,
            "initialize" => self.handle_initialize(request).await,
            _ => MCPResponse::error(request.id, format!("Unknown method: {}", request.method)),
        }
    }

    /// Handle tools/list request
    async fn handle_list_tools(&self, request: MCPRequest) -> MCPResponse {
        let tools = self.tool_manager.list_tools().await;

        MCPResponse::success(request.id, serde_json::json!({ "tools": tools }))
    }

    /// Handle tools/call request
    async fn handle_tool_call(&self, request: MCPRequest) -> MCPResponse {
        let tool_name = request
            .params
            .get("name")
            .and_then(|v| v.as_str())
            .unwrap_or("");

        let arguments = request
            .params
            .get("arguments")
            .cloned()
            .unwrap_or(serde_json::json!({}));

        match self.tool_manager.execute_tool(tool_name, arguments).await {
            Ok(result) => MCPResponse::success(request.id, serde_json::json!({ "result": result })),
            Err(e) => MCPResponse::error(request.id, e.to_string()),
        }
    }

    /// Handle resources/list request
    async fn handle_list_resources(&self, request: MCPRequest) -> MCPResponse {
        let resources = self.resources.read().await;

        let resource_list = resources
            .iter()
            .map(|r| {
                serde_json::json!({
                    "uri": r.uri,
                    "name": r.name,
                    "description": r.description,
                    "mime_type": r.mime_type,
                })
            })
            .collect::<Vec<_>>();

        MCPResponse::success(
            request.id,
            serde_json::json!({ "resources": resource_list }),
        )
    }

    /// Handle resources/read request
    async fn handle_read_resource(&self, request: MCPRequest) -> MCPResponse {
        let uri = request
            .params
            .get("uri")
            .and_then(|v| v.as_str())
            .unwrap_or("");

        let resources = self.resources.read().await;

        match resources.iter().find(|r| r.uri == uri) {
            Some(resource) => MCPResponse::success(
                request.id,
                serde_json::json!({
                    "contents": [{
                        "uri": resource.uri,
                        "mime_type": resource.mime_type,
                        "text": resource.content,
                    }]
                }),
            ),
            None => MCPResponse::error(request.id, format!("Resource not found: {}", uri)),
        }
    }

    /// Handle initialize request
    async fn handle_initialize(&self, request: MCPRequest) -> MCPResponse {
        MCPResponse::success(
            request.id,
            serde_json::json!({
                "protocolVersion": "2024-11-05",
                "serverInfo": {
                    "name": self.name,
                    "version": self.version,
                },
                "capabilities": {
                    "tools": {},
                    "resources": {},
                }
            }),
        )
    }

    /// Start the server (marker for async transport implementations)
    pub async fn start(&self) {
        let mut running = self.running.write().await;
        *running = true;
        info!("MCP Server started");
    }

    /// Stop the server
    pub async fn stop(&self) {
        let mut running = self.running.write().await;
        *running = false;
        info!("MCP Server stopped");
    }

    /// Check if server is running
    pub async fn is_running(&self) -> bool {
        *self.running.read().await
    }
}
