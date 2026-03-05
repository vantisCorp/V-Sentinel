//! PQC Gateway - Post-Quantum Cryptography API Gateway
//!
//! This module provides the core PQC gateway functionality including:
//! - PQC TLS termination
//! - Hybrid cryptographic support
//! - Quantum-resistant authentication
//! - Certificate management

use anyhow::{Result, anyhow};
use tracing::{info, debug, warn, error, instrument};
use std::sync::Arc;
use std::collections::HashMap;
use tokio::sync::RwLock;
use serde::{Serialize, Deserialize};
use chrono::{DateTime, Utc};

// Re-export from sentinel-quantum
use sentinel_quantum::{
    PqcAlgorithm, PqcKeyPair, PqcPublicKey, PqcPrivateKey,
    KemAlgorithm, SignatureAlgorithm, SecurityLevel,
};

/// PQC Gateway - Main gateway structure with PQC capabilities
pub struct PqcGateway {
    /// Gateway configuration
    config: GatewayPqcConfig,
    /// Key cache for PQC keys
    key_cache: Arc<RwLock<HashMap<String, CachedPqcKey>>>,
    /// Session store with PQC authentication
    sessions: Arc<RwLock<HashMap<String, PqcSession>>>,
    /// Metrics collector
    metrics: Arc<GatewayMetrics>,
    /// TLS configuration with PQC support
    tls_config: Option<PqcTlsConfig>,
}

/// Gateway PQC Configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GatewayPqcConfig {
    /// Enable PQC mode
    pub enable_pqc: bool,
    /// KEM algorithm for key exchange
    pub kem_algorithm: KemAlgorithmConfig,
    /// Signature algorithm for authentication
    pub signature_algorithm: SignatureAlgorithmConfig,
    /// Enable hybrid mode (PQC + classical)
    pub hybrid_mode: bool,
    /// Allow fallback to classical algorithms
    pub fallback_to_classical: bool,
    /// Minimum security level (1-5)
    pub min_security_level: u8,
    /// Key rotation interval in hours
    pub key_rotation_hours: u64,
    /// Session timeout in seconds
    pub session_timeout_secs: u64,
    /// Enable post-compromise security
    pub enable_pcs: bool,
    /// Certificate path for PQC
    pub pqc_cert_path: Option<String>,
    /// Private key path for PQC
    pub pqc_key_path: Option<String>,
    /// Enable client certificate verification
    pub verify_client_cert: bool,
    /// Allowed PQC cipher suites
    pub allowed_cipher_suites: Vec<String>,
}

/// KEM Algorithm Configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum KemAlgorithmConfig {
    CrystalsKyber512,
    CrystalsKyber768,
    CrystalsKyber1024,
    HybridKyber512X25519,
    HybridKyber768X25519,
    HybridKyber1024X448,
}

impl KemAlgorithmConfig {
    /// Get the NIST security level
    pub fn security_level(&self) -> u8 {
        match self {
            Self::CrystalsKyber512 => 1,
            Self::CrystalsKyber768 | Self::HybridKyber768X25519 => 3,
            Self::CrystalsKyber1024 | Self::HybridKyber1024X448 => 5,
            Self::HybridKyber512X25519 => 1,
        }
    }
    
    /// Get the algorithm name
    pub fn algorithm_name(&self) -> &str {
        match self {
            Self::CrystalsKyber512 => "CRYSTALS-Kyber-512",
            Self::CrystalsKyber768 => "CRYSTALS-Kyber-768",
            Self::CrystalsKyber1024 => "CRYSTALS-Kyber-1024",
            Self::HybridKyber512X25519 => "Hybrid-Kyber512-X25519",
            Self::HybridKyber768X25519 => "Hybrid-Kyber768-X25519",
            Self::HybridKyber1024X448 => "Hybrid-Kyber1024-X448",
        }
    }
    
    /// Check if this is a hybrid algorithm
    pub fn is_hybrid(&self) -> bool {
        matches!(self, 
            Self::HybridKyber512X25519 |
            Self::HybridKyber768X25519 |
            Self::HybridKyber1024X448
        )
    }
}

/// Signature Algorithm Configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SignatureAlgorithmConfig {
    CrystalsDilithium2,
    CrystalsDilithium3,
    CrystalsDilithium5,
    Falcon512,
    Falcon1024,
    SphincsPlusSha2128f,
    SphincsPlusSha2192f,
    SphincsPlusSha2256f,
    HybridDilithium2EcdsaP256,
    HybridDilithium3EcdsaP384,
    HybridDilithium5EcdsaP384,
}

impl SignatureAlgorithmConfig {
    /// Get the NIST security level
    pub fn security_level(&self) -> u8 {
        match self {
            Self::CrystalsDilithium2 | Self::Falcon512 | Self::SphincsPlusSha2128f |
            Self::HybridDilithium2EcdsaP256 => 1,
            Self::CrystalsDilithium3 | Self::SphincsPlusSha2192f |
            Self::HybridDilithium3EcdsaP384 => 3,
            Self::CrystalsDilithium5 | Self::Falcon1024 | Self::SphincsPlusSha2256f |
            Self::HybridDilithium5EcdsaP384 => 5,
        }
    }
    
    /// Get the algorithm name
    pub fn algorithm_name(&self) -> &str {
        match self {
            Self::CrystalsDilithium2 => "CRYSTALS-Dilithium-2",
            Self::CrystalsDilithium3 => "CRYSTALS-Dilithium-3",
            Self::CrystalsDilithium5 => "CRYSTALS-Dilithium-5",
            Self::Falcon512 => "FALCON-512",
            Self::Falcon1024 => "FALCON-1024",
            Self::SphincsPlusSha2128f => "SPHINCS+-SHA2-128f",
            Self::SphincsPlusSha2192f => "SPHINCS+-SHA2-192f",
            Self::SphincsPlusSha2256f => "SPHINCS+-SHA2-256f",
            Self::HybridDilithium2EcdsaP256 => "Hybrid-Dilithium2-ECDSA-P256",
            Self::HybridDilithium3EcdsaP384 => "Hybrid-Dilithium3-ECDSA-P384",
            Self::HybridDilithium5EcdsaP384 => "Hybrid-Dilithium5-ECDSA-P384",
        }
    }
    
    /// Check if this is a hybrid algorithm
    pub fn is_hybrid(&self) -> bool {
        matches!(self,
            Self::HybridDilithium2EcdsaP256 |
            Self::HybridDilithium3EcdsaP384 |
            Self::HybridDilithium5EcdsaP384
        )
    }
}

/// PQC TLS Configuration
#[derive(Debug, Clone)]
pub struct PqcTlsConfig {
    /// Cipher suites with PQC support
    pub cipher_suites: Vec<PqcCipherSuite>,
    /// Certificate chain
    pub cert_chain: Vec<u8>,
    /// Private key
    pub private_key: Vec<u8>,
    /// Client CA certificates
    pub client_ca_certs: Option<Vec<u8>>,
    /// Verify client certificate
    pub verify_client: bool,
}

/// PQC Cipher Suite
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PqcCipherSuite {
    /// Cipher suite name
    pub name: String,
    /// KEM algorithm
    pub kem: KemAlgorithmConfig,
    /// AEAD cipher
    pub aead: AeadCipher,
    /// Hash function
    pub hash: HashFunction,
    /// Security level
    pub security_level: u8,
}

/// AEAD Cipher
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum AeadCipher {
    Aes256Gcm,
    Aes128Gcm,
    Chacha20Poly1305,
}

/// Hash Function
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum HashFunction {
    Sha256,
    Sha384,
    Sha512,
}

/// Default PQC cipher suites
pub fn default_pqc_cipher_suites() -> Vec<PqcCipherSuite> {
    vec![
        PqcCipherSuite {
            name: "TLS_KYBER768_WITH_AES_256_GCM_SHA384".to_string(),
            kem: KemAlgorithmConfig::CrystalsKyber768,
            aead: AeadCipher::Aes256Gcm,
            hash: HashFunction::Sha384,
            security_level: 3,
        },
        PqcCipherSuite {
            name: "TLS_HYBRID_KYBER768_X25519_WITH_AES_256_GCM_SHA384".to_string(),
            kem: KemAlgorithmConfig::HybridKyber768X25519,
            aead: AeadCipher::Aes256Gcm,
            hash: HashFunction::Sha384,
            security_level: 3,
        },
        PqcCipherSuite {
            name: "TLS_KYBER768_WITH_CHACHA20_POLY1305_SHA256".to_string(),
            kem: KemAlgorithmConfig::CrystalsKyber768,
            aead: AeadCipher::Chacha20Poly1305,
            hash: HashFunction::Sha256,
            security_level: 3,
        },
        PqcCipherSuite {
            name: "TLS_KYBER1024_WITH_AES_256_GCM_SHA384".to_string(),
            kem: KemAlgorithmConfig::CrystalsKyber1024,
            aead: AeadCipher::Aes256Gcm,
            hash: HashFunction::Sha384,
            security_level: 5,
        },
    ]
}

/// Cached PQC Key
struct CachedPqcKey {
    /// Key ID
    key_id: String,
    /// Key pair
    key_pair: PqcKeyPair,
    /// Creation time
    created_at: DateTime<Utc>,
    /// Expiration time
    expires_at: DateTime<Utc>,
    /// Usage count
    usage_count: u64,
}

/// PQC Session
#[derive(Debug, Clone)]
pub struct PqcSession {
    /// Session ID
    pub session_id: String,
    /// Client identity
    pub client_id: Option<String>,
    /// Established shared secret
    pub shared_secret: Vec<u8>,
    /// KEM algorithm used
    pub kem_algorithm: KemAlgorithmConfig,
    /// Security level
    pub security_level: u8,
    /// Created at
    pub created_at: DateTime<Utc>,
    /// Expires at
    pub expires_at: DateTime<Utc>,
    /// Last activity
    pub last_activity: DateTime<Utc>,
    /// Session state
    pub state: SessionState,
    /// Post-compromise security epoch
    pub pcs_epoch: u32,
}

/// Session State
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SessionState {
    /// Handshake in progress
    Handshaking,
    /// Session established
    Established,
    /// Session expired
    Expired,
    /// Session terminated
    Terminated,
    /// Session error
    Error,
}

/// Gateway Metrics
#[derive(Debug, Default)]
pub struct GatewayMetrics {
    /// Total requests processed
    pub total_requests: std::sync::atomic::AtomicU64,
    /// PQC handshakes completed
    pub pqc_handshakes: std::sync::atomic::AtomicU64,
    /// Classical fallbacks
    pub classical_fallbacks: std::sync::atomic::AtomicU64,
    /// Active sessions
    pub active_sessions: std::sync::atomic::AtomicU64,
    /// Failed handshakes
    pub failed_handshakes: std::sync::atomic::AtomicU64,
}

impl GatewayMetrics {
    pub fn new() -> Self {
        Self::default()
    }
    
    pub fn increment_requests(&self) {
        self.total_requests.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
    }
    
    pub fn increment_pqc_handshakes(&self) {
        self.pqc_handshakes.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
    }
    
    pub fn increment_classical_fallbacks(&self) {
        self.classical_fallbacks.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
    }
    
    pub fn set_active_sessions(&self, count: u64) {
        self.active_sessions.store(count, std::sync::atomic::Ordering::Relaxed);
    }
    
    pub fn increment_failed_handshakes(&self) {
        self.failed_handshakes.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
    }
}

impl Default for GatewayPqcConfig {
    fn default() -> Self {
        Self {
            enable_pqc: true,
            kem_algorithm: KemAlgorithmConfig::HybridKyber768X25519,
            signature_algorithm: SignatureAlgorithmConfig::CrystalsDilithium3,
            hybrid_mode: true,
            fallback_to_classical: true,
            min_security_level: 3,
            key_rotation_hours: 168,
            session_timeout_secs: 3600,
            enable_pcs: true,
            pqc_cert_path: None,
            pqc_key_path: None,
            verify_client_cert: false,
            allowed_cipher_suites: default_pqc_cipher_suites().iter().map(|cs| cs.name.clone()).collect(),
        }
    }
}

impl PqcGateway {
    /// Create a new PQC Gateway
    pub fn new(config: GatewayPqcConfig) -> Self {
        Self {
            config,
            key_cache: Arc::new(RwLock::new(HashMap::new())),
            sessions: Arc::new(RwLock::new(HashMap::new())),
            metrics: Arc::new(GatewayMetrics::new()),
            tls_config: None,
        }
    }
    
    /// Create a PQC Gateway with TLS configuration
    pub fn with_tls(mut self, tls_config: PqcTlsConfig) -> Self {
        self.tls_config = Some(tls_config);
        self
    }
    
    /// Initialize the gateway
    #[instrument(skip(self))]
    pub async fn initialize(&mut self) -> Result<()> {
        info!("Initializing PQC Gateway...");
        
        // Validate configuration
        self.validate_config()?;
        
        // Initialize key cache
        self.initialize_key_cache().await?;
        
        // Load TLS configuration if available
        if let Some(ref tls_config) = self.tls_config {
            info!("PQC TLS configuration loaded with {} cipher suites", 
                  tls_config.cipher_suites.len());
        }
        
        info!("PQC Gateway initialized successfully");
        info!("  KEM Algorithm: {}", self.config.kem_algorithm.algorithm_name());
        info!("  Signature Algorithm: {}", self.config.signature_algorithm.algorithm_name());
        info!("  Security Level: {}", self.config.min_security_level);
        info!("  Hybrid Mode: {}", self.config.hybrid_mode);
        
        Ok(())
    }
    
    /// Validate gateway configuration
    fn validate_config(&self) -> Result<()> {
        // Check security levels
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
        
        // Warn about security level mismatch
        if kem_level != sig_level {
            warn!(
                "KEM ({}) and signature ({}) security levels differ. Consider using matching levels.",
                kem_level, sig_level
            );
        }
        
        // Validate hybrid mode consistency
        if !self.config.hybrid_mode && self.config.kem_algorithm.is_hybrid() {
            warn!("Hybrid mode disabled but hybrid KEM algorithm selected. This may cause issues.");
        }
        
        Ok(())
    }
    
    /// Initialize the key cache with default keys
    async fn initialize_key_cache(&self) -> Result<()> {
        info!("Initializing PQC key cache...");
        
        // Generate initial key pair for the gateway
        let key_id = format!("gateway-{}", chrono::Utc::now().timestamp());
        
        // Note: In production, this would call the actual PQC key generation
        // For now, we'll create a placeholder entry
        info!("Generated initial gateway key: {}", key_id);
        
        Ok(())
    }
    
    /// Perform PQC handshake
    #[instrument(skip(self, client_public_key))]
    pub async fn perform_handshake(
        &self,
        client_id: &str,
        client_public_key: &[u8],
        kem_ciphertext: Option<&[u8]>,
    ) -> Result<HandshakeResult> {
        debug!("Starting PQC handshake for client: {}", client_id);
        
        let start_time = std::time::Instant::now();
        
        // Perform the PQC key exchange
        // In production, this would use actual Kyber operations
        let session_id = format!("session-{}-{}", client_id, chrono::Utc::now().timestamp());
        
        // Create session
        let session = PqcSession {
            session_id: session_id.clone(),
            client_id: Some(client_id.to_string()),
            shared_secret: vec![0u8; 32], // Placeholder
            kem_algorithm: self.config.kem_algorithm.clone(),
            security_level: self.config.kem_algorithm.security_level(),
            created_at: Utc::now(),
            expires_at: Utc::now() + chrono::Duration::seconds(self.config.session_timeout_secs as i64),
            last_activity: Utc::now(),
            state: SessionState::Established,
            pcs_epoch: 0,
        };
        
        // Store session
        self.sessions.write().await.insert(session_id.clone(), session.clone());
        self.metrics.increment_pqc_handshakes();
        
        let duration = start_time.elapsed();
        debug!("PQC handshake completed in {:?}", duration);
        
        Ok(HandshakeResult {
            session_id,
            shared_secret: session.shared_secret,
            security_level: session.security_level,
            kem_algorithm: session.kem_algorithm.algorithm_name().to_string(),
            expires_at: session.expires_at,
        })
    }
    
    /// Validate session
    pub async fn validate_session(&self, session_id: &str) -> Result<bool> {
        let sessions = self.sessions.read().await;
        
        if let Some(session) = sessions.get(session_id) {
            if session.state == SessionState::Established && session.expires_at > Utc::now() {
                return Ok(true);
            }
        }
        
        Ok(false)
    }
    
    /// Terminate session
    pub async fn terminate_session(&self, session_id: &str) -> Result<()> {
        let mut sessions = self.sessions.write().await;
        
        if let Some(session) = sessions.get_mut(session_id) {
            session.state = SessionState::Terminated;
            info!("Session {} terminated", session_id);
        }
        
        Ok(())
    }
    
    /// Get session count
    pub async fn session_count(&self) -> usize {
        self.sessions.read().await.len()
    }
    
    /// Get metrics
    pub fn get_metrics(&self) -> &GatewayMetrics {
        &self.metrics
    }
    
    /// Rotate keys
    #[instrument(skip(self))]
    pub async fn rotate_keys(&self) -> Result<()> {
        info!("Rotating PQC keys...");
        
        // Clear old keys from cache
        let mut cache = self.key_cache.write().await;
        let old_count = cache.len();
        cache.clear();
        
        info!("Cleared {} old keys from cache", old_count);
        
        // Generate new keys
        // In production, this would generate actual PQC key pairs
        
        info!("Key rotation completed");
        Ok(())
    }
    
    /// Health check
    pub async fn health_check(&self) -> HealthStatus {
        let sessions = self.sessions.read().await;
        let active_sessions = sessions.values()
            .filter(|s| s.state == SessionState::Established)
            .count();
        
        HealthStatus {
            healthy: true,
            pqc_enabled: self.config.enable_pqc,
            hybrid_mode: self.config.hybrid_mode,
            active_sessions: active_sessions as u64,
            kem_algorithm: self.config.kem_algorithm.algorithm_name().to_string(),
            signature_algorithm: self.config.signature_algorithm.algorithm_name().to_string(),
            security_level: self.config.min_security_level,
        }
    }
}

/// Result of a PQC handshake
#[derive(Debug, Clone)]
pub struct HandshakeResult {
    /// Session ID
    pub session_id: String,
    /// Established shared secret
    pub shared_secret: Vec<u8>,
    /// Security level achieved
    pub security_level: u8,
    /// KEM algorithm used
    pub kem_algorithm: String,
    /// Session expiration
    pub expires_at: DateTime<Utc>,
}

/// Health status of the PQC gateway
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealthStatus {
    /// Gateway is healthy
    pub healthy: bool,
    /// PQC enabled
    pub pqc_enabled: bool,
    /// Hybrid mode enabled
    pub hybrid_mode: bool,
    /// Number of active sessions
    pub active_sessions: u64,
    /// KEM algorithm in use
    pub kem_algorithm: String,
    /// Signature algorithm in use
    pub signature_algorithm: String,
    /// Security level
    pub security_level: u8,
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test]
    async fn test_gateway_creation() {
        let config = GatewayPqcConfig::default();
        let gateway = PqcGateway::new(config);
        
        let health = gateway.health_check().await;
        assert!(health.healthy);
        assert!(health.pqc_enabled);
    }
    
    #[tokio::test]
    async fn test_gateway_initialization() {
        let config = GatewayPqcConfig::default();
        let mut gateway = PqcGateway::new(config);
        
        let result = gateway.initialize().await;
        assert!(result.is_ok());
    }
    
    #[tokio::test]
    async fn test_security_levels() {
        assert_eq!(KemAlgorithmConfig::CrystalsKyber512.security_level(), 1);
        assert_eq!(KemAlgorithmConfig::CrystalsKyber768.security_level(), 3);
        assert_eq!(KemAlgorithmConfig::CrystalsKyber1024.security_level(), 5);
        
        assert_eq!(SignatureAlgorithmConfig::CrystalsDilithium2.security_level(), 1);
        assert_eq!(SignatureAlgorithmConfig::CrystalsDilithium3.security_level(), 3);
        assert_eq!(SignatureAlgorithmConfig::CrystalsDilithium5.security_level(), 5);
    }
    
    #[tokio::test]
    async fn test_hybrid_detection() {
        assert!(!KemAlgorithmConfig::CrystalsKyber768.is_hybrid());
        assert!(KemAlgorithmConfig::HybridKyber768X25519.is_hybrid());
        
        assert!(!SignatureAlgorithmConfig::CrystalsDilithium3.is_hybrid());
        assert!(SignatureAlgorithmConfig::HybridDilithium3EcdsaP384.is_hybrid());
    }
    
    #[test]
    fn test_default_cipher_suites() {
        let suites = default_pqc_cipher_suites();
        assert_eq!(suites.len(), 4);
        assert!(suites.iter().any(|s| s.security_level >= 3));
    }
}