//! PQC TLS Implementation
//! 
//! This module provides post-quantum cryptography support for TLS connections,
//! including hybrid key exchange and quantum-safe authentication.

use anyhow::{Result, anyhow};
use tracing::{info, debug, warn, error};
use std::sync::Arc;
use tokio::sync::RwLock;
use serde::{Serialize, Deserialize};

use crate::{KemAlgorithm, SignatureAlgorithm};

/// PQC TLS Configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PqcTlsConfig {
    /// Enable PQC TLS
    pub enabled: bool,
    /// KEM algorithm for key exchange
    pub kem_algorithm: KemAlgorithm,
    /// Signature algorithm for certificates
    pub signature_algorithm: SignatureAlgorithm,
    /// Enable hybrid mode
    pub hybrid_mode: bool,
    /// TLS version
    pub tls_version: TlsVersion,
    /// Cipher suites
    pub cipher_suites: Vec<CipherSuite>,
}

/// TLS Version
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TlsVersion {
    Tls12,
    Tls13,
}

/// PQC Cipher Suites
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CipherSuite {
    // PQC KEM cipher suites
    TlsKyberWithAes256GcmSha384,
    TlsKyberWithChaCha20Poly1305Sha256,
    
    // Hybrid KEM cipher suites (PQC + classical)
    TlsHybridKyberX25519WithAes256GcmSha384,
    TlsHybridKyberX25519WithChaCha20Poly1305Sha256,
    
    // Classical cipher suites (for fallback)
    TlsAes256GcmSha384,
    TlsChaCha20Poly1305Sha256,
}

impl Default for PqcTlsConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            kem_algorithm: KemAlgorithm::CrystalsKyber768,
            signature_algorithm: SignatureAlgorithm::CrystalsDilithium3,
            hybrid_mode: true,
            tls_version: TlsVersion::Tls13,
            cipher_suites: vec![
                CipherSuite::TlsHybridKyberX25519WithAes256GcmSha384,
                CipherSuite::TlsHybridKyberX25519WithChaCha20Poly1305Sha256,
                CipherSuite::TlsKyberWithAes256GcmSha384,
                CipherSuite::TlsAes256GcmSha384, // Fallback
            ],
        }
    }
}

/// PQC TLS Manager
pub struct PqcTlsManager {
    config: Arc<RwLock<PqcTlsConfig>>,
    statistics: Arc<RwLock<TlsStatistics>>,
}

/// TLS Statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TlsStatistics {
    /// Total handshakes
    pub total_handshakes: u64,
    /// PQC handshakes
    pub pqc_handshakes: u64,
    /// Hybrid handshakes
    pub hybrid_handshakes: u64,
    /// Classical handshakes
    pub classical_handshakes: u64,
    /// Failed handshakes
    pub failed_handshakes: u64,
    /// Average handshake time (ms)
    pub avg_handshake_time_ms: f64,
}

impl Default for TlsStatistics {
    fn default() -> Self {
        Self {
            total_handshakes: 0,
            pqc_handshakes: 0,
            hybrid_handshakes: 0,
            classical_handshakes: 0,
            failed_handshakes: 0,
            avg_handshake_time_ms: 0.0,
        }
    }
}

impl PqcTlsManager {
    /// Create a new PQC TLS manager
    pub fn new() -> Result<Self> {
        info!("Creating PQC TLS Manager...");
        
        Ok(Self {
            config: Arc::new(RwLock::new(PqcTlsConfig::default())),
            statistics: Arc::new(RwLock::new(TlsStatistics::default())),
        })
    }
    
    /// Initialize PQC TLS
    pub async fn initialize(&self) -> Result<()> {
        info!("Initializing PQC TLS...");
        
        let config = self.config.read().await;
        
        if config.enabled {
            info!("PQC TLS enabled with hybrid mode: {}", config.hybrid_mode);
            info!("KEM Algorithm: {:?}", config.kem_algorithm);
            info!("Signature Algorithm: {:?}", config.signature_algorithm);
            info!("TLS Version: {:?}", config.tls_version);
            info!("Cipher Suites: {} configured", config.cipher_suites.len());
        } else {
            warn!("PQC TLS disabled - using classical TLS only");
        }
        
        Ok(())
    }
    
    /// Create PQC TLS context
    pub async fn create_context(&self) -> Result<PqcTlsContext> {
        let config = self.config.read().await;
        
        let context = PqcTlsContext {
            kem_algorithm: config.kem_algorithm,
            signature_algorithm: config.signature_algorithm,
            hybrid_mode: config.hybrid_mode,
            tls_version: config.tls_version,
            cipher_suites: config.cipher_suites.clone(),
        };
        
        Ok(context)
    }
    
    /// Update configuration
    pub async fn update_config(&self, config: PqcTlsConfig) -> Result<()> {
        self.validate_config(&config)?;
        *self.config.write().await = config;
        info!("PQC TLS configuration updated");
        Ok(())
    }
    
    /// Validate configuration
    fn validate_config(&self, config: &PqcTlsConfig) -> Result<()> {
        // Ensure at least one cipher suite
        if config.cipher_suites.is_empty() {
            return Err(anyhow!("At least one cipher suite must be configured"));
        }
        
        // Validate TLS version
        if config.tls_version != TlsVersion::Tls13 {
            warn!("TLS 1.2 is deprecated. TLS 1.3 is recommended for PQC.");
        }
        
        Ok(())
    }
    
    /// Get statistics
    pub async fn get_statistics(&self) -> TlsStatistics {
        self.statistics.read().await.clone()
    }
    
    /// Get cipher suite name
    pub fn get_cipher_suite_name(suite: &CipherSuite) -> &'static str {
        match suite {
            CipherSuite::TlsKyberWithAes256GcmSha384 => "TLS_KYBER_WITH_AES_256_GCM_SHA384",
            CipherSuite::TlsKyberWithChaCha20Poly1305Sha256 => "TLS_KYBER_WITH_CHACHA20_POLY1305_SHA256",
            CipherSuite::TlsHybridKyberX25519WithAes256GcmSha384 => "TLS_HYBRID_KYBER_X25519_WITH_AES_256_GCM_SHA384",
            CipherSuite::TlsHybridKyberX25519WithChaCha20Poly1305Sha256 => "TLS_HYBRID_KYBER_X25519_WITH_CHACHA20_POLY1305_SHA256",
            CipherSuite::TlsAes256GcmSha384 => "TLS_AES_256_GCM_SHA384",
            CipherSuite::TlsChaCha20Poly1305Sha256 => "TLS_CHACHA20_POLY1305_SHA256",
        }
    }
}

/// PQC TLS Context
#[derive(Debug, Clone)]
pub struct PqcTlsContext {
    pub kem_algorithm: KemAlgorithm,
    pub signature_algorithm: SignatureAlgorithm,
    pub hybrid_mode: bool,
    pub tls_version: TlsVersion,
    pub cipher_suites: Vec<CipherSuite>,
}

/// PQC Handshake Result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HandshakeResult {
    pub success: bool,
    pub pqc_used: bool,
    pub hybrid_used: bool,
    pub kem_algorithm: Option<KemAlgorithm>,
    pub signature_algorithm: Option<SignatureAlgorithm>,
    pub cipher_suite: Option<String>,
    pub handshake_time_ms: f64,
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test]
    async fn test_pqc_tls_manager_creation() {
        let manager = PqcTlsManager::new().unwrap();
        assert!(manager.initialize().await.is_ok());
    }
    
    #[tokio::test]
    async fn test_pqc_tls_configuration() {
        let manager = PqcTlsManager::new().unwrap();
        let context = manager.create_context().await.unwrap();
        
        assert!(context.hybrid_mode);
        assert_eq!(context.kem_algorithm, KemAlgorithm::CrystalsKyber768);
        assert_eq!(context.signature_algorithm, SignatureAlgorithm::CrystalsDilithium3);
        assert!(!context.cipher_suites.is_empty());
    }
    
    #[tokio::test]
    async fn test_cipher_suite_names() {
        let suite = CipherSuite::TlsHybridKyberX25519WithAes256GcmSha384;
        assert_eq!(
            PqcTlsManager::get_cipher_suite_name(&suite),
            "TLS_HYBRID_KYBER_X25519_WITH_AES_256_GCM_SHA384"
        );
    }
    
    #[tokio::test]
    async fn test_statistics() {
        let manager = PqcTlsManager::new().unwrap();
        let stats = manager.get_statistics().await;
        
        assert_eq!(stats.total_handshakes, 0);
        assert_eq!(stats.pqc_handshakes, 0);
    }
}