use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use crate::shadow_ai::{
    DetectedAIModel, AIModelType, RiskLevel, RegistrationStatus, ShadowAIError
};

/// Governance Engine - Manages AI policies and compliance
pub struct GovernanceEngine {
    /// Configuration
    config: GovernanceConfig,
    /// Policy rules
    policies: Arc<RwLock<Vec<PolicyRule>>>,
    /// AI model registry
    registry: Arc<RwLock<AIModelRegistry>>,
    /// Enforcement logs
    enforcement_logs: Arc<RwLock<Vec<EnforcementLog>>>,
}

/// Configuration for governance engine
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GovernanceConfig {
    /// Require registration for all AI models
    pub require_registration: bool,
    /// Auto-approve low-risk models
    pub auto_approve_low_risk: bool,
    /// Require approval threshold (0.0-1.0)
    pub approval_threshold: f64,
    /// Enable policy enforcement
    pub enable_enforcement: bool,
    /// Compliance frameworks to enforce
    pub compliance_frameworks: Vec<ComplianceFramework>,
    /// Audit log retention days
    pub audit_retention_days: u32,
}

impl Default for GovernanceConfig {
    fn default() -> Self {
        Self {
            require_registration: true,
            auto_approve_low_risk: false,
            approval_threshold: 0.7,
            enable_enforcement: true,
            compliance_frameworks: vec![
                ComplianceFramework::SOC2,
                ComplianceFramework::GDPR,
                ComplianceFramework::ISO27001,
            ],
            audit_retention_days: 365,
        }
    }
}

/// Compliance framework
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum ComplianceFramework {
    SOC2,
    SOX,
    GDPR,
    HIPAA,
    PCIDSS,
    ISO27001,
    NIST80053,
    Custom(String),
}

/// Policy rule for AI usage
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PolicyRule {
    /// Unique rule ID
    pub rule_id: String,
    /// Rule name
    pub name: String,
    /// Rule description
    pub description: String,
    /// Rule type
    pub rule_type: PolicyRuleType,
    /// Model types this rule applies to
    pub model_types: Vec<AIModelType>,
    /// Risk levels this rule applies to
    pub risk_levels: Vec<RiskLevel>,
    /// Required action
    pub action: PolicyAction,
    /// Rule priority (higher = more important)
    pub priority: u32,
    /// Whether rule is enabled
    pub enabled: bool,
    /// Creation timestamp
    pub created_at: DateTime<Utc>,
    /// Last modified timestamp
    pub modified_at: DateTime<Utc>,
}

/// Type of policy rule
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum PolicyRuleType {
    /// Require registration for model type
    RegistrationRequired,
    /// Require approval for model type
    ApprovalRequired,
    /// Restrict data access
    DataAccessRestriction,
    /// Block model usage
    Blocked,
    /// Require audit logging
    AuditLogging,
    /// Rate limiting
    RateLimiting,
    /// Custom rule
    Custom(String),
}

/// Policy action to take
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum PolicyAction {
    /// Allow usage
    Allow,
    /// Deny usage
    Deny,
    /// Require approval
    RequireApproval,
    /// Log warning
    Warn,
    /// Quarantine
    Quarantine,
    /// Custom action
    Custom(String),
}

/// AI Model Registry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AIModelRegistry {
    /// Registered models
    pub models: HashMap<String, RegisteredModel>,
    /// Registration requests pending approval
    pub pending_requests: Vec<RegistrationRequest>,
    /// Last updated
    pub last_updated: DateTime<Utc>,
}

/// Registered AI model
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegisteredModel {
    /// Model ID
    pub model_id: String,
    /// Model name
    pub model_name: String,
    /// Model type
    pub model_type: AIModelType,
    /// Owner/department
    pub owner: String,
    /// Registration date
    pub registered_at: DateTime<Utc>,
    /// Registration status
    pub status: RegistrationStatus,
    /// Approved by
    pub approved_by: Option<String>,
    /// Approval date
    pub approved_at: Option<DateTime<Utc>>,
    /// Expiry date
    pub expires_at: Option<DateTime<Utc>>,
    /// Allowed data sources
    pub allowed_data_sources: Vec<String>,
    /// Allowed usage contexts
    pub allowed_contexts: Vec<String>,
    /// Usage limits
    pub usage_limits: Option<UsageLimits>,
    /// Risk assessment
    pub risk_assessment: Option<ModelRiskAssessment>,
    /// Compliance certifications
    pub certifications: Vec<String>,
    /// Metadata
    pub metadata: HashMap<String, String>,
}

/// Model registration request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegistrationRequest {
    /// Request ID
    pub request_id: String,
    /// Model details
    pub model: DetectedAIModel,
    /// Requester
    pub requester: String,
    /// Requested data sources
    pub requested_data_sources: Vec<String>,
    /// Justification
    pub justification: String,
    /// Request timestamp
    pub requested_at: DateTime<Utc>,
    /// Review status
    pub status: RequestStatus,
    /// Reviewer comments
    pub reviewer_comments: Vec<String>,
}

/// Registration request status
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum RequestStatus {
    Pending,
    Approved,
    Rejected,
    UnderReview,
    NeedsMoreInfo,
}

/// Usage limits for registered models
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UsageLimits {
    /// Maximum requests per day
    pub max_requests_per_day: Option<u64>,
    /// Maximum tokens per day
    pub max_tokens_per_day: Option<u64>,
    /// Maximum data processed per day (bytes)
    pub max_data_per_day: Option<u64>,
    /// Allowed time windows
    pub allowed_time_windows: Vec<TimeWindow>,
}

/// Time window for usage
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimeWindow {
    /// Start hour (0-23)
    pub start_hour: u8,
    /// End hour (0-23)
    pub end_hour: u8,
    /// Days of week (0=Sunday, 6=Saturday)
    pub days_of_week: Vec<u8>,
}

/// Model risk assessment
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelRiskAssessment {
    /// Overall risk score (0.0-1.0)
    pub risk_score: f64,
    /// Risk level
    pub risk_level: RiskLevel,
    /// Data sensitivity impact
    pub data_sensitivity_impact: f64,
    /// Model capabilities impact
    pub capabilities_impact: f64,
    /// Access control impact
    pub access_control_impact: f64,
    /// Assessment notes
    pub notes: Vec<String>,
    /// Assessor
    pub assessed_by: String,
    /// Assessment date
    pub assessed_at: DateTime<Utc>,
}

/// Enforcement log entry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnforcementLog {
    /// Log ID
    pub log_id: String,
    /// Timestamp
    pub timestamp: DateTime<Utc>,
    /// Model involved
    pub model_id: Option<String>,
    /// User/service involved
    pub actor: String,
    /// Action taken
    pub action: EnforcementAction,
    /// Policy rule triggered
    pub rule_id: Option<String>,
    /// Outcome
    pub outcome: EnforcementOutcome,
    /// Details
    pub details: String,
}

/// Enforcement action
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum EnforcementAction {
    BlockedAccess,
    AllowedAccess,
    LoggedAccess,
    FlaggedForReview,
    QuarantinedModel,
    RevokedAccess,
}

/// Enforcement outcome
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum EnforcementOutcome {
    Success,
    Failure,
    Warning,
    Error,
}

impl GovernanceEngine {
    /// Create new governance engine
    pub fn new(config: GovernanceConfig) -> Self {
        Self {
            config,
            policies: Arc::new(RwLock::new(Vec::new())),
            registry: Arc::new(RwLock::new(AIModelRegistry {
                models: HashMap::new(),
                pending_requests: Vec::new(),
                last_updated: Utc::now(),
            })),
            enforcement_logs: Arc::new(RwLock::new(Vec::new())),
        }
    }

    /// Add a policy rule
    pub async fn add_policy_rule(&self, rule: PolicyRule) -> Result<(), ShadowAIError> {
        let mut policies = self.policies.write().await;
        policies.push(rule);
        Ok(())
    }

    /// Remove a policy rule
    pub async fn remove_policy_rule(&self, rule_id: &str) -> Result<(), ShadowAIError> {
        let mut policies = self.policies.write().await;
        policies.retain(|r| r.rule_id != rule_id);
        Ok(())
    }

    /// Get all policy rules
    pub async fn get_policy_rules(&self) -> Vec<PolicyRule> {
        self.policies.read().await.clone()
    }

    /// Evaluate a model against policies
    pub async fn evaluate_model(&self, model: &DetectedAIModel) -> PolicyEvaluationResult {
        let policies = self.policies.read().await;
        let mut evaluation = PolicyEvaluationResult {
            model_id: model.model_id.clone(),
            model_name: model.model_name.clone(),
            evaluation_id: uuid::Uuid::new_v4().to_string(),
            evaluated_at: Utc::now(),
            requires_registration: self.config.require_registration,
            requires_approval: false,
            allowed: true,
            blocked: false,
            triggered_rules: Vec::new(),
            recommended_action: PolicyAction::Allow,
            compliance_status: self.check_compliance(model).await,
        };

        for policy in policies.iter().filter(|p| p.enabled) {
            // Check if policy applies to this model type
            if !policy.model_types.is_empty() && !policy.model_types.contains(&model.model_type) {
                continue;
            }

            // Check if policy applies to this risk level
            if !policy.risk_levels.is_empty() && !policy.risk_levels.contains(&model.risk_level) {
                continue;
            }

            // Policy applies - record it
            evaluation.triggered_rules.push(policy.clone());

            match policy.rule_type {
                PolicyRuleType::RegistrationRequired => {
                    evaluation.requires_registration = true;
                }
                PolicyRuleType::ApprovalRequired => {
                    evaluation.requires_approval = true;
                }
                PolicyRuleType::Blocked => {
                    evaluation.blocked = true;
                    evaluation.allowed = false;
                    evaluation.recommended_action = PolicyAction::Deny;
                }
                PolicyRuleType::DataAccessRestriction => {
                    evaluation.allowed = false;
                    evaluation.recommended_action = PolicyAction::RequireApproval;
                }
                _ => {}
            }
        }

        // Auto-approve low-risk models if configured
        if self.config.auto_approve_low_risk && model.risk_level == RiskLevel::Low {
            evaluation.requires_approval = false;
        }

        evaluation
    }

    /// Check compliance status for a model
    async fn check_compliance(&self, model: &DetectedAIModel) -> ComplianceStatus {
        let mut compliant = true;
        let mut issues = Vec::new();

        // Check if model is registered
        if self.config.require_registration && !model.registered {
            compliant = false;
            issues.push("Model not registered".to_string());
        }

        // Check against compliance frameworks
        for framework in &self.config.compliance_frameworks {
            match framework {
                ComplianceFramework::SOC2 => {
                    if model.risk_level == RiskLevel::Critical {
                        issues.push("SOC2: Critical risk models require additional controls".to_string());
                        compliant = false;
                    }
                }
                ComplianceFramework::GDPR => {
                    if model.risk_level >= RiskLevel::High {
                        issues.push("GDPR: High-risk models require DPIA assessment".to_string());
                    }
                }
                ComplianceFramework::ISO27001 => {
                    if !model.registered {
                        issues.push("ISO27001: Models must be registered in asset inventory".to_string());
                        compliant = false;
                    }
                }
                _ => {}
            }
        }

        ComplianceStatus {
            compliant,
            frameworks: self.config.compliance_frameworks.clone(),
            issues,
            last_checked: Utc::now(),
        }
    }

    /// Register an AI model
    pub async fn register_model(&self, request: RegistrationRequest) -> Result<String, ShadowAIError> {
        let mut registry = self.registry.write().await;

        // Check if model already registered
        if registry.models.contains_key(&request.model.model_id) {
            return Err(ShadowAIError::GovernanceError(
                "Model already registered".to_string()
            ));
        }

        // Create registered model
        let registered_model = RegisteredModel {
            model_id: request.model.model_id.clone(),
            model_name: request.model.model_name.clone(),
            model_type: request.model.model_type.clone(),
            owner: request.requester.clone(),
            registered_at: Utc::now(),
            status: RegistrationStatus::Pending,
            approved_by: None,
            approved_at: None,
            expires_at: None,
            allowed_data_sources: request.requested_data_sources.clone(),
            allowed_contexts: Vec::new(),
            usage_limits: None,
            risk_assessment: Some(ModelRiskAssessment {
                risk_score: 0.5,
                risk_level: request.model.risk_level.clone(),
                data_sensitivity_impact: 0.3,
                capabilities_impact: 0.4,
                access_control_impact: 0.3,
                notes: vec![],
                assessed_by: "system".to_string(),
                assessed_at: Utc::now(),
            }),
            certifications: Vec::new(),
            metadata: HashMap::new(),
        };

        registry.models.insert(registered_model.model_id.clone(), registered_model);
        registry.pending_requests.push(request.clone());
        registry.last_updated = Utc::now();

        Ok(request.request_id)
    }

    /// Approve a model registration
    pub async fn approve_registration(&self, request_id: &str, approver: &str) -> Result<(), ShadowAIError> {
        let mut registry = self.registry.write().await;

        // Find the pending request
        let request = registry.pending_requests.iter()
            .find(|r| r.request_id == request_id)
            .cloned()
            .ok_or_else(|| ShadowAIError::GovernanceError(
                "Registration request not found".to_string()
            ))?;

        // Update model status
        if let Some(model) = registry.models.get_mut(&request.model.model_id) {
            model.status = RegistrationStatus::Approved;
            model.approved_by = Some(approver.to_string());
            model.approved_at = Some(Utc::now());
        }

        // Update request status
        registry.pending_requests.retain(|r| r.request_id != request_id);
        registry.last_updated = Utc::now();

        // Log the approval
        self.log_enforcement(EnforcementLog {
            log_id: uuid::Uuid::new_v4().to_string(),
            timestamp: Utc::now(),
            model_id: Some(request.model.model_id),
            actor: approver.to_string(),
            action: EnforcementAction::AllowedAccess,
            rule_id: None,
            outcome: EnforcementOutcome::Success,
            details: format!("Approved registration request {}", request_id),
        }).await;

        Ok(())
    }

    /// Reject a model registration
    pub async fn reject_registration(&self, request_id: &str, reason: &str, reviewer: &str) -> Result<(), ShadowAIError> {
        let mut registry = self.registry.write().await;

        // Find the pending request
        let request = registry.pending_requests.iter()
            .find(|r| r.request_id == request_id)
            .cloned()
            .ok_or_else(|| ShadowAIError::GovernanceError(
                "Registration request not found".to_string()
            ))?;

        // Update model status
        if let Some(model) = registry.models.get_mut(&request.model.model_id) {
            model.status = RegistrationStatus::Rejected;
        }

        // Update request status
        registry.pending_requests.retain(|r| r.request_id != request_id);
        registry.last_updated = Utc::now();

        // Log the rejection
        self.log_enforcement(EnforcementLog {
            log_id: uuid::Uuid::new_v4().to_string(),
            timestamp: Utc::now(),
            model_id: Some(request.model.model_id),
            actor: reviewer.to_string(),
            action: EnforcementAction::RevokedAccess,
            rule_id: None,
            outcome: EnforcementOutcome::Success,
            details: format!("Rejected registration request {}: {}", request_id, reason),
        }).await;

        Ok(())
    }

    /// Get registered models
    pub async fn get_registered_models(&self) -> Vec<RegisteredModel> {
        let registry = self.registry.read().await;
        registry.models.values().cloned().collect()
    }

    /// Get pending registration requests
    pub async fn get_pending_requests(&self) -> Vec<RegistrationRequest> {
        let registry = self.registry.read().await;
        registry.pending_requests.clone()
    }

    /// Generate compliance report
    pub async fn generate_compliance_report(&self) -> ComplianceReport {
        let registry = self.registry.read().await;
        let policies = self.policies.read().await;

        let mut report = ComplianceReport {
            report_id: uuid::Uuid::new_v4().to_string(),
            generated_at: Utc::now(),
            framework: self.config.compliance_frameworks.clone(),
            summary: ComplianceSummary {
                total_models: registry.models.len(),
                registered_models: registry.models.values().filter(|m| matches!(m.status, RegistrationStatus::Approved)).count(),
                pending_registrations: registry.pending_requests.len(),
                blocked_models: policies.iter().filter(|p| matches!(p.rule_type, PolicyRuleType::Blocked)).count(),
            },
            model_details: registry.models.values().cloned().collect(),
            policy_details: policies.clone(),
            recommendations: self.generate_recommendations(&registry, &policies),
        };

        report
    }

    /// Generate compliance recommendations
    fn generate_recommendations(&self, registry: &AIModelRegistry, policies: &[PolicyRule]) -> Vec<String> {
        let mut recommendations = Vec::new();

        // Check for unregistered high-risk models
        let unregistered_high_risk = registry.models.values()
            .filter(|m| m.status == RegistrationStatus::Pending)
            .filter(|m| matches!(m.risk_assessment.as_ref(), Some(ra) if ra.risk_level >= RiskLevel::High))
            .count();

        if unregistered_high_risk > 0 {
            recommendations.push(format!(
                "{} high-risk models pending registration require immediate review",
                unregistered_high_risk
            ));
        }

        // Check for expired registrations
        let now = Utc::now();
        let expired = registry.models.values()
            .filter(|m| {
                m.expires_at.map(|exp| exp < now).unwrap_or(false)
            })
            .count();

        if expired > 0 {
            recommendations.push(format!("{} model registrations have expired and require renewal", expired));
        }

        // Check policy coverage
        let model_types: Vec<AIModelType> = vec![
            AIModelType::LargeLanguageModel,
            AIModelType::GenerativeAI,
            AIModelType::NeuralNetwork,
        ];

        for model_type in model_types {
            let covered = policies.iter()
                .any(|p| p.model_types.contains(&model_type) && p.enabled);

            if !covered {
                recommendations.push(format!(
                    "Consider adding specific policies for {:?} models",
                    model_type
                ));
            }
        }

        recommendations
    }

    /// Log enforcement action
    async fn log_enforcement(&self, log: EnforcementLog) {
        let mut logs = self.enforcement_logs.write().await;
        logs.push(log);

        // Prune old logs based on retention policy
        let cutoff = Utc::now() - chrono::Duration::days(self.config.audit_retention_days as i64);
        logs.retain(|log| log.timestamp > cutoff);
    }

    /// Get enforcement logs
    pub async fn get_enforcement_logs(&self) -> Vec<EnforcementLog> {
        self.enforcement_logs.read().await.clone()
    }
}

/// Policy evaluation result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PolicyEvaluationResult {
    /// Model ID
    pub model_id: String,
    /// Model name
    pub model_name: String,
    /// Evaluation ID
    pub evaluation_id: String,
    /// Evaluation timestamp
    pub evaluated_at: DateTime<Utc>,
    /// Whether registration is required
    pub requires_registration: bool,
    /// Whether approval is required
    pub requires_approval: bool,
    /// Whether usage is allowed
    pub allowed: bool,
    /// Whether usage is blocked
    pub blocked: bool,
    /// Triggered policy rules
    pub triggered_rules: Vec<PolicyRule>,
    /// Recommended action
    pub recommended_action: PolicyAction,
    /// Compliance status
    pub compliance_status: ComplianceStatus,
}

/// Compliance status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComplianceStatus {
    /// Whether compliant
    pub compliant: bool,
    /// Applicable frameworks
    pub frameworks: Vec<ComplianceFramework>,
    /// Compliance issues
    pub issues: Vec<String>,
    /// Last checked
    pub last_checked: DateTime<Utc>,
}

/// Compliance report
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComplianceReport {
    /// Report ID
    pub report_id: String,
    /// Generated at
    pub generated_at: DateTime<Utc>,
    /// Compliance framework
    pub framework: Vec<ComplianceFramework>,
    /// Summary statistics
    pub summary: ComplianceSummary,
    /// Model details
    pub model_details: Vec<RegisteredModel>,
    /// Policy details
    pub policy_details: Vec<PolicyRule>,
    /// Recommendations
    pub recommendations: Vec<String>,
}

/// Compliance summary
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComplianceSummary {
    /// Total models
    pub total_models: usize,
    /// Registered models
    pub registered_models: usize,
    /// Pending registrations
    pub pending_registrations: usize,
    /// Blocked models
    pub blocked_models: usize,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_governance_config() {
        let config = GovernanceConfig::default();
        assert!(config.require_registration);
        assert!(config.enable_enforcement);
        assert!(!config.compliance_frameworks.is_empty());
    }

    #[test]
    fn test_compliance_status_creation() {
        let status = ComplianceStatus {
            compliant: true,
            frameworks: vec![ComplianceFramework::SOC2, ComplianceFramework::GDPR],
            issues: vec![],
            last_checked: Utc::now(),
        };

        assert!(status.compliant);
        assert_eq!(status.frameworks.len(), 2);
    }
}