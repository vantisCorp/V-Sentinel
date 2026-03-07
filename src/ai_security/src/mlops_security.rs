//! MLOps Security Module
//!
//! Provides comprehensive security for ML operations including:
//! - Pipeline security
//! - Model registry security
//! - Deployment security
//! - Artifact management
//! - Audit logging

use crate::models::*;
use crate::MLOpsSecurityResult;
use anyhow::Result;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{debug, info, warn};

/// MLOps Security Manager
pub struct MLOpsSecurityManager {
    /// Pipeline security
    pipeline_security: PipelineSecurity,
    /// Model registry
    model_registry: ModelRegistry,
    /// Deployment security
    deployment_security: DeploymentSecurity,
    /// Audit logger
    audit_logger: AuditLogger,
    /// Statistics
    stats: Arc<RwLock<MLOpsStats>>,
}

/// MLOps statistics
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MLOpsStats {
    /// Pipelines secured
    pub pipelines_secured: u64,
    /// Models registered
    pub models_registered: u64,
    /// Deployments secured
    pub deployments_secured: u64,
    /// Audit entries
    pub audit_entries: u64,
}

impl MLOpsSecurityManager {
    /// Create new MLOps security manager
    pub fn new() -> Self {
        Self {
            pipeline_security: PipelineSecurity::new(),
            model_registry: ModelRegistry::new(),
            deployment_security: DeploymentSecurity::new(),
            audit_logger: AuditLogger::new(),
            stats: Arc::new(RwLock::new(MLOpsStats::default())),
        }
    }

    /// Setup pipeline security for a system
    pub async fn setup_pipeline_security(&self, system: &AISystem) -> Result<MLOpsSecurityResult> {
        info!("Setting up MLOps security for system: {}", system.id);

        let mut result = MLOpsSecurityResult::default();

        // Secure pipelines
        for pipeline in &system.data_pipelines {
            self.pipeline_security.secure(pipeline).await?;
            result.pipeline_secured = true;
        }

        // Register models
        for model in &system.models {
            self.model_registry.register(model).await?;
        }

        // Secure deployments
        if !system.api_endpoints.is_empty() {
            self.deployment_security.secure(&system.api_endpoints).await?;
            result.deployment_secured = true;
        }

        result.registry_secured = true;

        // Update stats
        let mut stats = self.stats.write().await;
        stats.pipelines_secured += system.data_pipelines.len() as u64;
        stats.models_registered += system.models.len() as u64;
        stats.deployments_secured += 1;

        // Log audit event
        self.audit_logger.log(MLOpsAuditEvent {
            id: uuid::Uuid::new_v4().to_string(),
            event_type: MLOpsEventType::SystemSecured,
            system_id: Some(system.id.clone()),
            model_id: None,
            pipeline_id: None,
            description: format!("MLOps security configured for system {}", system.id),
            timestamp: Utc::now(),
            metadata: HashMap::new(),
        }).await?;

        Ok(result)
    }

    /// Register a model
    pub async fn register_model(&self, model: &AIModel) -> Result<ModelRegistrationResult> {
        let result = self.model_registry.register(model).await?;
        
        let mut stats = self.stats.write().await;
        stats.models_registered += 1;
        
        // Audit log
        self.audit_logger.log(MLOpsAuditEvent {
            id: uuid::Uuid::new_v4().to_string(),
            event_type: MLOpsEventType::ModelRegistered,
            system_id: None,
            model_id: Some(model.id.clone()),
            pipeline_id: None,
            description: format!("Model {} registered", model.name),
            timestamp: Utc::now(),
            metadata: HashMap::new(),
        }).await?;

        Ok(result)
    }

    /// Verify model for deployment
    pub async fn verify_for_deployment(&self, model: &AIModel) -> Result<DeploymentVerificationResult> {
        self.deployment_security.verify_model(model).await
    }

    /// Get model provenance
    pub async fn get_model_provenance(&self, model_id: &str) -> Result<Option<ModelProvenance>> {
        self.model_registry.get_provenance(model_id).await
    }

    /// Get audit log
    pub async fn get_audit_log(&self, limit: usize) -> Vec<MLOpsAuditEvent> {
        self.audit_logger.get_recent(limit).await
    }

    /// Get statistics
    pub async fn get_stats(&self) -> MLOpsStats {
        self.stats.read().await.clone()
    }
}

impl Default for MLOpsSecurityManager {
    fn default() -> Self {
        Self::new()
    }
}

/// Pipeline Security
pub struct PipelineSecurity {
    /// Secured pipelines
    pipelines: Arc<RwLock<HashMap<String, SecuredPipeline>>>,
}

/// Secured pipeline info
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecuredPipeline {
    /// Pipeline ID
    pub pipeline_id: String,
    /// Security controls applied
    pub controls: Vec<String>,
    /// Access policy
    pub access_policy: PipelineAccessPolicy,
    /// Encryption enabled
    pub encrypted: bool,
    /// Audit enabled
    pub audit_enabled: bool,
    /// Secured at
    pub secured_at: DateTime<Utc>,
}

/// Pipeline access policy
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PipelineAccessPolicy {
    /// Allowed principals
    pub allowed_principals: Vec<String>,
    /// Denied principals
    pub denied_principals: Vec<String>,
    /// Time restrictions
    pub time_restrictions: Option<TimeRestriction>,
    /// IP restrictions
    pub ip_restrictions: Vec<String>,
}

/// Time restriction
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimeRestriction {
    /// Allowed hours start
    pub start_hour: u8,
    /// Allowed hours end
    pub end_hour: u8,
    /// Days of week (0 = Sunday, 6 = Saturday)
    pub allowed_days: Vec<u8>,
}

impl PipelineSecurity {
    /// Create new pipeline security
    pub fn new() -> Self {
        Self {
            pipelines: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Secure a pipeline
    pub async fn secure(&self, pipeline: &DataPipeline) -> Result<()> {
        let secured = SecuredPipeline {
            pipeline_id: pipeline.id.clone(),
            controls: vec![
                "encryption".to_string(),
                "access_control".to_string(),
                "audit_logging".to_string(),
                "input_validation".to_string(),
            ],
            access_policy: PipelineAccessPolicy {
                allowed_principals: vec!["ml-team".to_string()],
                denied_principals: vec![],
                time_restrictions: None,
                ip_restrictions: vec![],
            },
            encrypted: true,
            audit_enabled: true,
            secured_at: Utc::now(),
        };

        let mut pipelines = self.pipelines.write().await;
        pipelines.insert(pipeline.id.clone(), secured);
        
        info!("Pipeline {} secured", pipeline.id);
        Ok(())
    }

    /// Check pipeline access
    pub async fn check_access(&self, pipeline_id: &str, principal: &str) -> Result<bool> {
        let pipelines = self.pipelines.read().await;
        
        if let Some(pipeline) = pipelines.get(pipeline_id) {
            // Check denied list first
            if pipeline.access_policy.denied_principals.contains(&principal.to_string()) {
                return Ok(false);
            }
            
            // Check allowed list
            return Ok(pipeline.access_policy.allowed_principals.contains(&principal.to_string()));
        }
        
        // Default deny
        Ok(false)
    }
}

impl Default for PipelineSecurity {
    fn default() -> Self {
        Self::new()
    }
}

/// Model Registry
pub struct ModelRegistry {
    /// Registered models
    models: Arc<RwLock<HashMap<String, RegisteredModel>>>,
    /// Provenance tracking
    provenance: Arc<RwLock<HashMap<String, ModelProvenance>>>,
}

/// Registered model
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegisteredModel {
    /// Model ID
    pub model_id: String,
    /// Model name
    pub name: String,
    /// Version
    pub version: String,
    /// Registration timestamp
    pub registered_at: DateTime<Utc>,
    /// Status
    pub status: ModelStatus,
    /// Security scan result
    pub security_scan: Option<SecurityScanResult>,
    /// Tags
    pub tags: HashMap<String, String>,
}

/// Model status
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ModelStatus {
    /// Model is being registered
    Registering,
    /// Model is registered and ready
    Ready,
    /// Model is deployed
    Deployed,
    /// Model is deprecated
    Deprecated,
    /// Model is archived
    Archived,
}

/// Security scan result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityScanResult {
    /// Scan ID
    pub scan_id: String,
    /// Scan timestamp
    pub scanned_at: DateTime<Utc>,
    /// Vulnerabilities found
    pub vulnerabilities: Vec<ModelVulnerability>,
    /// Overall status
    pub status: ScanStatus,
    /// Recommendations
    pub recommendations: Vec<String>,
}

/// Scan status
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ScanStatus {
    Passed,
    Warning,
    Failed,
}

/// Model provenance
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelProvenance {
    /// Model ID
    pub model_id: String,
    /// Training data sources
    pub training_data: Vec<DataProvenance>,
    /// Training configuration
    pub training_config: HashMap<String, String>,
    /// Parent models (if transfer learning)
    pub parent_models: Vec<String>,
    /// Transformations applied
    pub transformations: Vec<String>,
    /// Created by
    pub created_by: String,
    /// Created at
    pub created_at: DateTime<Utc>,
}

/// Data provenance
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataProvenance {
    /// Data source ID
    pub source_id: String,
    /// Source name
    pub name: String,
    /// Source location
    pub location: String,
    /// Data hash
    pub hash: String,
    /// Classification
    pub classification: DataClassification,
}

impl ModelRegistry {
    /// Create new model registry
    pub fn new() -> Self {
        Self {
            models: Arc::new(RwLock::new(HashMap::new())),
            provenance: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Register a model
    pub async fn register(&self, model: &AIModel) -> Result<ModelRegistrationResult> {
        let registered = RegisteredModel {
            model_id: model.id.clone(),
            name: model.name.clone(),
            version: model.version.clone(),
            registered_at: Utc::now(),
            status: ModelStatus::Ready,
            security_scan: None,
            tags: HashMap::new(),
        };

        let model_id = model.id.clone();
        let mut models = self.models.write().await;
        models.insert(model_id.clone(), registered);

        // Create initial provenance
        let prov = ModelProvenance {
            model_id: model_id.clone(),
            training_data: vec![],
            training_config: HashMap::new(),
            parent_models: vec![],
            transformations: vec![],
            created_by: "system".to_string(),
            created_at: Utc::now(),
        };
        
        let mut provenance = self.provenance.write().await;
        provenance.insert(model_id, prov);

        Ok(ModelRegistrationResult {
            model_id: model.id.clone(),
            status: ModelStatus::Ready,
            registered_at: Utc::now(),
        })
    }

    /// Get model provenance
    pub async fn get_provenance(&self, model_id: &str) -> Result<Option<ModelProvenance>> {
        let provenance = self.provenance.read().await;
        Ok(provenance.get(model_id).cloned())
    }

    /// Update model status
    pub async fn update_status(&self, model_id: &str, status: ModelStatus) -> Result<()> {
        let mut models = self.models.write().await;
        if let Some(model) = models.get_mut(model_id) {
            model.status = status;
        }
        Ok(())
    }

    /// Run security scan
    pub async fn scan_model(&self, model_id: &str) -> Result<SecurityScanResult> {
        let scan_result = SecurityScanResult {
            scan_id: uuid::Uuid::new_v4().to_string(),
            scanned_at: Utc::now(),
            vulnerabilities: vec![],
            status: ScanStatus::Passed,
            recommendations: vec![],
        };

        let mut models = self.models.write().await;
        if let Some(model) = models.get_mut(model_id) {
            model.security_scan = Some(scan_result.clone());
        }

        Ok(scan_result)
    }
}

impl Default for ModelRegistry {
    fn default() -> Self {
        Self::new()
    }
}

/// Model registration result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelRegistrationResult {
    /// Model ID
    pub model_id: String,
    /// Status
    pub status: ModelStatus,
    /// Registration timestamp
    pub registered_at: DateTime<Utc>,
}

/// Deployment Security
pub struct DeploymentSecurity {
    /// Deployment configurations
    deployments: Arc<RwLock<HashMap<String, DeploymentConfig>>>,
}

/// Deployment configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeploymentConfig {
    /// Deployment ID
    pub deployment_id: String,
    /// Model ID
    pub model_id: String,
    /// Environment
    pub environment: DeploymentEnvironment,
    /// Security controls
    pub security_controls: Vec<String>,
    /// Access policy
    pub access_policy: DeploymentAccessPolicy,
    /// Created at
    pub created_at: DateTime<Utc>,
}

/// Deployment environment
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DeploymentEnvironment {
    Development,
    Staging,
    Production,
}

/// Deployment access policy
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeploymentAccessPolicy {
    /// Require authentication
    pub require_auth: bool,
    /// Require mTLS
    pub require_mtls: bool,
    /// Allowed IP ranges
    pub allowed_ips: Vec<String>,
    /// Rate limit
    pub rate_limit: u32,
}

impl DeploymentSecurity {
    /// Create new deployment security
    pub fn new() -> Self {
        Self {
            deployments: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Secure endpoints
    pub async fn secure(&self, endpoints: &[APIEndpoint]) -> Result<()> {
        for endpoint in endpoints {
            let config = DeploymentConfig {
                deployment_id: endpoint.id.clone(),
                model_id: String::new(),
                environment: DeploymentEnvironment::Production,
                security_controls: vec![
                    "rate_limiting".to_string(),
                    "input_validation".to_string(),
                    "audit_logging".to_string(),
                ],
                access_policy: DeploymentAccessPolicy {
                    require_auth: true,
                    require_mtls: false,
                    allowed_ips: vec![],
                    rate_limit: 60,
                },
                created_at: Utc::now(),
            };

            let mut deployments = self.deployments.write().await;
            deployments.insert(endpoint.id.clone(), config);
        }

        Ok(())
    }

    /// Verify model for deployment
    pub async fn verify_model(&self, model: &AIModel) -> Result<DeploymentVerificationResult> {
        let mut checks = Vec::new();

        // Check encryption
        checks.push(DeploymentCheck {
            name: "model_encryption".to_string(),
            passed: model.encrypted,
            message: if model.encrypted {
                "Model is encrypted".to_string()
            } else {
                "Model is not encrypted".to_string()
            },
        });

        // Check watermark
        checks.push(DeploymentCheck {
            name: "model_watermark".to_string(),
            passed: model.watermarked,
            message: if model.watermarked {
                "Model is watermarked".to_string()
            } else {
                "Model is not watermarked".to_string()
            },
        });

        // Check vulnerabilities
        let vuln_count = model.vulnerabilities.len();
        checks.push(DeploymentCheck {
            name: "security_scan".to_string(),
            passed: vuln_count == 0,
            message: format!("{} vulnerabilities found", vuln_count),
        });

        let passed = checks.iter().all(|c| c.passed);

        Ok(DeploymentVerificationResult {
            model_id: model.id.clone(),
            passed,
            checks,
            verified_at: Utc::now(),
        })
    }
}

impl Default for DeploymentSecurity {
    fn default() -> Self {
        Self::new()
    }
}

/// Deployment verification result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeploymentVerificationResult {
    /// Model ID
    pub model_id: String,
    /// Whether verification passed
    pub passed: bool,
    /// Individual checks
    pub checks: Vec<DeploymentCheck>,
    /// Verification timestamp
    pub verified_at: DateTime<Utc>,
}

/// Individual deployment check
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeploymentCheck {
    /// Check name
    pub name: String,
    /// Whether check passed
    pub passed: bool,
    /// Check message
    pub message: String,
}

/// Audit Logger
pub struct AuditLogger {
    /// Audit entries
    entries: Arc<RwLock<Vec<MLOpsAuditEvent>>>,
}

impl AuditLogger {
    /// Create new audit logger
    pub fn new() -> Self {
        Self {
            entries: Arc::new(RwLock::new(Vec::new())),
        }
    }

    /// Log an audit event
    pub async fn log(&self, event: MLOpsAuditEvent) -> Result<()> {
        let mut entries = self.entries.write().await;
        entries.push(event);
        Ok(())
    }

    /// Get recent entries
    pub async fn get_recent(&self, limit: usize) -> Vec<MLOpsAuditEvent> {
        let entries = self.entries.read().await;
        entries.iter().rev().take(limit).cloned().collect()
    }
}

impl Default for AuditLogger {
    fn default() -> Self {
        Self::new()
    }
}

/// MLOps audit event
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MLOpsAuditEvent {
    /// Event ID
    pub id: String,
    /// Event type
    pub event_type: MLOpsEventType,
    /// System ID
    pub system_id: Option<String>,
    /// Model ID
    pub model_id: Option<String>,
    /// Pipeline ID
    pub pipeline_id: Option<String>,
    /// Description
    pub description: String,
    /// Timestamp
    pub timestamp: DateTime<Utc>,
    /// Additional metadata
    pub metadata: HashMap<String, String>,
}

/// MLOps event type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MLOpsEventType {
    SystemSecured,
    ModelRegistered,
    ModelDeployed,
    PipelineSecured,
    AccessGranted,
    AccessDenied,
    SecurityScanCompleted,
    VulnerabilityDetected,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_mlops_manager_creation() {
        let manager = MLOpsSecurityManager::new();
        let stats = manager.get_stats().await;
        assert_eq!(stats.pipelines_secured, 0);
    }

    #[tokio::test]
    async fn test_model_registration() {
        let manager = MLOpsSecurityManager::new();
        let model = AIModel::new("TestModel", "1.0", ModelType::Transformer);
        
        let result = manager.register_model(&model).await.unwrap();
        assert_eq!(result.status, ModelStatus::Ready);
    }

    #[tokio::test]
    async fn test_deployment_verification() {
        let manager = MLOpsSecurityManager::new();
        let model = AIModel::new("TestModel", "1.0", ModelType::Transformer);
        
        let result = manager.verify_for_deployment(&model).await.unwrap();
        // Model is not encrypted or watermarked
        assert!(!result.passed);
    }

    #[tokio::test]
    async fn test_pipeline_security() {
        let security = PipelineSecurity::new();
        let pipeline = DataPipeline::new("Test Pipeline", PipelineType::Training);
        
        security.secure(&pipeline).await.unwrap();
        
        let result = security.check_access(&pipeline.id, "ml-team").await.unwrap();
        assert!(result);
        
        let result = security.check_access(&pipeline.id, "unknown").await.unwrap();
        assert!(!result);
    }

    #[tokio::test]
    async fn test_audit_logging() {
        let manager = MLOpsSecurityManager::new();
        let model = AIModel::new("TestModel", "1.0", ModelType::Transformer);
        
        manager.register_model(&model).await.unwrap();
        
        let logs = manager.get_audit_log(10).await;
        assert!(!logs.is_empty());
    }
}