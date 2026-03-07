//! SENTINEL Shadow AI Detection and Governance Module
//! 
//! This module detects and govern unsanctioned AI models (Shadow AI)
//! deployed across organizational systems without proper authorization.
//!
//! # Components
//! - Discovery Engine: Find AI models deployed in the environment
//! - Traffic Analyzer: Detect AI-related network traffic patterns
//! - Governance Engine: Policy enforcement and approval workflows
//! - Risk Assessment: Evaluate risks of detected AI deployments

pub mod discovery;
pub mod traffic;
pub mod governance;
pub mod risk;
pub mod models;

pub use discovery::{DiscoveryEngine, DiscoveredAI, DiscoveryMethod};
pub use traffic::{TrafficAnalyzer, AITrafficPattern, TrafficSignature};
pub use governance::{GovernanceEngine, AIPolicy, ApprovalWorkflow, GovernanceAction};
pub use risk::{RiskAssessment, RiskLevel, RiskFactor};
pub use models::{AIModel, AIModelType, AIModelStatus};

use anyhow::Result;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{info, debug};
use std::collections::HashMap;

/// Shadow AI Manager
/// 
/// Central coordinator for Shadow AI detection and governance.
/// Addresses the growing risk of unsanctioned AI deployments.
pub struct ShadowAIManager {
    discovery: Arc<RwLock<DiscoveryEngine>>,
    traffic_analyzer: Arc<RwLock<TrafficAnalyzer>>,
    governance: Arc<RwLock<GovernanceEngine>>,
    risk_assessment: Arc<RwLock<RiskAssessment>>,
    config: ShadowAIConfig,
}

/// Configuration for Shadow AI Detection
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ShadowAIConfig {
    /// Enable automatic discovery
    pub auto_discovery_enabled: bool,
    /// Discovery interval in seconds
    pub discovery_interval_secs: u64,
    /// Enable traffic analysis
    pub traffic_analysis_enabled: bool,
    /// Enable automatic governance enforcement
    pub auto_enforcement_enabled: bool,
    /// Minimum risk score for alerting (0.0 - 1.0)
    pub alert_risk_threshold: f64,
    /// Block high-risk AI automatically
    pub auto_block_high_risk: bool,
    /// Known AI service endpoints
    pub known_ai_endpoints: Vec<String>,
    /// Approved AI models whitelist
    pub approved_models: Vec<String>,
}

impl Default for ShadowAIConfig {
    fn default() -> Self {
        Self {
            auto_discovery_enabled: true,
            discovery_interval_secs: 3600,
            traffic_analysis_enabled: true,
            auto_enforcement_enabled: false,
            alert_risk_threshold: 0.7,
            auto_block_high_risk: false,
            known_ai_endpoints: vec![
                "api.openai.com".to_string(),
                "api.anthropic.com".to_string(),
                "generativelanguage.googleapis.com".to_string(),
                "api.cohere.ai".to_string(),
                "api.together.xyz".to_string(),
            ],
            approved_models: vec![],
        }
    }
}

/// Shadow AI Detection Result
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct DetectionResult {
    /// Detection ID
    pub id: String,
    /// Discovered AI instance
    pub discovered: DiscoveredAI,
    /// Risk assessment
    pub risk_score: f64,
    /// Risk level
    pub risk_level: RiskLevel,
    /// Governance status
    pub governance_status: GovernanceStatus,
    /// Recommended actions
    pub recommended_actions: Vec<GovernanceAction>,
    /// Detection timestamp
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

/// Governance status
#[derive(Debug, Clone, Copy, serde::Serialize, serde::Deserialize)]
pub enum GovernanceStatus {
    Approved,
    Pending,
    Unapproved,
    Blocked,
    UnderReview,
    Exception,
}

/// AI Usage Event
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct AIUsageEvent {
    /// Event ID
    pub id: String,
    /// AI model identifier
    pub model_id: String,
    /// User or system using the AI
    pub user: String,
    /// Endpoint accessed
    pub endpoint: String,
    /// Request type
    pub request_type: AIRequestType,
    /// Data classification
    pub data_classification: DataClassification,
    /// Request size
    pub request_size: usize,
    /// Response size
    pub response_size: usize,
    /// Timestamp
    pub timestamp: chrono::DateTime<chrono::Utc>,
    /// Risk indicators
    pub risk_indicators: Vec<String>,
}

/// AI Request types
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub enum AIRequestType {
    ChatCompletion,
    Embedding,
    FineTuning,
    Inference,
    Training,
    Moderation,
    ImageGeneration,
    AudioTranscription,
    Translation,
    Custom(String),
}

/// Data classification levels
#[derive(Debug, Clone, Copy, serde::Serialize, serde::Deserialize)]
pub enum DataClassification {
    Public,
    Internal,
    Confidential,
    Restricted,
    Sensitive,
}

/// Shadow AI Statistics
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
pub struct ShadowAIStatistics {
    pub total_discovered: u64,
    pub approved_count: u64,
    pub unapproved_count: u64,
    pub blocked_count: u64,
    pub pending_count: u64,
    pub total_usage_events: u64,
    pub high_risk_count: u64,
    pub average_risk_score: f64,
}

impl ShadowAIManager {
    /// Create a new Shadow AI Manager
    pub fn new(config: ShadowAIConfig) -> Self {
        Self {
            discovery: Arc::new(RwLock::new(DiscoveryEngine::new())),
            traffic_analyzer: Arc::new(RwLock::new(TrafficAnalyzer::new())),
            governance: Arc::new(RwLock::new(GovernanceEngine::new())),
            risk_assessment: Arc::new(RwLock::new(RiskAssessment::new())),
            config,
        }
    }

    /// Run full discovery scan
    pub async fn run_discovery(&self) -> Result<Vec<DiscoveredAI>> {
        info!("Starting Shadow AI discovery scan");
        
        let mut discovery = self.discovery.write().await;
        let discovered = discovery.scan_all().await?;
        
        info!("Discovery complete: found {} AI instances", discovered.len());
        Ok(discovered)
    }

    /// Analyze traffic for AI patterns
    pub async fn analyze_traffic(&self, traffic_data: &[u8]) -> Result<Vec<AITrafficPattern>> {
        if !self.config.traffic_analysis_enabled {
            return Ok(vec![]);
        }
        
        let analyzer = self.traffic_analyzer.read().await;
        analyzer.analyze(traffic_data).await
    }

    /// Assess risk of discovered AI
    pub async fn assess_risk(&self, discovered: &DiscoveredAI) -> Result<(f64, RiskLevel)> {
        let risk = self.risk_assessment.read().await;
        risk.calculate(discovered).await
    }

    /// Check governance status
    pub async fn check_governance(&self, model_id: &str) -> Result<GovernanceStatus> {
        let governance = self.governance.read().await;
        governance.get_status(model_id).await
    }

    /// Process discovered AI (assess risk, check governance, recommend actions)
    pub async fn process_discovery(&self, discovered: DiscoveredAI) -> Result<DetectionResult> {
        // Assess risk
        let (risk_score, risk_level) = self.assess_risk(&discovered).await?;
        
        // Check governance status
        let governance_status = self.check_governance(&discovered.id).await?;
        
        // Generate recommended actions
        let recommended_actions = self.generate_recommendations(risk_score, governance_status).await?;
        
        Ok(DetectionResult {
            id: uuid::Uuid::new_v4().to_string(),
            discovered,
            risk_score,
            risk_level,
            governance_status,
            recommended_actions,
            timestamp: chrono::Utc::now(),
        })
    }

    /// Generate recommended actions based on risk and governance status
    async fn generate_recommendations(
        &self,
        risk_score: f64,
        governance_status: GovernanceStatus,
    ) -> Result<Vec<GovernanceAction>> {
        let mut actions = Vec::new();
        
        if risk_score >= self.config.alert_risk_threshold {
            actions.push(GovernanceAction::Alert);
        }
        
        if governance_status == GovernanceStatus::Unapproved {
            actions.push(GovernanceAction::RequestApproval);
        }
        
        if risk_score >= 0.9 && self.config.auto_block_high_risk {
            actions.push(GovernanceAction::Block);
        }
        
        if governance_status == GovernanceStatus::Pending {
            actions.push(GovernanceAction::ExpediteReview);
        }
        
        Ok(actions)
    }

    /// Register an AI model for governance
    pub async fn register_model(&self, model: AIModel) -> Result<()> {
        let mut governance = self.governance.write().await;
        governance.register(model).await
    }

    /// Approve an AI model
    pub async fn approve_model(&self, model_id: &str, approver: &str, justification: &str) -> Result<()> {
        let mut governance = self.governance.write().await;
        governance.approve(model_id, approver, justification).await
    }

    /// Block an AI model
    pub async fn block_model(&self, model_id: &str, reason: &str) -> Result<()> {
        let mut governance = self.governance.write().await;
        governance.block(model_id, reason).await
    }

    /// Record usage event
    pub async fn record_usage(&self, event: AIUsageEvent) -> Result<()> {
        // Analyze the usage event for risk indicators
        let risk = self.risk_assessment.read().await;
        let _risk_score = risk.analyze_usage(&event).await?;
        
        // Could trigger alerts based on usage patterns
        Ok(())
    }

    /// Get statistics
    pub async fn get_statistics(&self) -> Result<ShadowAIStatistics> {
        let governance = self.governance.read().await;
        let stats = governance.get_statistics().await;
        Ok(stats)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_shadow_ai_manager_creation() {
        let manager = ShadowAIManager::new(ShadowAIConfig::default());
        assert!(manager.config.auto_discovery_enabled);
    }
}