//! PQC Configuration Validator
//! 
//! This module provides validation for Post-Quantum Cryptography configuration
//! to ensure proper algorithm selection, security levels, and compatibility.

use anyhow::{Result, anyhow};
use super::{Config, ConfigValidator};

/// Valid PQC KEM algorithms
pub const VALID_KEM_ALGORITHMS: &[&str] = &[
    "CrystalsKyber512",
    "CrystalsKyber768", 
    "CrystalsKyber1024",
    "kyber512",
    "kyber768",
    "kyber1024",
    "crystals_kyber512",
    "crystals_kyber768",
    "crystals_kyber1024",
];

/// Valid PQC signature algorithms
pub const VALID_SIGNATURE_ALGORITHMS: &[&str] = &[
    "CrystalsDilithium2",
    "CrystalsDilithium3",
    "CrystalsDilithium5",
    "Falcon512",
    "Falcon1024",
    "SphincsPlusSha2128f",
    "SphincsPlusSha2192f",
    "SphincsPlusSha2256f",
    "dilithium2",
    "dilithium3",
    "dilithium5",
    "falcon512",
    "falcon1024",
    "crystals_dilithium2",
    "crystals_dilithium3",
    "crystals_dilithium5",
];

/// NIST security levels mapping for KEM algorithms
pub const KEM_SECURITY_LEVELS: &[(&str, u8)] = &[
    ("CrystalsKyber512", 1),
    ("kyber512", 1),
    ("crystals_kyber512", 1),
    ("CrystalsKyber768", 3),
    ("kyber768", 3),
    ("crystals_kyber768", 3),
    ("CrystalsKyber1024", 5),
    ("kyber1024", 5),
    ("crystals_kyber1024", 5),
];

/// NIST security levels mapping for signature algorithms
pub const SIGNATURE_SECURITY_LEVELS: &[(&str, u8)] = &[
    ("CrystalsDilithium2", 1),
    ("dilithium2", 1),
    ("crystals_dilithium2", 1),
    ("Falcon512", 1),
    ("falcon512", 1),
    ("SphincsPlusSha2128f", 1),
    ("CrystalsDilithium3", 3),
    ("dilithium3", 3),
    ("crystals_dilithium3", 3),
    ("SphincsPlusSha2192f", 3),
    ("CrystalsDilithium5", 5),
    ("dilithium5", 5),
    ("crystals_dilithium5", 5),
    ("Falcon1024", 5),
    ("falcon1024", 5),
    ("SphincsPlusSha2256f", 5),
];

/// PQC Configuration Validator
pub struct PqcConfigValidator {
    /// Minimum required security level
    min_security_level: u8,
    /// Require hybrid mode
    require_hybrid_mode: bool,
    /// Allowed KEM algorithms (empty = all valid)
    allowed_kem_algorithms: Vec<String>,
    /// Allowed signature algorithms (empty = all valid)
    allowed_signature_algorithms: Vec<String>,
}

impl PqcConfigValidator {
    /// Create a new PQC configuration validator with default settings
    pub fn new() -> Self {
        Self {
            min_security_level: 1,
            require_hybrid_mode: false,
            allowed_kem_algorithms: Vec::new(),
            allowed_signature_algorithms: Vec::new(),
        }
    }
    
    /// Create a validator with production-ready settings
    pub fn production() -> Self {
        Self {
            min_security_level: 3,
            require_hybrid_mode: true,
            allowed_kem_algorithms: vec![
                "CrystalsKyber768".to_string(),
                "CrystalsKyber1024".to_string(),
            ],
            allowed_signature_algorithms: vec![
                "CrystalsDilithium3".to_string(),
                "CrystalsDilithium5".to_string(),
                "Falcon1024".to_string(),
            ],
        }
    }
    
    /// Create a validator for CNSA 2.0 compliance
    pub fn cnsa_2_0() -> Self {
        Self {
            min_security_level: 3,
            require_hybrid_mode: true,
            allowed_kem_algorithms: vec![
                "CrystalsKyber768".to_string(),
                "CrystalsKyber1024".to_string(),
            ],
            allowed_signature_algorithms: vec![
                "CrystalsDilithium3".to_string(),
                "CrystalsDilithium5".to_string(),
            ],
        }
    }
    
    /// Set minimum security level
    pub fn with_min_security_level(mut self, level: u8) -> Self {
        self.min_security_level = level.clamp(1, 5);
        self
    }
    
    /// Set hybrid mode requirement
    pub fn with_require_hybrid_mode(mut self, require: bool) -> Self {
        self.require_hybrid_mode = require;
        self
    }
    
    /// Validate KEM algorithm name
    pub fn validate_kem_algorithm(&self, algorithm: &str) -> Result<u8> {
        // Check if algorithm is in valid list
        let is_valid = VALID_KEM_ALGORITHMS.iter()
            .any(|&valid| valid.eq_ignore_ascii_case(algorithm));
        
        if !is_valid {
            return Err(anyhow!(
                "Invalid PQC KEM algorithm: '{}'. Valid algorithms: {}",
                algorithm,
                VALID_KEM_ALGORITHMS.join(", ")
            ));
        }
        
        // Check if algorithm is in allowed list (if specified)
        if !self.allowed_kem_algorithms.is_empty() {
            let is_allowed = self.allowed_kem_algorithms.iter()
                .any(|allowed| allowed.eq_ignore_ascii_case(algorithm));
            
            if !is_allowed {
                return Err(anyhow!(
                    "KEM algorithm '{}' is not in the allowed list: {}",
                    algorithm,
                    self.allowed_kem_algorithms.join(", ")
                ));
            }
        }
        
        // Get security level
        let security_level = KEM_SECURITY_LEVELS.iter()
            .find(|(alg, _)| alg.eq_ignore_ascii_case(algorithm))
            .map(|&(_, level)| level)
            .unwrap_or(1);
        
        // Check minimum security level
        if security_level < self.min_security_level {
            return Err(anyhow!(
                "KEM algorithm '{}' has security level {}, which is below minimum required {}",
                algorithm,
                security_level,
                self.min_security_level
            ));
        }
        
        Ok(security_level)
    }
    
    /// Validate signature algorithm name
    pub fn validate_signature_algorithm(&self, algorithm: &str) -> Result<u8> {
        // Check if algorithm is in valid list
        let is_valid = VALID_SIGNATURE_ALGORITHMS.iter()
            .any(|&valid| valid.eq_ignore_ascii_case(algorithm));
        
        if !is_valid {
            return Err(anyhow!(
                "Invalid PQC signature algorithm: '{}'. Valid algorithms: {}",
                algorithm,
                VALID_SIGNATURE_ALGORITHMS.join(", ")
            ));
        }
        
        // Check if algorithm is in allowed list (if specified)
        if !self.allowed_signature_algorithms.is_empty() {
            let is_allowed = self.allowed_signature_algorithms.iter()
                .any(|allowed| allowed.eq_ignore_ascii_case(algorithm));
            
            if !is_allowed {
                return Err(anyhow!(
                    "Signature algorithm '{}' is not in the allowed list: {}",
                    algorithm,
                    self.allowed_signature_algorithms.join(", ")
                ));
            }
        }
        
        // Get security level
        let security_level = SIGNATURE_SECURITY_LEVELS.iter()
            .find(|(alg, _)| alg.eq_ignore_ascii_case(algorithm))
            .map(|&(_, level)| level)
            .unwrap_or(1);
        
        // Check minimum security level
        if security_level < self.min_security_level {
            return Err(anyhow!(
                "Signature algorithm '{}' has security level {}, which is below minimum required {}",
                algorithm,
                security_level,
                self.min_security_level
            ));
        }
        
        Ok(security_level)
    }
    
    /// Validate complete PQC configuration
    pub fn validate_pqc_config(
        &self,
        enable_pqc: bool,
        kem_algorithm: Option<&str>,
        signature_algorithm: Option<&str>,
        hybrid_mode: bool,
        fallback_to_classical: bool,
        min_security_level: u8,
    ) -> Result<PqcValidationResult> {
        let mut warnings = Vec::new();
        let mut errors = Vec::new();
        
        if !enable_pqc {
            return Ok(PqcValidationResult {
                is_valid: true,
                kem_security_level: None,
                signature_security_level: None,
                warnings: vec!["PQC is disabled. Consider enabling for quantum-resistant security.".to_string()],
                errors: Vec::new(),
            });
        }
        
        // Validate KEM algorithm
        let kem_level = if let Some(kem) = kem_algorithm {
            match self.validate_kem_algorithm(kem) {
                Ok(level) => Some(level),
                Err(e) => {
                    errors.push(e.to_string());
                    None
                }
            }
        } else {
            errors.push("PQC is enabled but no KEM algorithm is specified".to_string());
            None
        };
        
        // Validate signature algorithm
        let sig_level = if let Some(sig) = signature_algorithm {
            match self.validate_signature_algorithm(sig) {
                Ok(level) => Some(level),
                Err(e) => {
                    errors.push(e.to_string());
                    None
                }
            }
        } else {
            errors.push("PQC is enabled but no signature algorithm is specified".to_string());
            None
        };
        
        // Check hybrid mode
        if self.require_hybrid_mode && !hybrid_mode {
            errors.push("Hybrid mode is required but not enabled".to_string());
        }
        
        if !hybrid_mode && !fallback_to_classical {
            warnings.push(
                "Hybrid mode is disabled and fallback is disabled. \
                 This may cause compatibility issues with non-PQC clients.".to_string()
            );
        }
        
        // Check security level consistency
        if let (Some(kem), Some(sig)) = (kem_level, sig_level) {
            if kem != sig {
                warnings.push(format!(
                    "KEM algorithm security level ({}) differs from signature algorithm level ({}). \
                     Consider using matching security levels for consistent protection.",
                    kem, sig
                ));
            }
            
            let effective_min = min_security_level.max(self.min_security_level);
            if kem < effective_min || sig < effective_min {
                errors.push(format!(
                    "Security level is below minimum required ({}). KEM: {}, Signature: {}",
                    effective_min, kem, sig
                ));
            }
        }
        
        Ok(PqcValidationResult {
            is_valid: errors.is_empty(),
            kem_security_level: kem_level,
            signature_security_level: sig_level,
            warnings,
            errors,
        })
    }
}

impl Default for PqcConfigValidator {
    fn default() -> Self {
        Self::new()
    }
}

impl ConfigValidator for PqcConfigValidator {
    fn validate(&self, config: &Config) -> Result<()> {
        let result = self.validate_pqc_config(
            config.network.enable_pqc,
            config.network.pqc_kem_algorithm.as_deref(),
            config.network.pqc_signature_algorithm.as_deref(),
            config.network.pqc_hybrid_mode,
            config.network.pqc_fallback_to_classical,
            config.network.pqc_min_security_level,
        )?;
        
        if !result.is_valid {
            return Err(anyhow!(
                "PQC configuration validation failed: {}",
                result.errors.join("; ")
            ));
        }
        
        // Log warnings but don't fail
        for warning in &result.warnings {
            tracing::warn!("PQC configuration warning: {}", warning);
        }
        
        Ok(())
    }
    
    fn name(&self) -> &str {
        "pqc_config_validator"
    }
}

/// Result of PQC configuration validation
#[derive(Debug, Clone)]
pub struct PqcValidationResult {
    /// Whether the configuration is valid
    pub is_valid: bool,
    /// Security level of KEM algorithm
    pub kem_security_level: Option<u8>,
    /// Security level of signature algorithm
    pub signature_security_level: Option<u8>,
    /// Validation warnings
    pub warnings: Vec<String>,
    /// Validation errors
    pub errors: Vec<String>,
}

impl PqcValidationResult {
    /// Check if there are any warnings
    pub fn has_warnings(&self) -> bool {
        !self.warnings.is_empty()
    }
    
    /// Check if there are any errors
    pub fn has_errors(&self) -> bool {
        !self.errors.is_empty()
    }
    
    /// Get overall security level (minimum of KEM and signature)
    pub fn overall_security_level(&self) -> Option<u8> {
        match (self.kem_security_level, self.signature_security_level) {
            (Some(kem), Some(sig)) => Some(kem.min(sig)),
            (Some(kem), None) => Some(kem),
            (None, Some(sig)) => Some(sig),
            (None, None) => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_validate_kem_algorithm() {
        let validator = PqcConfigValidator::new();
        
        assert!(validator.validate_kem_algorithm("CrystalsKyber768").is_ok());
        assert!(validator.validate_kem_algorithm("kyber768").is_ok());
        assert!(validator.validate_kem_algorithm("invalid").is_err());
    }
    
    #[test]
    fn test_validate_signature_algorithm() {
        let validator = PqcConfigValidator::new();
        
        assert!(validator.validate_signature_algorithm("CrystalsDilithium3").is_ok());
        assert!(validator.validate_signature_algorithm("dilithium3").is_ok());
        assert!(validator.validate_signature_algorithm("invalid").is_err());
    }
    
    #[test]
    fn test_security_levels() {
        let validator = PqcConfigValidator::new();
        
        assert_eq!(validator.validate_kem_algorithm("CrystalsKyber512").unwrap(), 1);
        assert_eq!(validator.validate_kem_algorithm("CrystalsKyber768").unwrap(), 3);
        assert_eq!(validator.validate_kem_algorithm("CrystalsKyber1024").unwrap(), 5);
    }
    
    #[test]
    fn test_min_security_level() {
        let validator = PqcConfigValidator::new()
            .with_min_security_level(3);
        
        // Kyber512 is level 1, should fail
        assert!(validator.validate_kem_algorithm("CrystalsKyber512").is_err());
        // Kyber768 is level 3, should pass
        assert!(validator.validate_kem_algorithm("CrystalsKyber768").is_ok());
    }
    
    #[test]
    fn test_production_validator() {
        let validator = PqcConfigValidator::production();
        
        // Should require hybrid mode
        assert!(validator.require_hybrid_mode);
        // Should require security level 3+
        assert!(validator.validate_kem_algorithm("CrystalsKyber512").is_err());
        assert!(validator.validate_kem_algorithm("CrystalsKyber768").is_ok());
    }
    
    #[test]
    fn test_cnsa_2_0_validator() {
        let validator = PqcConfigValidator::cnsa_2_0();
        
        // Should allow only Kyber768/1024 and Dilithium3/5
        assert!(validator.validate_kem_algorithm("CrystalsKyber768").is_ok());
        assert!(validator.validate_kem_algorithm("CrystalsKyber1024").is_ok());
        assert!(validator.validate_kem_algorithm("CrystalsKyber512").is_err());
        
        assert!(validator.validate_signature_algorithm("CrystalsDilithium3").is_ok());
        assert!(validator.validate_signature_algorithm("CrystalsDilithium5").is_ok());
        assert!(validator.validate_signature_algorithm("CrystalsDilithium2").is_err());
    }
}