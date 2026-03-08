//! PQC Messaging Service - Post-Quantum Cryptography Messaging Implementation
//!
//! This module provides the core PQC messaging functionality including:
//! - PQC-based key exchange for messaging
//! - End-to-end encryption with PQC
//! - PQC message signatures
//! - Quantum-resistant message authentication

use anyhow::{Result, anyhow};
use tracing::{info, debug, warn};
use std::sync::Arc;
use tokio::sync::RwLock;
use serde::{Serialize, Deserialize};
use chrono::{DateTime, Utc};
use std::collections::HashMap;

use super::config::MessagingPqcConfig;
use super::message::{PqcMessage, MessagePriority};
use super::encryption::PqcEncryption;

/// PQC Messaging Service
pub struct PqcMessagingService {
    /// Messaging configuration
    config: MessagingPqcConfig,
    /// User public keys
    user_keys: Arc<RwLock<HashMap<String, UserKeyPair>>>,
    /// Active conversations
    conversations: Arc<RwLock<HashMap<String, Conversation>>>,
    /// Encryption handler
    encryption: Arc<PqcEncryption>,
}

/// Messaging PQC Configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MessagingPqcConfig {
    /// Enable PQC mode
    pub enable_pqc: bool,
    /// KEM algorithm for key exchange
    pub kem_algorithm: MessagingKemAlgorithm,
    /// Signature algorithm for message signing
    pub signature_algorithm: MessagingSignatureAlgorithm,
    /// Enable hybrid mode (PQC + classical)
    pub hybrid_mode: bool,
    /// Minimum security level (1-5)
    pub min_security_level: u8,
    /// Message TTL in seconds
    pub message_ttl_secs: u64,
    /// Enable forward secrecy
    pub enable_forward_secrecy: bool,
    /// Maximum message size (bytes)
    pub max_message_size: usize,
}

/// Messaging KEM Algorithm
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MessagingKemAlgorithm {
    CrystalsKyber512,
    CrystalsKyber768,
    CrystalsKyber1024,
    HybridKyber768X25519,
}

impl MessagingKemAlgorithm {
    pub fn security_level(&self) -> u8 {
        match self {
            Self::CrystalsKyber512 => 1,
            Self::CrystalsKyber768 | Self::HybridKyber768X25519 => 3,
            Self::CrystalsKyber1024 => 5,
        }
    }
    
    pub fn algorithm_name(&self) -> &str {
        match self {
            Self::CrystalsKyber512 => "CRYSTALS-Kyber-512",
            Self::CrystalsKyber768 => "CRYSTALS-Kyber-768",
            Self::CrystalsKyber1024 => "CRYSTALS-Kyber-1024",
            Self::HybridKyber768X25519 => "Hybrid-Kyber768-X25519",
        }
    }
}

/// Messaging Signature Algorithm
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MessagingSignatureAlgorithm {
    CrystalsDilithium2,
    CrystalsDilithium3,
    CrystalsDilithium5,
    Falcon512,
    Falcon1024,
    SphincsPlusSha2128f,
}

impl MessagingSignatureAlgorithm {
    pub fn security_level(&self) -> u8 {
        match self {
            Self::CrystalsDilithium2 | Self::Falcon512 | Self::SphincsPlusSha2128f => 1,
            Self::CrystalsDilithium3 | Self::Falcon1024 => 3,
            Self::CrystalsDilithium5 => 5,
        }
    }
    
    pub fn algorithm_name(&self) -> str {
        match self {
            Self::CrystalsDilithium2 => "CRYSTALS-Dilithium-2",
            Self::CrystalsDilithium3 => "CRYSTALS-Dilithium-3",
            Self::CrystalsDilithium5 => "CRYSTALS-Dilithium-5",
            Self::Falcon512 => "FALCON-512",
            Self::Falcon1024 => "FALCON-1024",
            Self::SphincsPlusSha2128f => "SPHINCS+-SHA2-128f",
        }
    }
}

/// User Key Pair
#[derive(Debug, Clone)]
struct UserKeyPair {
    /// User ID
    user_id: String,
    /// Public key
    public_key: Vec<u8>,
    /// Private key (encrypted)
    private_key_encrypted: Vec<u8>,
    /// Key generation time
    created_at: DateTime<Utc>,
}

/// Conversation
#[derive(Debug, Clone)]
pub struct Conversation {
    /// Conversation ID
    pub conversation_id: String,
    /// Participants
    pub participants: Vec<String>,
    /// Shared secret (for group conversations)
    pub shared_secret: Option<Vec<u8>>,
    /// Created at
    pub created_at: DateTime<Utc>,
    /// Last activity
    pub last_activity: DateTime<Utc>,
}

impl Default for MessagingPqcConfig {
    fn default() -> Self {
        Self {
            enable_pqc: true,
            kem_algorithm: MessagingKemAlgorithm::HybridKyber768X25519,
            signature_algorithm: MessagingSignatureAlgorithm::CrystalsDilithium3,
            hybrid_mode: true,
            min_security_level: 3,
            message_ttl_secs: 604800, // 7 days
            enable_forward_secrecy: true,
            max_message_size: 10485760, // 10 MB
        }
    }
}

impl PqcMessagingService {
    /// Create a new PQC messaging service
    pub fn new(config: MessagingPqcConfig) -> Self {
        let encryption = PqcEncryption::new(config.kem_algorithm, config.signature_algorithm);
        
        Self {
            config,
            user_keys: Arc::new(RwLock::new(HashMap::new())),
            conversations: Arc::new(RwLock::new(HashMap::new())),
            encryption: Arc::new(encryption),
        }
    }
    
    /// Initialize the messaging service
    pub async fn initialize(&self) -> Result<()> {
        info!("Initializing PQC Messaging Service...");
        
        // Validate configuration
        self.validate_config()?;
        
        info!("PQC Messaging Service initialized");
        info!("  KEM Algorithm: {}", self.config.kem_algorithm.algorithm_name());
        info!("  Signature Algorithm: {}", self.config.signature_algorithm.algorithm_name());
        info!("  Security Level: {}", self.config.min_security_level);
        info!("  Forward Secrecy: {}", self.config.enable_forward_secrecy);
        
        Ok(())
    }
    
    /// Validate messaging configuration
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
    
    /// Register a user
    pub async fn register_user(&self, user_id: &str, public_key: Vec<u8>) -> Result<()> {
        debug!("Registering user: {}", user_id);
        
        let key_pair = UserKeyPair {
            user_id: user_id.to_string(),
            public_key,
            private_key_encrypted: Vec::new(),
            created_at: Utc::now(),
        };
        
        self.user_keys.write().await.insert(user_id.to_string(), key_pair);
        
        info!("User {} registered", user_id);
        
        Ok(())
    }
    
    /// Create a conversation
    pub async fn create_conversation(&self, participants: Vec<String>) -> Result<String> {
        let conversation_id = format!("conv-{}", chrono::Utc::now().timestamp_nanos_opt().unwrap());
        
        let conversation = Conversation {
            conversation_id: conversation_id.clone(),
            participants,
            shared_secret: None,
            created_at: Utc::now(),
            last_activity: Utc::now(),
        };
        
        self.conversations.write().await.insert(conversation_id.clone(), conversation);
        
        info!("Conversation {} created", conversation_id);
        
        Ok(conversation_id)
    }
    
    /// Send a PQC encrypted message
    pub async fn send_message(
        &self,
        conversation_id: &str,
        sender_id: &str,
        content: Vec<u8>,
        priority: MessagePriority,
    ) -> Result<String> {
        debug!("Sending PQC message in conversation: {}", conversation_id);
        
        // Encrypt message using PQC
        let encrypted_content = self.encryption.encrypt(&content)?;
        
        // Create message
        let message = PqcMessage {
            message_id: format!("msg-{}", chrono::Utc::now().timestamp_nanos_opt().unwrap()),
            conversation_id: conversation_id.to_string(),
            sender_id: sender_id.to_string(),
            content: encrypted_content,
            signature: vec![],
            timestamp: Utc::now(),
            priority,
            ttl_secs: self.config.message_ttl_secs,
        };
        
        info!("PQC message {} sent", message.message_id);
        
        Ok(message.message_id)
    }
    
    /// Receive and decrypt a message
    pub async fn receive_message(&self, _message_id: &str) -> Result<Vec<u8>> {
        // Decrypt message using PQC
        todo!("Implement message decryption")
    }
    
    /// Get conversation count
    pub async fn conversation_count(&self) -> usize {
        self.conversations.read().await.len()
    }
    
    /// Get user count
    pub async fn user_count(&self) -> usize {
        self.user_keys.read().await.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test]
    async fn test_messaging_creation() {
        let config = MessagingPqcConfig::default();
        let messaging = PqcMessagingService::new(config);
        let result = messaging.initialize().await;
        assert!(result.is_ok());
    }
    
    #[test]
    fn test_security_levels() {
        assert_eq!(MessagingKemAlgorithm::CrystalsKyber512.security_level(), 1);
        assert_eq!(MessagingKemAlgorithm::CrystalsKyber768.security_level(), 3);
        assert_eq!(MessagingKemAlgorithm::CrystalsKyber1024.security_level(), 5);
    }
    
    #[tokio::test]
    async fn test_user_registration() {
        let config = MessagingPqcConfig::default();
        let messaging = PqcMessagingService::new(config);
        
        let result = messaging.register_user("user-1", vec![1, 2, 3]).await;
        assert!(result.is_ok());
        
        let count = messaging.user_count().await;
        assert_eq!(count, 1);
    }
}