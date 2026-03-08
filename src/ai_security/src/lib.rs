//! AI Security Module
//!
//! Comprehensive security for AI/ML systems including:
//! - AI Data Security: Pipeline controls, data encryption, access management
//! - AI Model Security: Model encryption, watermarking, integrity verification
//! - AI API Security: Prompt injection detection, input validation, rate limiting
//! - MLOps Security: Pipeline security, model registry, deployment controls

pub mod api_security;
pub mod data_security;
pub mod mlops_security;
pub mod model_security;
pub mod models;

use anyhow::Result;
use async_trait::async_trait;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{debug, error, info, warn};
use uuid::Uuid;

pub use api_security::{APISecurityManager, InputValidator, PromptInjectionDetector};
pub use data_security::DataSecurityManager;
pub use mlops_security::{MLOpsSecurityManager, ModelRegistry, PipelineSecurity};
pub use model_security::{ModelEncryption, ModelSecurityManager, ModelWatermark};
pub use models::{
    AIModel, AISystem, SecurityEvent, SecurityEventType, SecurityPolicy, ThreatLevel,
};
pub use models::{DataClassification, DataPipeline};

/// Main AI Security Manager
pub struct AISecurityManager {
    /// Data security manager
    data_security: Arc<RwLock<DataSecurityManager>>,
    /// Model security manager
    model_security: Arc<RwLock<ModelSecurityManager>>,
    /// API security manager
    api_security: Arc<RwLock<APISecurityManager>>,
    /// MLOps security manager
    mlops_security: Arc<RwLock<MLOpsSecurityManager>>,
    /// Configuration
    config: AISecurityConfig,
    /// Security policies
    policies: Arc<RwLock<HashMap<String, SecurityPolicy>>>,
    /// Security event log
    event_log: Arc<RwLock<Vec<SecurityEvent>>>,
    /// Statistics
    stats: Arc<RwLock<AISecurityStats>>,
}

/// Configuration for AI Security
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AISecurityConfig {
    /// Enable data encryption
    pub data_encryption_enabled: bool,
    /// Enable model watermarking
    pub model_watermarking_enabled: bool,
    /// Enable prompt injection detection
    pub prompt_injection_detection: bool,
    /// Enable input validation
    pub input_validation: bool,
    /// Enable audit logging
    pub audit_logging: bool,
    /// Maximum input size in bytes
    pub max_input_size: usize,
    /// Rate limit per minute
    pub rate_limit: u32,
    /// Enable threat detection
    pub threat_detection: bool,
    /// Security level (1-10)
    pub security_level: u8,
}

impl Default for AISecurityConfig {
    fn default() -> Self {
        Self {
            data_encryption_enabled: true,
            model_watermarking_enabled: true,
            prompt_injection_detection: true,
            input_validation: true,
            audit_logging: true,
            max_input_size: 100 * 1024, // 100KB
            rate_limit: 60,
            threat_detection: true,
            security_level: 7,
        }
    }
}

/// AI Security Statistics
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AISecurityStats {
    /// Total requests processed
    pub total_requests: u64,
    /// Blocked requests
    pub blocked_requests: u64,
    /// Prompt injections detected
    pub prompt_injections_detected: u64,
    /// Data breaches prevented
    pub data_breaches_prevented: u64,
    /// Models protected
    pub models_protected: u64,
    /// Security events
    pub security_events: u64,
}

impl AISecurityManager {
    /// Create a new AI Security Manager
    pub fn new(config: AISecurityConfig) -> Self {
        Self {
            data_security: Arc::new(RwLock::new(DataSecurityManager::new())),
            model_security: Arc::new(RwLock::new(ModelSecurityManager::new())),
            api_security: Arc::new(RwLock::new(APISecurityManager::new())),
            mlops_security: Arc::new(RwLock::new(MLOpsSecurityManager::new())),
            config,
            policies: Arc::new(RwLock::new(HashMap::new())),
            event_log: Arc::new(RwLock::new(Vec::new())),
            stats: Arc::new(RwLock::new(AISecurityStats::default())),
        }
    }

    /// Secure an AI system
    pub async fn secure_system(&self, system: &mut AISystem) -> Result<SecurityResult> {
        info!("Securing AI system: {}", system.id);
        let start_time = std::time::Instant::now();

        // Step 1: Secure data pipelines
        let data_result = self.secure_data_pipelines(system).await?;

        // Step 2: Secure models
        let model_result = self.secure_models(system).await?;

        // Step 3: Configure API security
        let api_result = self.configure_api_security(system).await?;

        // Step 4: Setup MLOps security
        let mlops_result = self.setup_mlops_security(system).await?;

        // Update system status
        system.secured = true;
        system.security_level = self.config.security_level;

        let result = SecurityResult {
            system_id: system.id.clone(),
            data_security: data_result,
            model_security: model_result,
            api_security: api_result,
            mlops_security: mlops_result,
            overall_status: SecurityStatus::Secured,
            applied_policies: self.get_applied_policies(system).await,
            duration_ms: start_time.elapsed().as_millis() as u64,
            timestamp: Utc::now(),
        };

        // Log event
        self.log_security_event(SecurityEvent {
            id: Uuid::new_v4().to_string(),
            event_type: SecurityEventType::SystemSecured,
            system_id: Some(system.id.clone()),
            model_id: None,
            description: format!("AI system {} secured", system.id),
            severity: ThreatLevel::Info,
            timestamp: Utc::now(),
            metadata: HashMap::new(),
        })
        .await?;

        Ok(result)
    }

    /// Secure data pipelines
    async fn secure_data_pipelines(&self, system: &AISystem) -> Result<DataSecurityResult> {
        let data_security = self.data_security.read().await;

        let mut result = DataSecurityResult::default();

        for pipeline in &system.data_pipelines {
            let pipeline_result = data_security.secure_pipeline(pipeline).await?;
            if pipeline_result.encrypted {
                result.encrypted_pipelines += 1;
            }
            if pipeline_result.access_controlled {
                result.access_controlled_pipelines += 1;
            }
            result.total_pipelines += 1;
        }

        Ok(result)
    }

    /// Secure models
    async fn secure_models(&self, system: &AISystem) -> Result<ModelSecurityResult> {
        let model_security = self.model_security.read().await;

        let mut result = ModelSecurityResult::default();

        for model in &system.models {
            let model_result = model_security.protect_model(model).await?;
            if model_result.encrypted {
                result.encrypted_models += 1;
            }
            if model_result.watermarked {
                result.watermarked_models += 1;
            }
            result.total_models += 1;
        }

        let mut stats = self.stats.write().await;
        stats.models_protected += result.total_models;
        drop(stats);

        Ok(result)
    }

    /// Configure API security
    async fn configure_api_security(&self, system: &AISystem) -> Result<APISecurityResult> {
        let api_security = self.api_security.read().await;

        let result = APISecurityResult {
            prompt_injection_enabled: self.config.prompt_injection_detection,
            input_validation_enabled: self.config.input_validation,
            rate_limit: self.config.rate_limit,
            max_input_size: self.config.max_input_size,
        };

        api_security.configure_for_system(system, &result).await?;

        Ok(result)
    }

    /// Setup MLOps security
    async fn setup_mlops_security(&self, system: &AISystem) -> Result<MLOpsSecurityResult> {
        let mlops_security = self.mlops_security.read().await;

        let result = mlops_security.setup_pipeline_security(system).await?;

        Ok(result)
    }

    /// Validate input for security threats
    pub async fn validate_input(
        &self,
        input: &str,
        context: &SecurityContext,
    ) -> Result<InputValidationResult> {
        let mut stats = self.stats.write().await;
        stats.total_requests += 1;
        drop(stats);

        // Check input size
        if input.len() > self.config.max_input_size {
            let mut stats = self.stats.write().await;
            stats.blocked_requests += 1;
            drop(stats);

            return Ok(InputValidationResult {
                valid: false,
                reason: Some("Input exceeds maximum size".to_string()),
                threat_level: ThreatLevel::Medium,
                detected_threats: vec!["oversized_input".to_string()],
                sanitized_input: None,
            });
        }

        // Check for prompt injection
        if self.config.prompt_injection_detection {
            let api_security = self.api_security.read().await;
            let injection_result = api_security.detect_prompt_injection(input).await?;

            if injection_result.detected {
                let mut stats = self.stats.write().await;
                stats.blocked_requests += 1;
                stats.prompt_injections_detected += 1;
                drop(stats);

                // Log event
                self.log_security_event(SecurityEvent {
                    id: Uuid::new_v4().to_string(),
                    event_type: SecurityEventType::PromptInjectionDetected,
                    system_id: context.system_id.clone(),
                    model_id: context.model_id.clone(),
                    description: format!(
                        "Prompt injection attempt detected: {}",
                        injection_result.attack_type
                    ),
                    severity: ThreatLevel::High,
                    timestamp: Utc::now(),
                    metadata: HashMap::new(),
                })
                .await?;

                return Ok(InputValidationResult {
                    valid: false,
                    reason: Some(format!(
                        "Prompt injection detected: {}",
                        injection_result.attack_type
                    )),
                    threat_level: ThreatLevel::High,
                    detected_threats: injection_result.indicators,
                    sanitized_input: None,
                });
            }
        }

        Ok(InputValidationResult {
            valid: true,
            reason: None,
            threat_level: ThreatLevel::Info,
            detected_threats: vec![],
            sanitized_input: Some(input.to_string()),
        })
    }

    /// Log a security event
    async fn log_security_event(&self, event: SecurityEvent) -> Result<()> {
        if self.config.audit_logging {
            let mut event_log = self.event_log.write().await;
            event_log.push(event);

            let mut stats = self.stats.write().await;
            stats.security_events += 1;
        }
        Ok(())
    }

    /// Get applied policies for a system
    async fn get_applied_policies(&self, _system: &AISystem) -> Vec<String> {
        let policies = self.policies.read().await;
        policies.keys().cloned().collect()
    }

    /// Add security policy
    pub async fn add_policy(&self, policy: SecurityPolicy) -> Result<()> {
        let mut policies = self.policies.write().await;
        policies.insert(policy.id.clone(), policy);
        Ok(())
    }

    /// Get security statistics
    pub async fn get_stats(&self) -> AISecurityStats {
        self.stats.read().await.clone()
    }

    /// Get security events
    pub async fn get_events(&self, limit: usize) -> Vec<SecurityEvent> {
        let event_log = self.event_log.read().await;
        event_log.iter().rev().take(limit).cloned().collect()
    }

    /// Encrypt model
    pub async fn encrypt_model(
        &self,
        model: &mut AIModel,
        key: Option<&[u8]>,
    ) -> Result<ModelEncryptionResult> {
        let model_security = self.model_security.read().await;
        model_security.encrypt_model(model, key).await
    }

    /// Verify model integrity
    pub async fn verify_model_integrity(&self, model: &AIModel) -> Result<ModelIntegrityResult> {
        let model_security = self.model_security.read().await;
        model_security.verify_integrity(model).await
    }

    /// Watermark model
    pub async fn watermark_model(
        &self,
        model: &mut AIModel,
        watermark: &ModelWatermark,
    ) -> Result<()> {
        let model_security = self.model_security.read().await;
        model_security.apply_watermark(model, watermark).await
    }
}

/// Result of securing a system
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityResult {
    /// System ID
    pub system_id: String,
    /// Data security result
    pub data_security: DataSecurityResult,
    /// Model security result
    pub model_security: ModelSecurityResult,
    /// API security result
    pub api_security: APISecurityResult,
    /// MLOps security result
    pub mlops_security: MLOpsSecurityResult,
    /// Overall security status
    pub overall_status: SecurityStatus,
    /// Applied policies
    pub applied_policies: Vec<String>,
    /// Duration in milliseconds
    pub duration_ms: u64,
    /// Timestamp
    pub timestamp: DateTime<Utc>,
}

/// Security status
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SecurityStatus {
    /// Not secured
    Unsecured,
    /// Partially secured
    Partial,
    /// Fully secured
    Secured,
    /// Security compromised
    Compromised,
}

/// Data security result
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DataSecurityResult {
    /// Total pipelines
    pub total_pipelines: u64,
    /// Encrypted pipelines
    pub encrypted_pipelines: u64,
    /// Access controlled pipelines
    pub access_controlled_pipelines: u64,
}

/// Model security result
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ModelSecurityResult {
    /// Total models
    pub total_models: u64,
    /// Encrypted models
    pub encrypted_models: u64,
    /// Watermarked models
    pub watermarked_models: u64,
}

/// API security result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct APISecurityResult {
    /// Prompt injection enabled
    pub prompt_injection_enabled: bool,
    /// Input validation enabled
    pub input_validation_enabled: bool,
    /// Rate limit
    pub rate_limit: u32,
    /// Max input size
    pub max_input_size: usize,
}

/// MLOps security result
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MLOpsSecurityResult {
    /// Pipeline secured
    pub pipeline_secured: bool,
    /// Registry secured
    pub registry_secured: bool,
    /// Deployment secured
    pub deployment_secured: bool,
}

/// Input validation result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InputValidationResult {
    /// Whether input is valid
    pub valid: bool,
    /// Reason if invalid
    pub reason: Option<String>,
    /// Threat level
    pub threat_level: ThreatLevel,
    /// Detected threats
    pub detected_threats: Vec<String>,
    /// Sanitized input if valid
    pub sanitized_input: Option<String>,
}

/// Model encryption result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelEncryptionResult {
    /// Whether encryption succeeded
    pub success: bool,
    /// Encryption key ID
    pub key_id: String,
    /// Encrypted model hash
    pub model_hash: String,
    /// Timestamp
    pub timestamp: DateTime<Utc>,
}

/// Model integrity result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelIntegrityResult {
    /// Whether integrity check passed
    pub valid: bool,
    /// Current hash
    pub current_hash: String,
    /// Expected hash
    pub expected_hash: String,
    /// Timestamp
    pub timestamp: DateTime<Utc>,
}

/// Security context for request validation
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SecurityContext {
    /// System ID
    pub system_id: Option<String>,
    /// Model ID
    pub model_id: Option<String>,
    /// User ID
    pub user_id: Option<String>,
    /// Session ID
    pub session_id: Option<String>,
    /// Request source
    pub source: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_ai_security_manager_creation() {
        let manager = AISecurityManager::new(AISecurityConfig::default());
        let stats = manager.get_stats().await;
        assert_eq!(stats.total_requests, 0);
    }

    #[tokio::test]
    async fn test_validate_input() {
        let manager = AISecurityManager::new(AISecurityConfig::default());
        let context = SecurityContext::default();

        let result = manager
            .validate_input("Hello, world!", &context)
            .await
            .unwrap();
        assert!(result.valid);
    }

    #[tokio::test]
    async fn test_validate_oversized_input() {
        let config = AISecurityConfig {
            max_input_size: 10,
            ..Default::default()
        };
        let manager = AISecurityManager::new(config);
        let context = SecurityContext::default();

        let result = manager
            .validate_input("This is a very long input string", &context)
            .await
            .unwrap();
        assert!(!result.valid);
    }
}
