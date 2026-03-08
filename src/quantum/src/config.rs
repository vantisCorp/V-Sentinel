//! PQC Configuration Module
//!
//! This module provides configuration management for post-quantum cryptographic algorithms,
//! allowing users to select algorithms, parameters, and security levels.

use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::Path;

/// PQC Configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PqcConfig {
    /// Default KEM algorithm
    pub default_kem: KemConfig,
    /// Default signature algorithm
    pub default_signature: SigConfig,
    /// Hybrid crypto settings
    pub hybrid_crypto: HybridConfig,
    /// Algorithm-specific parameters
    pub algorithm_params: HashMap<String, serde_json::Value>,
}

/// KEM Configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KemConfig {
    /// Algorithm variant
    pub algorithm: KemAlgorithm,
    /// Security level (1-5)
    pub security_level: u8,
    /// Enable hybrid mode (classical + PQC)
    pub hybrid_mode: bool,
}

/// Signature Configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SigConfig {
    /// Algorithm variant
    pub algorithm: SigAlgorithm,
    /// Security level (1-5)
    pub security_level: u8,
    /// Enable hybrid mode (classical + PQC)
    pub hybrid_mode: bool,
}

/// Hybrid Crypto Configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HybridConfig {
    /// KDF for key derivation
    pub kdf: KeyDerivationFunction,
    /// Classical encryption algorithm
    pub classical_algorithm: ClassicalAlgorithm,
    /// Enable post-compromise security
    pub enable_pcs: bool,
}

/// Key Derivation Function
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum KeyDerivationFunction {
    HKDFSha256,
    HKDFSha384,
    HKDFSha512,
    Argon2id,
}

/// Classical Algorithm for Hybrid Mode
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ClassicalAlgorithm {
    X25519,
    Ed25519,
    P256,
    P384,
    P521,
}

/// KEM Algorithms
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum KemAlgorithm {
    CrystalsKyber512,
    CrystalsKyber768,
    CrystalsKyber1024,
    ClassicMcEliece348864,
    ClassicMcEliece460896,
    NtruHps2048509,
    NtruHps4096821,
    SaberLightSaber,
    SaberSaber,
    SaberFireSaber,
}

/// Signature Algorithms
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SigAlgorithm {
    CrystalsDilithium2,
    CrystalsDilithium3,
    CrystalsDilithium5,
    Falcon512,
    Falcon1024,
    SphincsPlusSha256128f,
    SphincsPlusSha256128s,
    SphincsPlusShake256128f,
    SphincsPlusShake256128s,
}

impl Default for PqcConfig {
    fn default() -> Self {
        Self {
            default_kem: KemConfig {
                algorithm: KemAlgorithm::CrystalsKyber768,
                security_level: 3,
                hybrid_mode: true,
            },
            default_signature: SigConfig {
                algorithm: SigAlgorithm::CrystalsDilithium3,
                security_level: 3,
                hybrid_mode: true,
            },
            hybrid_crypto: HybridConfig {
                kdf: KeyDerivationFunction::HKDFSha256,
                classical_algorithm: ClassicalAlgorithm::X25519,
                enable_pcs: true,
            },
            algorithm_params: HashMap::new(),
        }
    }
}

impl PqcConfig {
    /// Create a new default configuration
    pub fn new() -> Self {
        Self::default()
    }

    /// Load configuration from file
    pub fn load_from_file<P: AsRef<Path>>(path: P) -> Result<Self> {
        let content = fs::read_to_string(path)?;
        let config: PqcConfig = serde_json::from_str(&content)?;
        Ok(config)
    }

    /// Save configuration to file
    pub fn save_to_file<P: AsRef<Path>>(&self, path: P) -> Result<()> {
        let content = serde_json::to_string_pretty(self)?;
        fs::write(path, content)?;
        Ok(())
    }

    /// Set default KEM algorithm
    pub fn set_default_kem(
        &mut self,
        algorithm: KemAlgorithm,
        security_level: u8,
        hybrid_mode: bool,
    ) {
        self.default_kem = KemConfig {
            algorithm,
            security_level,
            hybrid_mode,
        };
    }

    /// Set default signature algorithm
    pub fn set_default_signature(
        &mut self,
        algorithm: SigAlgorithm,
        security_level: u8,
        hybrid_mode: bool,
    ) {
        self.default_signature = SigConfig {
            algorithm,
            security_level,
            hybrid_mode,
        };
    }

    /// Set hybrid crypto configuration
    pub fn set_hybrid_config(
        &mut self,
        kdf: KeyDerivationFunction,
        classical: ClassicalAlgorithm,
        enable_pcs: bool,
    ) {
        self.hybrid_crypto = HybridConfig {
            kdf,
            classical_algorithm: classical,
            enable_pcs,
        };
    }

    /// Validate configuration
    pub fn validate(&self) -> Result<()> {
        // Validate security levels
        if self.default_kem.security_level < 1 || self.default_kem.security_level > 5 {
            return Err(anyhow!("KEM security level must be between 1 and 5"));
        }

        if self.default_signature.security_level < 1 || self.default_signature.security_level > 5 {
            return Err(anyhow!("Signature security level must be between 1 and 5"));
        }

        // Validate algorithm security level compatibility
        self.validate_algorithm_security()?;

        Ok(())
    }

    fn validate_algorithm_security(&self) -> Result<()> {
        match self.default_kem.algorithm {
            KemAlgorithm::CrystalsKyber512 => {
                if self.default_kem.security_level < 1 {
                    return Err(anyhow!("CrystalsKyber512 requires security level >= 1"));
                }
            }
            KemAlgorithm::CrystalsKyber768 => {
                if self.default_kem.security_level < 3 {
                    return Err(anyhow!("CrystalsKyber768 requires security level >= 3"));
                }
            }
            KemAlgorithm::CrystalsKyber1024 => {
                if self.default_kem.security_level < 5 {
                    return Err(anyhow!("CrystalsKyber1024 requires security level >= 5"));
                }
            }
            _ => {} // Other algorithms have their own validation rules
        }

        match self.default_signature.algorithm {
            SigAlgorithm::CrystalsDilithium2 => {
                if self.default_signature.security_level < 2 {
                    return Err(anyhow!("CrystalsDilithium2 requires security level >= 2"));
                }
            }
            SigAlgorithm::CrystalsDilithium3 => {
                if self.default_signature.security_level < 3 {
                    return Err(anyhow!("CrystalsDilithium3 requires security level >= 3"));
                }
            }
            SigAlgorithm::CrystalsDilithium5 => {
                if self.default_signature.security_level < 5 {
                    return Err(anyhow!("CrystalsDilithium5 requires security level >= 5"));
                }
            }
            _ => {} // Other algorithms have their own validation rules
        }

        Ok(())
    }

    /// Get algorithm security information
    pub fn get_security_info(&self) -> SecurityInfo {
        SecurityInfo {
            kem_algorithm: self.default_kem.algorithm,
            kem_security_bits: self.get_kem_security_bits(self.default_kem.algorithm),
            signature_algorithm: self.default_signature.algorithm,
            signature_security_bits: self
                .get_signature_security_bits(self.default_signature.algorithm),
            hybrid_mode: self.default_kem.hybrid_mode || self.default_signature.hybrid_mode,
        }
    }

    fn get_kem_security_bits(&self, algorithm: KemAlgorithm) -> u32 {
        match algorithm {
            KemAlgorithm::CrystalsKyber512 => 128,
            KemAlgorithm::CrystalsKyber768 => 192,
            KemAlgorithm::CrystalsKyber1024 => 256,
            KemAlgorithm::ClassicMcEliece348864 => 256,
            KemAlgorithm::ClassicMcEliece460896 => 256,
            KemAlgorithm::NtruHps2048509 => 128,
            KemAlgorithm::NtruHps4096821 => 256,
            KemAlgorithm::SaberLightSaber => 128,
            KemAlgorithm::SaberSaber => 192,
            KemAlgorithm::SaberFireSaber => 256,
        }
    }

    fn get_signature_security_bits(&self, algorithm: SigAlgorithm) -> u32 {
        match algorithm {
            SigAlgorithm::CrystalsDilithium2 => 128,
            SigAlgorithm::CrystalsDilithium3 => 192,
            SigAlgorithm::CrystalsDilithium5 => 256,
            SigAlgorithm::Falcon512 => 128,
            SigAlgorithm::Falcon1024 => 256,
            SigAlgorithm::SphincsPlusSha256128f => 128,
            SigAlgorithm::SphincsPlusSha256128s => 128,
            SigAlgorithm::SphincsPlusShake256128f => 128,
            SigAlgorithm::SphincsPlusShake256128s => 128,
        }
    }
}

/// Security Information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityInfo {
    pub kem_algorithm: KemAlgorithm,
    pub kem_security_bits: u32,
    pub signature_algorithm: SigAlgorithm,
    pub signature_security_bits: u32,
    pub hybrid_mode: bool,
}

impl std::fmt::Display for SecurityInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "PQC Security Configuration:\n")?;
        write!(
            f,
            "  KEM Algorithm: {:?} ({}-bit security)\n",
            self.kem_algorithm, self.kem_security_bits
        )?;
        write!(
            f,
            "  Signature Algorithm: {:?} ({}-bit security)\n",
            self.signature_algorithm, self.signature_security_bits
        )?;
        write!(f, "  Hybrid Mode: {}\n", self.hybrid_mode)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config() {
        let config = PqcConfig::new();
        assert_eq!(config.default_kem.algorithm, KemAlgorithm::CrystalsKyber768);
        assert_eq!(
            config.default_signature.algorithm,
            SigAlgorithm::CrystalsDilithium3
        );
        assert!(config.default_kem.hybrid_mode);
    }

    #[test]
    fn test_config_validation() {
        let config = PqcConfig::new();
        assert!(config.validate().is_ok());
    }

    #[test]
    fn test_invalid_security_level() {
        let mut config = PqcConfig::new();
        config.default_kem.security_level = 6;
        assert!(config.validate().is_err());
    }

    #[test]
    fn test_security_info() {
        let config = PqcConfig::new();
        let info = config.get_security_info();
        assert_eq!(info.kem_security_bits, 192);
        assert_eq!(info.signature_security_bits, 192);
    }

    #[test]
    fn test_config_serialization() {
        let config = PqcConfig::new();
        let json = serde_json::to_string(&config).unwrap();
        let deserialized: PqcConfig = serde_json::from_str(&json).unwrap();
        assert_eq!(
            config.default_kem.algorithm,
            deserialized.default_kem.algorithm
        );
    }
}
