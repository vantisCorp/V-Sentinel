//! Policy engine for Zero Trust access control decisions

use super::super::{ZeroTrustContext, AccessDecision, AccessResult, TrustLevel};
use crate::trust::trust_score::TrustScore;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Policy engine for evaluating access requests
pub struct PolicyEngine {
    /// Registered policies
    policies: HashMap<String, Policy>,
    
    /// Default policy when no matching policy found
    default_policy: Policy,
}

impl PolicyEngine {
    /// Create a new policy engine with default deny policy
    pub fn new() -> Self {
        let default_policy = Policy {
            id: "default-deny".to_string(),
            name: "Default Deny Policy".to_string(),
            description: "Deny access by default when no matching policy found".to_string(),
            policy_type: PolicyType::Access,
            conditions: vec![],
            actions: vec![PolicyAction::Deny],
            priority: 0,
            enabled: true,
        };
        
        Self {
            policies: HashMap::new(),
            default_policy,
        }
    }
    
    /// Register a policy
    pub fn register_policy(&mut self, policy: Policy) {
        self.policies.insert(policy.id.clone(), policy);
    }
    
    /// Remove a policy
    pub fn remove_policy(&mut self, policy_id: &str) -> Option<Policy> {
        self.policies.remove(policy_id)
    }
    
    /// Evaluate access request against all policies
    pub fn evaluate(
        &self,
        context: &ZeroTrustContext,
        trust_score: TrustScore,
        trust_level: TrustLevel,
    ) -> AccessResult {
        // Get matching policies sorted by priority
        let mut matching_policies: Vec<&Policy> = self.policies
            .values()
            .filter(|p| p.enabled && self.matches_context(p, context))
            .collect();
        
        matching_policies.sort_by(|a, b| b.priority.cmp(&a.priority));
        
        // If no matching policy, use default
        if matching_policies.is_empty() {
            return self.apply_default(context, trust_score, trust_level);
        }
        
        // Evaluate first (highest priority) matching policy
        let policy = matching_policies[0];
        self.apply_policy(policy, context, trust_score, trust_level)
    }
    
    /// Check if policy matches context
    fn matches_context(&self, policy: &Policy, context: &ZeroTrustContext) -> bool {
        // Check subject pattern
        if !policy.conditions.iter().all(|c| c.matches(context)) {
            return false;
        }
        true
    }
    
    /// Apply policy to context
    fn apply_policy(
        &self,
        policy: &Policy,
        context: &ZeroTrustContext,
        trust_score: TrustScore,
        trust_level: TrustLevel,
    ) -> AccessResult {
        let decision = self.determine_decision(policy, trust_score);
        
        AccessResult {
            decision,
            trust_score,
            trust_level,
            reason: format!("Policy '{}' evaluated", policy.name),
            required_actions: self.get_required_actions(policy, trust_score),
            timestamp: chrono::Utc::now(),
        }
    }
    
    /// Determine access decision based on policy and trust score
    fn determine_decision(&self, policy: &Policy, trust_score: TrustScore) -> AccessDecision {
        // Check trust score thresholds
        if trust_score < 0.25 {
            return AccessDecision::Deny;
        }
        
        if trust_score < 0.5 {
            return AccessDecision::RequireMfa;
        }
        
        // Apply policy actions
        for action in &policy.actions {
            match action {
                PolicyAction::Allow => return AccessDecision::Allow,
                PolicyAction::Deny => return AccessDecision::Deny,
                PolicyAction::RequireMfa => return AccessDecision::RequireMfa,
                PolicyAction::RequireStepUp => return AccessDecision::RequireStepUp,
                _ => {}
            }
        }
        
        AccessDecision::Deny
    }
    
    /// Get required actions for access
    fn get_required_actions(&self, policy: &Policy, trust_score: TrustScore) -> Vec<String> {
        let mut actions = Vec::new();
        
        if trust_score < 0.75 {
            actions.push("Verify identity with MFA".to_string());
        }
        
        if trust_score < 0.5 {
            actions.push("Complete step-up authentication".to_string());
        }
        
        for action in &policy.actions {
            match action {
                PolicyAction::LogAccess => actions.push("Log access event".to_string()),
                PolicyAction::Notify => actions.push("Notify security team".to_string()),
                _ => {}
            }
        }
        
        actions
    }
    
    /// Apply default policy
    fn apply_default(
        &self,
        context: &ZeroTrustContext,
        trust_score: TrustScore,
        trust_level: TrustLevel,
    ) -> AccessResult {
        AccessResult {
            decision: AccessDecision::Deny,
            trust_score,
            trust_level,
            reason: "No matching policy found - default deny".to_string(),
            required_actions: vec!["Contact administrator for access".to_string()],
            timestamp: chrono::Utc::now(),
        }
    }
}

impl Default for PolicyEngine {
    fn default() -> Self {
        Self::new()
    }
}

/// Policy definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Policy {
    /// Unique policy identifier
    pub id: String,
    
    /// Policy name
    pub name: String,
    
    /// Policy description
    pub description: String,
    
    /// Policy type
    pub policy_type: PolicyType,
    
    /// Conditions for policy to apply
    pub conditions: Vec<PolicyCondition>,
    
    /// Actions to take
    pub actions: Vec<PolicyAction>,
    
    /// Policy priority (higher = more important)
    pub priority: i32,
    
    /// Is policy enabled
    pub enabled: bool,
}

impl Policy {
    /// Create a new policy
    pub fn new(
        id: impl Into<String>,
        name: impl Into<String>,
        policy_type: PolicyType,
    ) -> Self {
        Self {
            id: id.into(),
            name: name.into(),
            description: String::new(),
            policy_type,
            conditions: Vec::new(),
            actions: Vec::new(),
            priority: 100,
            enabled: true,
        }
    }
    
    /// Add description
    pub fn with_description(mut self, description: impl Into<String>) -> Self {
        self.description = description.into();
        self
    }
    
    /// Add condition
    pub fn with_condition(mut self, condition: PolicyCondition) -> Self {
        self.conditions.push(condition);
        self
    }
    
    /// Add action
    pub fn with_action(mut self, action: PolicyAction) -> Self {
        self.actions.push(action);
        self
    }
    
    /// Set priority
    pub fn with_priority(mut self, priority: i32) -> Self {
        self.priority = priority;
        self
    }
}

/// Policy type classification
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PolicyType {
    /// Access control policy
    Access,
    /// Data handling policy
    Data,
    /// Network traffic policy
    Network,
    /// Device requirements policy
    Device,
    /// Behavioral policy
    Behavioral,
}

/// Policy condition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PolicyCondition {
    /// Condition type
    pub condition_type: ConditionType,
    
    /// Operator
    pub operator: Operator,
    
    /// Expected value
    pub value: serde_json::Value,
}

impl PolicyCondition {
    /// Create a new condition
    pub fn new(
        condition_type: ConditionType,
        operator: Operator,
        value: impl Into<serde_json::Value>,
    ) -> Self {
        Self {
            condition_type,
            operator,
            value: value.into(),
        }
    }
    
    /// Check if condition matches context
    pub fn matches(&self, context: &ZeroTrustContext) -> bool {
        match &self.condition_type {
            ConditionType::Subject => {
                self.compare_string(&context.subject)
            }
            ConditionType::Resource => {
                self.compare_string(&context.resource)
            }
            ConditionType::Action => {
                self.compare_string(&context.action)
            }
            ConditionType::DeviceType => {
                self.compare_string(&format!("{:?}", context.device.device_type))
            }
            ConditionType::NetworkType => {
                self.compare_string(&format!("{:?}", context.network.network_type))
            }
            ConditionType::TrustedDevice => {
                self.compare_bool(context.device.is_trusted)
            }
            ConditionType::TrustedNetwork => {
                self.compare_bool(context.network.is_trusted)
            }
            _ => false,
        }
    }
    
    fn compare_string(&self, value: &str) -> bool {
        match &self.operator {
            Operator::Equals => {
                if let Some(expected) = self.value.as_str() {
                    value == expected
                } else {
                    false
                }
            }
            Operator::NotEquals => {
                if let Some(expected) = self.value.as_str() {
                    value != expected
                } else {
                    true
                }
            }
            Operator::Contains => {
                if let Some(expected) = self.value.as_str() {
                    value.contains(expected)
                } else {
                    false
                }
            }
            Operator::Matches => {
                if let Some(pattern) = self.value.as_str() {
                    regex::Regex::new(pattern)
                        .map(|re| re.is_match(value))
                        .unwrap_or(false)
                } else {
                    false
                }
            }
            _ => false,
        }
    }
    
    fn compare_bool(&self, value: bool) -> bool {
        match &self.operator {
            Operator::Equals => {
                self.value.as_bool().map(|v| v == value).unwrap_or(false)
            }
            _ => false,
        }
    }
}

/// Condition types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ConditionType {
    Subject,
    Resource,
    Action,
    DeviceType,
    NetworkType,
    TrustedDevice,
    TrustedNetwork,
    Location,
    TimeOfDay,
}

/// Comparison operators
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum Operator {
    Equals,
    NotEquals,
    Contains,
    NotContains,
    Matches,
    NotMatches,
    GreaterThan,
    LessThan,
}

/// Policy actions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PolicyAction {
    /// Allow access
    Allow,
    /// Deny access
    Deny,
    /// Require MFA
    RequireMfa,
    /// Require step-up authentication
    RequireStepUp,
    /// Log access event
    LogAccess,
    /// Notify security team
    Notify,
    /// Apply rate limiting
    RateLimit { requests_per_minute: u32 },
    /// Apply data masking
    MaskData,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{DeviceType, NetworkType, DeviceInfo, NetworkInfo, LocationInfo};
    use chrono::Utc;

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

    #[test]
    fn test_policy_engine_creation() {
        let engine = PolicyEngine::new();
        assert!(engine.policies.is_empty());
    }

    #[test]
    fn test_policy_registration() {
        let mut engine = PolicyEngine::new();
        
        let policy = Policy::new("test-policy", "Test Policy", PolicyType::Access)
            .with_action(PolicyAction::Allow);
        
        engine.register_policy(policy);
        
        assert_eq!(engine.policies.len(), 1);
    }

    #[test]
    fn test_default_deny() {
        let engine = PolicyEngine::new();
        let context = create_test_context();
        
        let result = engine.evaluate(&context, 0.8, TrustLevel::High);
        
        assert_eq!(result.decision, AccessDecision::Deny);
    }

    #[test]
    fn test_allow_policy() {
        let mut engine = PolicyEngine::new();
        
        let policy = Policy::new("allow-api", "Allow API Access", PolicyType::Access)
            .with_condition(PolicyCondition::new(
                ConditionType::Resource,
                Operator::Contains,
                "/api/",
            ))
            .with_action(PolicyAction::Allow);
        
        engine.register_policy(policy);
        
        let context = create_test_context();
        let result = engine.evaluate(&context, 0.8, TrustLevel::High);
        
        assert_eq!(result.decision, AccessDecision::Allow);
    }

    #[test]
    fn test_low_trust_deny() {
        let mut engine = PolicyEngine::new();
        
        let policy = Policy::new("allow-api", "Allow API Access", PolicyType::Access)
            .with_action(PolicyAction::Allow);
        
        engine.register_policy(policy);
        
        let context = create_test_context();
        let result = engine.evaluate(&context, 0.2, TrustLevel::VeryLow);
        
        assert_eq!(result.decision, AccessDecision::Deny);
    }
}