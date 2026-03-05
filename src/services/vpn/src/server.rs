//! PQC VPN Server

use anyhow::Result;
use std::net::SocketAddr;
use tokio::net::TcpListener;

use super::pqc_vpn::PqcVpnService;

/// PQC VPN Server
pub struct PqcVpnServer {
    address: SocketAddr,
    service: PqcVpnService,
}

impl PqcVpnServer {
    /// Create a new VPN server
    pub fn new(address: SocketAddr, service: PqcVpnService) -> Self {
        Self { address, service }
    }
    
    /// Start the VPN server
    pub async fn start(&self) -> Result<()> {
        let listener = TcpListener::bind(self.address).await?;
        tracing::info!("PQC VPN Server listening on {}", self.address);
        
        loop {
            let (socket, addr) = listener.accept().await?;
            tracing::info!("New connection from: {}", addr);
            
            // Handle connection with PQC handshake
            tokio::spawn(async move {
                // Implement connection handling
            });
        }
    }
}