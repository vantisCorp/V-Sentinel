use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Detected AI Model
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DetectedAIModel {
    /// Unique identifier for the model
    pub model_id: String,
    /// Model name
    pub model_name: String,
    /// Model type
    pub model_type: AIModelType,
    /// Path to model file/directory
    pub model_path: String,
    /// Model size in bytes
    pub model_size_bytes: u64,
    /// SHA-256 hash of the model file
    pub file_hash: String,
    /// Fingerprint hash
    pub fingerprint: String,
    /// Framework used (PyTorch, TensorFlow, etc.)
    pub framework: Option<String>,
    /// Framework version
    pub version: Option<String>,
    /// When the model was discovered
    pub discovered_at: DateTime<Utc>,
    /// When the model was last modified
    pub last_modified: DateTime<Utc>,
    /// Risk level assessment
    pub risk_level: RiskLevel,
    /// Whether the model is registered
    pub registered: bool,
    /// Owner of the model
    pub owner: Option<String>,
    /// Description of the model
    pub description: Option<String>,
}

/// AI Model Type
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum AIModelType {
    /// Large Language Models (GPT-like)
    LargeLanguageModel,
    /// Generative AI (diffusion models, etc.)
    GenerativeAI,
    /// Computer Vision models
    ComputerVision,
    /// Natural Language Processing models
    NaturalLanguageProcessing,
    /// General neural networks
    NeuralNetwork,
    /// Traditional machine learning models
    MachineLearning,
    /// Unknown type
    Unknown,
}

/// Risk Level
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum RiskLevel {
    Low,
    Medium,
    High,
    Critical,
}

/// Registration Status
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum RegistrationStatus {
    Pending,
    Approved,
    Rejected,
    Expired,
    UnderReview,
}

/// Shadow AI Error Type
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

/// AI Service configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AIServiceConfig {
    /// Service name
    pub name: String,
    /// Service endpoint
    pub endpoint: String,
    /// API key (encrypted)
    pub api_key: Option<String>,
    /// Required permissions
    pub required_permissions: Vec<String>,
    /// Allowed models
    pub allowed_models: Vec<String>,
    /// Rate limits
    pub rate_limits: RateLimits,
}

/// Rate limits for AI service usage
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RateLimits {
    /// Requests per minute
    pub requests_per_minute: u64,
    /// Tokens per minute
    pub tokens_per_minute: u64,
    /// Maximum concurrent requests
    pub max_concurrent: u64,
}

impl Default for RateLimits {
    fn default() -> Self {
        Self {
            requests_per_minute: 60,
            tokens_per_minute: 100_000,
            max_concurrent: 10,
        }
    }
}

/// Shadow AI summary statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShadowAISummary {
    /// Total discovered models
    pub total_models: usize,
    /// Registered models
    pub registered_models: usize,
    /// Unregistered models
    pub unregistered_models: usize,
    /// Models by type
    pub models_by_type: HashMap<AIModelType, usize>,
    /// Models by risk level
    pub models_by_risk: HashMap<RiskLevel, usize>,
    /// Total AI API calls detected
    pub total_api_calls: usize,
    /// API calls by provider
    pub api_calls_by_provider: HashMap<String, usize>,
    /// Active alerts
    pub active_alerts: usize,
    /// Blocked models
    pub blocked_models: usize,
}

impl Default for ShadowAISummary {
    fn default() -> Self {
        Self {
            total_models: 0,
            registered_models: 0,
            unregistered_models: 0,
            models_by_type: HashMap::new(),
            models_by_risk: HashMap::new(),
            total_api_calls: 0,
            api_calls_by_provider: HashMap::new(),
            active_alerts: 0,
            blocked_models: 0,
        }
    }
}

/// AI Usage statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AIUsageStats {
    /// Period start
    pub period_start: DateTime<Utc>,
    /// Period end
    pub period_end: DateTime<Utc>,
    /// Total API calls
    pub total_calls: u64,
    /// Calls by provider
    pub calls_by_provider: HashMap<String, u64>,
    /// Calls by model type
    pub calls_by_type: HashMap<String, u64>,
    /// Total tokens used
    pub total_tokens: u64,
    /// Total data volume in bytes
    pub total_data_volume: u64,
    /// Average response time in ms
    pub avg_response_time_ms: f64,
    /// Error rate (0.0-1.0)
    pub error_rate: f64,
}

/// Model lineage record
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelLineage {
    /// Model ID
    pub model_id: String,
    /// Parent model (if derived)
    pub parent_model: Option<String>,
    /// Training data sources
    pub training_sources: Vec<String>,
    /// Fine-tuning history
    pub fine_tuning_history: Vec<FineTuningRecord>,
    /// Model version
    pub version: String,
    /// Created at
    pub created_at: DateTime<Utc>,
    /// Created by
    pub created_by: String,
}

/// Fine-tuning record
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FineTuningRecord {
    /// Record ID
    pub record_id: String,
    /// Timestamp
    pub timestamp: DateTime<Utc>,
    /// Training data used
    pub training_data: Vec<String>,
    /// Base model
    pub base_model: String,
    /// Output model name
    pub output_model: String,
    /// Training duration in seconds
    pub duration_secs: u64,
    /// Training metrics
    pub metrics: TrainingMetrics,
}

/// Training metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrainingMetrics {
    /// Final loss
    pub final_loss: f64,
    /// Number of epochs
    pub epochs: u64,
    /// Learning rate
    pub learning_rate: f64,
    /// Batch size
    pub batch_size: u64,
    /// Accuracy on validation set
    pub validation_accuracy: Option<f64>,
}

/// Data access pattern for AI models
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataAccessPattern {
    /// Pattern ID
    pub pattern_id: String,
    /// Model ID
    pub model_id: String,
    /// Data source being accessed
    pub data_source: String,
    /// Access frequency (per day)
    pub access_frequency: f64,
    /// Average data size per access
    pub avg_data_size: u64,
    /// Access times (hourly distribution)
    pub access_distribution: Vec<u64>,
    /// Last accessed
    pub last_accessed: DateTime<Utc>,
    /// Risk assessment
    pub data_risk: RiskLevel,
}

/// Compliance status for AI model
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AIComplianceStatus {
    /// Model ID
    pub model_id: String,
    /// Compliance frameworks checked
    pub frameworks: HashMap<String, FrameworkCompliance>,
    /// Overall compliant status
    pub overall_compliant: bool,
    /// Last assessment
    pub last_assessed: DateTime<Utc>,
    /// Next assessment due
    pub next_assessment: DateTime<Utc>,
}

/// Framework compliance details
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FrameworkCompliance {
    /// Framework name
    pub name: String,
    /// Is compliant
    pub compliant: bool,
    /// Issues found
    pub issues: Vec<ComplianceIssue>,
    /// Assessment date
    pub assessed_at: DateTime<Utc>,
}

/// Compliance issue
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComplianceIssue {
    /// Issue ID
    pub issue_id: String,
    /// Issue severity
    pub severity: String,
    /// Issue description
    pub description: String,
    /// Remediation steps
    pub remediation: Vec<String>,
    /// Due date
    pub due_date: DateTime<Utc>,
}

/// AI Security assessment
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AISecurityAssessment {
    /// Assessment ID
    pub assessment_id: String,
    /// Model ID
    pub model_id: String,
    /// Assessment timestamp
    pub timestamp: DateTime<Utc>,
    /// Vulnerabilities found
    pub vulnerabilities: Vec<AIVulnerability>,
    /// Security score (0.0-1.0)
    pub security_score: f64,
    /// Recommended actions
    pub recommendations: Vec<String>,
}

/// AI Vulnerability
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AIVulnerability {
    /// Vulnerability ID
    pub vuln_id: String,
    /// Vulnerability type
    pub vuln_type: AIVulnerabilityType,
    /// Severity (0.0-1.0)
    pub severity: f64,
    /// Description
    pub description: String,
    /// CVSS score
    pub cvss_score: Option<f64>,
    /// Remediation steps
    pub remediation: Vec<String>,
}

/// AI Vulnerability Types
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum AIVulnerabilityType {
    /// Model poisoning
    ModelPoisoning,
    /// Data extraction attack
    DataExtraction,
    /// Model inversion attack
    ModelInversion,
    /// Membership inference attack
    MembershipInference,
    /// Prompt injection
    PromptInjection,
    /// Output manipulation
    OutputManipulation,
    /// Adversarial input
    AdversarialInput,
    /// Data leakage
    DataLeakage,
    /// Unauthorized access
    UnauthorizedAccess,
    /// Configuration issue
    ConfigurationIssue,
    /// Other
    Other(String),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_risk_level_ordering() {
        assert!(RiskLevel::Critical > RiskLevel::High);
        assert!(RiskLevel::High > RiskLevel::Medium);
        assert!(RiskLevel::Medium > RiskLevel::Low);
    }

    #[test]
    fn test_default_shadow_ai_summary() {
        let summary = ShadowAISummary::default();
        assert_eq!(summary.total_models, 0);
        assert_eq!(summary.active_alerts, 0);
    }

    #[test]
    fn test_ai_model_type_serialization() {
        let model_type = AIModelType::LargeLanguageModel;
        let serialized = serde_json::to_string(&model_type).unwrap();
        assert!(serialized.contains("LargeLanguageModel"));
    }
}