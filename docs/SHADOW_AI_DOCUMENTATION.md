# Shadow AI Detection and Governance

## Overview

The Shadow AI Detection and Governance module provides comprehensive capabilities for detecting, assessing, and governing unauthorized AI usage within an organization. This module addresses the growing challenge of employees using AI tools without IT/Security approval, which poses significant security, compliance, and data protection risks.

## Architecture

The module consists of five main components:

1. **Detection System** - Discovers AI models and analyzes AI API calls
2. **Governance Framework** - Enforces policies and manages model registration
3. **Risk Assessment** - Evaluates risks from shadow AI usage
4. **Response System** - Automated alerts and remediation workflows
5. **Models Module** - Shared data structures and types

## Components

### 1. Detection System (`detection.rs`)

The detection system identifies unauthorized AI deployments through multiple methods:

#### AI Model Discovery
- Scans filesystem for AI model files (`.pkl`, `.h5`, `.onnx`, `.pt`, `.bin`, etc.)
- Supports configurable scan directories
- Handles glob patterns (e.g., `/home/*/models`)
- Calculates SHA-256 hashes for file integrity

#### Network Traffic Analysis
- Monitors for AI API calls to known services
- Detects OpenAI, Anthropic, Cohere, HuggingFace, etc.
- Analyzes request patterns and data volumes
- Identifies unauthorized external AI service usage

#### AI Model Fingerprinting
- Analyzes model headers to identify framework
- Detects model type (LLM, Computer Vision, etc.)
- Creates unique fingerprints for tracking
- Supports multiple frameworks (PyTorch, TensorFlow, ONNX)

#### Key Types
- `ShadowAIDetector` - Main detection engine
- `DiscoveryConfig` - Configuration for discovery
- `AIModelDiscovery` - Results of discovery operation
- `AIApiCall` - Detected AI API call
- `AIModelFingerprint` - Model fingerprint details
- `NetworkTrafficAnalysis` - Network traffic summary

### 2. Governance Framework (`governance.rs`)

The governance framework enforces AI usage policies and manages the AI model registry.

#### Policy Engine
- Define custom policy rules for AI usage
- Rule types: Registration, Approval, Blocking, Rate Limiting
- Priority-based rule evaluation
- Enable/disable individual rules

#### AI Model Registry
- Track all registered AI models
- Manage approval workflows
- Set usage limits and data source restrictions
- Maintain model ownership and metadata

#### Compliance Enforcement
- Support for multiple frameworks: SOC 2, SOX, GDPR, HIPAA, PCI DSS, ISO 27001
- Automated compliance checking
- Generate compliance reports
- Track compliance issues

#### Key Types
- `GovernanceEngine` - Main governance engine
- `PolicyRule` - AI usage policy rule
- `AIModelRegistry` - Registered models database
- `RegisteredModel` - Registered model details
- `RegistrationRequest` - Model registration request
- `ComplianceReport` - Compliance status report

### 3. Risk Assessment (`risk.rs`)

The risk assessment engine evaluates potential risks from shadow AI usage.

#### AI Risk Scoring
- Multi-factor risk assessment (0.0-1.0)
- Components: Capability, Data Sensitivity, Access Control, Operational
- Configurable risk thresholds
- Automatic risk-based response triggers

#### Risk Factors
- Model capability risks (LLMs, Generative AI)
- Data exposure risks
- Access control vulnerabilities
- Operational risks
- Compliance violations

#### Data Exposure Analysis
- Map AI models to data sources
- Identify exposure pathways
- Assess data sensitivity impact
- Generate exposure risk reports

#### Key Types
- `RiskAssessmentEngine` - Risk assessment engine
- `AIRiskScore` - Comprehensive risk score
- `RiskComponents` - Component risk breakdown
- `RiskFactor` - Individual risk factor
- `DataExposureRisk` - Data exposure assessment
- `RiskReport` - Overall risk report

### 4. Response System (`response.rs`)

The response system provides automated alerts and remediation workflows.

#### Alerting System
- Multiple alert channels: Email, Slack, SMS, Audit Log, Webhooks
- Severity-based alert routing
- Alert acknowledgment and resolution tracking
- Configurable alert thresholds

#### Automated Blocking
- Auto-block critical risk models
- Manage blocked models list
- Unblock workflows with approval
- Audit trail for blocking actions

#### Remediation Workflows
- Configurable remediation steps
- Step-by-step workflow execution
- Status tracking and reporting
- Integration with governance policies

#### Key Types
- `ResponseEngine` - Response automation engine
- `Alert` - Alert notification
- `RemediationWorkflow` - Remediation workflow
- `BlockedModel` - Blocked model record
- `RemediationStep` - Workflow step

### 5. Models Module (`models.rs`)

Contains shared data structures and types used across the Shadow AI module.

#### Core Types
- `DetectedAIModel` - Discovered AI model
- `AIModelType` - Type classification
- `RiskLevel` - Risk severity (Low, Medium, High, Critical)
- `RegistrationStatus` - Registration state

#### Supporting Types
- `ShadowAISummary` - Summary statistics
- `AIUsageStats` - Usage statistics
- `ModelLineage` - Model provenance
- `DataAccessPattern` - Data access patterns
- `AIComplianceStatus` - Compliance status
- `AISecurityAssessment` - Security assessment

## Usage Examples

### Basic Setup

```rust
use v_sentinel::shadow_ai::{ShadowAIManager, ShadowAIConfig};

// Create manager with default configuration
let config = ShadowAIConfig::default();
let manager = ShadowAIManager::new(config);

// Run a full scan for Shadow AI
let summary = manager.run_full_scan().await?;

println!("Found {} shadow AI models", summary.total_models);
println!("{} high-risk models detected", 
    summary.models_by_risk.get(&RiskLevel::High).unwrap_or(&0));
```

### Custom Detection Configuration

```rust
use v_sentinel::shadow_ai::{ShadowAIDetector, DiscoveryConfig};

let config = DiscoveryConfig {
    network_interfaces: vec!["eth0".to_string()],
    monitored_ports: vec![443, 8080],
    scan_directories: vec![
        "/opt/models".to_string(),
        "/home/*/models".to_string(),
    ],
    model_extensions: vec![
        "pkl".to_string(),
        "h5".to_string(),
        "onnx".to_string(),
    ],
    known_ai_endpoints: vec![
        "api.openai.com".to_string(),
        "api.anthropic.com".to_string(),
    ],
    scan_interval_secs: 600,
    passive_monitoring: true,
    active_scanning: true,
};

let detector = ShadowAIDetector::new(config);
let discovery = detector.discover_ai_models().await?;
```

### Risk Assessment

```rust
use v_sentinel::shadow_ai::{RiskAssessmentEngine, RiskConfig};

let risk_engine = RiskAssessmentEngine::new(RiskConfig::default());

for model in &discovery.detected_models {
    let risk_score = risk_engine.assess_model_risk(model).await?;
    
    println!("Model: {}", model.model_name);
    println!("Risk Score: {:.2}", risk_score.overall_score);
    println!("Risk Level: {:?}", risk_score.risk_level);
    
    for factor in &risk_score.risk_factors {
        println!("  - {}: {}", factor.category, factor.description);
    }
}
```

### Governance and Policy Enforcement

```rust
use v_sentinel::shadow_ai::{GovernanceEngine, PolicyRule, PolicyRuleType, PolicyAction};

let governance = GovernanceEngine::new(GovernanceConfig::default());

// Add a policy rule
let rule = PolicyRule {
    rule_id: "llm-registration-required".to_string(),
    name: "LLM Registration Required".to_string(),
    description: "All LLMs must be registered".to_string(),
    rule_type: PolicyRuleType::RegistrationRequired,
    model_types: vec![AIModelType::LargeLanguageModel],
    risk_levels: vec![],
    action: PolicyAction::RequireApproval,
    priority: 100,
    enabled: true,
    created_at: Utc::now(),
    modified_at: Utc::now(),
};

governance.add_policy_rule(rule).await?;

// Evaluate model against policies
let evaluation = governance.evaluate_model(&model).await;
if evaluation.blocked {
    println!("Model is blocked by policy");
}
```

### Automated Response

```rust
use v_sentinel::shadow_ai::{ResponseEngine, ResponseConfig};

let response = ResponseEngine::new(ResponseConfig::default());

// Process a detected model
let alerts = response.process_detection(&model, &risk_score).await?;

for alert in alerts {
    println!("Alert: [{}] {}", alert.severity, alert.message);
}

// Get alert history
let history = response.get_alert_history().await;

// Acknowledge an alert
response.acknowledge_alert(&alert.alert_id, "security-team").await?;
```

## Configuration

### ShadowAIConfig

```rust
pub struct ShadowAIConfig {
    pub detection: DetectionSettings,
    pub governance: GovernanceSettings,
    pub risk: RiskSettings,
    pub response: ResponseSettings,
}
```

### DetectionSettings

- `enable_discovery: bool` - Enable model discovery
- `enable_network_monitoring: bool` - Enable network monitoring
- `scan_interval_secs: u64` - Scan interval in seconds
- `passive_mode: bool` - Passive detection only

### GovernanceSettings

- `require_registration: bool` - Require registration for all models
- `auto_approve_low_risk: bool` - Auto-approve low-risk models
- `enable_enforcement: bool` - Enable policy enforcement

### RiskSettings

- `enable_risk_scoring: bool` - Enable risk scoring
- `block_critical: bool` - Block critical risk models
- `alert_threshold: f64` - Risk threshold for alerts (0.0-1.0)

### ResponseSettings

- `enable_automated_response: bool` - Enable automated response
- `enable_email_alerts: bool` - Enable email alerts
- `enable_slack_alerts: bool` - Enable Slack alerts
- `alert_on_high: bool` - Alert on high risk

## Risk Assessment Methodology

### Risk Components

1. **Capability Risk** (35% weight)
   - Model type (LLMs higher than traditional ML)
   - Model size (larger = more powerful)
   - Framework vulnerabilities

2. **Data Sensitivity Risk** (35% weight)
   - Model location (suspicious locations = higher risk)
   - Generative capabilities (potential for data leakage)
   - Registration status

3. **Access Control Risk** (30% weight)
   - Owner assignment
   - Registration status
   - File permissions

4. **Operational Risk** (15% weight)
   - Model age
   - Modification frequency

### Risk Levels

- **Low** (0.0-0.25): Acceptable risk, monitor
- **Medium** (0.25-0.50): Require registration and monitoring
- **High** (0.50-0.75): Require approval and enhanced controls
- **Critical** (0.75-1.0): Consider blocking, immediate review

## Compliance Frameworks

The module supports multiple compliance frameworks:

### SOC 2
- Control over shadow AI usage
- Audit trails for AI access
- Incident response for AI-related breaches

### GDPR
- Data Protection Impact Assessments (DPIA) for AI
- Data subject rights for AI processing
- Consent management for AI usage

### HIPAA
- Protected Health Information (PHI) in AI models
- Access controls for healthcare AI
- Audit logging for AI usage

### ISO 27001
- AI asset inventory
- Risk management for AI systems
- Access control and monitoring

## Best Practices

### 1. Detection
- Regularly scan for new AI models
- Monitor network traffic for AI API calls
- Keep model fingerprint database updated
- Use passive monitoring for minimal disruption

### 2. Governance
- Define clear AI usage policies
- Require registration for all AI models
- Implement approval workflows
- Regularly review and update policies

### 3. Risk Management
- Assess risk for all detected models
- Map models to data sources
- Implement controls based on risk level
- Conduct regular risk reviews

### 4. Response
- Configure appropriate alert thresholds
- Set up multiple alert channels
- Document remediation procedures
- Regularly test response workflows

### 5. Compliance
- Identify applicable compliance frameworks
- Generate regular compliance reports
- Maintain audit trails
- Prepare for audits

## Integration Points

### API Integration
The module can be integrated with:
- RESTful API endpoints for management
- Webhook notifications for alerts
- SIEM systems for log forwarding
- SOAR platforms for automated response

### Data Sources
- File system monitoring
- Network packet capture
- Cloud provider APIs
- Container registries
- Model artifact repositories

### External Systems
- Identity providers (SSO integration)
- ITSM systems (ticket creation)
- Data classification systems
- Access management systems

## Security Considerations

1. **Privilege Management**: Scanner needs appropriate file system access
2. **Data Protection**: Model hashes and fingerprints are sensitive
3. **Network Monitoring**: Comply with privacy laws for traffic analysis
4. **Audit Logging**: Maintain comprehensive audit trails
5. **Access Control**: Restrict who can view and manage shadow AI

## Performance Considerations

- Scan intervals should balance detection frequency vs. system load
- Network monitoring can be resource-intensive
- Cache risk scores to avoid repeated calculations
- Use incremental scanning for large filesystems
- Parallelize independent assessments

## Future Enhancements

- [ ] Real-time monitoring with inotify
- [ ] Integration with cloud AI service APIs
- [ ] Machine learning for anomaly detection
- [ ] Advanced threat hunting capabilities
- [ ] Integration with vulnerability scanners
- [ ] Automated policy recommendations
- [ ] Cost tracking for external AI services

## References

- IBM Cybersecurity Trends 2025
- NIST AI Risk Management Framework (AI RMF)
- OWASP Top 10 for LLM Applications
- ENISA Guidelines for Secure AI

## Contributing

When contributing to this module:
1. Follow Rust coding standards
2. Add comprehensive tests
3. Update documentation
4. Consider privacy implications
5. Test with various AI models and frameworks