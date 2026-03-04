//! SENTINEL Network Module
//! 
//! This module provides network communication with post-quantum cryptography (PQC)
//! support, including PQC-enabled TLS, hybrid key exchange, and quantum-safe
//! authentication mechanisms.

pub mod pqc_tls;
pub mod handshake;
pub mod certificates;

use anyhow::{Result, anyhow};
use tracing::{info, debug, warn, error};
use std::sync::Arc;
use tokio::sync::RwLock;
use serde::{Serialize, Deserialize};

/// Network Manager with PQC Support
pub struct NetworkManager {
    pqc_enabled: Arc<RwLock<bool>>,
    pqc_config: Arc<RwLock<PqcNetworkConfig>>,
    statistics: Arc<RwLock<NetworkStatistics>>,
}

/// PQC Network Configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PqcNetworkConfig {
    /// Enable PQC for TLS
    pub enable_pqc_tls: bool,
    /// KEM algorithm for key exchange
    pub kem_algorithm: KemAlgorithm,
    /// Signature algorithm for certificates
    pub signature_algorithm: SignatureAlgorithm,
    /// Enable hybrid mode (PQC + classical)
    pub hybrid_mode: bool,
    /// Fallback to classical if PQC fails
    pub fallback_to_classical: bool,
    /// Key rotation interval in hours
    pub key_rotation_hours: u64,
}

/// KEM Algorithms for Network
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum KemAlgorithm {
    CrystalsKyber512,
    CrystalsKyber768,
    CrystalsKyber1024,
}

/// Signature Algorithms for Network
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SignatureAlgorithm {
    CrystalsDilithium2,
    CrystalsDilithium3,
    CrystalsDilithium5,
    Falcon512,
    Falcon1024,
}

/// Network Statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkStatistics {
    /// Total connections
    pub total_connections: u64,
    /// PQC connections
    pub pqc_connections: u64,
    /// Classical connections
    pub classical_connections: u64,
    /// Failed connections
    pub failed_connections: u64,
    /// Average handshake time (ms)
    pub avg_handshake_time_ms: f64,
}

impl Default for NetworkStatistics {
    fn default() -> Self {
        Self {
            total_connections: 0,
            pqc_connections: 0,
            classical_connections: 0,
            failed_connections: 0,
            avg_handshake_time_ms: 0.0,
        }
    }
}

impl Default for PqcNetworkConfig {
    fn default() -> Self {
        Self {
            enable_pqc_tls: true,
            kem_algorithm: KemAlgorithm::CrystalsKyber768,
            signature_algorithm: SignatureAlgorithm::CrystalsDilithium3,
            hybrid_mode: true,
            fallback_to_classical: true,
            key_rotation_hours: 168, // 7 days
        }
    }
}

impl NetworkManager {
    /// Create a new network manager with PQC support
    pub fn new() -> Result<Self> {
        info!("Creating Network Manager with PQC support...");
        
        Ok(Self {
            pqc_enabled: Arc::new(RwLock::new(true)),
            pqc_config: Arc::new(RwLock::new(PqcNetworkConfig::default())),
            statistics: Arc::new(RwLock::new(NetworkStatistics::default())),
        })
    }
    
    /// Initialize network manager
    pub async fn initialize(&self) -> Result<()> {
        info!("Initializing Network Manager...");
        
        // Initialize PQC TLS components
        if *self.pqc_enabled.read().await {
            self.initialize_pqc().await?;
        }
        
        info!("Network Manager initialized successfully");
        Ok(())
    }
    
    /// Initialize PQC components
    async fn initialize_pqc(&self) -> Result<()> {
        info!("Initializing PQC components...");
        
        let config = self.pqc_config.read().await;
        
        // Validate configuration
        self.validate_config(&config)?;
        
        info!("PQC components initialized: {:?}", config);
        Ok(())
    }
    
    /// Validate PQC configuration
    fn validate_config(&self, config: &PqcNetworkConfig) -> Result<()> {
        if config.enable_pqc_tls {
            // Validate KEM algorithm
            match config.kem_algorithm {
                KemAlgorithm::CrystalsKyber512 => {}
                KemAlgorithm::CrystalsKyber768 => {}
                KemAlgorithm::CrystalsKyber1024 => {}
            }
            
            // Validate signature algorithm
            match config.signature_algorithm {
                SignatureAlgorithm::CrystalsDilithium2 => {}
                SignatureAlgorithm::CrystalsDilithium3 => {}
                SignatureAlgorithm::CrystalsDilithium5 => {}
                SignatureAlgorithm::Falcon512 => {}
                SignatureAlgorithm::Falcon1024 => {}
            }
            
            // Validate hybrid mode
            if config.hybrid_mode && !config.fallback_to_classical {
                warn!("Hybrid mode enabled but fallback disabled. This may cause compatibility issues.");
            }
        }
        
        Ok(())
    }
    
    /// Update PQC configuration
    pub async fn update_config(&self, config: PqcNetworkConfig) -> Result<()> {
        info!("Updating PQC network configuration...");
        
        // Validate new configuration
        self.validate_config(&config)?;
        
        // Update configuration
        *self.pqc_config.write().await = config;
        
        info!("PQC configuration updated successfully");
        Ok(())
    }
    
    /// Enable PQC
    pub async fn enable_pqc(&self) {
        *self.pqc_enabled.write().await = true;
        info!("PQC enabled");
    }
    
    /// Disable PQC
    pub async fn disable_pqc(&self) {
        *self.pqc_enabled.write().await = false;
        warn!("PQC disabled - using classical cryptography");
    }
    
    /// Check if PQC is enabled
    pub async fn is_pqc_enabled(&self) -> bool {
        *self.pqc_enabled.read().await
    }
    
    /// Get PQC configuration
    pub async fn get_config(&self) -> PqcNetworkConfig {
        self.pqc_config.read().await.clone()
    }
    
    /// Get network statistics
    pub async fn get_statistics(&self) -> NetworkStatistics {
        self.statistics.read().await.clone()
    }
    
    /// Reset statistics
    pub async fn reset_statistics(&self) {
        *self.statistics.write().await = NetworkStatistics::default();
        info!("Network statistics reset");
    }
}

/// Initialize network module
pub fn init() -> Result<()> {
    info!("Network Module initialized");
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test]
    async fn test_network_manager_creation() {
        let manager = NetworkManager::new().unwrap();
        assert!(manager.initialize().await.is_ok());
    }
    
    #[tokio::test]
    async fn test_pqc_configuration() {
        let manager = NetworkManager::new().unwrap();
        let config = manager.get_config().await;
        
        assert!(config.enable_pqc_tls);
        assert_eq!(config.kem_algorithm, KemAlgorithm::CrystalsKyber768);
        assert_eq!(config.signature_algorithm, SignatureAlgorithm::CrystalsDilithium3);
        assert!(config.hybrid_mode);
    }
    
    #[tokio::test]
    async fn test_pqc_enable_disable() {
        let manager = NetworkManager::new().unwrap();
        
        manager.enable_pqc().await;
        assert!(manager.is_pqc_enabled().await);
        
        manager.disable_pqc().await;
        assert!(!manager.is_pqc_enabled().await);
        
        manager.enable_pqc().await;
        assert!(manager.is_pqc_enabled().await);
    }
    
    #[tokio::test]
    async fn test_statistics_tracking() {
        let manager = NetworkManager::new().unwrap();
        let stats = manager.get_statistics().await;
        
        assert_eq!(stats.total_connections, 0);
        assert_eq!(stats.pqc_connections, 0);
        assert_eq!(stats.classical_connections, 0);
    }
}