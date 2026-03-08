//! Gateway Server Module

use anyhow::Result;
use axum::Router;
use std::net::SocketAddr;
use tracing::info;

use super::config::GatewayConfig;
use super::pqc_gateway::PqcGateway;

/// Gateway Server
pub struct GatewayServer {
    config: GatewayConfig,
    pqc_gateway: PqcGateway,
}

impl GatewayServer {
    /// Create a new gateway server
    pub fn new(config: GatewayConfig) -> Self {
        let pqc_gateway = PqcGateway::new(config.pqc.clone());
        Self { config, pqc_gateway }
    }
    
    /// Start the server
    pub async fn start(&self) -> Result<()> {
        let addr: SocketAddr = format!("{}:{}", self.config.address, self.config.port)
            .parse()
            .map_err(|e| anyhow::anyhow!("Invalid address: {}", e))?;
        
        info!("Starting Sentinel Gateway on {}", addr);
        info!("PQC enabled: {}", self.config.pqc.enable_pqc);
        
        // Build router
        let app = Router::new();
        
        // Start server
        let listener = tokio::net::TcpListener::bind(addr).await?;
        
        info!("Gateway server listening on {}", addr);
        
        axum::serve(listener, app).await?;
        
        Ok(())
    }
    
    /// Get PQC gateway
    pub fn pqc_gateway(&self) -> &PqcGateway {
        &self.pqc_gateway
    }
    
    /// Shutdown the server
    pub async fn shutdown(&self) -> Result<()> {
        info!("Shutting down gateway server");
        Ok(())
    }
}