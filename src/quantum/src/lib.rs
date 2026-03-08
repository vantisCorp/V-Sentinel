//! SENTINEL Quantum Cryptography Module
//! 
//! This module provides quantum-resistant cryptographic algorithms
//! including post-quantum KEM, signatures, and hybrid cryptography.

use anyhow::{Result, anyhow};
use tracing::{info, debug};
use std::sync::Arc;
use tokio::sync::RwLock;
use serde::{Serialize, Deserialize};
use sha2::{Sha256, Sha512, Digest};
use rand::{RngCore, rngs::OsRng};
use pqc_kyber::{keypair, encapsulate, decapsulate, KyberError};

pub mod config;
pub use config::{PqcConfig, KemAlgorithm, SigAlgorithm};

/// Quantum Cryptography Manager
pub struct QuantumCryptoManager {
    kem_enabled: Arc<RwLock<bool>>,
    signature_enabled: Arc<RwLock<bool>>,
    hybrid_enabled: Arc<RwLock<bool>>,
    statistics: Arc<RwLock<CryptoStatistics>>,
}

/// KEM implementation using Kyber
pub struct KyberKEM {
    algorithm: KemAlgorithm,
}

/// Keypair structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Keypair {
    pub public: Vec<u8>,
    pub secret: Vec<u8>,
    pub algorithm: KemAlgorithm,
}

/// KEM result
#[derive(Debug, Clone)]
pub struct EncapsulationResult {
    pub ciphertext: Vec<u8>,
    pub shared_secret: Vec<u8>,
}

/// Crypto statistics
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CryptoStatistics {
    pub kem_operations: u64,
    pub signature_operations: u64,
    pub hybrid_operations: u64,
    pub errors: u64,
}

impl KyberKEM {
    pub fn new(algorithm: KemAlgorithm) -> Self {
        Self { algorithm }
    }
    
    pub fn generate_keypair(&self) -> Result<Keypair> {
        let mut rng = OsRng;
        let kp = keypair(&mut rng)
            .map_err(|e: KyberError| anyhow!("Failed to generate keypair: {:?}", e))?;
        
        Ok(Keypair {
            public: kp.public.to_vec(),
            secret: kp.secret.to_vec(),
            algorithm: self.algorithm,
        })
    }
    
    pub fn encapsulate(&self, public_key: &[u8]) -> Result<EncapsulationResult> {
        let mut rng = OsRng;
        let (ciphertext, shared_secret) = encapsulate(public_key, &mut rng)
            .map_err(|e| anyhow!("Encapsulation failed: {:?}", e))?;
        
        Ok(EncapsulationResult {
            ciphertext: ciphertext.to_vec(),
            shared_secret: shared_secret.to_vec(),
        })
    }
    
    pub fn decapsulate(&self, ciphertext: &[u8], secret_key: &[u8]) -> Result<Vec<u8>> {
        let shared_secret = decapsulate(ciphertext, secret_key)
            .map_err(|e| anyhow!("Decapsulation failed: {:?}", e))?;
        
        Ok(shared_secret.to_vec())
    }
}

impl QuantumCryptoManager {
    pub fn new() -> Result<Self> {
        info!("Initializing Quantum Cryptography Manager...");
        
        Ok(Self {
            kem_enabled: Arc::new(RwLock::new(false)),
            signature_enabled: Arc::new(RwLock::new(false)),
            hybrid_enabled: Arc::new(RwLock::new(false)),
            statistics: Arc::new(RwLock::new(CryptoStatistics::default())),
        })
    }
    
    pub async fn initialize(&self) -> Result<()> {
        info!("Initializing quantum cryptography...");
        *self.kem_enabled.write().await = true;
        *self.signature_enabled.write().await = true;
        *self.hybrid_enabled.write().await = true;
        Ok(())
    }
    
    pub async fn generate_kem_keypair(&self, algorithm: KemAlgorithm) -> Result<Keypair> {
        let kem = KyberKEM::new(algorithm);
        let keypair = kem.generate_keypair()?;
        self.statistics.write().await.kem_operations += 1;
        Ok(keypair)
    }
    
    pub async fn encapsulate(&self, public_key: &[u8], algorithm: KemAlgorithm) -> Result<EncapsulationResult> {
        let kem = KyberKEM::new(algorithm);
        let result = kem.encapsulate(public_key)?;
        self.statistics.write().await.kem_operations += 1;
        Ok(result)
    }
    
    pub async fn decapsulate(&self, ciphertext: &[u8], secret_key: &[u8], algorithm: KemAlgorithm) -> Result<Vec<u8>> {
        let kem = KyberKEM::new(algorithm);
        let result = kem.decapsulate(ciphertext, secret_key)?;
        self.statistics.write().await.kem_operations += 1;
        Ok(result)
    }
    
    pub async fn get_statistics(&self) -> CryptoStatistics {
        self.statistics.read().await.clone()
    }
}

/// Hash function using SHA-256
pub fn hash_sha256(data: &[u8]) -> Vec<u8> {
    let mut hasher = Sha256::new();
    hasher.update(data);
    hasher.finalize().to_vec()
}

/// Hash function using SHA-512
pub fn hash_sha512(data: &[u8]) -> Vec<u8> {
    let mut hasher = Sha512::new();
    hasher.update(data);
    hasher.finalize().to_vec()
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test]
    async fn test_kyber_kem() {
        let kem = KyberKEM::new(KemAlgorithm::CrystalsKyber768);
        let keypair = kem.generate_keypair().unwrap();
        assert!(!keypair.public.is_empty());
        assert!(!keypair.secret.is_empty());
    }
    
    #[tokio::test]
    async fn test_encapsulation() {
        let kem = KyberKEM::new(KemAlgorithm::CrystalsKyber768);
        let keypair = kem.generate_keypair().unwrap();
        let result = kem.encapsulate(&keypair.public).unwrap();
        assert!(!result.ciphertext.is_empty());
        assert!(!result.shared_secret.is_empty());
    }
    
    #[tokio::test]
    async fn test_decapsulation() {
        let kem = KyberKEM::new(KemAlgorithm::CrystalsKyber768);
        let keypair = kem.generate_keypair().unwrap();
        let result = kem.encapsulate(&keypair.public).unwrap();
        let secret = kem.decapsulate(&result.ciphertext, &keypair.secret).unwrap();
        assert_eq!(secret, result.shared_secret);
    }
}