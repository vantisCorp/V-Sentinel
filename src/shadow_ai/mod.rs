//! Shadow AI Detection and Governance Module
//!
//! This module provides comprehensive capabilities for detecting, assessing, and governing
//! unauthorized AI usage within an organization (Shadow AI).

pub mod detection;
pub mod governance;
pub mod risk;
pub mod response;
pub mod models;

// Re-export main types from models
pub use models::{
    DetectedAIModel, AIModelType, RiskLevel, RegistrationStatus,
    AIServiceConfig, RateLimits, ShadowAISummary, AIUsageStats,
    ModelLineage, FineTuningRecord, TrainingMetrics, DataAccessPattern,
    AIComplianceStatus, AISecurityAssessment, AIVulnerability, AIVulnerabilityType,
};

// Re-export detection types
pub use detection::{
    ShadowAIDetector, DiscoveryConfig, AIModelDiscovery, AIApiCall,
    AIProvider, AIRequestType, AIModelFingerprint, NetworkTrafficAnalysis,
};

// Re-export governance types
pub use governance::{
    GovernanceEngine, GovernanceConfig, PolicyRule, PolicyRuleType, PolicyAction,
    AIModelRegistry, RegisteredModel, RegistrationRequest, RequestStatus,
    UsageLimits, TimeWindow, ModelRiskAssessment, EnforcementLog,
    EnforcementAction, EnforcementOutcome, PolicyEvaluationResult,
    ComplianceStatus, ComplianceReport, ComplianceSummary,
    ComplianceFramework,
};

// Re-export risk types
pub use risk::{
    RiskAssessmentEngine, RiskConfig, AIRiskScore, RiskComponents,
    RiskFactor, RiskCategory, DataSensitivity, DataExposureRisk,
    DataSourceMapping, DataType, AccessPattern, AccessFrequency, AccessType,
    ExposurePathway, ExposureMechanism, RiskReport, RiskSummary,
};

// Re-export response types
pub use response::{
    ResponseEngine, ResponseConfig, Alert, AlertSeverity, AlertType,
    AlertStatus, AlertChannel, RemediationWorkflow, WorkflowStatus,
    RemediationStep, RemediationStepType, StepStatus, WorkflowResult,
    BlockedModel,
};

/// Error type for Shadow AI operations
#[derive(Debug, Clone, thiserror::Error)]
pub enum ShadowAIError {
    #[error("Discovery error: {0}")]
    DiscoveryError(String),
    
    #[error("Governance error: {0}")]
    GovernanceError(String),
    
    #[error("Risk assessment error: {0}")]
    RiskError(String),
    
    #[error("Response error: {0}")]
    ResponseError(String),
    
    #[error("Configuration error: {0}")]
    ConfigError(String),
    
    #[error("IO error: {0}")]
    IoError(String),
    
    #[error("Network error: {0}")]
    NetworkError(String),
    
    #[error("Analysis error: {0}")]
    AnalysisError(String),
}

/// Configuration for Shadow AI module
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ShadowAIConfig {
    /// Detection settings
    pub detection: DetectionSettings,
    /// Governance settings
    pub governance: GovernanceSettings,
    /// Risk settings
    pub risk: RiskSettings,
    /// Response settings
    pub response: ResponseSettings,
}

impl Default for ShadowAIConfig {
    fn default() -> Self {
        Self {
            detection: DetectionSettings::default(),
            governance: GovernanceSettings::default(),
            risk: RiskSettings::default(),
            response: ResponseSettings::default(),
        }
    }
}

/// Detection settings
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct DetectionSettings {
    /// Enable model discovery
    pub enable_discovery: bool,
    /// Enable network monitoring
    pub enable_network_monitoring: bool,
    /// Scan interval in seconds
    pub scan_interval_secs: u64,
    /// Enable passive detection only
    pub passive_mode: bool,
}

impl Default for DetectionSettings {
    fn default() -> Self {
        Self {
            enable_discovery: true,
            enable_network_monitoring: true,
            scan_interval_secs: 300,
            passive_mode: false,
        }
    }
}

/// Governance settings
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct GovernanceSettings {
    /// Require registration for all models
    pub require_registration: bool,
    /// Auto-approve low-risk models
    pub auto_approve_low_risk: bool,
    /// Enable policy enforcement
    pub enable_enforcement: bool,
}

impl Default for GovernanceSettings {
    fn default() -> Self {
        Self {
            require_registration: true,
            auto_approve_low_risk: false,
            enable_enforcement: true,
        }
    }
}

/// Risk settings
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct RiskSettings {
    /// Enable risk scoring
    pub enable_risk_scoring: bool,
    /// Block critical risk models
    pub block_critical: bool,
    /// Risk threshold for alerts
    pub alert_threshold: f64,
}

impl Default for RiskSettings {
    fn default() -> Self {
        Self {
            enable_risk_scoring: true,
            block_critical: true,
            alert_threshold: 0.7,
        }
    }
}

/// Response settings
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ResponseSettings {
    /// Enable automated response
    pub enable_automated_response: bool,
    /// Enable email alerts
    pub enable_email_alerts: bool,
    /// Enable Slack alerts
    pub enable_slack_alerts: bool,
    /// Alert on high risk
    pub alert_on_high: bool,
}

impl Default for ResponseSettings {
    fn default() -> Self {
        Self {
            enable_automated_response: true,
            enable_email_alerts: true,
            enable_slack_alerts: false,
            alert_on_high: true,
        }
    }
}

/// Main Shadow AI Manager - integrates all components
pub struct ShadowAIManager {
    config: ShadowAIConfig,
    detector: detection::ShadowAIDetector,
    governance: governance::GovernanceEngine,
    risk_engine: risk::RiskAssessmentEngine,
    response_engine: response::ResponseEngine,
}

impl ShadowAIManager {
    /// Create a new Shadow AI Manager
    pub fn new(config: ShadowAIConfig) -> Self {
        Self {
            detector: detection::ShadowAIDetector::new(
                detection::DiscoveryConfig::default()
            ),
            governance: governance::GovernanceEngine::new(
                governance::GovernanceConfig::default()
            ),
            risk_engine: risk::RiskAssessmentEngine::new(
                risk::RiskConfig::default()
            ),
            response_engine: response::ResponseEngine::new(
                response::ResponseConfig::default()
            ),
            config,
        }
    }

    /// Run a full scan for Shadow AI
    pub async fn run_full_scan(&self) -> Result<ShadowAISummary, ShadowAIError> {
        // Discover AI models
        let discovery = self.detector.discover_ai_models().await?;
        
        let mut summary = ShadowAISummary::default();
        summary.total_models = discovery.detected_models.len();
        
        // Assess risk for each discovered model
        for model in &discovery.detected_models {
            let risk_score = self.risk_engine.assess_model_risk(model).await?;
            
            // Update risk counts
            match risk_score.risk_level {
                RiskLevel::Critical => {
                    *summary.models_by_risk.entry(RiskLevel::Critical).or_insert(0) += 1;
                }
                RiskLevel::High => {
                    *summary.models_by_risk.entry(RiskLevel::High).or_insert(0) += 1;
                }
                RiskLevel::Medium => {
                    *summary.models_by_risk.entry(RiskLevel::Medium).or_insert(0) += 1;
                }
                RiskLevel::Low => {
                    *summary.models_by_risk.entry(RiskLevel::Low).or_insert(0) += 1;
                }
            }

            // Process response
            self.response_engine.process_detection(model, &risk_score).await?;

            // Evaluate governance
            let evaluation = self.governance.evaluate_model(model).await;
            if evaluation.requires_registration && !model.registered {
                summary.unregistered_models += 1;
            } else if model.registered {
                summary.registered_models += 1;
            }
        }

        // Get API calls count
        summary.total_api_calls = discovery.api_calls.len();

        Ok(summary)
    }

    /// Get all discovered models
    pub async fn get_discovered_models(&self) -> Vec<DetectedAIModel> {
        self.detector.get_discovered_models().await
    }

    /// Get all alerts
    pub async fn get_alerts(&self) -> Vec<Alert> {
        self.response_engine.get_alert_history().await
    }

    /// Get compliance report
    pub async fn get_compliance_report(&self) -> ComplianceReport {
        self.governance.generate_compliance_report().await
    }

    /// Get risk report
    pub async fn get_risk_report(&self) -> RiskReport {
        let models = self.detector.get_discovered_models().await;
        self.risk_engine.generate_risk_report(&models).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config() {
        let config = ShadowAIConfig::default();
        assert!(config.detection.enable_discovery);
        assert!(config.governance.require_registration);
        assert!(config.risk.enable_risk_scoring);
        assert!(config.response.enable_automated_response);
    }

    #[test]
    fn test_manager_creation() {
        let config = ShadowAIConfig::default();
        let manager = ShadowAIManager::new(config);
        assert!(true); // Just verify it doesn't panic
    }
}