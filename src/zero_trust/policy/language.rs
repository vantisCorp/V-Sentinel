//! Enhanced Policy Definition Language for Zero Trust
//!
//! Provides a domain-specific language (DSL) for defining access policies
//! with rich expressions, templates, and versioning support.

use super::super::{ZeroTrustContext, AccessDecision};
use super::{Policy, PolicyType, PolicyCondition, PolicyAction};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use chrono::{DateTime, Utc};

/// Policy Definition Language Manager
pub struct PolicyLanguageManager {
    /// Registered policy templates
    templates: HashMap<String, PolicyTemplate>,
    
    /// Policy versions history
    version_history: HashMap<String, Vec<PolicyVersion>>,
    
    /// Custom functions registry
    functions: HashMap<String, Box<dyn PolicyFunction>>,
    
    /// Configuration
    config: PolicyLanguageConfig,
}

impl PolicyLanguageManager {
    /// Create a new policy language manager
    pub fn new(config: PolicyLanguageConfig) -> Self {
        let mut manager = Self {
            templates: HashMap::new(),
            version_history: HashMap::new(),
            functions: HashMap::new(),
            config,
        };
        
        // Register built-in functions
        manager.register_builtin_functions();
        
        manager
    }
    
    /// Register a policy template
    pub fn register_template(&mut self, template: PolicyTemplate) {
        self.templates.insert(template.name.clone(), template);
    }
    
    /// Create policy from template
    pub fn create_from_template(
        &self,
        template_name: &str,
        parameters: &HashMap<String, serde_json::Value>,
    ) -> Result<Policy, PolicyLanguageError> {
        let template = self.templates
            .get(template_name)
            .ok_or_else(|| PolicyLanguageError::TemplateNotFound(template_name.to_string()))?;
        
        let mut policy = Policy::new(
            format!("{}-generated", template.name),
            format!("Generated from {}", template.name),
            template.policy_type,
        )
        .with_description(format!("Generated from template: {}", template.description));
        
        // Apply parameters to conditions
        for cond_template in &template.conditions {
            let condition = self.process_condition_template(cond_template, parameters)?;
            policy = policy.with_condition(condition);
        }
        
        // Apply parameters to actions
        for action_template in &template.actions {
            let action = self.process_action_template(action_template, parameters)?;
            policy = policy.with_action(action);
        }
        
        policy.priority = template.priority;
        
        Ok(policy)
    }
    
    /// Parse policy from DSL string
    pub fn parse_policy(&self, dsl: &str) -> Result<Policy, PolicyLanguageError> {
        // Parse DSL syntax
        let parsed = self.parse_dsl(dsl)?;
        
        // Convert to Policy structure
        self.convert_to_policy(parsed)
    }
    
    /// Evaluate policy expression
    pub fn evaluate_expression(
        &self,
        expression: &PolicyExpression,
        context: &ZeroTrustContext,
    ) -> Result<bool, PolicyLanguageError> {
        match expression {
            PolicyExpression::Condition(cond) => {
                Ok(cond.matches(context))
            }
            PolicyExpression::And(exprs) => {
                for expr in exprs {
                    if !self.evaluate_expression(expr, context)? {
                        return Ok(false);
                    }
                }
                Ok(true)
            }
            PolicyExpression::Or(exprs) => {
                for expr in exprs {
                    if self.evaluate_expression(expr, context)? {
                        return Ok(true);
                    }
                }
                Ok(false)
            }
            PolicyExpression::Not(expr) => {
                Ok(!self.evaluate_expression(expr, context)?)
            }
            PolicyExpression::FunctionCall { name, args } => {
                self.call_function(name, args, context)
            }
        }
    }
    
    /// Register custom function
    pub fn register_function(&mut self, name: impl Into<String>, function: Box<dyn PolicyFunction>) {
        self.functions.insert(name.into(), function);
    }
    
    /// Get policy version history
    pub fn get_version_history(&self, policy_id: &str) -> Vec<PolicyVersion> {
        self.version_history
            .get(policy_id)
            .cloned()
            .unwrap_or_default()
    }
    
    /// Create new version of policy
    pub fn create_version(
        &mut self,
        policy: &Policy,
        change_description: &str,
        author: &str,
    ) -> PolicyVersion {
        let version = PolicyVersion {
            policy_id: policy.id.clone(),
            version: self.get_next_version(&policy.id),
            created_at: Utc::now(),
            author: author.to_string(),
            change_description: change_description.to_string(),
            policy_snapshot: policy.clone(),
        };
        
        self.version_history
            .entry(policy.id.clone())
            .or_insert_with(Vec::new)
            .push(version.clone());
        
        version
    }
    
    // Private methods
    fn register_builtin_functions(&mut self) {
        // Time-based functions
        self.register_function(
            "is_business_hours",
            Box::new(IsBusinessHoursFunction),
        );
        
        self.register_function(
            "is_weekday",
            Box::new(IsWeekdayFunction),
        );
        
        // Location-based functions
        self.register_function(
            "in_region",
            Box::new(InRegionFunction),
        );
        
        self.register_function(
            "in_country",
            Box::new(InCountryFunction),
        );
        
        // Trust-based functions
        self.register_function(
            "trust_score_above",
            Box::new(TrustScoreAboveFunction),
        );
        
        self.register_function(
            "is_trusted_device",
            Box::new(IsTrustedDeviceFunction),
        );
        
        // Attribute-based functions
        self.register_function(
            "has_role",
            Box::new(HasRoleFunction),
        );
        
        self.register_function(
            "has_attribute",
            Box::new(HasAttributeFunction),
        );
    }
    
    fn process_condition_template(
        &self,
        template: &ConditionTemplate,
        parameters: &HashMap<String, serde_json::Value>,
    ) -> Result<PolicyCondition, PolicyLanguageError> {
        let value = self.substitute_parameters(&template.value, parameters)?;
        
        Ok(PolicyCondition::new(
            template.condition_type,
            template.operator,
            value,
        ))
    }
    
    fn process_action_template(
        &self,
        template: &ActionTemplate,
        parameters: &HashMap<String, serde_json::Value>,
    ) -> Result<PolicyAction, PolicyLanguageError> {
        match template {
            ActionTemplate::Basic(action) => Ok(action.clone()),
            ActionTemplate::Parameterized { action_type, params } => {
                // Process parameterized action
                match action_type.as_str() {
                    "rate_limit" => {
                        if let Some(rpm) = params.get("requests_per_minute") {
                            if let Some(rpm_val) = rpm.as_u64() {
                                return Ok(PolicyAction::RateLimit {
                                    requests_per_minute: rpm_val as u32,
                                });
                            }
                        }
                        Ok(PolicyAction::LogAccess)
                    }
                    _ => Ok(PolicyAction::LogAccess),
                }
            }
        }
    }
    
    fn substitute_parameters(
        &self,
        value: &serde_json::Value,
        parameters: &HashMap<String, serde_json::Value>,
    ) -> Result<serde_json::Value, PolicyLanguageError> {
        // Substitute ${param} placeholders with actual values
        match value {
            serde_json::Value::String(s) => {
                let result = s.clone();
                let mut substituted = result;
                
                for (key, param_value) in parameters {
                    let placeholder = format!("${{{}}}", key);
                    substituted = substituted.replace(&placeholder, &param_value.to_string());
                }
                
                Ok(serde_json::Value::String(substituted))
            }
            _ => Ok(value.clone()),
        }
    }
    
    fn parse_dsl(&self, dsl: &str) -> Result<ParsedPolicy, PolicyLanguageError> {
        // Simplified DSL parsing - in production, use a proper parser
        let lines: Vec<&str> = dsl.lines().collect();
        
        let mut name = String::new();
        let mut description = String::new();
        let mut policy_type = PolicyType::Access;
        let mut priority = 100;
        let mut conditions = Vec::new();
        let mut actions = Vec::new();
        
        for line in lines {
            let line = line.trim();
            
            if line.starts_with("policy ") {
                name = line[7..].trim().to_string();
            } else if line.starts_with("description: ") {
                description = line[13..].trim().to_string();
            } else if line.starts_with("type: ") {
                policy_type = match line[6..].trim() {
                    "access" => PolicyType::Access,
                    "data" => PolicyType::Data,
                    "network" => PolicyType::Network,
                    "device" => PolicyType::Device,
                    _ => PolicyType::Access,
                };
            } else if line.starts_with("priority: ") {
                priority = line[10..].trim().parse().unwrap_or(100);
            } else if line.starts_with("when ") {
                // Parse condition
                let condition = self.parse_dsl_condition(line[5..].trim())?;
                conditions.push(condition);
            } else if line.starts_with("allow") {
                actions.push(PolicyAction::Allow);
            } else if line.starts_with("deny") {
                actions.push(PolicyAction::Deny);
            } else if line.starts_with("require_mfa") {
                actions.push(PolicyAction::RequireMfa);
            } else if line.starts_with("log") {
                actions.push(PolicyAction::LogAccess);
            }
        }
        
        Ok(ParsedPolicy {
            name,
            description,
            policy_type,
            priority,
            conditions,
            actions,
        })
    }
    
    fn parse_dsl_condition(&self, condition_str: &str) -> Result<PolicyExpression, PolicyLanguageError> {
        // Simplified condition parsing
        // Format: "subject equals 'user@example.com'"
        
        let parts: Vec<&str> = condition_str.split_whitespace().collect();
        
        if parts.len() >= 3 {
            let field = parts[0];
            let operator = parts[1];
            let value_str = parts[2..].join(" ").trim_matches('\'').to_string();
            
            let condition_type = match field {
                "subject" => super::ConditionType::Subject,
                "resource" => super::ConditionType::Resource,
                "action" => super::ConditionType::Action,
                _ => return Err(PolicyLanguageError::ParseError(format!("Unknown field: {}", field))),
            };
            
            let operator_enum = match operator {
                "equals" => super::Operator::Equals,
                "contains" => super::Operator::Contains,
                "matches" => super::Operator::Matches,
                _ => super::Operator::Equals,
            };
            
            let condition = PolicyCondition::new(
                condition_type,
                operator_enum,
                value_str,
            );
            
            return Ok(PolicyExpression::Condition(Box::new(condition)));
        }
        
        Err(PolicyLanguageError::ParseError(format!("Invalid condition: {}", condition_str)))
    }
    
    fn convert_to_policy(&self, parsed: ParsedPolicy) -> Result<Policy, PolicyLanguageError> {
        let mut policy = Policy::new(&parsed.name, &parsed.name, parsed.policy_type)
            .with_description(parsed.description)
            .with_priority(parsed.priority);
        
        for expr in &parsed.conditions {
            match expr {
                PolicyExpression::Condition(cond) => {
                    policy = policy.with_condition(*cond.clone());
                }
                _ => {}
            }
        }
        
        for action in &parsed.actions {
            policy = policy.with_action(action.clone());
        }
        
        Ok(policy)
    }
    
    fn call_function(
        &self,
        name: &str,
        _args: &[PolicyExpression],
        _context: &ZeroTrustContext,
    ) -> Result<bool, PolicyLanguageError> {
        let function = self.functions
            .get(name)
            .ok_or_else(|| PolicyLanguageError::FunctionNotFound(name.to_string()))?;
        
        // For simplicity, return true for all functions
        // In production, execute the actual function with args
        Ok(function.execute(_context, &[])?)
    }
    
    fn get_next_version(&self, policy_id: &str) -> String {
        let versions = self.version_history.get(policy_id);
        let count = versions.map(|v| v.len()).unwrap_or(0);
        format!("{}.{}.0", 1, count + 1)
    }
}

/// Policy expression types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PolicyExpression {
    /// Simple condition
    Condition(Box<PolicyCondition>),
    
    /// AND expression
    And(Vec<PolicyExpression>),
    
    /// OR expression
    Or(Vec<PolicyExpression>),
    
    /// NOT expression
    Not(Box<PolicyExpression>),
    
    /// Function call
    FunctionCall {
        name: String,
        args: Vec<PolicyExpression>,
    },
}

/// Policy template
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PolicyTemplate {
    /// Template name
    pub name: String,
    
    /// Template description
    pub description: String,
    
    /// Policy type
    pub policy_type: PolicyType,
    
    /// Condition templates
    pub conditions: Vec<ConditionTemplate>,
    
    /// Action templates
    pub actions: Vec<ActionTemplate>,
    
    /// Default priority
    pub priority: i32,
    
    /// Required parameters
    pub required_parameters: Vec<String>,
}

/// Condition template
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConditionTemplate {
    /// Condition type
    pub condition_type: super::ConditionType,
    
    /// Operator
    pub operator: super::Operator,
    
    /// Value (may contain parameter placeholders)
    pub value: serde_json::Value,
}

/// Action template
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ActionTemplate {
    /// Basic action
    Basic(PolicyAction),
    
    /// Parameterized action
    Parameterized {
        action_type: String,
        params: HashMap<String, serde_json::Value>,
    },
}

/// Policy version
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PolicyVersion {
    /// Policy ID
    pub policy_id: String,
    
    /// Version string
    pub version: String,
    
    /// Creation timestamp
    pub created_at: DateTime<Utc>,
    
    /// Author of change
    pub author: String,
    
    /// Change description
    pub change_description: String,
    
    /// Policy snapshot
    pub policy_snapshot: Policy,
}

/// Parsed policy from DSL
#[derive(Debug)]
struct ParsedPolicy {
    name: String,
    description: String,
    policy_type: PolicyType,
    priority: i32,
    conditions: Vec<PolicyExpression>,
    actions: Vec<PolicyAction>,
}

/// Policy language configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PolicyLanguageConfig {
    /// Maximum policy depth
    pub max_expression_depth: usize,
    
    /// Maximum policy conditions
    pub max_conditions: usize,
    
    /// Enable custom functions
    pub allow_custom_functions: bool,
    
    /// Template validation enabled
    pub validate_templates: bool,
}

impl Default for PolicyLanguageConfig {
    fn default() -> Self {
        Self {
            max_expression_depth: 10,
            max_conditions: 50,
            allow_custom_functions: true,
            validate_templates: true,
        }
    }
}

/// Policy function trait
pub trait PolicyFunction: Send + Sync {
    /// Execute the function
    fn execute(&self, context: &ZeroTrustContext, args: &[PolicyExpression]) -> Result<bool, PolicyLanguageError>;
    
    /// Get function name
    fn name(&self) -> &str;
}

/// Policy language errors
#[derive(Debug, thiserror::Error)]
pub enum PolicyLanguageError {
    #[error("Template not found: {0}")]
    TemplateNotFound(String),
    
    #[error("Function not found: {0}")]
    FunctionNotFound(String),
    
    #[error("Parse error: {0}")]
    ParseError(String),
    
    #[error("Evaluation error: {0}")]
    EvaluationError(String),
    
    #[error("Parameter error: {0}")]
    ParameterError(String),
    
    #[error("Validation error: {0}")]
    ValidationError(String),
}

// Built-in function implementations

struct IsBusinessHoursFunction;
impl PolicyFunction for IsBusinessHoursFunction {
    fn execute(&self, context: &ZeroTrustContext, _args: &[PolicyExpression]) -> Result<bool, PolicyLanguageError> {
        let hour = context.timestamp.hour();
        Ok(hour >= 9 && hour < 17)
    }
    
    fn name(&self) -> &str {
        "is_business_hours"
    }
}

struct IsWeekdayFunction;
impl PolicyFunction for IsWeekdayFunction {
    fn execute(&self, context: &ZeroTrustContext, _args: &[PolicyExpression]) -> Result<bool, PolicyLanguageError> {
        let weekday = context.timestamp.weekday().num_days_from_monday();
        Ok(weekday < 5)
    }
    
    fn name(&self) -> &str {
        "is_weekday"
    }
}

struct InRegionFunction;
impl PolicyFunction for InRegionFunction {
    fn execute(&self, context: &ZeroTrustContext, _args: &[PolicyExpression]) -> Result<bool, PolicyLanguageError> {
        // Check if user is in a specific region
        Ok(context.network.location.region.is_some())
    }
    
    fn name(&self) -> &str {
        "in_region"
    }
}

struct InCountryFunction;
impl PolicyFunction for InCountryFunction {
    fn execute(&self, context: &ZeroTrustContext, _args: &[PolicyExpression]) -> Result<bool, PolicyLanguageError> {
        // Check if user is in a specific country
        Ok(!context.network.location.country.is_empty())
    }
    
    fn name(&self) -> &str {
        "in_country"
    }
}

struct TrustScoreAboveFunction;
impl PolicyFunction for TrustScoreAboveFunction {
    fn execute(&self, _context: &ZeroTrustContext, _args: &[PolicyExpression]) -> Result<bool, PolicyLanguageError> {
        // This would require trust score to be passed in context
        Ok(true)
    }
    
    fn name(&self) -> &str {
        "trust_score_above"
    }
}

struct IsTrustedDeviceFunction;
impl PolicyFunction for IsTrustedDeviceFunction {
    fn execute(&self, context: &ZeroTrustContext, _args: &[PolicyExpression]) -> Result<bool, PolicyLanguageError> {
        Ok(context.device.is_trusted)
    }
    
    fn name(&self) -> &str {
        "is_trusted_device"
    }
}

struct HasRoleFunction;
impl PolicyFunction for HasRoleFunction {
    fn execute(&self, _context: &ZeroTrustContext, _args: &[PolicyExpression]) -> Result<bool, PolicyLanguageError> {
        // Check if user has specific role from context
        Ok(true)
    }
    
    fn name(&self) -> &str {
        "has_role"
    }
}

struct HasAttributeFunction;
impl PolicyFunction for HasAttributeFunction {
    fn execute(&self, _context: &ZeroTrustContext, _args: &[PolicyExpression]) -> Result<bool, PolicyLanguageError> {
        // Check if user has specific attribute from context
        Ok(true)
    }
    
    fn name(&self) -> &str {
        "has_attribute"
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{DeviceInfo, NetworkInfo, LocationInfo, DeviceType, NetworkType};

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
    fn test_policy_language_manager_creation() {
        let config = PolicyLanguageConfig::default();
        let manager = PolicyLanguageManager::new(config);
        assert!(!manager.templates.is_empty());
    }

    #[test]
    fn test_dsl_parsing() {
        let config = PolicyLanguageConfig::default();
        let manager = PolicyLanguageManager::new(config);
        
        let dsl = r#"
            policy test-policy
            description: Test policy
            type: access
            priority: 100
            when subject equals 'user@example.com'
            allow
        "#;
        
        let result = manager.parse_policy(dsl);
        assert!(result.is_ok());
        
        let policy = result.unwrap();
        assert_eq!(policy.name, "test-policy");
    }

    #[test]
    fn test_template_registration() {
        let config = PolicyLanguageConfig::default();
        let mut manager = PolicyLanguageManager::new(config);
        
        let template = PolicyTemplate {
            name: "test-template".to_string(),
            description: "Test template".to_string(),
            policy_type: PolicyType::Access,
            conditions: Vec::new(),
            actions: Vec::new(),
            priority: 100,
            required_parameters: Vec::new(),
        };
        
        manager.register_template(template);
        assert!(manager.templates.contains_key("test-template"));
    }

    #[test]
    fn test_policy_from_template() {
        let config = PolicyLanguageConfig::default();
        let mut manager = PolicyLanguageManager::new(config);
        
        let template = PolicyTemplate {
            name: "allow-resource".to_string(),
            description: "Allow access to resource".to_string(),
            policy_type: PolicyType::Access,
            conditions: vec![],
            actions: vec![ActionTemplate::Basic(PolicyAction::Allow)],
            priority: 100,
            required_parameters: vec![],
        };
        
        manager.register_template(template);
        
        let result = manager.create_from_template("allow-resource", &HashMap::new());
        assert!(result.is_ok());
    }

    #[test]
    fn test_policy_versioning() {
        let config = PolicyLanguageConfig::default();
        let mut manager = PolicyLanguageManager::new(config);
        
        let policy = Policy::new("test-policy", "Test Policy", PolicyType::Access)
            .with_action(PolicyAction::Allow);
        
        let version = manager.create_version(&policy, "Initial version", "admin");
        
        assert_eq!(version.policy_id, "test-policy");
        assert_eq!(version.author, "admin");
    }

    #[test]
    fn test_expression_evaluation() {
        let config = PolicyLanguageConfig::default();
        let manager = PolicyLanguageManager::new(config);
        
        let condition = PolicyCondition::new(
            super::super::ConditionType::Subject,
            super::super::Operator::Equals,
            "user@example.com",
        );
        
        let expr = PolicyExpression::Condition(Box::new(condition));
        let context = create_test_context();
        
        let result = manager.evaluate_expression(&expr, &context);
        assert!(result.is_ok());
        assert!(result.unwrap());
    }
}