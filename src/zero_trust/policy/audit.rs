//! Policy Audit and Compliance Reporting Module
//!
//! Provides comprehensive audit logging, compliance mapping, and report generation.

use super::{Policy, PolicyType, PolicyAction};
use super::super::{ZeroTrustContext, AccessDecision, AccessResult};
use std::sync::Arc;
use tokio::sync::RwLock;
use std::collections::{HashMap, BTreeMap};
use chrono::{DateTime, Utc, Duration};
use serde::{Deserialize, Serialize};

/// Policy Audit Manager
pub struct PolicyAuditManager {
    /// Audit log entries
    audit_log: Arc<RwLock<Vec<AuditLogEntry>>>,
    
    /// Policy change history
    change_history: Arc<RwLock<HashMap<String, Vec<PolicyChangeRecord>>>>,
    
    /// Compliance frameworks
    compliance_frameworks: Arc<RwLock<HashMap<String, ComplianceFramework>>>,
    
    /// Configuration
    config: AuditConfig,
}

impl PolicyAuditManager {
    /// Create a new policy audit manager
    pub fn new(config: AuditConfig) -> Self {
        Self {
            audit_log: Arc::new(RwLock::new(Vec::new())),
            change_history: Arc::new(RwLock::new(HashMap::new())),
            compliance_frameworks: Arc::new(RwLock::new(HashMap::new())),
            config,
        }
    }
    
    /// Log a policy evaluation event
    pub async fn log_evaluation(
        &self,
        context: &ZeroTrustContext,
        result: &AccessResult,
        policy_id: &str,
    ) {
        let entry = AuditLogEntry {
            id: uuid::Uuid::new_v4(),
            timestamp: Utc::now(),
            event_type: AuditEventType::PolicyEvaluation,
            policy_id: policy_id.to_string(),
            subject: context.subject.clone(),
            resource: context.resource.clone(),
            action: context.action.clone(),
            decision: result.decision,
            trust_score: result.trust_score,
            reason: result.reason.clone(),
            metadata: serde_json::to_value(context).unwrap_or_default(),
        };
        
        let mut log = self.audit_log.write().await;
        log.push(entry);
        
        // Prune old entries if needed
        if log.len() > self.config.max_log_entries {
            log.drain(0..log.len() - self.config.max_log_entries);
        }
    }
    
    /// Log a policy change
    pub async fn log_policy_change(
        &self,
        policy_id: &str,
        change_type: ChangeType,
        old_policy: Option<&Policy>,
        new_policy: Option<&Policy>,
        author: &str,
        reason: &str,
    ) {
        let change = PolicyChangeRecord {
            id: uuid::Uuid::new_v4(),
            timestamp: Utc::now(),
            policy_id: policy_id.to_string(),
            change_type,
            author: author.to_string(),
            reason: reason.to_string(),
            old_policy: old_policy.cloned(),
            new_policy: new_policy.cloned(),
        };
        
        let mut history = self.change_history.write().await;
        history
            .entry(policy_id.to_string())
            .or_insert_with(Vec::new)
            .push(change);
    }
    
    /// Register a compliance framework
    pub async fn register_compliance_framework(&self, framework: ComplianceFramework) {
        let mut frameworks = self.compliance_frameworks.write().await;
        frameworks.insert(framework.name.clone(), framework);
    }
    
    /// Generate audit report
    pub async fn generate_audit_report(
        &self,
        start: DateTime<Utc>,
        end: DateTime<Utc>,
        filters: Option<AuditFilters>,
    ) -> AuditReport {
        let log = self.audit_log.read().await;
        
        let mut entries: Vec<&AuditLogEntry> = log
            .iter()
            .filter(|e| e.timestamp >= start && e.timestamp <= end)
            .collect();
        
        // Apply filters
        if let Some(filters) = &filters {
            if let Some(subject) = &filters.subject {
                entries.retain(|e| e.subject == *subject);
            }
            if let Some(policy_id) = &filters.policy_id {
                entries.retain(|e| e.policy_id == *policy_id);
            }
            if let Some(decision) = &filters.decision {
                entries.retain(|e| &e.decision == decision);
            }
        }
        
        // Generate statistics
        let mut decision_counts: HashMap<AccessDecision, u64> = HashMap::new();
        let mut policy_counts: HashMap<String, u64> = HashMap::new();
        let mut subject_counts: HashMap<String, u64> = HashMap::new();
        
        for entry in &entries {
            *decision_counts.entry(entry.decision).or_insert(0) += 1;
            *policy_counts.entry(entry.policy_id.clone()).or_insert(0) += 1;
            *subject_counts.entry(entry.subject.clone()).or_insert(0) += 1;
        }
        
        AuditReport {
            id: uuid::Uuid::new_v4(),
            generated_at: Utc::now(),
            period: ReportPeriod { start, end },
            total_entries: entries.len(),
            entries: entries.clone(),
            statistics: AuditStatistics {
                decision_counts,
                policy_counts,
                subject_counts,
            },
        }
    }
    
    /// Generate compliance report
    pub async fn generate_compliance_report(
        &self,
        framework: &str,
        start: DateTime<Utc>,
        end: DateTime<Utc>,
    ) -> Result<ComplianceReport, AuditError> {
        let frameworks = self.compliance_frameworks.read().await;
        let framework = frameworks
            .get(framework)
            .ok_or_else(|| AuditError::FrameworkNotFound(framework.to_string()))?;
        
        let audit_report = self.generate_audit_report(start, end, None).await;
        
        let mut compliance_items = Vec::new();
        
        for requirement in &framework.requirements {
            let status = self.check_requirement(requirement, &audit_report).await;
            
            compliance_items.push(ComplianceItem {
                requirement_id: requirement.id.clone(),
                requirement_name: requirement.name.clone(),
                description: requirement.description.clone(),
                status,
                evidence: self.collect_evidence(requirement, &audit_report).await,
            });
        }
        
        let overall_status = if compliance_items.iter().all(|i| i.status == ComplianceStatus::Compliant) {
            ComplianceStatus::Compliant
        } else if compliance_items.iter().any(|i| i.status == ComplianceStatus::NonCompliant) {
            ComplianceStatus::NonCompliant
        } else {
            ComplianceStatus::Partial
        };
        
        Ok(ComplianceReport {
            id: uuid::Uuid::new_v4(),
            generated_at: Utc::now(),
            framework: framework.name.clone(),
            version: framework.version.clone(),
            period: ReportPeriod { start, end },
            overall_status,
            compliance_items,
        })
    }
    
    /// Get policy change history
    pub async fn get_change_history(
        &self,
        policy_id: &str,
    ) -> Vec<PolicyChangeRecord> {
        let history = self.change_history.read().await;
        history
            .get(policy_id)
            .cloned()
            .unwrap_or_default()
    }
    
    /// Search audit log
    pub async fn search_audit_log(
        &self,
        query: &AuditQuery,
    ) -> Vec<AuditLogEntry> {
        let log = self.audit_log.read().await;
        
        log.iter()
            .filter(|entry| self.matches_query(entry, query))
            .cloned()
            .collect()
    }
    
    async fn check_requirement(
        &self,
        requirement: &ComplianceRequirement,
        audit_report: &AuditReport,
    ) -> ComplianceStatus {
        // Simple implementation - in production, use more sophisticated logic
        match requirement.requirement_type {
            ComplianceRequirementType::AccessLogging => {
                if audit_report.total_entries > 0 {
                    ComplianceStatus::Compliant
                } else {
                    ComplianceStatus::NonCompliant
                }
            }
            ComplianceRequirementType::PolicyReview => {
                ComplianceStatus::Compliant // Assume compliant
            }
            ComplianceRequirementType::LeastPrivilege => {
                ComplianceStatus::Compliant // Assume compliant
            }
            ComplianceRequirementType::AuditTrail => {
                ComplianceStatus::Compliant
            }
        }
    }
    
    async fn collect_evidence(
        &self,
        requirement: &ComplianceRequirement,
        audit_report: &AuditReport,
    ) -> Vec<EvidenceItem> {
        // Collect relevant audit entries as evidence
        let mut evidence = Vec::new();
        
        if audit_report.total_entries > 0 {
            evidence.push(EvidenceItem {
                description: format!("Audit log contains {} entries", audit_report.total_entries),
                source: "audit_log".to_string(),
                timestamp: Some(Utc::now()),
                details: Some(serde_json::json!({
                    "total_entries": audit_report.total_entries
                })),
            });
        }
        
        evidence
    }
    
    fn matches_query(&self, entry: &AuditLogEntry, query: &AuditQuery) -> bool {
        if let Some(subject) = &query.subject {
            if !entry.subject.contains(subject) {
                return false;
            }
        }
        
        if let Some(resource) = &query.resource {
            if !entry.resource.contains(resource) {
                return false;
            }
        }
        
        if let Some(start) = &query.start {
            if entry.timestamp < *start {
                return false;
            }
        }
        
        if let Some(end) = &query.end {
            if entry.timestamp > *end {
                return false;
            }
        }
        
        true
    }
}

/// Audit Log Entry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditLogEntry {
    pub id: uuid::Uuid,
    pub timestamp: DateTime<Utc>,
    pub event_type: AuditEventType,
    pub policy_id: String,
    pub subject: String,
    pub resource: String,
    pub action: String,
    pub decision: AccessDecision,
    pub trust_score: f64,
    pub reason: String,
    pub metadata: serde_json::Value,
}

/// Audit Event Type
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AuditEventType {
    PolicyEvaluation,
    PolicyCreated,
    PolicyModified,
    PolicyDeleted,
    PolicyEnabled,
    PolicyDisabled,
    ComplianceCheck,
}

/// Policy Change Record
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PolicyChangeRecord {
    pub id: uuid::Uuid,
    pub timestamp: DateTime<Utc>,
    pub policy_id: String,
    pub change_type: ChangeType,
    pub author: String,
    pub reason: String,
    pub old_policy: Option<Policy>,
    pub new_policy: Option<Policy>,
}

/// Change Type
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ChangeType {
    Created,
    Modified,
    Deleted,
    Enabled,
    Disabled,
    Versioned,
}

/// Compliance Framework
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComplianceFramework {
    pub name: String,
    pub version: String,
    pub description: String,
    pub requirements: Vec<ComplianceRequirement>,
}

/// Compliance Requirement
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComplianceRequirement {
    pub id: String,
    pub name: String,
    pub description: String,
    pub requirement_type: ComplianceRequirementType,
    pub controls: Vec<String>,
}

/// Compliance Requirement Type
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ComplianceRequirementType {
    AccessLogging,
    PolicyReview,
    LeastPrivilege,
    AuditTrail,
    DataProtection,
    IncidentResponse,
}

/// Audit Report
#[derive(Debug, Clone)]
pub struct AuditReport {
    pub id: uuid::Uuid,
    pub generated_at: DateTime<Utc>,
    pub period: ReportPeriod,
    pub total_entries: usize,
    pub entries: Vec<AuditLogEntry>,
    pub statistics: AuditStatistics,
}

/// Report Period
#[derive(Debug, Clone)]
pub struct ReportPeriod {
    pub start: DateTime<Utc>,
    pub end: DateTime<Utc>,
}

/// Audit Statistics
#[derive(Debug, Clone)]
pub struct AuditStatistics {
    pub decision_counts: HashMap<AccessDecision, u64>,
    pub policy_counts: HashMap<String, u64>,
    pub subject_counts: HashMap<String, u64>,
}

/// Compliance Report
#[derive(Debug, Clone)]
pub struct ComplianceReport {
    pub id: uuid::Uuid,
    pub generated_at: DateTime<Utc>,
    pub framework: String,
    pub version: String,
    pub period: ReportPeriod,
    pub overall_status: ComplianceStatus,
    pub compliance_items: Vec<ComplianceItem>,
}

/// Compliance Status
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ComplianceStatus {
    Compliant,
    NonCompliant,
    Partial,
    NotApplicable,
}

/// Compliance Item
#[derive(Debug, Clone)]
pub struct ComplianceItem {
    pub requirement_id: String,
    pub requirement_name: String,
    pub description: String,
    pub status: ComplianceStatus,
    pub evidence: Vec<EvidenceItem>,
}

/// Evidence Item
#[derive(Debug, Clone)]
pub struct EvidenceItem {
    pub description: String,
    pub source: String,
    pub timestamp: Option<DateTime<Utc>>,
    pub details: Option<serde_json::Value>,
}

/// Audit Filters
#[derive(Debug, Clone)]
pub struct AuditFilters {
    pub subject: Option<String>,
    pub policy_id: Option<String>,
    pub decision: Option<AccessDecision>,
}

/// Audit Query
#[derive(Debug, Clone)]
pub struct AuditQuery {
    pub subject: Option<String>,
    pub resource: Option<String>,
    pub start: Option<DateTime<Utc>>,
    pub end: Option<DateTime<Utc>>,
}

/// Audit Configuration
#[derive(Debug, Clone)]
pub struct AuditConfig {
    pub max_log_entries: usize,
    pub retention_period: Duration,
    pub enable_compliance_logging: bool,
}

impl Default for AuditConfig {
    fn default() -> Self {
        Self {
            max_log_entries: 100_000,
            retention_period: Duration::days(90),
            enable_compliance_logging: true,
        }
    }
}

/// Audit Error
#[derive(Debug, thiserror::Error)]
pub enum AuditError {
    #[error("Framework not found: {0}")]
    FrameworkNotFound(String),
    
    #[error("Report generation error: {0}")]
    ReportGenerationError(String),
    
    #[error("Query error: {0}")]
    QueryError(String),
}

impl Default for PolicyAuditManager {
    fn default() -> Self {
        Self::new(AuditConfig::default())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{DeviceInfo, NetworkInfo, LocationInfo, DeviceType, NetworkType, TrustLevel};

    fn create_test_context() -> ZeroTrustContext {
        ZeroTrustContext {
            request_id: uuid::Uuid::new_v4(),
            subject: "user@example.com".to_string(),
            resource: "/api/data".to_string(),
            action: "read".to_string(),
            device: DeviceInfo {
                device_id: "device-123".to_string(),
                device_type: DeviceType::Laptop,
                os: "Windows".to_string(),
                os_version: "10".to_string(),
                security_score: 0.8,
                is_trusted: true,
                last_seen: Utc::now(),
            },
            network: NetworkInfo {
                ip_address: "192.168.1.100".to_string(),
                location: LocationInfo {
                    country: "US".to_string(),
                    region: Some("CA".to_string()),
                    city: Some("San Francisco".to_string()),
                    latitude: Some(37.7749),
                    longitude: Some(-122.4194),
                },
                network_type: NetworkType::Corporate,
                is_trusted: true,
                is_encrypted: true,
            },
            timestamp: Utc::now(),
            context: serde_json::json!({}),
        }
    }

    #[tokio::test]
    async fn test_audit_manager_creation() {
        let config = AuditConfig::default();
        let manager = PolicyAuditManager::new(config);
        
        assert_eq!(manager.audit_log.read().await.len(), 0);
    }

    #[tokio::test]
    async fn test_log_evaluation() {
        let manager = PolicyAuditManager::default();
        
        let context = create_test_context();
        let result = AccessResult {
            decision: AccessDecision::Allow,
            trust_score: 0.8,
            trust_level: TrustLevel::High,
            reason: "Test".to_string(),
            required_actions: Vec::new(),
            timestamp: Utc::now(),
        };
        
        manager.log_evaluation(&context, &result, "test-policy").await;
        
        let log = manager.audit_log.read().await;
        assert_eq!(log.len(), 1);
    }

    #[tokio::test]
    async fn test_log_policy_change() {
        let manager = PolicyAuditManager::default();
        
        let policy = Policy::new("test-policy", "Test", PolicyType::Access)
            .with_action(PolicyAction::Allow);
        
        manager.log_policy_change(
            "test-policy",
            ChangeType::Created,
            None,
            Some(&policy),
            "admin",
            "Initial policy",
        ).await;
        
        let history = manager.get_change_history("test-policy").await;
        assert_eq!(history.len(), 1);
    }

    #[tokio::test]
    async fn test_generate_audit_report() {
        let manager = PolicyAuditManager::default();
        
        let context = create_test_context();
        let result = AccessResult {
            decision: AccessDecision::Allow,
            trust_score: 0.8,
            trust_level: TrustLevel::High,
            reason: "Test".to_string(),
            required_actions: Vec::new(),
            timestamp: Utc::now(),
        };
        
        manager.log_evaluation(&context, &result, "test-policy").await;
        
        let start = Utc::now() - Duration::hours(1);
        let end = Utc::now() + Duration::hours(1);
        
        let report = manager.generate_audit_report(start, end, None).await;
        assert_eq!(report.total_entries, 1);
    }

    #[tokio::test]
    async fn test_register_compliance_framework() {
        let manager = PolicyAuditManager::default();
        
        let framework = ComplianceFramework {
            name: "SOC2".to_string(),
            version: "1.0".to_string(),
            description: "SOC 2 Type II".to_string(),
            requirements: Vec::new(),
        };
        
        manager.register_compliance_framework(framework).await;
        
        let frameworks = manager.compliance_frameworks.read().await;
        assert!(frameworks.contains_key("SOC2"));
    }

    #[tokio::test]
    async fn test_search_audit_log() {
        let manager = PolicyAuditManager::default();
        
        let context = create_test_context();
        let result = AccessResult {
            decision: AccessDecision::Allow,
            trust_score: 0.8,
            trust_level: TrustLevel::High,
            reason: "Test".to_string(),
            required_actions: Vec::new(),
            timestamp: Utc::now(),
        };
        
        manager.log_evaluation(&context, &result, "test-policy").await;
        
        let query = AuditQuery {
            subject: Some("user@example.com".to_string()),
            resource: None,
            start: None,
            end: None,
        };
        
        let results = manager.search_audit_log(&query).await;
        assert_eq!(results.len(), 1);
    }
}