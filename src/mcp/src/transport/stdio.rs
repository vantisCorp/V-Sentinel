//! Standard I/O transport for MCP server

use crate::error::MCPError;
use crate::server::MCPServer;
use crate::transport::Transport;
use crate::types::{MCPRequest, MCPResponse};
use async_trait::async_trait;
use std::io::{self, BufRead, Write};
use tokio::sync::RwLock;
use tracing::{debug, error, info};

/// Standard I/O transport
pub struct StdioTransport {
    server: std::sync::Arc<MCPServer>,
    running: RwLock<bool>,
}

impl StdioTransport {
    /// Create a new stdio transport
    pub fn new(server: std::sync::Arc<MCPServer>) -> Self {
        Self {
            server,
            running: RwLock::new(false),
        }
    }

    /// Run the stdio transport
    pub async fn run(&self) -> Result<(), MCPError> {
        self.start().await?;

        let stdin = io::stdin();
        let stdout = io::stdout();
        let mut stdout_lock = stdout.lock();

        info!("Stdio transport started. Waiting for requests...");

        for line in stdin.lock().lines() {
            match line {
                Ok(json_line) => {
                    debug!("Received: {}", json_line);

                    // Parse request
                    match serde_json::from_str::<MCPRequest>(&json_line) {
                        Ok(request) => {
                            // Handle request
                            let response = self.server.handle_request(request).await;

                            // Send response
                            let response_json = serde_json::to_string(&response)
                                .map_err(|e| MCPError::SerializationError(e))?;

                            writeln!(stdout_lock, "{}", response_json)
                                .map_err(|e| MCPError::IoError(e))?;

                            debug!("Sent response");
                        }
                        Err(e) => {
                            error!("Failed to parse request: {}", e);
                        }
                    }
                }
                Err(e) => {
                    error!("Failed to read line: {}", e);
                    break;
                }
            }

            if !*self.running.read().await {
                break;
            }
        }

        info!("Stdio transport stopped");
        Ok(())
    }
}

#[async_trait]
impl Transport for StdioTransport {
    async fn start(&self) -> Result<(), MCPError> {
        let mut running = self.running.write().await;
        *running = true;
        self.server.start().await;
        Ok(())
    }

    async fn stop(&self) -> Result<(), MCPError> {
        let mut running = self.running.write().await;
        *running = false;
        self.server.stop().await;
        Ok(())
    }

    fn is_running(&self) -> bool {
        // Use blocking read for is_running
        tokio::task::block_in_place(|| {
            tokio::runtime::Handle::current().block_on(async { *self.running.read().await })
        })
    }
}
