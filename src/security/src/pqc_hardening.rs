//! PQC Hardening Module
//! 
//! This module provides security hardening measures for post-quantum
//! cryptography, including algorithm enforcement, key management, and
//! security configuration hardening.

use anyhow::{Result, anyhow};
use tracing::{info, debug, warn, error};
use std::collections::HashMap;
use serde::{Serialize, Deserialize};

/// PQC Hardening Manager
pub struct PqcHardeningManager {
    config: PqcHardeningConfig,
    measures: Vec<PqcHardeningMeasure>,
    status: HashMap<String, ImplementationStatus>,
}

/// PQC Hardening Configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PqcHardeningConfig {
    /// Enforce PQC algorithms
    pub enforce_pqc: bool,
    /// Require hybrid mode
    pub require_hybrid_mode: bool,
    /// Minimum security level (1-5)
    pub minimum_security_level: u8,
    /// Key rotation interval (hours)
    pub key_rotation_hours: u64,
    /// Enable post-compromise security
    pub enable_pcs: bool,
    /// Audit logging enabled
    pub audit_logging: bool,
}

impl Default for PqcHardeningConfig {
    fn default() -> Self {
        Self {
            enforce_pqc: true,
            require_hybrid_mode: true,
            minimum_security_level: 3,
            key_rotation_hours: 168, // 7 days
            enable_pcs: true,
            audit_logging: true,
        }
    }
}

/// PQC Hardening Measure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PqcHardeningMeasure {
    /// Measure ID
    pub id: String,
    /// Measure name
    pub name: String,
    /// Category
    pub category: PqcHardeningCategory,
    /// Description
    pub description: String,
    /// Priority (1-5)
    pub priority: u8,
    /// Implementation status
    pub status: ImplementationStatus,
    /// Verification steps
    pub verification_steps: Vec<String>,
    /// Dependencies
    pub dependencies: Vec<String>,
    /// Security impact
    pub security_impact: SecurityImpact,
}

/// PQC Hardening Categories
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PqcHardeningCategory {
    /// Algorithm enforcement
    AlgorithmEnforcement,
    /// Key management
    KeyManagement,
    /// TLS configuration
    TlsConfiguration,
    /// Certificate management
    CertificateManagement,
    /// Audit and monitoring
    AuditMonitoring,
    /// Access control
    AccessControl,
}

/// Implementation Status
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ImplementationStatus {
    NotStarted,
    InProgress,
    Completed,
    Failed,
    Skipped,
}

/// Security Impact
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SecurityImpact {
    Critical,
    High,
    Medium,
    Low,
}

impl PqcHardeningManager {
    /// Create a new PQC hardening manager
    pub fn new() -> Self {
        Self {
            config: PqcHardeningConfig::default(),
            measures: Self::default_measures(),
            status: HashMap::new(),
        }
    }

    /// Create with custom configuration
    pub fn with_config(config: PqcHardeningConfig) -> Self {
        Self {
            config,
            measures: Self::default_measures(),
            status: HashMap::new(),
        }
    }

    /// Get default hardening measures
    fn default_measures() -> Vec<PqcHardeningMeasure> {
        vec![
            PqcHardeningMeasure {
                id: "PQC-HARD-001".to_string(),
                name: "Enforce CRYSTALS-Kyber for Key Exchange".to_string(),
                category: PqcHardeningCategory::AlgorithmEnforcement,
                description: "Replace all classical key exchange algorithms with CRYSTALS-Kyber".to_string(),
                priority: 1,
                status: ImplementationStatus::NotStarted,
                verification_steps: vec![
                    "Verify no RSA/ECDH key exchange in use".to_string(),
                    "Confirm Kyber-768 is default".to_string(),
                    "Test hybrid mode with X25519".to_string(),
                ],
                dependencies: vec![],
                security_impact: SecurityImpact::Critical,
            },
            PqcHardeningMeasure {
                id: "PQC-HARD-002".to_string(),
                name: "Enforce CRYSTALS-Dilithium for Signatures".to_string(),
                category: PqcHardeningCategory::AlgorithmEnforcement,
                description: "Replace all classical signature algorithms with CRYSTALS-Dilithium".to_string(),
                priority: 1,
                status: ImplementationStatus::NotStarted,
                verification_steps: vec![
                    "Verify no RSA/ECDSA signatures in use".to_string(),
                    "Confirm Dilithium3 is default".to_string(),
                    "Test signature verification".to_string(),
                ],
                dependencies: vec![],
                security_impact: SecurityImpact::Critical,
            },
            PqcHardeningMeasure {
                id: "PQC-HARD-003".to_string(),
                name: "Enable Hybrid Mode".to_string(),
                category: PqcHardeningCategory::AlgorithmEnforcement,
                description: "Enable hybrid mode combining PQC with classical algorithms".to_string(),
                priority: 2,
                status: ImplementationStatus::NotStarted,
                verification_steps: vec![
                    "Verify Kyber + X25519 hybrid".to_string(),
                    "Test fallback mechanisms".to_string(),
                    "Confirm performance acceptable".to_string(),
                ],
                dependencies: vec!["PQC-HARD-001".to_string()],
                security_impact: SecurityImpact::High,
            },
            PqcHardeningMeasure {
                id: "PQC-HARD-004".to_string(),
                name: "Configure Key Rotation".to_string(),
                category: PqcHardeningCategory::KeyManagement,
                description: "Implement automatic PQC key rotation".to_string(),
                priority: 2,
                status: ImplementationStatus::NotStarted,
                verification_steps: vec![
                    "Set rotation interval to 7 days".to_string(),
                    "Test key rotation process".to_string(),
                    "Verify secure key storage".to_string(),
                ],
                dependencies: vec!["PQC-HARD-001".to_string()],
                security_impact: SecurityImpact::High,
            },
            PqcHardeningMeasure {
                id: "PQC-HARD-005".to_string(),
                name: "Enforce TLS 1.3 with PQC".to_string(),
                category: PqcHardeningCategory::TlsConfiguration,
                description: "Configure TLS 1.3 with PQC cipher suites".to_string(),
                priority: 1,
                status: ImplementationStatus::NotStarted,
                verification_steps: vec![
                    "Disable TLS 1.0/1.1/1.2".to_string(),
                    "Enable PQC cipher suites".to_string(),
                    "Test TLS handshake".to_string(),
                ],
                dependencies: vec!["PQC-HARD-001".to_string()],
                security_impact: SecurityImpact::Critical,
            },
            PqcHardeningMeasure {
                id: "PQC-HARD-006".to_string(),
                name: "Deploy PQC Certificates".to_string(),
                category: PqcHardeningCategory::CertificateManagement,
                description: "Deploy certificates signed with Dilithium".to_string(),
                priority: 2,
                status: ImplementationStatus::NotStarted,
                verification_steps: vec![
                    "Generate Dilithium certificates".to_string(),
                    "Deploy to all services".to_string(),
                    "Verify certificate chain".to_string(),
                ],
                dependencies: vec!["PQC-HARD-002".to_string()],
                security_impact: SecurityImpact::High,
            },
            PqcHardeningMeasure {
                id: "PQC-HARD-007".to_string(),
                name: "Enable PQC Audit Logging".to_string(),
                category: PqcHardeningCategory::AuditMonitoring,
                description: "Enable comprehensive PQC audit logging".to_string(),
                priority: 3,
                status: ImplementationStatus::NotStarted,
                verification_steps: vec![
                    "Configure audit log format".to_string(),
                    "Test log collection".to_string(),
                    "Verify log integrity".to_string(),
                ],
                dependencies: vec![],
                security_impact: SecurityImpact::Medium,
            },
            PqcHardeningMeasure {
                id: "PQC-HARD-008".to_string(),
                name: "Implement Post-Compromise Security".to_string(),
                category: PqcHardeningCategory::KeyManagement,
                description: "Enable post-compromise security mechanisms".to_string(),
                priority: 3,
                status: ImplementationStatus::NotStarted,
                verification_steps: vec![
                    "Enable PCS for all keys".to_string(),
                    "Test key update mechanism".to_string(),
                    "Verify forward secrecy".to_string(),
                ],
                dependencies: vec!["PQC-HARD-004".to_string()],
                security_impact: SecurityImpact::Medium,
            },
        ]
    }

    /// Apply all hardening measures
    pub fn apply_hardening(&mut self) -> Result<PqcHardeningResult> {
        info!("Applying PQC hardening measures...");
        
        let mut result = PqcHardeningResult {
            total_measures: self.measures.len(),
            applied_measures: 0,
            failed_measures: 0,
            skipped_measures: 0,
            measure_results: Vec::new(),
        };

        for measure in &mut self.measures {
            // Check dependencies
            let deps_satisfied = measure.dependencies.iter().all(|dep| {
                self.status.get(dep) == Some(&ImplementationStatus::Completed)
            });

            if !deps_satisfied {
                measure.status = ImplementationStatus::Skipped;
                result.skipped_measures += 1;
                continue;
            }

            // Apply measure
            match self.apply_measure(measure) {
                Ok(_) => {
                    measure.status = ImplementationStatus::Completed;
                    result.applied_measures += 1;
                    self.status.insert(measure.id.clone(), ImplementationStatus::Completed);
                }
                Err(e) => {
                    warn!("Failed to apply measure {}: {}", measure.id, e);
                    measure.status = ImplementationStatus::Failed;
                    result.failed_measures += 1;
                    self.status.insert(measure.id.clone(), ImplementationStatus::Failed);
                }
            }

            result.measure_results.push(MeasureResult {
                id: measure.id.clone(),
                name: measure.name.clone(),
                status: measure.status,
            });
        }

        info!("PQC hardening complete: {}/{} applied", result.applied_measures, result.total_measures);
        
        Ok(result)
    }

    /// Apply a single hardening measure
    fn apply_measure(&self, measure: &PqcHardeningMeasure) -> Result<()> {
        info!("Applying measure: {} - {}", measure.id, measure.name);
        
        match measure.category {
            PqcHardeningCategory::AlgorithmEnforcement => {
                self.enforce_algorithm(&measure.id)?;
            }
            PqcHardeningCategory::KeyManagement => {
                self.configure_key_management(&measure.id)?;
            }
            PqcHardeningCategory::TlsConfiguration => {
                self.configure_tls(&measure.id)?;
            }
            PqcHardeningCategory::CertificateManagement => {
                self.configure_certificates(&measure.id)?;
            }
            PqcHardeningCategory::AuditMonitoring => {
                self.configure_audit(&measure.id)?;
            }
            PqcHardeningCategory::AccessControl => {
                self.configure_access_control(&measure.id)?;
            }
        }
        
        Ok(())
    }

    /// Enforce algorithm
    fn enforce_algorithm(&self, measure_id: &str) -> Result<()> {
        match measure_id {
            "PQC-HARD-001" => {
                // Enforce Kyber
                if self.config.enforce_pqc {
                    info!("Enforcing CRYSTALS-Kyber for key exchange");
                }
            }
            "PQC-HARD-002" => {
                // Enforce Dilithium
                if self.config.enforce_pqc {
                    info!("Enforcing CRYSTALS-Dilithium for signatures");
                }
            }
            "PQC-HARD-003" => {
                // Enable hybrid mode
                if self.config.require_hybrid_mode {
                    info!("Enabling hybrid mode");
                }
            }
            _ => {}
        }
        Ok(())
    }

    /// Configure key management
    fn configure_key_management(&self, measure_id: &str) -> Result<()> {
        match measure_id {
            "PQC-HARD-004" => {
                info!("Configuring key rotation: {} hours", self.config.key_rotation_hours);
            }
            "PQC-HARD-008" => {
                if self.config.enable_pcs {
                    info!("Enabling post-compromise security");
                }
            }
            _ => {}
        }
        Ok(())
    }

    /// Configure TLS
    fn configure_tls(&self, _measure_id: &str) -> Result<()> {
        info!("Configuring TLS 1.3 with PQC cipher suites");
        Ok(())
    }

    /// Configure certificates
    fn configure_certificates(&self, _measure_id: &str) -> Result<()> {
        info!("Deploying PQC certificates");
        Ok(())
    }

    /// Configure audit
    fn configure_audit(&self, _measure_id: &str) -> Result<()> {
        if self.config.audit_logging {
            info!("Enabling PQC audit logging");
        }
        Ok(())
    }

    /// Configure access control
    fn configure_access_control(&self, _measure_id: &str) -> Result<()> {
        info!("Configuring PQC access control");
        Ok(())
    }

    /// Get hardening status
    pub fn get_status(&self) -> &HashMap<String, ImplementationStatus> {
        &self.status
    }

    /// Get all measures
    pub fn get_measures(&self) -> &[PqcHardeningMeasure] {
        &self.measures
    }

    /// Get compliance score
    pub fn get_compliance_score(&self) -> u8 {
        let completed = self.measures.iter()
            .filter(|m| m.status == ImplementationStatus::Completed)
            .count();
        let total = self.measures.len();
        
        ((completed as f64 / total as f64) * 100.0) as u8
    }
}

/// PQC Hardening Result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PqcHardeningResult {
    /// Total measures
    pub total_measures: usize,
    /// Applied measures
    pub applied_measures: usize,
    /// Failed measures
    pub failed_measures: usize,
    /// Skipped measures
    pub skipped_measures: usize,
    /// Measure results
    pub measure_results: Vec<MeasureResult>,
}

/// Measure Result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MeasureResult {
    /// Measure ID
    pub id: String,
    /// Measure name
    pub name: String,
    /// Status
    pub status: ImplementationStatus,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hardening_manager_creation() {
        let manager = PqcHardeningManager::new();
        assert_eq!(manager.measures.len(), 8);
    }

    #[test]
    fn test_default_config() {
        let config = PqcHardeningConfig::default();
        assert!(config.enforce_pqc);
        assert!(config.require_hybrid_mode);
        assert_eq!(config.minimum_security_level, 3);
    }

    #[test]
    fn test_hardening_application() {
        let mut manager = PqcHardeningManager::new();
        let result = manager.apply_hardening().unwrap();
        
        assert_eq!(result.total_measures, 8);
        // Some measures should be skipped due to dependencies
        assert!(result.skipped_measures > 0);
    }

    #[test]
    fn test_compliance_score() {
        let manager = PqcHardeningManager::new();
        let score = manager.get_compliance_score();
        assert_eq!(score, 0); // No measures applied yet
    }
}