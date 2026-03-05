//! VPN Configuration Module

use anyhow::{Result, anyhow};
use serde::{Serialize, Deserialize};
use std::net::SocketAddr;

/// VPN Configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VpnConfig {
    /// Server address
    pub server_address: SocketAddr,
    /// VPN network CIDR
    pub network_cidr: String,
    /// PQC configuration
    pub pqc: VpnPqcConfig,
    /// Client configuration
    pub clients: Vec<ClientConfig>,
}

/// VPN PQC Configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VpnPqcConfig {
    pub enable_pqc: bool,
    pub kem_algorithm: VpnKemAlgorithm,
    pub signature_algorithm: VpnSignatureAlgorithm,
    pub hybrid_mode: bool,
    pub fallback_to_classical: bool,
    pub min_security_level: u8,
    pub rekey_interval_secs: u64,
    pub mtu: u16,
    pub enable_pfs: bool,
    pub compression: bool,
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
            rekey_interval_secs: 3600,
            mtu: 1500,
            enable_pfs: true,
            compression: true,
        }
    }
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

/// Client Configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClientConfig {
    pub client_id: String,
    pub public_key: Vec<u8>,
    pub allowed_ips: Vec<String>,
}

impl Default for VpnConfig {
    fn default() -> Self {
        Self {
            server_address: "0.0.0.0:1194".parse().unwrap(),
            network_cidr: "10.8.0.0/24".to_string(),
            pqc: VpnPqcConfig::default(),
            clients: Vec::new(),
        }
    }
}

impl VpnConfig {
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