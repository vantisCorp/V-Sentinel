//! Policy Testing and Validation Module
//!
//! Provides policy simulation, impact analysis, and regression testing capabilities.

use super::super::{ZeroTrustContext, AccessDecision};
use super::{Policy, PolicyEngine, PolicyType};
use std::collections::HashMap;
use std::sync::Arc;
use chrono::{DateTime, Utc};

/// Policy Validator
pub struct PolicyValidator {
    validation_rules: Vec<ValidationRule>,
    config: ValidationConfig,
}

impl PolicyValidator {
    /// Create a new policy validator
    pub fn new(config: ValidationConfig) -> Self {
        let mut validator = Self {
            validation_rules: Vec::new(),
            config,
        };
        
        validator.register_default_rules();
        validator
    }
    
    /// Validate a policy
    pub fn validate_policy(&self, policy: &Policy) -> ValidationResult {
        let mut errors = Vec::new();
        let mut warnings = Vec::new();
        
        // Run all validation rules
        for rule in &self.validation_rules {
            let result = rule.validate(policy);
            errors.extend(result.errors);
            warnings.extend(result.warnings);
        }
        
        ValidationResult {
            is_valid: errors.is_empty(),
            errors,
            warnings,
        }
    }
    
    /// Validate multiple policies
    pub fn validate_policies(&self, policies: &[Policy]) -> Vec<(String, ValidationResult)> {
        policies
            .iter()
            .map(|p| (p.id.clone(), self.validate_policy(p)))
            .collect()
    }
    
    /// Register a validation rule
    pub fn register_rule(&mut self, rule: ValidationRule) {
        self.validation_rules.push(rule);
    }
    
    fn register_default_rules(&mut self) {
        self.validation_rules.push(ValidationRule {
            name: "Policy ID Format".to_string(),
            description: "Policy ID must follow naming convention".to_string(),
            check: Box::new(|policy| {
                let mut errors = Vec::new();
                let mut warnings = Vec::new();
                
                // Check ID format (alphanumeric with hyphens)
                if !policy.id.chars().all(|c| c.is_alphanumeric() || c == '-') {
                    errors.push(ValidationError {
                        field: "id".to_string(),
                        message: "Policy ID must contain only alphanumeric characters and hyphens".to_string(),
                    });
                }
                
                RuleResult { errors, warnings }
            }),
        });
        
        self.validation_rules.push(ValidationRule {
            name: "Policy Has Conditions".to_string(),
            description: "Policy must have at least one condition".to_string(),
            check: Box::new(|policy| {
                let mut errors = Vec::new();
                let mut warnings = Vec::new();
                
                if policy.conditions.is_empty() {
                    errors.push(ValidationError {
                        field: "conditions".to_string(),
                        message: "Policy must have at least one condition to avoid overly broad access".to_string(),
                    });
                }
                
                RuleResult { errors, warnings }
            }),
        });
        
        self.validation_rules.push(ValidationRule {
            name: "Policy Has Actions".to_string(),
            description: "Policy must have at least one action".to_string(),
            check: Box::new(|policy| {
                let mut errors = Vec::new();
                let mut warnings = Vec::new();
                
                if policy.actions.is_empty() {
                    errors.push(ValidationError {
                        field: "actions".to_string(),
                        message: "Policy must have at least one action".to_string(),
                    });
                }
                
                RuleResult { errors, warnings }
            }),
        });
        
        self.validation_rules.push(ValidationRule {
            name: "Policy Priority Range".to_string(),
            description: "Policy priority must be within valid range".to_string(),
            check: Box::new(|policy| {
                let mut errors = Vec::new();
                let mut warnings = Vec::new();
                
                if policy.priority < 0 || policy.priority > 1000 {
                    warnings.push(ValidationError {
                        field: "priority".to_string(),
                        message: "Policy priority outside recommended range (0-1000)".to_string(),
                    });
                }
                
                RuleResult { errors, warnings }
            }),
        });
        
        self.validation_rules.push(ValidationRule {
            name: "No Allow All Policy".to_string(),
            description: "Warn about policies that allow all access".to_string(),
            check: Box::new(|policy| {
                let mut errors = Vec::new();
                let mut warnings = Vec::new();
                
                let has_allow = policy.actions.iter().any(|a| matches!(a, super::PolicyAction::Allow));
                let has_conditions = !policy.conditions.is_empty();
                
                if has_allow && !has_conditions {
                    warnings.push(ValidationError {
                        field: "policy".to_string(),
                        message: "Policy allows access without conditions - this may be overly permissive".to_string(),
                    });
                }
                
                RuleResult { errors, warnings }
            }),
        });
    }
}

/// Policy Simulator
pub struct PolicySimulator {
    policy_engine: Arc<PolicyEngine>,
    test_scenarios: Vec<TestScenario>,
}

impl PolicySimulator {
    /// Create a new policy simulator
    pub fn new(policy_engine: Arc<PolicyEngine>) -> Self {
        Self {
            policy_engine,
            test_scenarios: Vec::new(),
        }
    }
    
    /// Add a test scenario
    pub fn add_scenario(&mut self, scenario: TestScenario) {
        self.test_scenarios.push(scenario);
    }
    
    /// Run all test scenarios
    pub fn run_scenarios(&self) -> SimulationResults {
        let mut results = Vec::new();
        
        for scenario in &self.test_scenarios {
            let result = self.run_scenario(scenario);
            results.push(result);
        }
        
        SimulationResults {
            scenarios: self.test_scenarios.clone(),
            results,
            summary: self.generate_summary(&results),
        }
    }
    
    /// Run a single test scenario
    pub fn run_scenario(&self, scenario: &TestScenario) -> ScenarioResult {
        let access_result = self.policy_engine.evaluate(
            &scenario.context,
            scenario.trust_score,
            scenario.trust_score.into(),
        );
        
        let passed = access_result.decision == scenario.expected_decision;
        
        ScenarioResult {
            scenario_name: scenario.name.clone(),
            passed,
            actual_decision: access_result.decision,
            expected_decision: scenario.expected_decision,
            reason: access_result.reason,
            execution_time: std::time::Duration::from_millis(5), // Simulated
        }
    }
    
    /// Generate impact analysis for policy changes
    pub fn analyze_impact(
        &self,
        old_policy: &Policy,
        new_policy: &Policy,
        contexts: &[ZeroTrustContext],
    ) -> ImpactAnalysis {
        let mut changed_decisions = Vec::new();
        let mut same_decisions = 0;
        
        for context in contexts {
            let old_decision = self.policy_engine.evaluate(
                context,
                0.5,
                crate::TrustLevel::Medium,
            );
            
            // Temporarily replace policy and evaluate
            let mut temp_engine = (*self.policy_engine).clone();
            temp_engine.register_policy(new_policy.clone());
            
            let new_decision = temp_engine.evaluate(
                context,
                0.5,
                crate::TrustLevel::Medium,
            );
            
            if old_decision.decision != new_decision.decision {
                changed_decisions.push(ImpactChange {
                    context: context.clone(),
                    old_decision: old_decision.decision,
                    new_decision: new_decision.decision,
                    impact_type: self.determine_impact_type(
                        &old_decision.decision,
                        &new_decision.decision,
                    ),
                });
            } else {
                same_decisions += 1;
            }
        }
        
        ImpactAnalysis {
            policy_id: new_policy.id.clone(),
            total_contexts: contexts.len(),
            changed_decisions: changed_decisions.len(),
            same_decisions,
            changes: changed_decisions,
        }
    }
    
    fn generate_summary(&self, results: &[ScenarioResult]) -> SimulationSummary {
        let total = results.len();
        let passed = results.iter().filter(|r| r.passed).count();
        let failed = total - passed;
        
        SimulationSummary {
            total_scenarios: total,
            passed_scenarios: passed,
            failed_scenarios: failed,
            pass_rate: if total > 0 { (passed as f64 / total as f64) * 100.0 } else { 0.0 },
        }
    }
    
    fn determine_impact_type(
        &self,
        old: &AccessDecision,
        new: &AccessDecision,
    ) -> ImpactType {
        match (old, new) {
            (AccessDecision::Allow, AccessDecision::Deny) => ImpactType::Regressive,
            (AccessDecision::Deny, AccessDecision::Allow) => ImpactType::Expansive,
            _ => ImpactType::Neutral,
        }
    }
}

/// Policy Regression Tester
pub struct PolicyRegressionTester {
    baseline_results: HashMap<String, Vec<ScenarioResult>>,
    policy_engine: Arc<PolicyEngine>,
}

impl PolicyRegressionTester {
    /// Create a new regression tester
    pub fn new(policy_engine: Arc<PolicyEngine>) -> Self {
        Self {
            baseline_results: HashMap::new(),
            policy_engine,
        }
    }
    
    /// Set baseline results
    pub fn set_baseline(&mut self, policy_id: &str, results: Vec<ScenarioResult>) {
        self.baseline_results.insert(policy_id.to_string(), results);
    }
    
    /// Run regression test
    pub fn run_regression(
        &self,
        policy_id: &str,
        scenarios: &[TestScenario],
    ) -> RegressionTestResult {
        let baseline = match self.baseline_results.get(policy_id) {
            Some(results) => results,
            None => {
                return RegressionTestResult {
                    policy_id: policy_id.to_string(),
                    regressions: Vec::new(),
                    improvements: Vec::new(),
                    baseline_missing: true,
                };
            }
        };
        
        let mut regressions = Vec::new();
        let mut improvements = Vec::new();
        
        for (i, scenario) in scenarios.iter().enumerate() {
            if let Some(baseline_result) = baseline.get(i) {
                let current_result = self.policy_engine.evaluate(
                    &scenario.context,
                    scenario.trust_score,
                    scenario.trust_score.into(),
                );
                
                if baseline_result.passed && current_result.decision != scenario.expected_decision {
                    regressions.push(Regression {
                        scenario_name: scenario.name.clone(),
                        baseline_passed: true,
                        current_passed: current_result.decision == scenario.expected_decision,
                        baseline_decision: baseline_result.actual_decision,
                        current_decision: current_result.decision,
                    });
                } else if !baseline_result.passed && current_result.decision == scenario.expected_decision {
                    improvements.push(Regression {
                        scenario_name: scenario.name.clone(),
                        baseline_passed: false,
                        current_passed: true,
                        baseline_decision: baseline_result.actual_decision,
                        current_decision: current_result.decision,
                    });
                }
            }
        }
        
        RegressionTestResult {
            policy_id: policy_id.to_string(),
            regressions,
            improvements,
            baseline_missing: false,
        }
    }
}

/// Validation Rule
pub struct ValidationRule {
    pub name: String,
    pub description: String,
    pub check: Box<dyn Fn(&Policy) -> RuleResult + Send + Sync>,
}

/// Validation Result
#[derive(Debug, Clone)]
pub struct ValidationResult {
    pub is_valid: bool,
    pub errors: Vec<ValidationError>,
    pub warnings: Vec<ValidationError>,
}

/// Validation Error
#[derive(Debug, Clone)]
pub struct ValidationError {
    pub field: String,
    pub message: String,
}

/// Rule Result
#[derive(Debug)]
struct RuleResult {
    errors: Vec<ValidationError>,
    warnings: Vec<ValidationError>,
}

/// Validation Configuration
#[derive(Debug, Clone)]
pub struct ValidationConfig {
    pub strict_mode: bool,
    pub custom_rules: Vec<String>,
}

impl Default for ValidationConfig {
    fn default() -> Self {
        Self {
            strict_mode: false,
            custom_rules: Vec::new(),
        }
    }
}

/// Test Scenario
#[derive(Debug, Clone)]
pub struct TestScenario {
    pub name: String,
    pub description: String,
    pub context: ZeroTrustContext,
    pub trust_score: f64,
    pub expected_decision: AccessDecision,
}

/// Scenario Result
#[derive(Debug, Clone)]
pub struct ScenarioResult {
    pub scenario_name: String,
    pub passed: bool,
    pub actual_decision: AccessDecision,
    pub expected_decision: AccessDecision,
    pub reason: String,
    pub execution_time: std::time::Duration,
}

/// Simulation Results
#[derive(Debug)]
pub struct SimulationResults {
    pub scenarios: Vec<TestScenario>,
    pub results: Vec<ScenarioResult>,
    pub summary: SimulationSummary,
}

/// Simulation Summary
#[derive(Debug, Clone)]
pub struct SimulationSummary {
    pub total_scenarios: usize,
    pub passed_scenarios: usize,
    pub failed_scenarios: usize,
    pub pass_rate: f64,
}

/// Impact Analysis
#[derive(Debug)]
pub struct ImpactAnalysis {
    pub policy_id: String,
    pub total_contexts: usize,
    pub changed_decisions: usize,
    pub same_decisions: usize,
    pub changes: Vec<ImpactChange>,
}

/// Impact Change
#[derive(Debug, Clone)]
pub struct ImpactChange {
    pub context: ZeroTrustContext,
    pub old_decision: AccessDecision,
    pub new_decision: AccessDecision,
    pub impact_type: ImpactType,
}

/// Impact Type
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ImpactType {
    Regressive,
    Expansive,
    Neutral,
}

/// Regression Test Result
#[derive(Debug)]
pub struct RegressionTestResult {
    pub policy_id: String,
    pub regressions: Vec<Regression>,
    pub improvements: Vec<Regression>,
    pub baseline_missing: bool,
}

/// Regression
#[derive(Debug, Clone)]
pub struct Regression {
    pub scenario_name: String,
    pub baseline_passed: bool,
    pub current_passed: bool,
    pub baseline_decision: AccessDecision,
    pub current_decision: AccessDecision,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{DeviceInfo, NetworkInfo, LocationInfo, DeviceType, NetworkType};

    fn create_test_policy() -> Policy {
        Policy::new("test-policy", "Test Policy", PolicyType::Access)
            .with_action(super::super::PolicyAction::Allow)
    }

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
    fn test_policy_validator() {
        let validator = PolicyValidator::new(ValidationConfig::default());
        let policy = create_test_policy();
        
        let result = validator.validate_policy(&policy);
        assert!(!result.is_valid); // Should fail due to no conditions
    }

    #[test]
    fn test_policy_validation_with_conditions() {
        let validator = PolicyValidator::new(ValidationConfig::default());
        
        let policy = Policy::new("test-policy", "Test Policy", PolicyType::Access)
            .with_condition(super::super::PolicyCondition::new(
                super::super::ConditionType::Subject,
                super::super::Operator::Equals,
                "user@example.com",
            ))
            .with_action(super::super::PolicyAction::Allow);
        
        let result = validator.validate_policy(&policy);
        assert!(result.is_valid);
    }

    #[test]
    fn test_policy_simulator() {
        let policy_engine = Arc::new(PolicyEngine::new());
        let mut simulator = PolicySimulator::new(policy_engine);
        
        let scenario = TestScenario {
            name: "Test scenario".to_string(),
            description: "Test scenario description".to_string(),
            context: create_test_context(),
            trust_score: 0.8,
            expected_decision: AccessDecision::Deny, // Default deny
        };
        
        simulator.add_scenario(scenario);
        
        let results = simulator.run_scenarios();
        assert_eq!(results.summary.total_scenarios, 1);
    }

    #[test]
    fn test_impact_analysis() {
        let policy_engine = Arc::new(PolicyEngine::new());
        let simulator = PolicySimulator::new(policy_engine);
        
        let old_policy = create_test_policy();
        let new_policy = Policy::new("test-policy", "Test Policy", PolicyType::Access)
            .with_action(super::super::PolicyAction::Allow);
        
        let contexts = vec![create_test_context()];
        let analysis = simulator.analyze_impact(&old_policy, &new_policy, &contexts);
        
        assert_eq!(analysis.total_contexts, 1);
    }

    #[test]
    fn test_regression_testing() {
        let policy_engine = Arc::new(PolicyEngine::new());
        let mut tester = PolicyRegressionTester::new(policy_engine);
        
        let baseline = vec![
            ScenarioResult {
                scenario_name: "test".to_string(),
                passed: true,
                actual_decision: AccessDecision::Deny,
                expected_decision: AccessDecision::Deny,
                reason: "test".to_string(),
                execution_time: std::time::Duration::from_millis(5),
            },
        ];
        
        tester.set_baseline("test-policy", baseline);
        
        let scenario = TestScenario {
            name: "test".to_string(),
            description: "test".to_string(),
            context: create_test_context(),
            trust_score: 0.8,
            expected_decision: AccessDecision::Deny,
        };
        
        let result = tester.run_regression("test-policy", &[scenario]);
        assert_eq!(result.policy_id, "test-policy");
    }
}