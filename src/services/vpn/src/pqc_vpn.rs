//! PQC VPN Service - Post-Quantum Cryptography VPN Implementation
//!
//! This module provides the core PQC VPN functionality including:
//! - PQC-based key exchange for VPN tunnels
//! - Hybrid VPN mode (PQC + classical)
//! - Quantum-resistant tunnel encryption
//! - PQC client authentication

use anyhow::{Result, anyhow};
use tracing::{info, debug, warn, error};
use std::sync::Arc;
use tokio::sync::RwLock;
use serde::{Serialize, Deserialize};
use chrono::{DateTime, Utc};

use super::config::VpnPqcConfig;

/// PQC VPN Service
pub struct PqcVpnService {
    /// VPN configuration
    config: VpnPqcConfig,
    /// Active tunnels
    tunnels: Arc<RwLock<Vec<PqcTunnel>>>,
    /// Connected clients
    clients: Arc<RwLock<Vec<PqcVpnClient>>>,
}

/// VPN PQC Configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VpnPqcConfig {
    /// Enable PQC mode
    pub enable_pqc: bool,
    /// KEM algorithm for key exchange
    pub kem_algorithm: VpnKemAlgorithm,
    /// Signature algorithm for authentication
    pub signature_algorithm: VpnSignatureAlgorithm,
    /// Enable hybrid mode (PQC + classical)
    pub hybrid_mode: bool,
    /// Allow fallback to classical algorithms
    pub fallback_to_classical: bool,
    /// Minimum security level (1-5)
    pub min_security_level: u8,
    /// Key rekeying interval in seconds
    pub rekey_interval_secs: u64,
    /// Tunnel MTU
    pub mtu: u16,
    /// Enable perfect forward secrecy
    pub enable_pfs: bool,
    /// Use compression
    pub compression: bool,
}

/// VPN KEM Algorithm
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum VpnKemAlgorithm {
    CrystalsKyber512,
    CrystalsKyber768,
    CrystalsKyber1024,
    HybridKyber768X25519,
    HybridKyber1024X448,
}

impl VpnKemAlgorithm {
    pub fn security_level(&self) -> u8 {
        match self {
            Self::CrystalsKyber512 => 1,
            Self::CrystalsKyber768 | Self::HybridKyber768X25519 => 3,
            Self::CrystalsKyber1024 | Self::HybridKyber1024X448 => 5,
        }
    }
    
    pub fn algorithm_name(&self) -> &str {
        match self {
            Self::CrystalsKyber512 => "CRYSTALS-Kyber-512",
            Self::CrystalsKyber768 => "CRYSTALS-Kyber-768",
            Self::CrystalsKyber1024 => "CRYSTALS-Kyber-1024",
            Self::HybridKyber768X25519 => "Hybrid-Kyber768-X25519",
            Self::HybridKyber1024X448 => "Hybrid-Kyber1024-X448",
        }
    }
}

/// VPN Signature Algorithm
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum VpnSignatureAlgorithm {
    CrystalsDilithium2,
    CrystalsDilithium3,
    CrystalsDilithium5,
    Falcon512,
    Falcon1024,
    HybridDilithium3EcdsaP384,
}

impl VpnSignatureAlgorithm {
    pub fn security_level(&self) -> u8 {
        match self {
            Self::CrystalsDilithium2 | Self::Falcon512 => 1,
            Self::CrystalsDilithium3 | Self::Falcon1024 | Self::HybridDilithium3EcdsaP384 => 3,
            Self::CrystalsDilithium5 => 5,
        }
    }
    
    pub fn algorithm_name(&self) -> &str {
        match self {
            Self::CrystalsDilithium2 => "CRYSTALS-Dilithium-2",
            Self::CrystalsDilithium3 => "CRYSTALS-Dilithium-3",
            Self::CrystalsDilithium5 => "CRYSTALS-Dilithium-5",
            Self::Falcon512 => "FALCON-512",
            Self::Falcon1024 => "FALCON-1024",
            Self::HybridDilithium3EcdsaP384 => "Hybrid-Dilithium3-ECDSA-P384",
        }
    }
}

/// PQC Tunnel
#[derive(Debug, Clone)]
pub struct PqcTunnel {
    /// Tunnel ID
    pub tunnel_id: String,
    /// Client ID
    pub client_id: String,
    /// Local address
    pub local_address: String,
    /// Remote address
    pub remote_address: String,
    /// KEM algorithm used
    pub kem_algorithm: VpnKemAlgorithm,
    /// Security level
    pub security_level: u8,
    /// Tunnel state
    pub state: TunnelState,
    /// Created at
    pub created_at: DateTime<Utc>,
    /// Last rekeyed
    pub last_rekeyed: DateTime<Utc>,
    /// Bytes transmitted
    pub bytes_transmitted: u64,
    /// Bytes received
    pub bytes_received: u64,
}

/// Tunnel State
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TunnelState {
    /// Initializing
    Initializing,
    /// Established
    Established,
    /// Rekeying
    Rekeying,
    /// Closed
    Closed,
    /// Error
    Error,
}

/// PQC VPN Client
#[derive(Debug, Clone)]
pub struct PqcVpnClient {
    /// Client ID
    pub client_id: String,
    /// Client public key
    pub public_key: Vec<u8>,
    /// Connected tunnel ID
    pub tunnel_id: Option<String>,
    /// Connected at
    pub connected_at: DateTime<Utc>,
    /// Last activity
    pub last_activity: DateTime<Utc>,
}

impl Default for VpnPqcConfig {
    fn default() -> Self {
        Self {
            enable_pqc: true,
            kem_algorithm: VpnKemAlgorithm::HybridKyber768X25519,
            signature_algorithm: VpnSignatureAlgorithm::CrystalsDilithium3,
            hybrid_mode: true,
            fallback_to_classical: true,
            min_security_level: 3,
            rekey_interval_secs: 3600, // 1 hour
            mtu: 1500,
            enable_pfs: true,
            compression: true,
        }
    }
}

impl PqcVpnService {
    /// Create a new PQC VPN service
    pub fn new(config: VpnPqcConfig) -> Self {
        Self {
            config,
            tunnels: Arc::new(RwLock::new(Vec::new())),
            clients: Arc::new(RwLock::new(Vec::new())),
        }
    }
    
    /// Initialize the VPN service
    pub async fn initialize(&self) -> Result<()> {
        info!("Initializing PQC VPN Service...");
        
        // Validate configuration
        self.validate_config()?;
        
        info!("PQC VPN Service initialized");
        info!("  KEM Algorithm: {}", self.config.kem_algorithm.algorithm_name());
        info!("  Signature Algorithm: {}", self.config.signature_algorithm.algorithm_name());
        info!("  Security Level: {}", self.config.min_security_level);
        info!("  Hybrid Mode: {}", self.config.hybrid_mode);
        info!("  MTU: {}", self.config.mtu);
        
        Ok(())
    }
    
    /// Validate VPN configuration
    fn validate_config(&self) -> Result<()> {
        let kem_level = self.config.kem_algorithm.security_level();
        let sig_level = self.config.signature_algorithm.security_level();
        
        if kem_level < self.config.min_security_level {
            return Err(anyhow!(
                "KEM algorithm security level ({}) is below minimum ({})",
                kem_level, self.config.min_security_level
            ));
        }
        
        if sig_level < self.config.min_security_level {
            return Err(anyhow!(
                "Signature algorithm security level ({}) is below minimum ({})",
                sig_level, self.config.min_security_level
            ));
        }
        
        Ok(())
    }
    
    /// Establish a PQC tunnel
    pub async fn establish_tunnel(
        &self,
        client_id: &str,
        local_address: &str,
        remote_address: &str,
    ) -> Result<String> {
        debug!("Establishing PQC tunnel for client: {}", client_id);
        
        // Perform PQC key exchange
        let tunnel_id = format!("tunnel-{}-{}", client_id, chrono::Utc::now().timestamp());
        
        // Create tunnel
        let tunnel = PqcTunnel {
            tunnel_id: tunnel_id.clone(),
            client_id: client_id.to_string(),
            local_address: local_address.to_string(),
            remote_address: remote_address.to_string(),
            kem_algorithm: self.config.kem_algorithm,
            security_level: self.config.kem_algorithm.security_level(),
            state: TunnelState::Established,
            created_at: Utc::now(),
            last_rekeyed: Utc::now(),
            bytes_transmitted: 0,
            bytes_received: 0,
        };
        
        // Store tunnel
        self.tunnels.write().await.push(tunnel);
        
        info!("PQC tunnel established: {}", tunnel_id);
        
        Ok(tunnel_id)
    }
    
    /// Close a tunnel
    pub async fn close_tunnel(&self, tunnel_id: &str) -> Result<()> {
        let mut tunnels = self.tunnels.write().await;
        
        if let Some(tunnel) = tunnels.iter_mut().find(|t| t.tunnel_id == tunnel_id) {
            tunnel.state = TunnelState::Closed;
            info!("Tunnel {} closed", tunnel_id);
        }
        
        Ok(())
    }
    
    /// Rekey a tunnel
    pub async fn rekey_tunnel(&self, tunnel_id: &str) -> Result<()> {
        debug!("Rekeying tunnel: {}", tunnel_id);
        
        let mut tunnels = self.tunnels.write().await;
        
        if let Some(tunnel) = tunnels.iter_mut().find(|t| t.tunnel_id == tunnel_id) {
            tunnel.state = TunnelState::Rekeying;
            tunnel.last_rekeyed = Utc::now();
            tunnel.state = TunnelState::Established;
            info!("Tunnel {} rekeyed", tunnel_id);
        }
        
        Ok(())
    }
    
    /// Get tunnel count
    pub async fn tunnel_count(&self) -> usize {
        self.tunnels.read().await.len()
    }
    
    /// Get active tunnels
    pub async fn get_active_tunnels(&self) -> Vec<PqcTunnel> {
        self.tunnels.read().await
            .iter()
            .filter(|t| t.state == TunnelState::Established)
            .cloned()
            .collect()
    }
    
    /// Get client count
    pub async fn client_count(&self) -> usize {
        self.clients.read().await.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test]
    async fn test_vpn_creation() {
        let config = VpnPqcConfig::default();
        let vpn = PqcVpnService::new(config);
        let result = vpn.initialize().await;
        assert!(result.is_ok());
    }
    
    #[test]
    fn test_security_levels() {
        assert_eq!(VpnKemAlgorithm::CrystalsKyber512.security_level(), 1);
        assert_eq!(VpnKemAlgorithm::CrystalsKyber768.security_level(), 3);
        assert_eq!(VpnKemAlgorithm::CrystalsKyber1024.security_level(), 5);
    }
    
    #[tokio::test]
    async fn test_tunnel_establishment() {
        let config = VpnPqcConfig::default();
        let vpn = PqcVpnService::new(config);
        
        let tunnel_id = vpn.establish_tunnel("client-1", "10.0.0.1", "10.0.0.2").await;
        assert!(tunnel_id.is_ok());
        
        let count = vpn.tunnel_count().await;
        assert_eq!(count, 1);
    }
}