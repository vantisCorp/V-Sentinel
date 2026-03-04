// Security Module for SENTINEL Security System
// Provides security hardening, penetration testing, and vulnerability scanning

pub mod hardening;
pub mod penetration_testing;
pub mod vulnerability_scanning;
pub mod security_audit;
pub mod enhanced_hardening;
pub mod advanced_penetration_testing;
pub mod advanced_security_audit;
pub mod secure_coding;

pub use hardening::{
    SecurityHardeningManager,
    HardeningMeasure,
    HardeningConfig,
    HardeningScore,
    SecurityLevel,
    HardeningCategory,
    ImplementationStatus,
};

pub use penetration_testing::{
    PenetrationTestingFramework,
    PenTestResult,
    PenTestType,
    VulnerabilityFinding,
    VulnerabilitySeverity,
    VulnerabilityStatus,
    PenTestSummary,
    PenTestConfig,
};

pub use vulnerability_scanning::{
    VulnerabilityScanner,
    VulnerabilityScanResult,
    ScannerType,
    Vulnerability,
    VulnerabilitySource,
    ScanSummary,
    ScannerConfig,
};

pub use security_audit::{
    SecurityAuditor,
    AuditResult,
    AuditType,
    AuditFinding,
    AuditSeverity,
    FindingStatus,
    AuditSummary,
    ComplianceStatus,
    ComplianceStandard,
    AuditConfig,
};

pub use secure_coding::{
    SecureCodingEnforcer,
    CodeAnalysisResult,
    SecureCodingRule,
    CodingCategory,
    RuleSeverity,
    CodeViolation,
    AnalysisSummary,
    EnforcerConfig,
};

/// Security module version
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Initialize security module
pub fn init() {
    // Initialize security monitoring systems
    // This would set up security hardening, penetration testing, etc.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_module_init() {
        init();
        // Module should initialize without errors
    }
}