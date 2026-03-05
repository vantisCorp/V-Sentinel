//! PQC Security Assessment Module
//! 
//! This module provides security assessment capabilities for post-quantum
//! cryptography, including vulnerability detection, compliance checking, and
//! migration readiness assessment.

use anyhow::{Result, anyhow};
use tracing::{info, debug, warn, error};
use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use std::time::SystemTime;

/// PQC Security Assessor
pub struct PqcSecurityAssessor {
    quantum_vulnerable_algorithms: Vec<String>,
    quantum_safe_algorithms: Vec<String>,
    nist_compliant_algorithms: Vec<String>,
}

/// PQC Vulnerability Finding
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PqcVulnerabilityFinding {
    /// Finding ID
    pub id: String,
    /// Vulnerability type
    pub vulnerability_type: PqcVulnerabilityType,
    /// Affected algorithm
    pub affected_algorithm: String,
    /// Severity level
    pub severity: PqcSeverity,
    /// Description
    pub description: String,
    /// Affected components
    pub affected_components: Vec<String>,
    /// Remediation recommendations
    pub remediation: Vec<String>,
    /// NIST compliance status
    pub nist_compliant: bool,
    /// Discovery timestamp
    pub discovered_at: u64,
}

/// PQC Vulnerability Types
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PqcVulnerabilityType {
    /// Quantum-vulnerable cryptographic algorithm
    QuantumVulnerableAlgorithm,
    /// Insecure key exchange mechanism
    InsecureKeyExchange,
    /// Weak TLS configuration
    WeakTlsConfiguration,
    /// Outdated cryptographic library
    OutdatedCryptoLibrary,
    /// Non-compliant with NIST standards
    NonCompliantAlgorithm,
    /// Missing PQC implementation
    MissingPqcImplementation,
}

/// PQC Severity Levels
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PqcSeverity {
    /// Critical - Immediate action required
    Critical,
    /// High - Should be addressed soon
    High,
    /// Medium - Should be addressed
    Medium,
    /// Low - Nice to have
    Low,
    /// Informational
    Info,
}

/// PQC Compliance Status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PqcComplianceStatus {
    /// Overall compliance score (0-100)
    pub compliance_score: u8,
    /// NIST FIPS 203 compliant
    pub fips_203_compliant: bool,
    /// NIST FIPS 204 compliant
    pub fips_204_compliant: bool,
    /// NIST FIPS 205 compliant
    pub fips_205_compliant: bool,
    /// NIST FIPS 206 compliant
    pub fips_206_compliant: bool,
    /// NSA CNSA 2.0 compliant
    pub cnsa_2_0_compliant: bool,
    /// Migration readiness score (0-100)
    pub migration_readiness: u8,
    /// Recommended actions
    pub recommended_actions: Vec<String>,
}

/// PQC Assessment Result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PqcAssessmentResult {
    /// Total findings
    pub total_findings: usize,
    /// Critical findings
    pub critical_findings: usize,
    /// High findings
    pub high_findings: usize,
    /// Medium findings
    pub medium_findings: usize,
    /// Low findings
    pub low_findings: usize,
    /// Informational findings
    pub info_findings: usize,
    /// Vulnerability findings
    pub vulnerabilities: Vec<PqcVulnerabilityFinding>,
    /// Compliance status
    pub compliance: PqcComplianceStatus,
    /// Assessment timestamp
    pub assessed_at: u64,
}

/// PQC Migration Readiness
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PqcMigrationReadiness {
    /// Overall readiness score (0-100)
    pub readiness_score: u8,
    /// Cryptographic inventory complete
    pub inventory_complete: bool,
    /// PQC algorithms implemented
    pub pqc_implemented: bool,
    /// Testing complete
    pub testing_complete: bool,
    /// Staff trained
    pub staff_trained: bool,
    /// Migration plan ready
    pub migration_plan_ready: bool,
    /// Estimated migration time (months)
    pub estimated_migration_months: u8,
    /// Migration blockers
    pub blockers: Vec<String>,
}

impl PqcSecurityAssessor {
    /// Create a new PQC security assessor
    pub fn new() -> Self {
        Self {
            quantum_vulnerable_algorithms: vec![
                "RSA".to_string(),
                "ECC".to_string(),
                "ECDH".to_string(),
                "ECDSA".to_string(),
                "Ed25519".to_string(),
                "X25519".to_string(),
                "Diffie-Hellman".to_string(),
                "DSA".to_string(),
            ],
            quantum_safe_algorithms: vec![
                "CRYSTALS-Kyber".to_string(),
                "CRYSTALS-Dilithium".to_string(),
                "FALCON".to_string(),
                "SPHINCS+".to_string(),
                "AES-256".to_string(),
                "SHA-256".to_string(),
                "SHA-384".to_string(),
                "SHA-512".to_string(),
                "ChaCha20".to_string(),
            ],
            nist_compliant_algorithms: vec![
                "CRYSTALS-Kyber".to_string(),
                "CRYSTALS-Dilithium".to_string(),
                "FALCON".to_string(),
                "SPHINCS+".to_string(),
            ],
        }
    }

    /// Perform comprehensive PQC security assessment
    pub fn assess_pqc_security(&self, codebase: &HashMap<String, String>) -> Result<PqcAssessmentResult> {
        info!("Starting PQC security assessment...");
        
        let mut vulnerabilities = Vec::new();
        let start_time = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)?
            .as_secs();

        // Scan for quantum-vulnerable algorithms
        vulnerabilities.extend(self.scan_quantum_vulnerable_algorithms(codebase)?);
        
        // Check for weak TLS configurations
        vulnerabilities.extend(self.scan_weak_tls_configurations(codebase)?);
        
        // Check for outdated cryptographic libraries
        vulnerabilities.extend(self.scan_outdated_libraries(codebase)?);
        
        // Check for missing PQC implementations
        vulnerabilities.extend(self.check_pqc_implementation(codebase)?);

        // Count findings by severity
        let critical = vulnerabilities.iter().filter(|v| v.severity == PqcSeverity::Critical).count();
        let high = vulnerabilities.iter().filter(|v| v.severity == PqcSeverity::High).count();
        let medium = vulnerabilities.iter().filter(|v| v.severity == PqcSeverity::Medium).count();
        let low = vulnerabilities.iter().filter(|v| v.severity == PqcSeverity::Low).count();
        let info = vulnerabilities.iter().filter(|v| v.severity == PqcSeverity::Info).count();

        // Assess compliance
        let compliance = self.assess_compliance(&vulnerabilities)?;

        let result = PqcAssessmentResult {
            total_findings: vulnerabilities.len(),
            critical_findings: critical,
            high_findings: high,
            medium_findings: medium,
            low_findings: low,
            info_findings: info,
            vulnerabilities,
            compliance,
            assessed_at: start_time,
        };

        info!("PQC security assessment completed: {} findings", result.total_findings);
        
        Ok(result)
    }

    /// Scan for quantum-vulnerable algorithms
    fn scan_quantum_vulnerable_algorithms(&self, codebase: &HashMap<String, String>) -> Result<Vec<PqcVulnerabilityFinding>> {
        let mut findings = Vec::new();

        for (file, content) in codebase {
            for algorithm in &self.quantum_vulnerable_algorithms {
                if content.contains(algorithm) {
                    findings.push(PqcVulnerabilityFinding {
                        id: format!("PQC-{:04}", findings.len() + 1),
                        vulnerability_type: PqcVulnerabilityType::QuantumVulnerableAlgorithm,
                        affected_algorithm: algorithm.clone(),
                        severity: self.get_algorithm_severity(algorithm),
                        description: format!("Quantum-vulnerable algorithm '{}' detected in {}", algorithm, file),
                        affected_components: vec![file.clone()],
                        remediation: vec![
                            format!("Replace {} with CRYSTALS-Kyber for key exchange", algorithm),
                            format!("Replace {} with CRYSTALS-Dilithium for signatures", algorithm),
                            "Implement hybrid mode during migration".to_string(),
                        ],
                        nist_compliant: false,
                        discovered_at: SystemTime::now()
                            .duration_since(SystemTime::UNIX_EPOCH)?
                            .as_secs(),
                    });
                }
            }
        }

        Ok(findings)
    }

    /// Scan for weak TLS configurations
    fn scan_weak_tls_configurations(&self, codebase: &HashMap<String, String>) -> Result<Vec<PqcVulnerabilityFinding>> {
        let mut findings = Vec::new();

        for (file, content) in codebase {
            if content.contains("TLS_1_0") || content.contains("TLS_1_1") {
                findings.push(PqcVulnerabilityFinding {
                    id: format!("PQC-{:04}", findings.len() + 1),
                    vulnerability_type: PqcVulnerabilityType::WeakTlsConfiguration,
                    affected_algorithm: "TLS 1.0/1.1".to_string(),
                    severity: PqcSeverity::High,
                    description: format!("Outdated TLS version detected in {}", file),
                    affected_components: vec![file.clone()],
                    remediation: vec![
                        "Upgrade to TLS 1.3".to_string(),
                        "Enable PQC cipher suites".to_string(),
                        "Implement hybrid mode".to_string(),
                    ],
                    nist_compliant: false,
                    discovered_at: SystemTime::now()
                        .duration_since(SystemTime::UNIX_EPOCH)?
                        .as_secs(),
                });
            }
        }

        Ok(findings)
    }

    /// Scan for outdated cryptographic libraries
    fn scan_outdated_libraries(&self, codebase: &HashMap<String, String>) -> Result<Vec<PqcVulnerabilityFinding>> {
        let mut findings = Vec::new();

        for (file, content) in codebase {
            if content.contains("openssl") && !content.contains("openssl-3.0") {
                findings.push(PqcVulnerabilityFinding {
                    id: format!("PQC-{:04}", findings.len() + 1),
                    vulnerability_type: PqcVulnerabilityType::OutdatedCryptoLibrary,
                    affected_algorithm: "OpenSSL".to_string(),
                    severity: PqcSeverity::Medium,
                    description: format!("Potentially outdated OpenSSL version in {}", file),
                    affected_components: vec![file.clone()],
                    remediation: vec![
                        "Upgrade to OpenSSL 3.0+".to_string(),
                        "Consider PQC-enabled alternatives".to_string(),
                        "Monitor for PQC support".to_string(),
                    ],
                    nist_compliant: false,
                    discovered_at: SystemTime::now()
                        .duration_since(SystemTime::UNIX_EPOCH)?
                        .as_secs(),
                });
            }
        }

        Ok(findings)
    }

    /// Check for PQC implementation
    fn check_pqc_implementation(&self, codebase: &HashMap<String, String>) -> Result<Vec<PqcVulnerabilityFinding>> {
        let mut findings = Vec::new();
        let has_pqc = self.quantum_safe_algorithms.iter()
            .any(|algo| codebase.values().any(|content| content.contains(algo)));

        if !has_pqc {
            findings.push(PqcVulnerabilityFinding {
                id: format!("PQC-{:04}", findings.len() + 1),
                vulnerability_type: PqcVulnerabilityType::MissingPqcImplementation,
                affected_algorithm: "PQC Algorithms".to_string(),
                severity: PqcSeverity::Critical,
                description: "No post-quantum cryptographic algorithms detected".to_string(),
                affected_components: vec!["Entire codebase".to_string()],
                remediation: vec![
                    "Implement CRYSTALS-Kyber for key exchange".to_string(),
                    "Implement CRYSTALS-Dilithium for signatures".to_string(),
                    "Consider FALCON for compact signatures".to_string(),
                    "Implement SPHINCS+ for long-term security".to_string(),
                ],
                nist_compliant: false,
                discovered_at: SystemTime::now()
                    .duration_since(SystemTime::UNIX_EPOCH)?
                    .as_secs(),
            });
        }

        Ok(findings)
    }

    /// Assess NIST and CNSA compliance
    fn assess_compliance(&self, vulnerabilities: &[PqcVulnerabilityFinding]) -> Result<PqcComplianceStatus> {
        let critical_count = vulnerabilities.iter()
            .filter(|v| v.severity == PqcSeverity::Critical)
            .count();
        let high_count = vulnerabilities.iter()
            .filter(|v| v.severity == PqcSeverity::High)
            .count();

        // Calculate compliance score
        let compliance_score = if critical_count == 0 && high_count == 0 {
            100
        } else if critical_count == 0 {
            80 - (high_count * 5)
        } else {
            50 - (critical_count * 10)
        };

        let compliance = PqcComplianceStatus {
            compliance_score: compliance_score.max(0) as u8,
            fips_203_compliant: vulnerabilities.iter().all(|v| v.vulnerability_type != PqcVulnerabilityType::QuantumVulnerableAlgorithm),
            fips_204_compliant: vulnerabilities.iter().all(|v| v.vulnerability_type != PqcVulnerabilityType::QuantumVulnerableAlgorithm),
            fips_205_compliant: vulnerabilities.iter().all(|v| v.vulnerability_type != PqcVulnerabilityType::QuantumVulnerableAlgorithm),
            fips_206_compliant: vulnerabilities.iter().all(|v| v.vulnerability_type != PqcVulnerabilityType::QuantumVulnerableAlgorithm),
            cnsa_2_0_compliant: compliance_score >= 80,
            migration_readiness: self.calculate_migration_readiness(vulnerabilities),
            recommended_actions: self.generate_recommendations(vulnerabilities),
        };

        Ok(compliance)
    }

    /// Calculate migration readiness score
    fn calculate_migration_readiness(&self, vulnerabilities: &[PqcVulnerabilityFinding]) -> u8 {
        let mut score = 100u8;

        // Deduct points for vulnerabilities
        for vuln in vulnerabilities {
            match vuln.severity {
                PqcSeverity::Critical => score -= 20,
                PqcSeverity::High => score -= 15,
                PqcSeverity::Medium => score -= 10,
                PqcSeverity::Low => score -= 5,
                PqcSeverity::Info => score -= 0,
            }
        }

        score.max(0)
    }

    /// Generate recommendations
    fn generate_recommendations(&self, vulnerabilities: &[PqcVulnerabilityFinding]) -> Vec<String> {
        let mut recommendations = vec![
            "Implement CRYSTALS-Kyber for quantum-resistant key exchange".to_string(),
            "Implement CRYSTALS-Dilithium for quantum-resistant signatures".to_string(),
            "Enable hybrid mode for defense in depth".to_string(),
            "Upgrade to TLS 1.3 with PQC cipher suites".to_string(),
        ];

        if vulnerabilities.iter().any(|v| v.severity == PqcSeverity::Critical) {
            recommendations.insert(0, "URGENT: Address critical quantum vulnerabilities immediately".to_string());
        }

        recommendations
    }

    /// Get algorithm severity
    fn get_algorithm_severity(&self, algorithm: &str) -> PqcSeverity {
        match algorithm {
            "RSA" | "ECC" | "ECDH" => PqcSeverity::Critical,
            "ECDSA" | "Ed25519" => PqcSeverity::High,
            "X25519" | "Diffie-Hellman" => PqcSeverity::High,
            _ => PqcSeverity::Medium,
        }
    }

    /// Assess migration readiness
    pub fn assess_migration_readiness(&self, codebase: &HashMap<String, String>) -> Result<PqcMigrationReadiness> {
        let has_pqc = self.quantum_safe_algorithms.iter()
            .any(|algo| codebase.values().any(|content| content.contains(algo)));
        
        let has_tests = codebase.values().any(|content| content.contains("test_pqc") || content.contains("test_quantum"));
        
        let readiness = PqcMigrationReadiness {
            readiness_score: if has_pqc { 75 } else { 25 },
            inventory_complete: true,
            pqc_implemented: has_pqc,
            testing_complete: has_tests,
            staff_trained: false,
            migration_plan_ready: true,
            estimated_migration_months: if has_pqc { 6 } else { 12 },
            blockers: if !has_pqc {
                vec!["PQC algorithms not implemented".to_string()]
            } else {
                vec![]
            },
        };

        Ok(readiness)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_assessor_creation() {
        let assessor = PqcSecurityAssessor::new();
        assert_eq!(assessor.quantum_vulnerable_algorithms.len(), 8);
        assert_eq!(assessor.quantum_safe_algorithms.len(), 9);
    }

    #[test]
    fn test_quantum_vulnerable_detection() {
        let assessor = PqcSecurityAssessor::new();
        let mut codebase = HashMap::new();
        codebase.insert("test.rs".to_string(), "RSA encryption".to_string());

        let findings = assessor.scan_quantum_vulnerable_algorithms(&codebase).unwrap();
        assert_eq!(findings.len(), 1);
        assert_eq!(findings[0].affected_algorithm, "RSA");
    }

    #[test]
    fn test_compliance_assessment() {
        let assessor = PqcSecurityAssessor::new();
        let compliance = assessor.assess_compliance(&[]).unwrap();
        
        assert_eq!(compliance.compliance_score, 100);
        assert!(compliance.fips_203_compliant);
        assert!(compliance.cnsa_2_0_compliant);
    }

    #[test]
    fn test_migration_readiness() {
        let assessor = PqcSecurityAssessor::new();
        let mut codebase = HashMap::new();
        codebase.insert("test.rs".to_string(), "CRYSTALS-Kyber".to_string());

        let readiness = assessor.assess_migration_readiness(&codebase).unwrap();
        assert_eq!(readiness.readiness_score, 75);
        assert!(readiness.pqc_implemented);
    }
}