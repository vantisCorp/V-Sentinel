//! Governance Engine Module
//!
//! Implements AI governance policies, approval workflows, and enforcement.

use anyhow::{anyhow, Result};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tracing::{info, warn};

use super::models::{AIModel, AIModelStatus};
use super::ShadowAIStatistics;

/// Governance Engine
///
/// Manages AI governance policies and approval workflows
pub struct GovernanceEngine {
    policies: HashMap<String, AIPolicy>,
    approvals: HashMap<String, ApprovalRequest>,
    workflows: HashMap<String, ApprovalWorkflow>,
    registered_models: HashMap<String, RegisteredModel>,
    audit_log: Vec<GovernanceAuditEntry>,
}

/// AI Policy
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AIPolicy {
    /// Policy ID
    pub id: String,
    /// Policy name
    pub name: String,
    /// Description
    pub description: String,
    /// Policy type
    pub policy_type: AIPolicyType,
    /// Policy rules
    pub rules: Vec<PolicyRule>,
    /// Enforcement mode
    pub enforcement_mode: EnforcementMode,
    /// Scope (which AI models this applies to)
    pub scope: PolicyScope,
    /// Created at
    pub created_at: DateTime<Utc>,
    /// Updated at
    pub updated_at: DateTime<Utc>,
    /// Created by
    pub created_by: String,
    /// Is active
    pub is_active: bool,
}

/// Policy types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AIPolicyType {
    AllowedProviders,
    DataClassification,
    ApprovalRequired,
    UsageLimits,
    SecurityRequirements,
    Compliance,
    CostControl,
    Custom(String),
}

/// Policy rule
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PolicyRule {
    /// Rule ID
    pub id: String,
    /// Rule name
    pub name: String,
    /// Condition
    pub condition: RuleCondition,
    /// Action when condition is met
    pub action: PolicyAction,
    /// Priority
    pub priority: u32,
}

/// Rule condition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RuleCondition {
    pub field: String,
    pub operator: ConditionOperator,
    pub value: serde_json::Value,
}

/// Condition operators
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ConditionOperator {
    Equals,
    NotEquals,
    Contains,
    GreaterThan,
    LessThan,
    InList,
    Matches,
}

/// Policy actions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PolicyAction {
    Allow,
    Deny,
    RequireApproval,
    Log,
    Alert,
    Block,
    RateLimit { requests_per_minute: u32 },
    RequireDataClassification { classification: String },
}

/// Enforcement mode
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum EnforcementMode {
    Enforce,
    Audit,
    Disabled,
}

/// Policy scope
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PolicyScope {
    /// Include models matching these patterns
    pub include_patterns: Vec<String>,
    /// Exclude models matching these patterns
    pub exclude_patterns: Vec<String>,
    /// Include providers
    pub include_providers: Vec<String>,
    /// Exclude providers
    pub exclude_providers: Vec<String>,
}

/// Approval request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApprovalRequest {
    /// Request ID
    pub id: String,
    /// AI Model being requested
    pub model_id: String,
    /// Model details
    pub model_name: String,
    /// Requester
    pub requester: String,
    /// Justification
    pub justification: String,
    /// Business case
    pub business_case: String,
    /// Data types to be used
    pub data_types: Vec<String>,
    /// Requested duration
    pub requested_duration_days: Option<u32>,
    /// Status
    pub status: ApprovalStatus,
    /// Approvers
    pub approvers: Vec<Approver>,
    /// Created at
    pub created_at: DateTime<Utc>,
    /// Updated at
    pub updated_at: DateTime<Utc>,
    /// Expires at
    pub expires_at: Option<DateTime<Utc>>,
    /// Risk assessment
    pub risk_assessment: Option<RiskAssessmentSummary>,
}

/// Approval status
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum ApprovalStatus {
    Pending,
    Approved,
    Rejected,
    Expired,
    Revoked,
    UnderReview,
    Exception,
}

/// Approver
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Approver {
    pub user_id: String,
    pub role: String,
    pub decision: Option<ApprovalDecision>,
    pub decision_at: Option<DateTime<Utc>>,
    pub comments: Option<String>,
}

/// Approval decision
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum ApprovalDecision {
    Approved,
    Rejected,
    Escalated,
}

/// Risk assessment summary
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RiskAssessmentSummary {
    pub overall_risk: f64,
    pub risk_level: String,
    pub key_risks: Vec<String>,
}

/// Approval workflow
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApprovalWorkflow {
    /// Workflow ID
    pub id: String,
    /// Workflow name
    pub name: String,
    /// Description
    pub description: String,
    /// Workflow steps
    pub steps: Vec<WorkflowStep>,
    /// Triggers
    pub triggers: Vec<WorkflowTrigger>,
    /// Is active
    pub is_active: bool,
}

/// Workflow step
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkflowStep {
    pub order: u32,
    pub step_type: WorkflowStepType,
    pub approver_roles: Vec<String>,
    pub timeout_hours: u32,
    pub auto_action: Option<AutoAction>,
}

/// Workflow step types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum WorkflowStepType {
    ManagerApproval,
    SecurityReview,
    DataOfficerReview,
    LegalReview,
    ComplianceReview,
    AutomaticRiskCheck,
    Custom(String),
}

/// Auto action on timeout
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AutoAction {
    Approve,
    Reject,
    Escalate,
}

/// Workflow trigger
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum WorkflowTrigger {
    NewAIRegistration,
    HighRiskAI,
    DataClassification { classification: String },
    ProviderType { provider: String },
    Custom(String),
}

/// Registered model
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegisteredModel {
    pub model: AIModel,
    pub registration_date: DateTime<Utc>,
    pub approved_by: Option<String>,
    pub approval_date: Option<DateTime<Utc>>,
    pub expiration_date: Option<DateTime<Utc>>,
    pub status: RegistrationStatus,
    pub conditions: Vec<String>,
}

/// Registration status
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum RegistrationStatus {
    Active,
    Expired,
    Revoked,
    Suspended,
    Pending,
}

/// Governance action
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum GovernanceAction {
    Register,
    Approve,
    Reject,
    Block,
    Suspend,
    Revoke,
    Alert,
    RequestApproval,
    ExpediteReview,
    GrantException,
    UpdateConditions,
}

/// Audit entry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GovernanceAuditEntry {
    pub id: String,
    pub action: GovernanceAction,
    pub model_id: String,
    pub actor: String,
    pub timestamp: DateTime<Utc>,
    pub details: HashMap<String, serde_json::Value>,
    pub previous_state: Option<String>,
    pub new_state: Option<String>,
}

impl GovernanceEngine {
    /// Create a new Governance Engine
    pub fn new() -> Self {
        let mut engine = Self {
            policies: HashMap::new(),
            approvals: HashMap::new(),
            workflows: HashMap::new(),
            registered_models: HashMap::new(),
            audit_log: Vec::new(),
        };

        engine.add_default_policies();
        engine.add_default_workflows();
        engine
    }

    /// Add default policies
    fn add_default_policies(&mut self) {
        // Policy: Require approval for external AI providers
        let approval_policy = AIPolicy {
            id: "require-approval-external".to_string(),
            name: "Require Approval for External AI".to_string(),
            description: "All external AI services require approval before use".to_string(),
            policy_type: AIPolicyType::ApprovalRequired,
            rules: vec![PolicyRule {
                id: "external-approval-rule".to_string(),
                name: "External Provider Approval".to_string(),
                condition: RuleCondition {
                    field: "provider_type".to_string(),
                    operator: ConditionOperator::Equals,
                    value: serde_json::json!("external"),
                },
                action: PolicyAction::RequireApproval,
                priority: 100,
            }],
            enforcement_mode: EnforcementMode::Enforce,
            scope: PolicyScope {
                include_patterns: vec!["*".to_string()],
                exclude_patterns: vec![],
                include_providers: vec![],
                exclude_providers: vec!["internal".to_string()],
            },
            created_at: Utc::now(),
            updated_at: Utc::now(),
            created_by: "system".to_string(),
            is_active: true,
        };
        self.policies
            .insert(approval_policy.id.clone(), approval_policy);

        // Policy: Block sensitive data with external AI
        let sensitive_data_policy = AIPolicy {
            id: "block-sensitive-external".to_string(),
            name: "Block Sensitive Data with External AI".to_string(),
            description: "Prevent sending sensitive data to external AI services".to_string(),
            policy_type: AIPolicyType::DataClassification,
            rules: vec![
                PolicyRule {
                    id: "block-restricted".to_string(),
                    name: "Block Restricted Data".to_string(),
                    condition: RuleCondition {
                        field: "data_classification".to_string(),
                        operator: ConditionOperator::Equals,
                        value: serde_json::json!("Restricted"),
                    },
                    action: PolicyAction::Block,
                    priority: 100,
                },
                PolicyRule {
                    id: "block-topsecret".to_string(),
                    name: "Block Top Secret Data".to_string(),
                    condition: RuleCondition {
                        field: "data_classification".to_string(),
                        operator: ConditionOperator::Equals,
                        value: serde_json::json!("TopSecret"),
                    },
                    action: PolicyAction::Block,
                    priority: 100,
                },
            ],
            enforcement_mode: EnforcementMode::Enforce,
            scope: PolicyScope {
                include_patterns: vec!["*".to_string()],
                exclude_patterns: vec![],
                include_providers: vec![],
                exclude_providers: vec!["internal".to_string()],
            },
            created_at: Utc::now(),
            updated_at: Utc::now(),
            created_by: "system".to_string(),
            is_active: true,
        };
        self.policies
            .insert(sensitive_data_policy.id.clone(), sensitive_data_policy);

        // Policy: Allowed providers
        let allowed_providers_policy = AIPolicy {
            id: "allowed-providers".to_string(),
            name: "Allowed AI Providers".to_string(),
            description: "Define which AI providers are allowed".to_string(),
            policy_type: AIPolicyType::AllowedProviders,
            rules: vec![PolicyRule {
                id: "allowed-providers-rule".to_string(),
                name: "Check Provider".to_string(),
                condition: RuleCondition {
                    field: "provider".to_string(),
                    operator: ConditionOperator::InList,
                    value: serde_json::json!(["OpenAI", "Anthropic", "Azure", "Internal"]),
                },
                action: PolicyAction::Allow,
                priority: 50,
            }],
            enforcement_mode: EnforcementMode::Enforce,
            scope: PolicyScope {
                include_patterns: vec!["*".to_string()],
                exclude_patterns: vec![],
                include_providers: vec![],
                exclude_providers: vec![],
            },
            created_at: Utc::now(),
            updated_at: Utc::now(),
            created_by: "system".to_string(),
            is_active: true,
        };
        self.policies.insert(
            allowed_providers_policy.id.clone(),
            allowed_providers_policy,
        );
    }

    /// Add default workflows
    fn add_default_workflows(&mut self) {
        // Standard approval workflow
        let standard_workflow = ApprovalWorkflow {
            id: "standard-approval".to_string(),
            name: "Standard AI Approval".to_string(),
            description: "Standard approval workflow for AI services".to_string(),
            steps: vec![
                WorkflowStep {
                    order: 1,
                    step_type: WorkflowStepType::AutomaticRiskCheck,
                    approver_roles: vec![],
                    timeout_hours: 1,
                    auto_action: Some(AutoAction::Escalate),
                },
                WorkflowStep {
                    order: 2,
                    step_type: WorkflowStepType::ManagerApproval,
                    approver_roles: vec!["manager".to_string()],
                    timeout_hours: 48,
                    auto_action: Some(AutoAction::Escalate),
                },
                WorkflowStep {
                    order: 3,
                    step_type: WorkflowStepType::SecurityReview,
                    approver_roles: vec!["security-team".to_string()],
                    timeout_hours: 72,
                    auto_action: Some(AutoAction::Escalate),
                },
            ],
            triggers: vec![WorkflowTrigger::NewAIRegistration],
            is_active: true,
        };
        self.workflows
            .insert(standard_workflow.id.clone(), standard_workflow);

        // High-risk approval workflow
        let high_risk_workflow = ApprovalWorkflow {
            id: "high-risk-approval".to_string(),
            name: "High Risk AI Approval".to_string(),
            description: "Extended approval workflow for high-risk AI services".to_string(),
            steps: vec![
                WorkflowStep {
                    order: 1,
                    step_type: WorkflowStepType::AutomaticRiskCheck,
                    approver_roles: vec![],
                    timeout_hours: 1,
                    auto_action: Some(AutoAction::Escalate),
                },
                WorkflowStep {
                    order: 2,
                    step_type: WorkflowStepType::ManagerApproval,
                    approver_roles: vec!["manager".to_string()],
                    timeout_hours: 24,
                    auto_action: Some(AutoAction::Escalate),
                },
                WorkflowStep {
                    order: 3,
                    step_type: WorkflowStepType::SecurityReview,
                    approver_roles: vec!["security-team".to_string()],
                    timeout_hours: 48,
                    auto_action: Some(AutoAction::Escalate),
                },
                WorkflowStep {
                    order: 4,
                    step_type: WorkflowStepType::DataOfficerReview,
                    approver_roles: vec!["data-officer".to_string()],
                    timeout_hours: 72,
                    auto_action: Some(AutoAction::Escalate),
                },
            ],
            triggers: vec![WorkflowTrigger::HighRiskAI],
            is_active: true,
        };
        self.workflows
            .insert(high_risk_workflow.id.clone(), high_risk_workflow);
    }

    /// Register an AI model
    pub async fn register(&mut self, model: AIModel) -> Result<()> {
        info!("Registering AI model: {} ({})", model.name, model.id);

        let registered = RegisteredModel {
            model: model.clone(),
            registration_date: Utc::now(),
            approved_by: None,
            approval_date: None,
            expiration_date: None,
            status: RegistrationStatus::Pending,
            conditions: vec![],
        };

        self.registered_models.insert(model.id.clone(), registered);

        self.add_audit_entry(
            GovernanceAction::Register,
            &model.id,
            "system",
            vec![("model_name", serde_json::json!(model.name))],
        );

        Ok(())
    }

    /// Approve an AI model
    pub async fn approve(
        &mut self,
        model_id: &str,
        approver: &str,
        justification: &str,
    ) -> Result<()> {
        let registered = self
            .registered_models
            .get_mut(model_id)
            .ok_or_else(|| anyhow!("Model {} not found", model_id))?;

        registered.status = RegistrationStatus::Active;
        registered.approved_by = Some(approver.to_string());
        registered.approval_date = Some(Utc::now());
        registered.model.status = AIModelStatus::Approved;

        info!("Approved AI model {} by {}", model_id, approver);

        self.add_audit_entry(
            GovernanceAction::Approve,
            model_id,
            approver,
            vec![("justification", serde_json::json!(justification))],
        );

        Ok(())
    }

    /// Block an AI model
    pub async fn block(&mut self, model_id: &str, reason: &str) -> Result<()> {
        let registered = self
            .registered_models
            .get_mut(model_id)
            .ok_or_else(|| anyhow!("Model {} not found", model_id))?;

        registered.status = RegistrationStatus::Suspended;
        registered.model.status = AIModelStatus::Blocked;

        warn!("Blocked AI model {}: {}", model_id, reason);

        self.add_audit_entry(
            GovernanceAction::Block,
            model_id,
            "system",
            vec![("reason", serde_json::json!(reason))],
        );

        Ok(())
    }

    /// Get governance status
    pub async fn get_status(&self, model_id: &str) -> super::GovernanceStatus {
        match self.registered_models.get(model_id) {
            Some(registered) => match registered.status {
                RegistrationStatus::Active => super::GovernanceStatus::Approved,
                RegistrationStatus::Expired => super::GovernanceStatus::Unapproved,
                RegistrationStatus::Revoked => super::GovernanceStatus::Blocked,
                RegistrationStatus::Suspended => super::GovernanceStatus::Blocked,
                RegistrationStatus::Pending => super::GovernanceStatus::Pending,
            },
            None => super::GovernanceStatus::Unapproved,
        }
    }

    /// Create approval request
    pub async fn create_approval_request(&mut self, request: ApprovalRequest) -> Result<()> {
        if self.approvals.contains_key(&request.id) {
            return Err(anyhow!("Approval request {} already exists", request.id));
        }

        info!("Creating approval request for model {}", request.model_id);
        self.approvals.insert(request.id.clone(), request);
        Ok(())
    }

    /// Get statistics
    pub async fn get_statistics(&self) -> ShadowAIStatistics {
        let mut stats = ShadowAIStatistics::default();

        for registered in self.registered_models.values() {
            stats.total_discovered += 1;
            match registered.status {
                RegistrationStatus::Active => stats.approved_count += 1,
                RegistrationStatus::Pending => stats.pending_count += 1,
                RegistrationStatus::Suspended | RegistrationStatus::Revoked => {
                    stats.blocked_count += 1
                }
                RegistrationStatus::Expired => stats.unapproved_count += 1,
            }
        }

        stats
    }

    /// Add audit entry
    fn add_audit_entry(
        &mut self,
        action: GovernanceAction,
        model_id: &str,
        actor: &str,
        details: Vec<(&str, serde_json::Value)>,
    ) {
        let entry = GovernanceAuditEntry {
            id: uuid::Uuid::new_v4().to_string(),
            action,
            model_id: model_id.to_string(),
            actor: actor.to_string(),
            timestamp: Utc::now(),
            details: details
                .into_iter()
                .map(|(k, v)| (k.to_string(), v))
                .collect(),
            previous_state: None,
            new_state: None,
        };

        self.audit_log.push(entry);
    }

    /// Get policies
    pub fn get_policies(&self) -> Vec<&AIPolicy> {
        self.policies.values().collect()
    }

    /// Get registered models
    pub fn get_registered_models(&self) -> Vec<&RegisteredModel> {
        self.registered_models.values().collect()
    }

    /// Get audit log
    pub fn get_audit_log(&self, limit: usize) -> Vec<&GovernanceAuditEntry> {
        self.audit_log.iter().rev().take(limit).collect()
    }
}

impl Default for GovernanceEngine {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_governance_engine_creation() {
        let engine = GovernanceEngine::new();
        assert!(!engine.policies.is_empty());
        assert!(!engine.workflows.is_empty());
    }
}
