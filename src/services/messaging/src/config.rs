//! Messaging Configuration Module

use anyhow::{Result, anyhow};
use serde::{Serialize, Deserialize};

/// Messaging Configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MessagingConfig {
    /// PQC configuration
    pub pqc: MessagingPqcConfig,
    /// Storage configuration
    pub storage: StorageConfig,
}

/// Messaging PQC Configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MessagingPqcConfig {
    pub enable_pqc: bool,
    pub kem_algorithm: MessagingKemAlgorithm,
    pub signature_algorithm: MessagingSignatureAlgorithm,
    pub hybrid_mode: bool,
    pub min_security_level: u8,
    pub message_ttl_secs: u64,
    pub enable_forward_secrecy: bool,
    pub max_message_size: usize,
}

impl Default for MessagingPqcConfig {
    fn default() -> Self {
        Self {
            enable_pqc: true,
            kem_algorithm: MessagingKemAlgorithm::HybridKyber768X25519,
            signature_algorithm: MessagingSignatureAlgorithm::CrystalsDilithium3,
            hybrid_mode: true,
            min_security_level: 3,
            message_ttl_secs: 604800,
            enable_forward_secrecy: true,
            max_message_size: 10485760,
        }
    }
}

/// Messaging KEM Algorithm
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MessagingKemAlgorithm {
    CrystalsKyber512,
    CrystalsKyber768,
    CrystalsKyber1024,
    HybridKyber768X25519,
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

/// Storage Configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorageConfig {
    pub backend: StorageBackend,
    pub path: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum StorageBackend {
    Memory,
    File,
    Database,
}

impl Default for MessagingConfig {
    fn default() -> Self {
        Self {
            pqc: MessagingPqcConfig::default(),
            storage: StorageConfig {
                backend: StorageBackend::Memory,
                path: None,
            },
        }
    }
}

impl MessagingConfig {
    pub fn validate(&self) -> Result<()> {
        if self.pqc.enable_pqc {
            let kem_level = self.pqc.kem_algorithm.security_level();
            if kem_level < self.pqc.min_security_level {
                return Err(anyhow!(
                    "KEM algorithm security level ({}) is below minimum ({})",
                    kem_level, self.pqc.min_security_level
                ));
            }
        }
        Ok(())
    }
}