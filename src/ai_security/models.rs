//! Shared data models for AI Security Module

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// =============================================================================
// Data Security Types
// =============================================================================

/// Data pipeline security level
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum DataSecurityLevel {
    Public,
    Internal,
    Confidential,
    Restricted,
    TopSecret,
}

/// Data input for security processing
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataInput {
    /// Unique identifier
    pub id: String,
    /// Data source
    pub source: String,
    /// Data type
    pub data_type: DataType,
    /// Data content or reference
    pub content: DataContent,
    /// Metadata
    pub metadata: HashMap<String, String>,
    /// Security classification
    pub classification: DataSecurityLevel,
    /// Creation timestamp
    pub created_at: DateTime<Utc>,
}

/// Types of data in AI pipelines
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum DataType {
    Training,
    Validation,
    Test,
    Inference,
    Feature,
    Label,
    Embedding,
    Metadata,
}

/// Data content representation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DataContent {
    Text(String),
    Binary(Vec<u8>),
    Reference(String),
    Structured(serde_json::Value),
}

/// Data lineage record
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataLineage {
    /// Data ID
    pub data_id: String,
    /// Source system
    pub source_system: String,
    /// Transformations applied
    pub transformations: Vec<DataTransformation>,
    /// Destination systems
    pub destinations: Vec<String>,
    /// Timestamp
    pub timestamp: DateTime<Utc>,
}

/// Data transformation record
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataTransformation {
    /// Transformation type
    pub transform_type: String,
    /// Description
    pub description: String,
    /// Parameters used
    pub parameters: HashMap<String, serde_json::Value>,
    /// Timestamp
    pub timestamp: DateTime<Utc>,
}

/// Data poisoning detection result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PoisoningDetectionResult {
    /// Data ID
    pub data_id: String,
    /// Is poisoned
    pub is_poisoned: bool,
    /// Confidence score
    pub confidence: f64,
    /// Poisoning type
    pub poisoning_type: Option<PoisoningType>,
    /// Affected samples
    pub affected_samples: Vec<String>,
    /// Detection method
    pub detection_method: String,
    /// Timestamp
    pub timestamp: DateTime<Utc>,
}

/// Types of data poisoning
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum PoisoningType {
    LabelFlipping,
    Backdoor,
    TriggerInjection,
    NoiseInjection,
    CleanLabel,
    Targeted,
    Untargeted,
}

// =============================================================================
// Model Security Types
// =============================================================================

/// Model input for protection
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelInput {
    /// Model identifier
    pub id: String,
    /// Model name
    pub name: String,
    /// Model type
    pub model_type: ModelType,
    /// Model format
    pub format: ModelFormat,
    /// Model data (weights, architecture)
    pub data: Vec<u8>,
    /// Metadata
    pub metadata: HashMap<String, String>,
}

/// Types of AI models
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ModelType {
    Classification,
    Regression,
    Generation,
    Embedding,
    ReinforcementLearning,
    Clustering,
    AnomalyDetection,
    Recommender,
}

/// Model format
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ModelFormat {
    ONNX,
    TensorFlow,
    PyTorch,
    SKLearn,
    XGBoost,
    Custom,
}

/// Protected model with security features
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProtectedModel {
    /// Original model ID
    pub model_id: String,
    /// Encrypted model data
    pub encrypted_data: Vec<u8>,
    /// Digital signature
    pub signature: String,
    /// Watermark
    pub watermark: Option<ModelWatermark>,
    /// Integrity hash
    pub integrity_hash: String,
    /// Protection timestamp
    pub protected_at: DateTime<Utc>,
}

/// Model watermark
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelWatermark {
    /// Watermark ID
    pub id: String,
    /// Watermark type
    pub watermark_type: ModelWatermarkType,
    /// Embedding method
    pub embedding_method: String,
    /// Owner identifier
    pub owner: String,
    /// Creation timestamp
    pub created_at: DateTime<Utc>,
}

/// Model watermark types
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ModelWatermarkType {
    WeightsEmbedding,
    ArchitectureEmbedding,
    OutputBased,
    TriggerSet,
}

/// Model integrity verification result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelIntegrityResult {
    /// Model ID
    pub model_id: String,
    /// Is valid
    pub is_valid: bool,
    /// Integrity score
    pub integrity_score: f64,
    /// Detected modifications
    pub modifications: Vec<ModelModification>,
    /// Verification timestamp
    pub verified_at: DateTime<Utc>,
}

/// Model modification record
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelModification {
    /// Modification type
    pub modification_type: String,
    /// Layer affected
    pub layer: Option<String>,
    /// Description
    pub description: String,
    /// Severity
    pub severity: Severity,
}

// =============================================================================
// API Security Types
// =============================================================================

/// AI API Request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct APIRequest {
    /// Request ID
    pub id: String,
    /// Client identifier
    pub client_id: String,
    /// API endpoint
    pub endpoint: String,
    /// Request payload
    pub payload: serde_json::Value,
    /// Authentication token
    pub auth_token: String,
    /// Request timestamp
    pub timestamp: DateTime<Utc>,
    /// Client IP
    pub client_ip: String,
    /// User agent
    pub user_agent: String,
}

/// AI API Response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct APIResponse {
    /// Request ID
    pub request_id: String,
    /// Response status
    pub status: APIResponseStatus,
    /// Response data
    pub data: Option<serde_json::Value>,
    /// Security warnings
    pub security_warnings: Vec<String>,
    /// Rate limit info
    pub rate_limit: RateLimitInfo,
    /// Response timestamp
    pub timestamp: DateTime<Utc>,
}

/// API response status
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum APIResponseStatus {
    Success,
    Rejected,
    Throttled,
    Unauthorized,
    Blocked,
    Error,
}

/// Rate limit information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RateLimitInfo {
    /// Requests allowed per window
    pub limit: u32,
    /// Requests remaining
    pub remaining: u32,
    /// Reset timestamp
    pub reset_at: DateTime<Utc>,
}

/// Prompt injection detection result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PromptInjectionResult {
    /// Request ID
    pub request_id: String,
    /// Is injection detected
    pub is_injection: bool,
    /// Injection type
    pub injection_type: Option<InjectionType>,
    /// Confidence score
    pub confidence: f64,
    /// Detected patterns
    pub patterns: Vec<String>,
    /// Mitigation applied
    pub mitigation: Option<String>,
    /// Timestamp
    pub timestamp: DateTime<Utc>,
}

/// Types of prompt injection
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum InjectionType {
    DirectInjection,
    IndirectInjection,
    Jailbreak,
    RolePlay,
    DAN,
    TokenSmuggling,
    MultiModal,
}

// =============================================================================
// MLOps Security Types
// =============================================================================

/// MLOps metrics for monitoring
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MLOpsMetrics {
    /// Pipeline ID
    pub pipeline_id: String,
    /// Model version
    pub model_version: String,
    /// Performance metrics
    pub performance: PerformanceMetrics,
    /// Drift metrics
    pub drift: Option<DriftMetrics>,
    /// Resource metrics
    pub resources: ResourceMetrics,
    /// Timestamp
    pub timestamp: DateTime<Utc>,
}

/// Performance metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceMetrics {
    /// Accuracy
    pub accuracy: Option<f64>,
    /// Precision
    pub precision: Option<f64>,
    /// Recall
    pub recall: Option<f64>,
    /// F1 Score
    pub f1_score: Option<f64>,
    /// Latency (ms)
    pub latency_ms: f64,
    /// Throughput (requests/sec)
    pub throughput: f64,
}

/// Drift metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DriftMetrics {
    /// Data drift score
    pub data_drift: f64,
    /// Model drift score
    pub model_drift: f64,
    /// Concept drift score
    pub concept_drift: f64,
    /// Is drift detected
    pub drift_detected: bool,
}

/// Resource metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceMetrics {
    /// CPU usage %
    pub cpu_usage: f64,
    /// Memory usage %
    pub memory_usage: f64,
    /// GPU usage %
    pub gpu_usage: Option<f64>,
    /// Disk I/O (MB/s)
    pub disk_io: f64,
    /// Network I/O (MB/s)
    pub network_io: f64,
}

/// MLOps security report
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MLOpsReport {
    /// Pipeline ID
    pub pipeline_id: String,
    /// Security status
    pub status: MLOpsSecurityStatus,
    /// Issues detected
    pub issues: Vec<MLOpsIssue>,
    /// Recommendations
    pub recommendations: Vec<String>,
    /// Report timestamp
    pub generated_at: DateTime<Utc>,
}

/// MLOps security status
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum MLOpsSecurityStatus {
    Secure,
    Warning,
    AtRisk,
    Compromised,
}

/// MLOps security issue
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MLOpsIssue {
    /// Issue ID
    pub id: String,
    /// Issue type
    pub issue_type: MLOpsIssueType,
    /// Description
    pub description: String,
    /// Severity
    pub severity: Severity,
    /// Affected component
    pub component: String,
    /// Timestamp
    pub detected_at: DateTime<Utc>,
}

/// MLOps issue types
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum MLOpsIssueType {
    ModelDrift,
    DataDrift,
    ConceptDrift,
    PerformanceDegradation,
    SecurityViolation,
    UnauthorizedAccess,
    ConfigurationDrift,
    InfrastructureCompromise,
}

// =============================================================================
// Threat Defense Types
// =============================================================================

/// AI Threat representation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AIThreat {
    /// Threat ID
    pub id: String,
    /// Threat type
    pub threat_type: AIThreatType,
    /// Source
    pub source: String,
    /// Target
    pub target: String,
    /// Severity
    pub severity: Severity,
    /// Indicators
    pub indicators: Vec<ThreatIndicator>,
    /// Timestamp
    pub detected_at: DateTime<Utc>,
}

/// AI threat types
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum AIThreatType {
    AdversarialAttack,
    ModelExtraction,
    ModelInversion,
    DataPoisoning,
    PromptInjection,
    EvasionAttack,
    BackdoorAttack,
    MembershipInference,
    AttributeInference,
    ModelStealing,
}

/// Threat indicator
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThreatIndicator {
    /// Indicator type
    pub indicator_type: String,
    /// Value
    pub value: String,
    /// Confidence
    pub confidence: f64,
    /// Context
    pub context: String,
}

/// Threat response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThreatResponse {
    /// Threat ID
    pub threat_id: String,
    /// Response actions taken
    pub actions: Vec<ResponseAction>,
    /// Response status
    pub status: ResponseStatus,
    /// Additional recommendations
    pub recommendations: Vec<String>,
    /// Response timestamp
    pub responded_at: DateTime<Utc>,
}

/// Response action
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResponseAction {
    /// Action type
    pub action_type: ResponseActionType,
    /// Description
    pub description: String,
    /// Result
    pub result: ActionResult,
    /// Timestamp
    pub executed_at: DateTime<Utc>,
}

/// Response action types
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ResponseActionType {
    Block,
    Quarantine,
    Alert,
    Mitigate,
    Investigate,
    Rollback,
    Patch,
    Isolate,
}

/// Action result
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ActionResult {
    Success,
    Partial,
    Failed,
    Pending,
}

/// Response status
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ResponseStatus {
    Resolved,
    Mitigated,
    Monitoring,
    Escalated,
    Unresolved,
}

// =============================================================================
// Common Types
// =============================================================================

/// Severity levels
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, PartialOrd)]
pub enum Severity {
    Low,
    Medium,
    High,
    Critical,
}

/// Secure pipeline result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurePipeline {
    /// Data ID
    pub data_id: String,
    /// Encryption status
    pub encrypted: bool,
    /// Lineage tracked
    pub lineage_tracked: bool,
    /// Poisoning check passed
    pub poisoning_check: bool,
    /// Security score
    pub security_score: f64,
    /// Timestamp
    pub secured_at: DateTime<Utc>,
}

/// Overall security status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityStatus {
    /// Data security status
    pub data_security: ComponentStatus,
    /// Model security status
    pub model_security: ComponentStatus,
    /// API security status
    pub api_security: ComponentStatus,
    /// MLOps security status
    pub mlops_security: ComponentStatus,
    /// Threat defense status
    pub threat_defense: ComponentStatus,
    /// Timestamp
    pub timestamp: DateTime<Utc>,
}

/// Component security status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComponentStatus {
    /// Is healthy
    pub healthy: bool,
    /// Active protections
    pub active_protections: u32,
    /// Threats detected
    pub threats_detected: u32,
    /// Last check timestamp
    pub last_check: DateTime<Utc>,
}

/// AI Security Error
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AISecurityError {
    /// Error code
    pub code: String,
    /// Error message
    pub message: String,
    /// Error details
    pub details: Option<String>,
}

impl std::fmt::Display for AISecurityError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "AISecurityError: {} - {}", self.code, self.message)
    }
}

impl std::error::Error for AISecurityError {}

// Helper implementations
impl AISecurityError {
    pub fn new(code: impl Into<String>, message: impl Into<String>) -> Self {
        Self {
            code: code.into(),
            message: message.into(),
            details: None,
        }
    }

    pub fn with_details(code: impl Into<String>, message: impl Into<String>, details: impl Into<String>) -> Self {
        Self {
            code: code.into(),
            message: message.into(),
            details: Some(details.into()),
        }
    }
}