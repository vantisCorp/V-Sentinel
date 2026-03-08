//! Data models for AI Security module

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// AI System representation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AISystem {
    /// System ID
    pub id: String,
    /// System name
    pub name: String,
    /// System description
    pub description: String,
    /// System type
    pub system_type: AISystemType,
    /// Data pipelines
    pub data_pipelines: Vec<DataPipeline>,
    /// AI models
    pub models: Vec<AIModel>,
    /// API endpoints
    pub api_endpoints: Vec<APIEndpoint>,
    /// Whether system is secured
    pub secured: bool,
    /// Security level (1-10)
    pub security_level: u8,
    /// Creation timestamp
    pub created_at: DateTime<Utc>,
    /// Last security assessment
    pub last_assessment: Option<DateTime<Utc>>,
    /// Metadata
    pub metadata: HashMap<String, String>,
}

impl AISystem {
    /// Create a new AI system
    pub fn new(name: &str, system_type: AISystemType) -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            name: name.to_string(),
            description: String::new(),
            system_type,
            data_pipelines: Vec::new(),
            models: Vec::new(),
            api_endpoints: Vec::new(),
            secured: false,
            security_level: 0,
            created_at: Utc::now(),
            last_assessment: None,
            metadata: HashMap::new(),
        }
    }

    /// Add a data pipeline
    pub fn add_pipeline(&mut self, pipeline: DataPipeline) {
        self.data_pipelines.push(pipeline);
    }

    /// Add a model
    pub fn add_model(&mut self, model: AIModel) {
        self.models.push(model);
    }

    /// Add an API endpoint
    pub fn add_endpoint(&mut self, endpoint: APIEndpoint) {
        self.api_endpoints.push(endpoint);
    }
}

/// AI System Type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AISystemType {
    /// Large Language Model
    LLM,
    /// Computer Vision
    ComputerVision,
    /// Natural Language Processing
    NLP,
    /// Recommendation System
    Recommendation,
    /// Anomaly Detection
    AnomalyDetection,
    /// Time Series Analysis
    TimeSeries,
    /// Multi-modal System
    MultiModal,
    /// Custom System
    Custom,
}

/// AI Model representation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AIModel {
    /// Model ID
    pub id: String,
    /// Model name
    pub name: String,
    /// Model version
    pub version: String,
    /// Model type
    pub model_type: ModelType,
    /// Model format
    pub format: ModelFormat,
    /// Model size in bytes
    pub size: u64,
    /// Model hash
    pub hash: Option<String>,
    /// Whether model is encrypted
    pub encrypted: bool,
    /// Encryption key ID
    pub encryption_key_id: Option<String>,
    /// Whether model is watermarked
    pub watermarked: bool,
    /// Watermark ID
    pub watermark_id: Option<String>,
    /// Training data sources
    pub training_sources: Vec<String>,
    /// Model capabilities
    pub capabilities: Vec<String>,
    /// Security vulnerabilities
    pub vulnerabilities: Vec<ModelVulnerability>,
    /// Creation timestamp
    pub created_at: DateTime<Utc>,
    /// Last modified
    pub modified_at: DateTime<Utc>,
}

impl AIModel {
    /// Create a new AI model
    pub fn new(name: &str, version: &str, model_type: ModelType) -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            name: name.to_string(),
            version: version.to_string(),
            model_type,
            format: ModelFormat::PyTorch,
            size: 0,
            hash: None,
            encrypted: false,
            encryption_key_id: None,
            watermarked: false,
            watermark_id: None,
            training_sources: Vec::new(),
            capabilities: Vec::new(),
            vulnerabilities: Vec::new(),
            created_at: Utc::now(),
            modified_at: Utc::now(),
        }
    }
}

/// Model type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ModelType {
    /// Transformer model
    Transformer,
    /// CNN model
    CNN,
    /// RNN model
    RNN,
    /// GAN model
    GAN,
    /// Diffusion model
    Diffusion,
    /// Autoencoder
    Autoencoder,
    /// Ensemble model
    Ensemble,
    /// Custom model
    Custom,
}

/// Model format
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ModelFormat {
    PyTorch,
    TensorFlow,
    ONNX,
    TensorRT,
    CoreML,
    TFLite,
    Custom,
}

/// Model vulnerability
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelVulnerability {
    /// Vulnerability ID
    pub id: String,
    /// Vulnerability name
    pub name: String,
    /// Vulnerability type
    pub vuln_type: VulnerabilityType,
    /// Severity
    pub severity: ThreatLevel,
    /// Description
    pub description: String,
    /// Mitigation
    pub mitigation: Option<String>,
    /// Whether mitigated
    pub mitigated: bool,
}

/// Vulnerability type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum VulnerabilityType {
    /// Adversarial attack vulnerability
    AdversarialPerturbation,
    /// Model extraction vulnerability
    ModelExtraction,
    /// Membership inference vulnerability
    MembershipInference,
    /// Data poisoning vulnerability
    DataPoisoning,
    /// Backdoor vulnerability
    Backdoor,
    /// Model inversion vulnerability
    ModelInversion,
    /// Evasion attack vulnerability
    EvasionAttack,
}

/// Data Pipeline representation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataPipeline {
    /// Pipeline ID
    pub id: String,
    /// Pipeline name
    pub name: String,
    /// Pipeline type
    pub pipeline_type: PipelineType,
    /// Data sources
    pub sources: Vec<DataSource>,
    /// Data transformations
    pub transformations: Vec<DataTransformation>,
    /// Data classification
    pub classification: DataClassification,
    /// Whether pipeline is encrypted
    pub encrypted: bool,
    /// Access controls
    pub access_controls: Vec<AccessControl>,
    /// Creation timestamp
    pub created_at: DateTime<Utc>,
}

impl DataPipeline {
    /// Create a new data pipeline
    pub fn new(name: &str, pipeline_type: PipelineType) -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            name: name.to_string(),
            pipeline_type,
            sources: Vec::new(),
            transformations: Vec::new(),
            classification: DataClassification::Internal,
            encrypted: false,
            access_controls: Vec::new(),
            created_at: Utc::now(),
        }
    }
}

/// Pipeline type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PipelineType {
    /// Training pipeline
    Training,
    /// Inference pipeline
    Inference,
    /// Evaluation pipeline
    Evaluation,
    /// Data preprocessing
    Preprocessing,
    /// Feature extraction
    FeatureExtraction,
}

/// Data source
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataSource {
    /// Source ID
    pub id: String,
    /// Source name
    pub name: String,
    /// Source type
    pub source_type: DataSourceType,
    /// Source location/URI
    pub location: String,
    /// Data format
    pub format: String,
    /// Whether source is verified
    pub verified: bool,
}

/// Data source type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DataSourceType {
    File,
    Database,
    API,
    Stream,
    Bucket,
    Custom,
}

/// Data transformation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataTransformation {
    /// Transformation name
    pub name: String,
    /// Transformation type
    pub transformation_type: String,
    /// Parameters
    pub parameters: HashMap<String, serde_json::Value>,
}

/// Data classification level
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DataClassification {
    /// Public data
    Public,
    /// Internal data
    Internal,
    /// Confidential data
    Confidential,
    /// Highly confidential
    HighlyConfidential,
    /// Restricted
    Restricted,
}

/// Access control rule
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccessControl {
    /// Rule ID
    pub id: String,
    /// Principal (user/role)
    pub principal: String,
    /// Permission
    pub permission: Permission,
    /// Resource
    pub resource: String,
    /// Conditions
    pub conditions: Vec<AccessCondition>,
    /// Whether rule is active
    pub active: bool,
}

impl AccessControl {
    /// Create a new access control rule
    pub fn new(principal: &str, permission: Permission, resource: &str) -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            principal: principal.to_string(),
            permission,
            resource: resource.to_string(),
            conditions: Vec::new(),
            active: true,
        }
    }
}

/// Permission type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Permission {
    Read,
    Write,
    Execute,
    Admin,
    Delete,
}

/// Access condition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccessCondition {
    /// Condition type
    pub condition_type: String,
    /// Condition value
    pub value: String,
}

/// API Endpoint representation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct APIEndpoint {
    /// Endpoint ID
    pub id: String,
    /// Endpoint path
    pub path: String,
    /// HTTP methods allowed
    pub methods: Vec<String>,
    /// Whether endpoint requires authentication
    pub requires_auth: bool,
    /// Rate limit
    pub rate_limit: Option<u32>,
    /// Input validation schema
    pub input_schema: Option<serde_json::Value>,
    /// Output schema
    pub output_schema: Option<serde_json::Value>,
}

impl APIEndpoint {
    /// Create a new API endpoint
    pub fn new(path: &str) -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            path: path.to_string(),
            methods: vec!["GET".to_string(), "POST".to_string()],
            requires_auth: true,
            rate_limit: Some(60),
            input_schema: None,
            output_schema: None,
        }
    }
}

/// Security policy
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityPolicy {
    /// Policy ID
    pub id: String,
    /// Policy name
    pub name: String,
    /// Policy description
    pub description: String,
    /// Policy type
    pub policy_type: PolicyType,
    /// Policy rules
    pub rules: Vec<PolicyRule>,
    /// Whether policy is active
    pub active: bool,
    /// Creation timestamp
    pub created_at: DateTime<Utc>,
    /// Last modified
    pub modified_at: DateTime<Utc>,
}

impl SecurityPolicy {
    /// Create a new security policy
    pub fn new(name: &str, policy_type: PolicyType) -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            name: name.to_string(),
            description: String::new(),
            policy_type,
            rules: Vec::new(),
            active: true,
            created_at: Utc::now(),
            modified_at: Utc::now(),
        }
    }
}

/// Policy type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PolicyType {
    DataSecurity,
    ModelSecurity,
    APIAccess,
    Encryption,
    Audit,
    Compliance,
    Custom,
}

/// Policy rule
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PolicyRule {
    /// Rule name
    pub name: String,
    /// Rule condition
    pub condition: String,
    /// Rule action
    pub action: String,
    /// Rule priority
    pub priority: u32,
}

/// Security event
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityEvent {
    /// Event ID
    pub id: String,
    /// Event type
    pub event_type: SecurityEventType,
    /// Related system ID
    pub system_id: Option<String>,
    /// Related model ID
    pub model_id: Option<String>,
    /// Event description
    pub description: String,
    /// Severity
    pub severity: ThreatLevel,
    /// Timestamp
    pub timestamp: DateTime<Utc>,
    /// Additional metadata
    pub metadata: HashMap<String, String>,
}

/// Threat level
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ThreatLevel {
    Info,
    Low,
    Medium,
    High,
    Critical,
}

impl std::fmt::Display for ThreatLevel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ThreatLevel::Info => write!(f, "info"),
            ThreatLevel::Low => write!(f, "low"),
            ThreatLevel::Medium => write!(f, "medium"),
            ThreatLevel::High => write!(f, "high"),
            ThreatLevel::Critical => write!(f, "critical"),
        }
    }
}

/// Security event type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SecurityEventType {
    SystemSecured,
    PromptInjectionDetected,
    DataBreachPrevented,
    ModelTamperingDetected,
    UnauthorizedAccess,
    PolicyViolation,
    AnomalyDetected,
}

/// Encryption configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EncryptionConfig {
    /// Encryption algorithm
    pub algorithm: EncryptionAlgorithm,
    /// Key size in bits
    pub key_size: u32,
    /// Key derivation function
    pub kdf: KeyDerivationFunction,
}

impl Default for EncryptionConfig {
    fn default() -> Self {
        Self {
            algorithm: EncryptionAlgorithm::AES256GCM,
            key_size: 256,
            kdf: KeyDerivationFunction::HKDF,
        }
    }
}

/// Encryption algorithm
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EncryptionAlgorithm {
    AES256GCM,
    ChaCha20Poly1305,
    AES256CBC,
}

/// Key derivation function
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum KeyDerivationFunction {
    HKDF,
    PBKDF2,
    Argon2,
}

/// Model watermark configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelWatermarkConfig {
    /// Watermark type
    pub watermark_type: ModelWatermarkType,
    /// Watermark payload
    pub payload: String,
    /// Whether watermark is detectable
    pub detectable: bool,
    /// Robustness level (1-10)
    pub robustness: u8,
}

impl Default for ModelWatermarkConfig {
    fn default() -> Self {
        Self {
            watermark_type: ModelWatermarkType::WeightEmbedding,
            payload: uuid::Uuid::new_v4().to_string(),
            detectable: true,
            robustness: 7,
        }
    }
}

/// Model watermark type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ModelWatermarkType {
    WeightEmbedding,
    BackdoorTrigger,
    OutputFingerprint,
    GradientBased,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ai_system_creation() {
        let system = AISystem::new("Test System", AISystemType::LLM);
        assert!(!system.id.is_empty());
        assert_eq!(system.name, "Test System");
        assert!(!system.secured);
    }

    #[test]
    fn test_ai_model_creation() {
        let model = AIModel::new("GPT-4", "1.0", ModelType::Transformer);
        assert!(!model.id.is_empty());
        assert_eq!(model.name, "GPT-4");
        assert!(!model.encrypted);
    }

    #[test]
    fn test_data_pipeline_creation() {
        let pipeline = DataPipeline::new("Training Pipeline", PipelineType::Training);
        assert!(!pipeline.id.is_empty());
        assert_eq!(pipeline.pipeline_type, PipelineType::Training);
    }

    #[test]
    fn test_security_policy_creation() {
        let policy = SecurityPolicy::new("API Security Policy", PolicyType::APIAccess);
        assert!(!policy.id.is_empty());
        assert!(policy.active);
    }
}