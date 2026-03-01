// Secure Coding Practices for SENTINEL Security System
// Provides secure coding guidelines and enforcement

use std::collections::{HashMap, HashSet};
use std::sync::Arc;
use tokio::sync::RwLock;
use serde::{Serialize, Deserialize};
use anyhow::{Result, anyhow};

/// Secure coding rule
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecureCodingRule {
    pub id: String,
    pub title: String,
    pub description: String,
    pub category: CodingCategory,
    pub severity: RuleSeverity,
    pub examples: Vec<String>,
    pub references: Vec<String>,
}

/// Coding category
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CodingCategory {
    InputValidation,
    OutputEncoding,
    Authentication,
    Authorization,
    Cryptography,
    ErrorHandling,
    Logging,
    MemoryManagement,
    Concurrency,
    Configuration,
}

/// Rule severity
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum RuleSeverity {
    Info,
    Low,
    Medium,
    High,
    Critical,
}

/// Code analysis result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CodeAnalysisResult {
    pub analysis_id: String,
    pub target: String,
    pub started_at: String,
    pub completed_at: String,
    pub duration_seconds: u64,
    pub violations: Vec<CodeViolation>,
    pub summary: AnalysisSummary,
}

/// Code violation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CodeViolation {
    pub id: String,
    pub rule_id: String,
    pub title: String,
    pub description: String,
    pub file_path: String,
    pub line_number: usize,
    pub severity: RuleSeverity,
    pub suggestion: String,
}

/// Analysis summary
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnalysisSummary {
    pub total_violations: usize,
    pub critical_count: usize,
    pub high_count: usize,
    pub medium_count: usize,
    pub low_count: usize,
    pub info_count: usize,
    pub security_score: f64,
}

/// Secure coding enforcer
pub struct SecureCodingEnforcer {
    rules: Arc<RwLock<Vec<SecureCodingRule>>>,
    violations: Arc<RwLock<Vec<CodeViolation>>>,
    config: EnforcerConfig,
}

/// Enforcer configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnforcerConfig {
    pub auto_scan_enabled: bool,
    pub scan_on_commit: bool,
    pub severity_threshold: RuleSeverity,
    pub fail_build_on_critical: bool,
}

impl Default for EnforcerConfig {
    fn default() -> Self {
        Self {
            auto_scan_enabled: true,
            scan_on_commit: true,
            severity_threshold: RuleSeverity::Medium,
            fail_build_on_critical: true,
        }
    }
}

impl SecureCodingEnforcer {
    pub fn new() -> Self {
        let mut enforcer = Self {
            rules: Arc::new(RwLock::new(Vec::new())),
            violations: Arc::new(RwLock::new(Vec::new())),
            config: EnforcerConfig::default(),
        };
        
        // Initialize rules
        enforcer.initialize_rules();
        
        enforcer
    }

    fn initialize_rules(&self) {
        let rules = vec![
            SecureCodingRule {
                id: "SC-001".to_string(),
                title: "Validate All User Input".to_string(),
                description: "All user input must be validated before use".to_string(),
                category: CodingCategory::InputValidation,
                severity: RuleSeverity::Critical,
                examples: vec![
                    "Use parameterized queries to prevent SQL injection".to_string(),
                    "Validate and sanitize all input data".to_string(),
                ],
                references: vec!["OWASP A03:2021".to_string(), "CWE-20".to_string()],
            },
            
            SecureCodingRule {
                id: "SC-002".to_string(),
                title: "Encode All Output".to_string(),
                description: "All output must be properly encoded to prevent XSS".to_string(),
                category: CodingCategory::OutputEncoding,
                severity: RuleSeverity::Critical,
                examples: vec![
                    "Use context-aware output encoding".to_string(),
                    "Implement Content Security Policy (CSP)".to_string(),
                ],
                references: vec!["OWASP A03:2021".to_string(), "CWE-79".to_string()],
            },
            
            SecureCodingRule {
                id: "SC-003".to_string(),
                title: "Use Strong Authentication".to_string(),
                description: "Implement strong authentication mechanisms".to_string(),
                category: CodingCategory::Authentication,
                severity: RuleSeverity::Critical,
                examples: vec![
                    "Require multi-factor authentication".to_string(),
                    "Implement secure password policies".to_string(),
                    "Use secure session management".to_string(),
                ],
                references: vec!["OWASP A07:2021".to_string(), "CWE-287".to_string()],
            },
            
            SecureCodingRule {
                id: "SC-004".to_string(),
                title: "Implement Proper Authorization".to_string(),
                description: "Ensure proper authorization checks on all operations".to_string(),
                category: CodingCategory::Authorization,
                severity: RuleSeverity::Critical,
                examples: vec![
                    "Implement principle of least privilege".to_string(),
                    "Check authorization on every request".to_string(),
                    "Use role-based access control".to_string(),
                ],
                references: vec!["OWASP A01:2021".to_string(), "CWE-862".to_string()],
            },
            
            SecureCodingRule {
                id: "SC-005".to_string(),
                title: "Use Strong Cryptography".to_string(),
                description: "Use approved cryptographic algorithms and proper key management".to_string(),
                category: CodingCategory::Cryptography,
                severity: RuleSeverity::Critical,
                examples: vec![
                    "Use AES-256 for encryption".to_string(),
                    "Use TLS 1.3 for secure communication".to_string(),
                    "Implement proper key lifecycle management".to_string(),
                ],
                references: vec!["OWASP A02:2021".to_string(), "CWE-327".to_string()],
            },
            
            SecureCodingRule {
                id: "SC-006".to_string(),
                title: "Handle Errors Securely".to_string(),
                description: "Implement proper error handling without exposing sensitive information".to_string(),
                category: CodingCategory::ErrorHandling,
                severity: RuleSeverity::High,
                examples: vec![
                    "Don't expose stack traces in production".to_string(),
                    "Log errors securely".to_string(),
                    "Implement graceful error recovery".to_string(),
                ],
                references: vec!["OWASP A05:2021".to_string(), "CWE-209".to_string()],
            },
            
            SecureCodingRule {
                id: "SC-007".to_string(),
                title: "Implement Secure Logging".to_string(),
                description: "Log security-relevant events without exposing sensitive data".to_string(),
                category: CodingCategory::Logging,
                severity: RuleSeverity::High,
                examples: vec![
                    "Log authentication attempts".to_string(),
                    "Log authorization failures".to_string(),
                    "Don't log sensitive data".to_string(),
                ],
                references: vec!["OWASP A09:2021".to_string(), "CWE-532".to_string()],
            },
            
            SecureCodingRule {
                id: "SC-008".to_string(),
                title: "Prevent Memory Safety Issues".to_string(),
                description: "Prevent buffer overflows, use-after-free, and other memory issues".to_string(),
                category: CodingCategory::MemoryManagement,
                severity: RuleSeverity::Critical,
                examples: vec![
                    "Use memory-safe languages like Rust".to_string(),
                    "Enable compiler protections (ASLR, DEP)".to_string(),
                    "Use bounds checking".to_string(),
                ],
                references: vec!["CWE-119".to_string(), "CWE-416".to_string()],
            },
            
            SecureCodingRule {
                id: "SC-009".to_string(),
                title: "Implement Thread Safety".to_string(),
                description: "Ensure thread-safe operations and prevent race conditions".to_string(),
                category: CodingCategory::Concurrency,
                severity: RuleSeverity::High,
                examples: vec![
                    "Use proper synchronization primitives".to_string(),
                    "Avoid shared mutable state".to_string(),
                    "Use atomic operations where appropriate".to_string(),
                ],
                references: vec!["CWE-362".to_string(), "CWE-366".to_string()],
            },
            
            SecureCodingRule {
                id: "SC-010".to_string(),
                title: "Secure Configuration Management".to_string(),
                description: "Implement secure configuration management practices".to_string(),
                category: CodingCategory::Configuration,
                severity: RuleSeverity::High,
                examples: vec![
                    "Don't hardcode credentials".to_string(),
                    "Use environment variables for secrets".to_string(),
                    "Implement configuration validation".to_string(),
                ],
                references: vec!["OWASP A05:2021".to_string(), "CWE-798".to_string()],
            },
        ];
        
        let mut rules_lock = self.rules.blocking_write();
        rules_lock.extend(rules);
    }

    pub async fn analyze_code(&self, target: &str) -> Result<CodeAnalysisResult> {
        let analysis_id = uuid::Uuid::new_v4().to_string();
        let started_at = chrono::Utc::now().to_rfc3339();
        
        // Simulate code analysis
        tokio::time::sleep(tokio::time::Duration::from_secs(3)).await;
        
        let completed_at = chrono::Utc::now().to_rfc3339();
        let duration = 3;
        
        let mut violations = Vec::new();
        
        // Simulate finding violations
        violations.push(CodeViolation {
            id: uuid::Uuid::new_v4().to_string(),
            rule_id: "SC-001".to_string(),
            title: "Missing Input Validation".to_string(),
            description: "User input is not validated before use".to_string(),
            file_path: "src/api/login.rs".to_string(),
            line_number: 42,
            severity: RuleSeverity::Critical,
            suggestion: "Add input validation and sanitization".to_string(),
        });

        violations.push(CodeViolation {
            id: uuid::Uuid::new_v4().to_string(),
            rule_id: "SC-005".to_string(),
            title: "Weak Cryptography".to_string(),
            description: "Using deprecated cryptographic algorithm".to_string(),
            file_path: "src/crypto/encryption.rs".to_string(),
            line_number: 15,
            severity: RuleSeverity::Critical,
            suggestion: "Use AES-256 instead of DES".to_string(),
        });

        violations.push(CodeViolation {
            id: uuid::Uuid::new_v4().to_string(),
            rule_id: "SC-006".to_string(),
            title: "Insecure Error Handling".to_string(),
            description: "Error messages expose sensitive information".to_string(),
            file_path: "src/api/errors.rs".to_string(),
            line_number: 28,
            severity: RuleSeverity::High,
            suggestion: "Remove sensitive information from error messages".to_string(),
        });

        violations.push(CodeViolation {
            id: uuid::Uuid::new_v4().to_string(),
            rule_id: "SC-010".to_string(),
            title: "Hardcoded Credentials".to_string(),
            description: "Credentials are hardcoded in source code".to_string(),
            file_path: "src/config/database.rs".to_string(),
            line_number: 10,
            severity: RuleSeverity::Critical,
            suggestion: "Move credentials to environment variables or secret management".to_string(),
        });

        let summary = self.calculate_summary(&violations);
        
        let result = CodeAnalysisResult {
            analysis_id: analysis_id.clone(),
            target: target.to_string(),
            started_at,
            completed_at,
            duration_seconds: duration,
            violations: violations.clone(),
            summary,
        };
        
        // Store violations
        let mut all_violations = self.violations.write().await;
        all_violations.extend(violations);
        
        Ok(result)
    }

    pub async fn get_rules(&self) -> Vec<SecureCodingRule> {
        let rules = self.rules.read().await;
        rules.clone()
    }

    pub async fn get_rules_by_category(&self, category: CodingCategory) -> Vec<SecureCodingRule> {
        let rules = self.rules.read().await;
        rules.iter()
            .filter(|r| r.category == category)
            .cloned()
            .collect()
    }

    pub async fn get_violations(&self) -> Vec<CodeViolation> {
        let violations = self.violations.read().await;
        violations.clone()
    }

    pub async fn get_violations_by_severity(&self, severity: RuleSeverity) -> Vec<CodeViolation> {
        let violations = self.violations.read().await;
        violations.iter()
            .filter(|v| v.severity == severity)
            .cloned()
            .collect()
    }

    pub async fn generate_report(&self) -> String {
        let violations = self.violations.read().await;
        let rules = self.rules.read().await;
        
        let mut report = String::new();
        report.push_str("# SENTINEL Secure Coding Analysis Report\n\n");
        
        // Executive summary
        report.push_str("## Executive Summary\n\n");
        
        let total_violations = violations.len();
        let critical = violations.iter().filter(|v| v.severity == RuleSeverity::Critical).count();
        let high = violations.iter().filter(|v| v.severity == RuleSeverity::High).count();
        let medium = violations.iter().filter(|v| v.severity == RuleSeverity::Medium).count();
        let low = violations.iter().filter(|v| v.severity == RuleSeverity::Low).count();
        let info = violations.iter().filter(|v| v.severity == RuleSeverity::Info).count();
        
        report.push_str(&format!("**Total Violations:** {}\n\n", total_violations));
        report.push_str(&format!("- 🔴 Critical: {}\n", critical));
        report.push_str(&format!("- 🟠 High: {}\n", high));
        report.push_str(&format!("- 🟡 Medium: {}\n", medium));
        report.push_str(&format!("- 🟢 Low: {}\n", low));
        report.push_str(&format!("- 🔵 Info: {}\n\n", info));
        
        // Secure coding rules
        report.push_str("## Secure Coding Rules\n\n");
        
        for category in &[CodingCategory::InputValidation, CodingCategory::OutputEncoding, 
                          CodingCategory::Authentication, CodingCategory::Authorization,
                          CodingCategory::Cryptography, CodingCategory::ErrorHandling,
                          CodingCategory::Logging, CodingCategory::MemoryManagement,
                          CodingCategory::Concurrency, CodingCategory::Configuration] {
            let category_rules: Vec<_> = rules.iter()
                .filter(|r| r.category == *category)
                .collect();
            
            if !category_rules.is_empty() {
                report.push_str(&format!("### {:?}\n\n", category));
                
                for rule in category_rules {
                    let severity_icon = match rule.severity {
                        RuleSeverity::Info => "🔵",
                        RuleSeverity::Low => "🟢",
                        RuleSeverity::Medium => "🟡",
                        RuleSeverity::High => "🟠",
                        RuleSeverity::Critical => "🔴",
                    };
                    
                    report.push_str(&format!("{} {} - {}\n\n", severity_icon, rule.id, rule.title));
                    report.push_str(&format!("**Description:** {}\n", rule.description));
                    report.push_str(&format!("**Examples:**\n"));
                    for example in &rule.examples {
                        report.push_str(&format!("- {}\n", example));
                    }
                    report.push_str(&format!("**References:** {}\n\n", rule.references.join(", ")));
                }
            }
        }
        
        // Violation details
        report.push_str("## Violation Details\n\n");
        
        for severity in &[RuleSeverity::Critical, RuleSeverity::High, 
                          RuleSeverity::Medium, RuleSeverity::Low, RuleSeverity::Info] {
            let severity_violations: Vec<_> = violations.iter()
                .filter(|v| v.severity == *severity)
                .collect();
            
            if !severity_violations.is_empty() {
                report.push_str(&format!("### {:?} Violations ({})\n\n", severity, severity_violations.len()));
                
                for violation in severity_violations {
                    report.push_str(&format!("#### {} - {}\n\n", violation.file_path, violation.line_number));
                    report.push_str(&format!("**Rule:** {}\n", violation.rule_id));
                    report.push_str(&format!("**Title:** {}\n", violation.title));
                    report.push_str(&format!("**Description:** {}\n", violation.description));
                    report.push_str(&format!("**Suggestion:** {}\n\n", violation.suggestion));
                }
            }
        }
        
        // Recommendations
        report.push_str("## Recommendations\n\n");
        report.push_str("1. **Immediate Actions (Critical):**\n");
        report.push_str("   - Fix all Critical violations immediately\n");
        report.push_str("   - Implement code review process\n\n");
        
        report.push_str("2. **Short-term Actions (High):**\n");
        report.push_str("   - Fix High violations within 7 days\n");
        report.push_str("   - Add automated security testing to CI/CD\n\n");
        
        report.push_str("3. **Medium-term Actions (Medium):**\n");
        report.push_str("   - Fix Medium violations within 30 days\n");
        report.push_str("   - Conduct security training for developers\n\n");
        
        report.push_str("4. **Long-term Actions (Low/Info):**\n");
        report.push_str("   - Address Low and Info violations in next release\n");
        report.push_str("   - Implement continuous security monitoring\n\n");
        
        report
    }

    fn calculate_summary(&self, violations: &[CodeViolation]) -> AnalysisSummary {
        let total = violations.len();
        let critical = violations.iter().filter(|v| v.severity == RuleSeverity::Critical).count();
        let high = violations.iter().filter(|v| v.severity == RuleSeverity::High).count();
        let medium = violations.iter().filter(|v| v.severity == RuleSeverity::Medium).count();
        let low = violations.iter().filter(|v| v.severity == RuleSeverity::Low).count();
        let info = violations.iter().filter(|v| v.severity == RuleSeverity::Info).count();
        
        let security_score = if total > 0 {
            100.0 - ((critical as f64 * 25.0 + high as f64 * 15.0 + medium as f64 * 10.0 + low as f64 * 5.0) / total as f64)
        } else {
            100.0
        };
        
        AnalysisSummary {
            total_violations: total,
            critical_count: critical,
            high_count: high,
            medium_count: medium,
            low_count: low,
            info_count: info,
            security_score: security_score.max(0.0),
        }
    }
}

impl Default for SecureCodingEnforcer {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_secure_coding_enforcer_initialization() {
        let enforcer = SecureCodingEnforcer::new();
        
        let rules = enforcer.get_rules().await;
        assert!(rules.len() > 0);
    }

    #[tokio::test]
    async fn test_code_analysis() {
        let enforcer = SecureCodingEnforcer::new();
        let result = enforcer.analyze_code("src/").await.unwrap();
        
        assert!(result.summary.total_violations > 0);
    }

    #[tokio::test]
    async fn test_secure_coding_report() {
        let enforcer = SecureCodingEnforcer::new();
        enforcer.analyze_code("src/").await.unwrap();
        
        let report = enforcer.generate_report().await;
        assert!(report.contains("SENTINEL Secure Coding Analysis Report"));
        assert!(report.contains("Executive Summary"));
    }
}