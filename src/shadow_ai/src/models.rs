//! AI Model Definitions
//!
//! Core data structures for AI models in the Shadow AI system.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// AI Model
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AIModel {
    /// Unique identifier
    pub id: String,
    /// Model name
    pub name: String,
    /// Model type
    pub model_type: AIModelType,
    /// Provider
    pub provider: String,
    /// Version
    pub version: Option<String>,
    /// Endpoint
    pub endpoint: Option<String>,
    /// API key (masked)
    pub api_key_masked: Option<String>,
    /// Capabilities
    pub capabilities: Vec<String>,
    /// Data types it can process
    pub data_types: Vec<String>,
    /// Status
    pub status: AIModelStatus,
    /// Risk level
    pub risk_level: Option<f64>,
    /// Owner
    pub owner: Option<String>,
    /// Department
    pub department: Option<String>,
    /// Cost center
    pub cost_center: Option<String>,
    /// Created at
    pub created_at: DateTime<Utc>,
    /// Updated at
    pub updated_at: DateTime<Utc>,
    /// Last used
    pub last_used: Option<DateTime<Utc>>,
    /// Metadata
    pub metadata: HashMap<String, serde_json::Value>,
}

/// AI Model types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AIModelType {
    LLM,            // Large Language Models (GPT, Claude, LLaMA)
    Embedding,      // Embedding models
    Image,          // Image generation (DALL-E, Midjourney, Stable Diffusion)
    Audio,          // Audio processing (Whisper, TTS)
    Video,          // Video generation
    Transformer,    // General transformer models
    Diffusion,      // Diffusion models
    GAN,            // Generative Adversarial Networks
    RAG,            // Retrieval-Augmented Generation
    FineTuned,      // Fine-tuned models
    MultiModal,     // Multi-modal models
    Agent,          // AI Agents
    Custom(String), // Custom model type
}

/// AI Model status
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AIModelStatus {
    Active,
    Inactive,
    Pending,
    Approved,
    Blocked,
    UnderReview,
    Deprecated,
}

/// AI Provider information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AIProvider {
    pub id: String,
    pub name: String,
    pub provider_type: ProviderType,
    pub trust_level: f64,
    pub compliance: Vec<String>,
    pub data_residency: Vec<String>,
    pub security_certifications: Vec<String>,
    pub is_approved: bool,
}

/// Provider types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ProviderType {
    CloudAPI,
    SelfHosted,
    OnPremise,
    Hybrid,
    Edge,
}

/// AI Usage Statistics
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AIUsageStatistics {
    pub total_requests: u64,
    pub total_tokens: u64,
    pub total_cost: f64,
    pub unique_users: u64,
    pub average_latency_ms: f64,
    pub error_rate: f64,
    pub last_24h_requests: u64,
    pub last_7d_requests: u64,
    pub last_30d_requests: u64,
}

/// AI Capability
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AICapability {
    pub name: String,
    pub description: String,
    pub data_types: Vec<String>,
    pub risk_level: f64,
}

impl AIModel {
    /// Create a new AI Model
    pub fn new(id: String, name: String, model_type: AIModelType, provider: String) -> Self {
        let now = Utc::now();
        Self {
            id,
            name,
            model_type,
            provider,
            version: None,
            endpoint: None,
            api_key_masked: None,
            capabilities: Vec::new(),
            data_types: Vec::new(),
            status: AIModelStatus::Pending,
            risk_level: None,
            owner: None,
            department: None,
            cost_center: None,
            created_at: now,
            updated_at: now,
            last_used: None,
            metadata: HashMap::new(),
        }
    }

    /// Update the model
    pub fn update(&mut self) {
        self.updated_at = Utc::now();
    }

    /// Set status
    pub fn set_status(&mut self, status: AIModelStatus) {
        self.status = status;
        self.update();
    }

    /// Record usage
    pub fn record_usage(&mut self) {
        self.last_used = Some(Utc::now());
        self.update();
    }

    /// Check if model is active
    pub fn is_active(&self) -> bool {
        matches!(self.status, AIModelStatus::Active | AIModelStatus::Approved)
    }

    /// Check if model is approved
    pub fn is_approved(&self) -> bool {
        matches!(self.status, AIModelStatus::Approved)
    }
}

impl Default for AIModelType {
    fn default() -> Self {
        AIModelType::LLM
    }
}

impl Default for AIModelStatus {
    fn default() -> Self {
        AIModelStatus::Pending
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ai_model_creation() {
        let model = AIModel::new(
            "test-1".to_string(),
            "Test Model".to_string(),
            AIModelType::LLM,
            "OpenAI".to_string(),
        );

        assert_eq!(model.id, "test-1");
        assert_eq!(model.name, "Test Model");
        assert_eq!(model.status, AIModelStatus::Pending);
    }

    #[test]
    fn test_ai_model_status_changes() {
        let mut model = AIModel::new(
            "test-1".to_string(),
            "Test Model".to_string(),
            AIModelType::LLM,
            "OpenAI".to_string(),
        );

        assert!(!model.is_approved());

        model.set_status(AIModelStatus::Approved);
        assert!(model.is_approved());
        assert!(model.is_active());
    }
}
