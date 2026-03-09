//! Policy Engine Module
//!
//! Implements the Policy Decision Point (PDP) and Policy Administration Point (PAP)
//! following XACML and NIST SP 800-207 guidelines.

use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tracing::{debug, info};

use super::{AccessRequest, Action, RequestContext, Resource, Subject};

/// Policy Engine
///
/// Central Policy Decision Point (PDP) that evaluates access requests
/// against defined policies.
pub struct PolicyEngine {
    policies: HashMap<String, Policy>,
    policy_sets: HashMap<String, PolicySet>,
    default_decision: DecisionType,
    statistics: PolicyStatistics,
}

/// Policy Set (collection of policies)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PolicySet {
    pub id: String,
    pub name: String,
    pub description: String,
    pub policies: Vec<String>,
    pub combining_algorithm: PolicyCombiningAlgorithm,
    pub target: Target,
}

/// Policy definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Policy {
    pub id: String,
    pub name: String,
    pub description: String,
    pub version: String,
    pub target: Target,
    pub rules: Vec<Rule>,
    pub combining_algorithm: RuleCombiningAlgorithm,
    pub obligations: Vec<Obligation>,
    pub status: PolicyStatus,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

/// Policy target (what the policy applies to)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Target {
    pub subjects: Vec<SubjectMatch>,
    pub resources: Vec<ResourceMatch>,
    pub actions: Vec<ActionMatch>,
    pub environments: Vec<EnvironmentMatch>,
}

/// Subject matching criteria
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubjectMatch {
    pub attribute: String,
    pub matcher: MatchOperator,
    pub value: serde_json::Value,
}

/// Resource matching criteria
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceMatch {
    pub attribute: String,
    pub matcher: MatchOperator,
    pub value: serde_json::Value,
}

/// Action matching criteria
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActionMatch {
    pub action_type: Option<super::ActionType>,
    pub attribute: Option<String>,
    pub matcher: Option<MatchOperator>,
    pub value: Option<serde_json::Value>,
}

/// Environment matching criteria
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnvironmentMatch {
    pub attribute: String,
    pub matcher: MatchOperator,
    pub value: serde_json::Value,
}

/// Match operators
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MatchOperator {
    Equals,
    NotEquals,
    Contains,
    StartsWith,
    EndsWith,
    GreaterThan,
    LessThan,
    GreaterThanOrEqual,
    LessThanOrEqual,
    InList,
    Regex,
    IPInRange,
    TimeInRange,
}

/// Policy rule
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Rule {
    pub id: String,
    pub name: String,
    pub description: String,
    pub target: Option<Target>,
    pub condition: Option<Condition>,
    pub effect: Effect,
    pub obligations: Vec<Obligation>,
}

/// Rule effect
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum Effect {
    Permit,
    Deny,
}

/// Condition expression
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Condition {
    pub expression: Expression,
}

/// Expression for conditions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Expression {
    Literal(serde_json::Value),
    Attribute(String),
    Function {
        name: String,
        arguments: Vec<Expression>,
    },
    And(Vec<Expression>),
    Or(Vec<Expression>),
    Not(Box<Expression>),
    Equals(Box<Expression>, Box<Expression>),
    GreaterThan(Box<Expression>, Box<Expression>),
    LessThan(Box<Expression>, Box<Expression>),
    Contains(Box<Expression>, Box<Expression>),
}

/// Policy combining algorithms
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PolicyCombiningAlgorithm {
    DenyOverrides,
    PermitOverrides,
    FirstApplicable,
    OnlyOneApplicable,
    OrderedDenyOverrides,
    OrderedPermitOverrides,
}

/// Rule combining algorithms
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RuleCombiningAlgorithm {
    DenyOverrides,
    PermitOverrides,
    FirstApplicable,
    OrderedDenyOverrides,
    OrderedPermitOverrides,
}

/// Obligation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Obligation {
    pub id: String,
    pub obligation_type: ObligationType,
    pub parameters: HashMap<String, serde_json::Value>,
    pub fulfill_on: Vec<Effect>,
}

/// Obligation types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ObligationType {
    Reauthenticate,
    ProvideMFA,
    ApproveViaManager,
    CompleteTraining,
    AcknowledgePolicy,
    LogAccess,
    NotifyAdmin,
    EncryptData,
    ApplyWatermark,
}

/// Policy status
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PolicyStatus {
    Active,
    Inactive,
    Draft,
    Deprecated,
}

/// Decision types
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub enum DecisionType {
    Allow,
    Deny,
    Challenge,
    NotApplicable,
    Indeterminate,
}

/// Policy decision result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PolicyDecision {
    pub decision: DecisionType,
    pub applied_policies: Vec<String>,
    pub matched_rules: Vec<String>,
    pub obligations: Vec<Obligation>,
    pub recommendations: Vec<String>,
    pub evaluation_time_ms: u64,
}

/// Policy statistics
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PolicyStatistics {
    pub total_evaluations: u64,
    pub allow_count: u64,
    pub deny_count: u64,
    pub challenge_count: u64,
    pub average_evaluation_time_ms: f64,
}

impl PolicyEngine {
    /// Create a new Policy Engine
    pub fn new() -> Self {
        let mut engine = Self {
            policies: HashMap::new(),
            policy_sets: HashMap::new(),
            default_decision: DecisionType::Deny,
            statistics: PolicyStatistics::default(),
        };

        // Add default policies
        engine.add_default_policies();
        engine
    }

    /// Add default security policies
    fn add_default_policies(&mut self) {
        // Default deny-all policy for sensitive resources
        let deny_sensitive = Policy {
            id: "default-deny-sensitive".to_string(),
            name: "Deny Access to Sensitive Resources".to_string(),
            description: "Default deny policy for highly sensitive resources".to_string(),
            version: "1.0".to_string(),
            target: Target {
                subjects: vec![],
                resources: vec![ResourceMatch {
                    attribute: "sensitivity".to_string(),
                    matcher: MatchOperator::Equals,
                    value: serde_json::json!("TopSecret"),
                }],
                actions: vec![],
                environments: vec![],
            },
            rules: vec![Rule {
                id: "deny-rule".to_string(),
                name: "Deny All".to_string(),
                description: "Deny all access to top secret resources".to_string(),
                target: None,
                condition: None,
                effect: Effect::Deny,
                obligations: vec![],
            }],
            combining_algorithm: RuleCombiningAlgorithm::FirstApplicable,
            obligations: vec![],
            status: PolicyStatus::Active,
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
        };

        self.policies
            .insert(deny_sensitive.id.clone(), deny_sensitive);

        // Require MFA for administrative actions
        let require_mfa_admin = Policy {
            id: "require-mfa-admin".to_string(),
            name: "Require MFA for Admin Actions".to_string(),
            description: "MFA required for any administrative action".to_string(),
            version: "1.0".to_string(),
            target: Target {
                subjects: vec![],
                resources: vec![],
                actions: vec![ActionMatch {
                    action_type: Some(super::ActionType::Admin),
                    attribute: None,
                    matcher: None,
                    value: None,
                }],
                environments: vec![],
            },
            rules: vec![Rule {
                id: "mfa-rule".to_string(),
                name: "Require MFA".to_string(),
                description: "Challenge for MFA on admin actions".to_string(),
                target: None,
                condition: None,
                effect: Effect::Permit,
                obligations: vec![Obligation {
                    id: "mfa-obligation".to_string(),
                    obligation_type: ObligationType::ProvideMFA,
                    parameters: HashMap::new(),
                    fulfill_on: vec![Effect::Permit],
                }],
            }],
            combining_algorithm: RuleCombiningAlgorithm::FirstApplicable,
            obligations: vec![],
            status: PolicyStatus::Active,
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
        };

        self.policies
            .insert(require_mfa_admin.id.clone(), require_mfa_admin);

        // Block access from unmanaged devices to sensitive resources
        let block_unmanaged = Policy {
            id: "block-unmanaged-devices".to_string(),
            name: "Block Unmanaged Devices".to_string(),
            description: "Deny access from unmanaged devices to sensitive resources".to_string(),
            version: "1.0".to_string(),
            target: Target {
                subjects: vec![SubjectMatch {
                    attribute: "device.is_managed".to_string(),
                    matcher: MatchOperator::Equals,
                    value: serde_json::json!(false),
                }],
                resources: vec![ResourceMatch {
                    attribute: "sensitivity".to_string(),
                    matcher: MatchOperator::InList,
                    value: serde_json::json!(["Confidential", "Restricted", "TopSecret"]),
                }],
                actions: vec![],
                environments: vec![],
            },
            rules: vec![Rule {
                id: "block-unmanaged-rule".to_string(),
                name: "Block Unmanaged".to_string(),
                description: "Block access from unmanaged devices".to_string(),
                target: None,
                condition: None,
                effect: Effect::Deny,
                obligations: vec![],
            }],
            combining_algorithm: RuleCombiningAlgorithm::FirstApplicable,
            obligations: vec![],
            status: PolicyStatus::Active,
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
        };

        self.policies
            .insert(block_unmanaged.id.clone(), block_unmanaged);
    }

    /// Add a policy
    pub fn add_policy(&mut self, policy: Policy) -> Result<()> {
        if self.policies.contains_key(&policy.id) {
            return Err(anyhow!("Policy with ID {} already exists", policy.id));
        }

        info!("Adding policy: {} ({})", policy.name, policy.id);
        self.policies.insert(policy.id.clone(), policy);
        Ok(())
    }

    /// Remove a policy
    pub fn remove_policy(&mut self, policy_id: &str) -> Result<()> {
        if self.policies.remove(policy_id).is_none() {
            return Err(anyhow!("Policy with ID {} not found", policy_id));
        }

        info!("Removed policy: {}", policy_id);
        Ok(())
    }

    /// Update a policy
    pub fn update_policy(&mut self, policy: Policy) -> Result<()> {
        if !self.policies.contains_key(&policy.id) {
            return Err(anyhow!("Policy with ID {} not found", policy.id));
        }

        info!("Updating policy: {} ({})", policy.name, policy.id);
        self.policies.insert(policy.id.clone(), policy);
        Ok(())
    }

    /// Get a policy by ID
    pub fn get_policy(&self, policy_id: &str) -> Option<&Policy> {
        self.policies.get(policy_id)
    }

    /// Evaluate an access request against all applicable policies
    pub async fn evaluate(&self, request: &AccessRequest) -> Result<PolicyDecision> {
        let start = std::time::Instant::now();

        info!("Evaluating access request {} against policies", request.id);

        let mut applicable_policies: Vec<&Policy> = Vec::new();
        let mut matched_rules: Vec<String> = Vec::new();
        let mut obligations: Vec<Obligation> = Vec::new();
        let mut recommendations: Vec<String> = Vec::new();

        // Find applicable policies
        for policy in self.policies.values() {
            if policy.status != PolicyStatus::Active {
                continue;
            }

            if self.is_policy_applicable(policy, request) {
                debug!("Policy {} is applicable", policy.id);
                applicable_policies.push(policy);
            }
        }

        // Evaluate applicable policies using deny-overrides algorithm
        let mut final_decision = DecisionType::Allow;

        for policy in &applicable_policies {
            let (rule_decision, rule_id) = self.evaluate_rules(policy, request).await?;

            if let Some(rid) = rule_id {
                matched_rules.push(rid);
            }

            match rule_decision {
                DecisionType::Deny => {
                    final_decision = DecisionType::Deny;
                    recommendations.push(format!("Denied by policy: {}", policy.name));
                }
                DecisionType::Challenge => {
                    if final_decision != DecisionType::Deny {
                        final_decision = DecisionType::Challenge;
                    }
                }
                DecisionType::Allow => {
                    // Collect obligations from allow rules
                    obligations.extend(policy.obligations.clone());
                }
                _ => {}
            }
        }

        // If no applicable policies, use default decision
        if applicable_policies.is_empty() {
            final_decision = self.default_decision;
            recommendations.push("No applicable policy found, using default".to_string());
        }

        let evaluation_time = start.elapsed().as_millis() as u64;

        Ok(PolicyDecision {
            decision: final_decision,
            applied_policies: applicable_policies.iter().map(|p| p.id.clone()).collect(),
            matched_rules,
            obligations,
            recommendations,
            evaluation_time_ms: evaluation_time,
        })
    }

    /// Check if a policy is applicable to a request
    fn is_policy_applicable(&self, policy: &Policy, request: &AccessRequest) -> bool {
        // Check subject matches
        if !self.matches_subjects(&policy.target.subjects, &request.subject) {
            return false;
        }

        // Check resource matches
        if !self.matches_resources(&policy.target.resources, &request.resource) {
            return false;
        }

        // Check action matches
        if !self.matches_actions(&policy.target.actions, &request.action) {
            return false;
        }

        // Check environment matches
        if !self.matches_environments(&policy.target.environments, &request.context) {
            return false;
        }

        true
    }

    /// Check subject matches
    fn matches_subjects(&self, matches: &[SubjectMatch], subject: &Subject) -> bool {
        if matches.is_empty() {
            return true;
        }

        matches.iter().all(|m| {
            let value = subject.attributes.get(&m.attribute);
            self.match_value(value, &m.matcher, &m.value)
        })
    }

    /// Check resource matches
    fn matches_resources(&self, matches: &[ResourceMatch], resource: &Resource) -> bool {
        if matches.is_empty() {
            return true;
        }

        matches.iter().all(|m| {
            let value = match m.attribute.as_str() {
                "sensitivity" => Some(serde_json::to_value(resource.sensitivity).unwrap()),
                "resource_type" => Some(serde_json::to_value(&resource.resource_type).unwrap()),
                "owner" => resource.owner.as_ref().map(|v| serde_json::json!(v)),
                _ => resource.attributes.get(&m.attribute).cloned(),
            };
            self.match_value(value.as_ref(), &m.matcher, &m.value)
        })
    }

    /// Check action matches
    fn matches_actions(&self, matches: &[ActionMatch], action: &Action) -> bool {
        if matches.is_empty() {
            return true;
        }

        matches.iter().all(|m| {
            if let Some(ref action_type) = m.action_type {
                return action.action_type == *action_type;
            }
            true
        })
    }

    /// Check environment matches
    fn matches_environments(
        &self,
        matches: &[EnvironmentMatch],
        _context: &RequestContext,
    ) -> bool {
        if matches.is_empty() {
            return true;
        }

        // For now, return true - would implement full environment matching
        true
    }

    /// Match a value against a pattern
    fn match_value(
        &self,
        value: Option<&serde_json::Value>,
        matcher: &MatchOperator,
        pattern: &serde_json::Value,
    ) -> bool {
        let value = match value {
            Some(v) => v,
            None => return false,
        };

        match matcher {
            MatchOperator::Equals => value == pattern,
            MatchOperator::NotEquals => value != pattern,
            MatchOperator::Contains => {
                if let Some(arr) = value.as_array() {
                    if let Some(s) = pattern.as_str() {
                        arr.contains(&serde_json::json!(s))
                    } else {
                        false
                    }
                } else if let (serde_json::Value::String(v), serde_json::Value::String(p)) =
                    (value, pattern)
                {
                    v.contains(p)
                } else {
                    false
                }
            }
            MatchOperator::InList => {
                if let Some(arr) = pattern.as_array() {
                    arr.contains(value)
                } else {
                    false
                }
            }
            _ => value == pattern, // Simplified for other matchers
        }
    }

    /// Evaluate rules within a policy
    async fn evaluate_rules(
        &self,
        policy: &Policy,
        request: &AccessRequest,
    ) -> Result<(DecisionType, Option<String>)> {
        for rule in &policy.rules {
            // Check rule target
            if let Some(ref target) = rule.target {
                if !self.matches_subjects(&target.subjects, &request.subject)
                    || !self.matches_resources(&target.resources, &request.resource)
                    || !self.matches_actions(&target.actions, &request.action)
                {
                    continue;
                }
            }

            // Check rule condition
            if let Some(ref condition) = rule.condition {
                if !self
                    .evaluate_condition(&condition.expression, request)
                    .await?
                {
                    continue;
                }
            }

            // Rule matches
            let decision = match rule.effect {
                Effect::Permit => {
                    if !rule.obligations.is_empty() {
                        DecisionType::Challenge
                    } else {
                        DecisionType::Allow
                    }
                }
                Effect::Deny => DecisionType::Deny,
            };

            return Ok((decision, Some(rule.id.clone())));
        }

        Ok((DecisionType::NotApplicable, None))
    }

    /// Evaluate a condition expression
    fn evaluate_condition<'a>(
        &'a self,
        expression: &'a Expression,
        request: &'a AccessRequest,
    ) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<bool>> + Send + 'a>> {
        Box::pin(async move {
            match expression {
                Expression::Literal(v) => Ok(v.as_bool().unwrap_or(false)),
                Expression::Attribute(path) => {
                    // Resolve attribute from request
                    Ok(self.resolve_attribute(path, request))
                }
                Expression::And(expressions) => {
                    for expr in expressions {
                        if !self.evaluate_condition(expr, request).await? {
                            return Ok(false);
                        }
                    }
                    Ok(true)
                }
                Expression::Or(expressions) => {
                    for expr in expressions {
                        if self.evaluate_condition(expr, request).await? {
                            return Ok(true);
                        }
                    }
                    Ok(false)
                }
                Expression::Not(expr) => Ok(!self.evaluate_condition(expr, request).await?),
                Expression::Equals(_left, _right) => {
                    // Simplified equality check
                    Ok(false)
                }
                _ => Ok(false),
            }
        })
    }

    /// Resolve an attribute path from the request
    fn resolve_attribute(&self, _path: &str, _request: &AccessRequest) -> bool {
        // Simplified attribute resolution
        false
    }

    /// Get policy count
    pub fn policy_count(&self) -> usize {
        self.policies.len()
    }

    /// List all policies
    pub fn list_policies(&self) -> Vec<&Policy> {
        self.policies.values().collect()
    }
}

impl Default for PolicyEngine {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{ActionType, ResourceType, SensitivityLevel, SubjectType};

    #[test]
    fn test_policy_engine_creation() {
        let engine = PolicyEngine::new();
        assert!(!engine.policies.is_empty());
    }

    #[tokio::test]
    async fn test_policy_evaluation() {
        let engine = PolicyEngine::new();
        let request = AccessRequest {
            id: uuid::Uuid::new_v4(),
            subject: Subject {
                id: "user-123".to_string(),
                subject_type: SubjectType::User,
                attributes: HashMap::new(),
                identity_provider: None,
                device: None,
            },
            resource: Resource {
                id: "resource-1".to_string(),
                resource_type: ResourceType::Data,
                attributes: HashMap::new(),
                sensitivity: SensitivityLevel::Internal,
                owner: None,
                segment: None,
            },
            action: Action {
                action_type: ActionType::Read,
                details: HashMap::new(),
            },
            context: RequestContext {
                source_ip: "192.168.1.1".to_string(),
                geo_location: None,
                access_time: chrono::Utc::now(),
                network_segment: None,
                session_id: None,
                session_request_count: 0,
                risk_indicators: vec![],
            },
            timestamp: chrono::Utc::now(),
        };

        let decision = engine.evaluate(&request).await.unwrap();
        assert!(
            decision.decision == DecisionType::Allow || decision.decision == DecisionType::Deny
        );
    }
}
