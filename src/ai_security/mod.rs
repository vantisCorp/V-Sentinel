//! AI Security and Protection Module
//! 
//! Provides comprehensive security capabilities for AI systems and data
//! throughout their lifecycle, based on IBM's 2025 cybersecurity trends.

use std::sync::Arc;
use tokio::sync::RwLock;
use serde::{Deserialize, Serialize};

pub mod data_security;
pub mod model_security;
pub mod api_security;
pub mod mlops_security;
pub mod threat_defense;
pub mod models;

use models::*;

/// Configuration for AI Security Module
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AISecurityConfig {
    /// Data security configuration
    pub data_security: data_security::DataSecurityConfig,
    /// Model security configuration
    pub model_security: model_security::ModelSecurityConfig,
    /// API security configuration
    pub api_security: api_security::APISecurityConfig,
    /// MLOps security configuration
    pub mlops_security: mlops_security::MLOpsSecurityConfig,
    /// Threat defense configuration
    pub threat_defense: threat_defense::ThreatDefenseConfig,
}

impl Default for AISecurityConfig {
    fn default() -> Self {
        Self {
            data_security: data_security::DataSecurityConfig::default(),
            model_security: model_security::ModelSecurityConfig::default(),
            api_security: api_security::APISecurityConfig::default(),
            mlops_security: mlops_security::MLOpsSecurityConfig::default(),
            threat_defense: threat_defense::ThreatDefenseConfig::default(),
        }
    }
}

/// Main AI Security Manager
/// 
/// Orchestrates all AI security components and provides unified interface
pub struct AISecurityManager {
    config: AISecurityConfig,
    data_security_engine: Arc<RwLock<data_security::DataSecurityEngine>>,
    model_security_engine: Arc<RwLock<model_security::ModelSecurityEngine>>,
    api_security_engine: Arc<RwLock<api_security::APISecurityEngine>>,
    mlops_security_engine: Arc<RwLock<mlops_security::MLOpsSecurityEngine>>,
    threat_defense_engine: Arc<RwLock<threat_defense::ThreatDefenseEngine>>,
}

impl AISecurityManager {
    /// Create a new AI Security Manager
    pub async fn new(config: AISecurityConfig) -> Result<Self, AISecurityError> {
        let data_security_engine = Arc::new(RwLock::new(
            data_security::DataSecurityEngine::new(config.data_security.clone()).await?
        ));
        
        let model_security_engine = Arc::new(RwLock::new(
            model_security::ModelSecurityEngine::new(config.model_security.clone()).await?
        ));
        
        let api_security_engine = Arc::new(RwLock::new(
            api_security::APISecurityEngine::new(config.api_security.clone()).await?
        ));
        
        let mlops_security_engine = Arc::new(RwLock::new(
            mlops_security::MLOpsSecurityEngine::new(config.mlops_security.clone()).await?
        ));
        
        let threat_defense_engine = Arc::new(RwLock::new(
            threat_defense::ThreatDefenseEngine::new(config.threat_defense.clone()).await?
        ));

        Ok(Self {
            config,
            data_security_engine,
            model_security_engine,
            api_security_engine,
            mlops_security_engine,
            threat_defense_engine,
        })
    }

    /// Secure AI data pipeline
    pub async fn secure_data_pipeline(&self, data: &DataInput) -> Result<SecurePipeline, AISecurityError> {
        self.data_security_engine.read().await
            .secure_pipeline(data).await
    }

    /// Protect AI model
    pub async fn protect_model(&self, model: &ModelInput) -> Result<ProtectedModel, AISecurityError> {
        self.model_security_engine.read().await
            .protect_model(model).await
    }

    /// Secure AI API request
    pub async fn secure_api_request(&self, request: &APIRequest) -> Result<APIResponse, AISecurityError> {
        self.api_security_engine.read().await
            .secure_request(request).await
    }

    /// Monitor MLOps pipeline
    pub async fn monitor_mlops(&self, metrics: &MLOpsMetrics) -> Result<MLOpsReport, AISecurityError> {
        self.mlops_security_engine.read().await
            .monitor_pipeline(metrics).await
    }

    /// Defend against AI threats
    pub async fn defend_threat(&self, threat: &AIThreat) -> Result<ThreatResponse, AISecurityError> {
        self.threat_defense_engine.read().await
            .defend_against(threat).await
    }

    /// Get comprehensive security status
    pub async fn get_security_status(&self) -> Result<SecurityStatus, AISecurityError> {
        let data_status = self.data_security_engine.read().await.get_status().await?;
        let model_status = self.model_security_engine.read().await.get_status().await?;
        let api_status = self.api_security_engine.read().await.get_status().await?;
        let mlops_status = self.mlops_security_engine.read().await.get_status().await?;
        let threat_status = self.threat_defense_engine.read().await.get_status().await?;

        Ok(SecurityStatus {
            data_security: data_status,
            model_security: model_status,
            api_security: api_status,
            mlops_security: mlops_status,
            threat_defense: threat_status,
            timestamp: chrono::Utc::now(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_ai_security_manager_creation() {
        let config = AISecurityConfig::default();
        let manager = AISecurityManager::new(config).await;
        assert!(manager.is_ok());
    }
}