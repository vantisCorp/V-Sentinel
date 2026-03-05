//! Zero Trust Architecture Implementation
//! 
//! This module provides comprehensive Zero Trust capabilities including:
//! - Policy Engine for access control decisions
//! - Continuous authentication with behavioral biometrics
//! - Micro-segmentation for network, application, and data
//! - Identity fabric for unified identity management
//! - Trust scoring for real-time access evaluation

pub mod policy;
pub mod auth;
pub mod segmentation;
pub mod identity;
pub mod trust;

pub use trust::trust_score::{TrustScore, TrustLevel};

// Policy module exports
pub use policy::{
    // Policy Engine
    PolicyEngine, Policy, PolicyType, PolicyCondition, PolicyAction,
    ConditionType, Operator,
    // Policy Language
    PolicyLanguageManager, PolicyLanguageConfig, PolicyExpression,
    PolicyTemplate, PolicyVersion,
    // Enforcement
    EnforcementPointManager, EnforcementConfig, EnforcementStats,
    ApiGatewayEnforcer, ApiGatewayConfig, ApiRequest,
    ServiceMeshEnforcer, ServiceMeshConfig, MeshRequest,
    DatabaseEnforcer, DatabaseConfig, DatabaseRequest, QueryType,
    // Validation
    PolicyValidator, ValidationConfig, ValidationResult,
    PolicySimulator, TestScenario, SimulationResults,
    ImpactAnalysis, ImpactType,
    // Audit
    PolicyAuditManager, AuditConfig, AuditLogEntry,
    ComplianceFramework, ComplianceReport, ComplianceStatus,
};
pub use auth::continuous::ContinuousAuthManager;
pub use auth::mfa::{MfaManager, MfaConfig, MfaMethod, MfaVerificationResult};
pub use auth::biometrics::{BiometricsManager, BiometricsConfig, BiometricAnalysisResult};
pub use auth::adaptive::{AdaptiveAuthManager, AdaptiveAuthConfig, AdaptiveAuthDecision, AuthRequirement};
pub use segmentation::network::NetworkSegmenter;
pub use segmentation::application::{ApplicationSegmenter, ApplicationSegment, ServiceMeshConfig};
pub use segmentation::data::{DataSegmenter, DataSegment, DataClassification, UserAttributes};

// Identity module exports
pub use identity::{
    // Identity Fabric
    IdentityFabric, IdentityProvider, UserInfo,
    // SSO
    SsoManager, SsoConfig, SsoSession, SsoSessionState,
    SamlProvider, SamlConfig, OAuthProvider, OAuthConfig,
    JitProvisioningConfig, ServiceProviderConfig,
    // Identity Sync
    IdentitySyncManager, SyncConfig, SyncStatus, IdentityMapping,
    ConflictResolution, ConflictRecord, SyncRecord, SyncDirection,
    // Identity Analytics
    IdentityAnalyticsManager, AnalyticsConfig, AccessEvent,
    UserProfile, RiskScore, AnomalyType, AnomalyRecord,
    AccessPatternReport, ComplianceReport, ComplianceFramework,
};

/// Zero Trust Architecture Error Types
#[derive(Debug, thiserror::Error)]
pub enum ZeroTrustError {
    #[error("Policy evaluation error: {0}")]
    PolicyEvaluation(String),
    
    #[error("Authentication failed: {0}")]
    AuthenticationFailed(String),
    
    #[error("Trust score calculation error: {0}")]
    TrustScoreError(String),
    
    #[error("Segmentation error: {0}")]
    SegmentationError(String),
    
    #[error("Identity error: {0}")]
    IdentityError(String),
    
    #[error("Configuration error: {0}")]
    ConfigurationError(String),
}

/// Result type for Zero Trust operations
pub type Result<T> = std::result::Result<T, ZeroTrustError>;

/// Zero Trust context for access decisions
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ZeroTrustContext {
    /// Request identifier
    pub request_id: uuid::Uuid,
    
    /// User or service identity
    pub subject: String,
    
    /// Resource being accessed
    pub resource: String,
    
    /// Action being performed
    pub action: String,
    
    /// Device information
    pub device: DeviceInfo,
    
    /// Network information
    pub network: NetworkInfo,
    
    /// Timestamp
    pub timestamp: chrono::DateTime<chrono::Utc>,
    
    /// Additional context data
    pub context: serde_json::Value,
}

/// Device information for trust evaluation
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct DeviceInfo {
    /// Device ID
    pub device_id: String,
    
    /// Device type
    pub device_type: DeviceType,
    
    /// Operating system
    pub os: String,
    
    /// OS version
    pub os_version: String,
    
    /// Security posture score
    pub security_score: f64,
    
    /// Is trusted device
    pub is_trusted: bool,
    
    /// Last seen timestamp
    pub last_seen: chrono::DateTime<chrono::Utc>,
}

/// Device type classification
#[derive(Debug, Clone, Copy, serde::Serialize, serde::Deserialize, PartialEq, Eq)]
pub enum DeviceType {
    Desktop,
    Laptop,
    Mobile,
    Tablet,
    Server,
    IoT,
    Unknown,
}

/// Network information for trust evaluation
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct NetworkInfo {
    /// IP address
    pub ip_address: String,
    
    /// Network location
    pub location: LocationInfo,
    
    /// Network type
    pub network_type: NetworkType,
    
    /// Is trusted network
    pub is_trusted: bool,
    
    /// Connection security
    pub is_encrypted: bool,
}

/// Location information
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct LocationInfo {
    /// Country code
    pub country: String,
    
    /// Region/state
    pub region: Option<String>,
    
    /// City
    pub city: Option<String>,
    
    /// Latitude
    pub latitude: Option<f64>,
    
    /// Longitude
    pub longitude: Option<f64>,
}

/// Network type classification
#[derive(Debug, Clone, Copy, serde::Serialize, serde::Deserialize, PartialEq, Eq)]
pub enum NetworkType {
    Corporate,
    Home,
    Public,
    VPN,
    Cellular,
    Unknown,
}

/// Access decision from Zero Trust evaluation
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq, Eq)]
pub enum AccessDecision {
    /// Access granted
    Allow,
    
    /// Access denied
    Deny,
    
    /// Require additional authentication
    RequireMfa,
    
    /// Require step-up authentication
    RequireStepUp,
    
    /// Require device registration
    RequireDeviceRegistration,
}

/// Access decision with additional information
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct AccessResult {
    /// The access decision
    pub decision: AccessDecision,
    
    /// Trust score at time of decision
    pub trust_score: f64,
    
    /// Trust level
    pub trust_level: TrustLevel,
    
    /// Reason for decision
    pub reason: String,
    
    /// Required actions for allow decision
    pub required_actions: Vec<String>,
    
    /// Timestamp of decision
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

impl AccessResult {
    /// Create a new access result
    pub fn new(
        decision: AccessDecision,
        trust_score: f64,
        trust_level: TrustLevel,
        reason: impl Into<String>,
    ) -> Self {
        Self {
            decision,
            trust_score,
            trust_level,
            reason: reason.into(),
            required_actions: Vec::new(),
            timestamp: chrono::Utc::now(),
        }
    }
    
    /// Add required action
    pub fn with_required_action(mut self, action: impl Into<String>) -> Self {
        self.required_actions.push(action.into());
        self
    }
    
    /// Check if access is granted
    pub fn is_allowed(&self) -> bool {
        matches!(self.decision, AccessDecision::Allow)
    }
}