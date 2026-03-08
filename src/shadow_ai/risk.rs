use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use crate::shadow_ai::{
    DetectedAIModel, AIModelType, RiskLevel, ShadowAIError
};

/// Risk Assessment Engine
pub struct RiskAssessmentEngine {
    config: RiskConfig,
    /// Risk score cache
    risk_cache: Arc<RwLock<HashMap<String, AIRiskScore>>>,
    /// Data source risk mappings
    data_source_risks: Arc<RwLock<HashMap<String, DataSensitivity>>>,
}

/// Risk assessment configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RiskConfig {
    /// Enable risk scoring
    pub enable_scoring: bool,
    /// Risk threshold for automatic blocking
    pub blocking_threshold: f64,
    /// Risk threshold for requiring approval
    pub approval_threshold: f64,
    /// Weight for model capability risk
    pub capability_weight: f64,
    /// Weight for data sensitivity risk
    pub data_sensitivity_weight: f64,
    /// Weight for access control risk
    pub access_control_weight: f64,
    /// Enable continuous monitoring
    pub continuous_monitoring: bool,
    /// Monitoring interval in seconds
    pub monitoring_interval_secs: u64,
}

impl Default for RiskConfig {
    fn default() -> Self {
        Self {
            enable_scoring: true,
            blocking_threshold: 0.85,
            approval_threshold: 0.6,
            capability_weight: 0.35,
            data_sensitivity_weight: 0.35,
            access_control_weight: 0.30,
            continuous_monitoring: true,
            monitoring_interval_secs: 300,
        }
    }
}

/// AI Risk Score
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AIRiskScore {
    /// Model ID
    pub model_id: String,
    /// Overall risk score (0.0-1.0)
    pub overall_score: f64,
    /// Risk level
    pub risk_level: RiskLevel,
    /// Component scores
    pub components: RiskComponents,
    /// Risk factors identified
    pub risk_factors: Vec<RiskFactor>,
    /// Mitigation recommendations
    pub recommendations: Vec<String>,
    /// Assessment timestamp
    pub assessed_at: DateTime<Utc>,
    /// Confidence in assessment (0.0-1.0)
    pub confidence: f64,
}

/// Risk component scores
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RiskComponents {
    /// Model capability risk (0.0-1.0)
    pub capability_risk: f64,
    /// Data sensitivity risk (0.0-1.0)
    pub data_sensitivity_risk: f64,
    /// Access control risk (0.0-1.0)
    pub access_control_risk: f64,
    /// Operational risk (0.0-1.0)
    pub operational_risk: f64,
}

/// Risk factor
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RiskFactor {
    /// Factor ID
    pub factor_id: String,
    /// Factor category
    pub category: RiskCategory,
    /// Factor description
    pub description: String,
    /// Severity (0.0-1.0)
    pub severity: f64,
    /// Evidence
    pub evidence: Vec<String>,
}

/// Risk category
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum RiskCategory {
    /// Model capability risks
    Capability,
    /// Data protection risks
    DataProtection,
    /// Access control risks
    AccessControl,
    /// Operational risks
    Operational,
    /// Compliance risks
    Compliance,
    /// Privacy risks
    Privacy,
}

/// Data source sensitivity
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum DataSensitivity {
    Public,
    Internal,
    Confidential,
    Restricted,
    HighlyRestricted,
}

/// Data exposure risk assessment
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataExposureRisk {
    /// Assessment ID
    pub assessment_id: String,
    /// Model being assessed
    pub model_id: String,
    /// Data sources mapped
    pub data_sources: Vec<DataSourceMapping>,
    /// Exposure pathways
    pub exposure_pathways: Vec<ExposurePathway>,
    /// Overall exposure risk (0.0-1.0)
    pub overall_exposure_risk: f64,
    /// High-risk data sources
    pub high_risk_sources: Vec<String>,
    /// Assessment timestamp
    pub assessed_at: DateTime<Utc>,
}

/// Data source mapping
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataSourceMapping {
    /// Data source identifier
    pub source_id: String,
    /// Data source name
    pub source_name: String,
    /// Data type
    pub data_type: DataType,
    /// Sensitivity level
    pub sensitivity: DataSensitivity,
    /// Access pattern
    pub access_pattern: AccessPattern,
    /// Data volume (bytes)
    pub data_volume: Option<u64>,
    /// Last accessed
    pub last_accessed: DateTime<Utc>,
}

/// Data type
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum DataType {
    PersonalIdentifiableInformation,
    FinancialData,
    HealthData,
    IntellectualProperty,
    TradeSecrets,
    CustomerData,
    EmployeeData,
    OperationalData,
    LogsAndMetrics,
    PublicData,
}

/// Access pattern
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccessPattern {
    /// Frequency of access
    pub frequency: AccessFrequency,
    /// Access type
    pub access_type: Vec<AccessType>,
    /// Average data transferred per access (bytes)
    pub avg_data_per_access: u64,
    /// Time windows
    pub time_windows: Vec<String>,
}

/// Access frequency
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum AccessFrequency {
    Realtime,
    Hourly,
    Daily,
    Weekly,
    Monthly,
    Sporadic,
}

/// Access type
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum AccessType {
    Read,
    Write,
    Delete,
    Export,
    Share,
    Transform,
}

/// Exposure pathway
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExposurePathway {
    /// Pathway ID
    pub pathway_id: String,
    /// Description
    pub description: String,
    /// Exposure mechanism
    pub mechanism: ExposureMechanism,
    /// Likelihood (0.0-1.0)
    pub likelihood: f64,
    /// Impact severity (0.0-1.0)
    pub impact: f64,
    /// Risk score (0.0-1.0)
    pub risk_score: f64,
    /// Affected data sources
    pub affected_sources: Vec<String>,
}

/// Exposure mechanism
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum ExposureMechanism {
    /// Data leakage through model outputs
    OutputLeakage,
    /// Model memorization and reconstruction
    Memorization,
    /// Training data extraction
    TrainingDataExtraction,
    /// Model inversion attacks
    ModelInversion,
    /// Membership inference attacks
    MembershipInference,
    /// Data poisoning leading to exposure
    DataPoisoning,
    /// Direct API access to sensitive data
    DirectAPIAccess,
}

impl RiskAssessmentEngine {
    /// Create new risk assessment engine
    pub fn new(config: RiskConfig) -> Self {
        let mut engine = Self {
            config,
            risk_cache: Arc::new(RwLock::new(HashMap::new())),
            data_source_risks: Arc::new(RwLock::new(HashMap::new())),
        };

        // Initialize default data source risks
        tokio::spawn(async move {
            engine.initialize_default_data_source_risks().await;
        });

        engine
    }

    /// Initialize default data source risk mappings
    async fn initialize_default_data_source_risks(&self) {
        let mut risks = self.data_source_risks.write().await;
        
        risks.insert("public".to_string(), DataSensitivity::Public);
        risks.insert("internal".to_string(), DataSensitivity::Internal);
        risks.insert("confidential".to_string(), DataSensitivity::Confidential);
        risks.insert("restricted".to_string(), DataSensitivity::Restricted);
        risks.insert("hr_data".to_string(), DataSensitivity::HighlyRestricted);
        risks.insert("financial_data".to_string(), DataSensitivity::HighlyRestricted);
        risks.insert("customer_pii".to_string(), DataSensitivity::HighlyRestricted);
        risks.insert("health_records".to_string(), DataSensitivity::HighlyRestricted);
        risks.insert("trade_secrets".to_string(), DataSensitivity::Restricted);
        risks.insert("intellectual_property".to_string(), DataSensitivity::Restricted);
    }

    /// Assess risk for a detected AI model
    pub async fn assess_model_risk(&self, model: &DetectedAIModel) -> Result<AIRiskScore, ShadowAIError> {
        // Check cache first
        let cache_key = format!("{}:{}:{}", model.model_id, model.file_hash, model.risk_level);
        if let Some(cached) = self.risk_cache.read().await.get(&cache_key).cloned() {
            return Ok(cached);
        }

        // Calculate component risks
        let capability_risk = self.assess_capability_risk(model).await;
        let data_sensitivity_risk = self.assess_data_sensitivity_risk(model).await;
        let access_control_risk = self.assess_access_control_risk(model).await;
        let operational_risk = self.assess_operational_risk(model).await;

        // Calculate weighted overall score
        let overall_score = 
            (capability_risk * self.config.capability_weight) +
            (data_sensitivity_risk * self.config.data_sensitivity_weight) +
            (access_control_risk * self.config.access_control_weight) +
            (operational_risk * 0.15); // Fixed weight for operational risk

        // Determine risk level
        let risk_level = match overall_score {
            s if s < 0.25 => RiskLevel::Low,
            s if s < 0.5 => RiskLevel::Medium,
            s if s < 0.75 => RiskLevel::High,
            _ => RiskLevel::Critical,
        };

        // Identify risk factors
        let risk_factors = self.identify_risk_factors(model, &capability_risk, &data_sensitivity_risk).await;

        // Generate recommendations
        let recommendations = self.generate_recommendations(&risk_level, &risk_factors);

        let risk_score = AIRiskScore {
            model_id: model.model_id.clone(),
            overall_score,
            risk_level,
            components: RiskComponents {
                capability_risk,
                data_sensitivity_risk,
                access_control_risk,
                operational_risk,
            },
            risk_factors,
            recommendations,
            assessed_at: Utc::now(),
            confidence: 0.8,
        };

        // Cache the result
        self.risk_cache.write().await.insert(cache_key, risk_score.clone());

        Ok(risk_score)
    }

    /// Assess model capability risk
    async fn assess_capability_risk(&self, model: &DetectedAIModel) -> f64 {
        let mut risk = 0.0;

        // Risk by model type
        match model.model_type {
            AIModelType::LargeLanguageModel => risk += 0.7,
            AIModelType::GenerativeAI => risk += 0.65,
            AIModelType::ComputerVision => risk += 0.5,
            AIModelType::NaturalLanguageProcessing => risk += 0.55,
            AIModelType::NeuralNetwork => risk += 0.45,
            AIModelType::MachineLearning => risk += 0.35,
            AIModelType::Unknown => risk += 0.3,
        }

        // Risk by model size
        let size_gb = model.model_size_bytes as f64 / (1024.0 * 1024.0 * 1024.0);
        if size_gb > 10.0 {
            risk += 0.15;
        } else if size_gb > 1.0 {
            risk += 0.10;
        } else if size_gb > 0.1 {
            risk += 0.05;
        }

        // Risk by framework (some frameworks have more known vulnerabilities)
        if let Some(framework) = &model.framework {
            if framework.contains("pytorch") || framework.contains("torch") {
                risk += 0.05;
            } else if framework.contains("tensorflow") {
                risk += 0.05;
            }
        }

        risk.min(1.0)
    }

    /// Assess data sensitivity risk
    async fn assess_data_sensitivity_risk(&self, model: &DetectedAIModel) -> f64 {
        let mut risk = 0.0;

        // Higher risk if model is in suspicious location
        if model.model_path.contains("/tmp/") || model.model_path.contains("/cache/") {
            risk += 0.6;
        } else if model.model_path.contains("~/") && !model.model_path.contains("/models/") {
            risk += 0.4;
        }

        // Higher risk for generative models (potential data leakage)
        if matches!(model.model_type, AIModelType::LargeLanguageModel | AIModelType::GenerativeAI) {
            risk += 0.3;
        }

        // Higher risk if not registered
        if !model.registered {
            risk += 0.5;
        }

        risk.min(1.0)
    }

    /// Assess access control risk
    async fn assess_access_control_risk(&self, model: &DetectedAIModel) -> f64 {
        let mut risk = 0.0;

        // Risk if no owner defined
        if model.owner.is_none() {
            risk += 0.5;
        }

        // Risk if not registered
        if !model.registered {
            risk += 0.6;
        }

        // Risk by file permissions (simplified check)
        if model.model_path.contains("/home/") {
            risk += 0.3;
        }

        risk.min(1.0)
    }

    /// Assess operational risk
    async fn assess_operational_risk(&self, model: &DetectedAIModel) -> f64 {
        let mut risk = 0.0;

        // Risk by model age (older models may have vulnerabilities)
        let model_age = Utc::now().signed_duration_since(model.discovered_at).num_days();
        if model_age > 365 {
            risk += 0.3;
        } else if model_age > 90 {
            risk += 0.2;
        }

        // Risk if model is frequently modified
        let modification_age = Utc::now().signed_duration_since(model.last_modified).num_days();
        if modification_age < 7 {
            risk += 0.2;
        }

        risk.min(1.0)
    }

    /// Identify risk factors
    async fn identify_risk_factors(&self, model: &DetectedAIModel, capability_risk: &f64, data_risk: &f64) -> Vec<RiskFactor> {
        let mut factors = Vec::new();

        // Capability risk factors
        if *capability_risk > 0.6 {
            factors.push(RiskFactor {
                factor_id: uuid::Uuid::new_v4().to_string(),
                category: RiskCategory::Capability,
                description: format!("High capability risk: {:?} model", model.model_type),
                severity: *capability_risk,
                evidence: vec![format!("Model type: {:?}", model.model_type)],
            });
        }

        // Data sensitivity risk factors
        if *data_risk > 0.5 {
            factors.push(RiskFactor {
                factor_id: uuid::Uuid::new_v4().to_string(),
                category: RiskCategory::DataProtection,
                description: "Potential data exposure risk from unregistered model".to_string(),
                severity: *data_risk,
                evidence: vec![
                    format!("Location: {}", model.model_path),
                    format!("Registered: {}", model.registered),
                ],
            });
        }

        // Access control factors
        if model.owner.is_none() {
            factors.push(RiskFactor {
                factor_id: uuid::Uuid::new_v4().to_string(),
                category: RiskCategory::AccessControl,
                description: "No owner assigned to model".to_string(),
                severity: 0.6,
                evidence: vec!["Model lacks ownership attribution".to_string()],
            });
        }

        // Location factors
        if model.model_path.contains("/tmp/") {
            factors.push(RiskFactor {
                factor_id: uuid::Uuid::new_v4().to_string(),
                category: RiskCategory::Operational,
                description: "Model located in temporary directory".to_string(),
                severity: 0.7,
                evidence: vec![format!("Path: {}", model.model_path)],
            });
        }

        factors
    }

    /// Generate risk mitigation recommendations
    fn generate_recommendations(&self, risk_level: &RiskLevel, factors: &[RiskFactor]) -> Vec<String> {
        let mut recommendations = Vec::new();

        match risk_level {
            RiskLevel::Critical => {
                recommendations.push("IMMEDIATE ACTION REQUIRED: Consider blocking this model".to_string());
                recommendations.push("Conduct immediate security review of model and its data sources".to_string());
                recommendations.push("Implement strict access controls and monitoring".to_string());
            }
            RiskLevel::High => {
                recommendations.push("Review model access controls and data sources".to_string());
                recommendations.push("Consider requiring explicit approval for usage".to_string());
                recommendations.push("Implement additional monitoring and audit logging".to_string());
            }
            RiskLevel::Medium => {
                recommendations.push("Register the model in the AI governance system".to_string());
                recommendations.push("Assign an owner and document usage policies".to_string());
                recommendations.push("Monitor model usage and access patterns".to_string());
            }
            RiskLevel::Low => {
                recommendations.push("Consider registering the model for better visibility".to_string());
                recommendations.push("Document model purpose and usage guidelines".to_string());
            }
        }

        // Category-specific recommendations
        for factor in factors {
            match factor.category {
                RiskCategory::Capability => {
                    recommendations.push("Evaluate model capability controls and output filtering".to_string());
                }
                RiskCategory::DataProtection => {
                    recommendations.push("Review data sources and implement data protection measures".to_string());
                }
                RiskCategory::AccessControl => {
                    recommendations.push("Strengthen access controls and authentication".to_string());
                }
                RiskCategory::Operational => {
                    recommendations.push("Review model storage and deployment practices".to_string());
                }
                _ => {}
            }
        }

        recommendations.dedup();
        recommendations.truncate(10); // Limit to top 10 recommendations

        recommendations
    }

    /// Assess data exposure risk for a model
    pub async fn assess_data_exposure_risk(&self, model: &DetectedAIModel) -> Result<DataExposureRisk, ShadowAIError> {
        // Identify potential data sources based on model location and type
        let data_sources = self.identify_data_sources(model).await;

        // Analyze exposure pathways
        let exposure_pathways = self.analyze_exposure_pathways(model, &data_sources).await;

        // Calculate overall exposure risk
        let overall_exposure_risk = if exposure_pathways.is_empty() {
            0.0
        } else {
            exposure_pathways.iter()
                .map(|p| p.risk_score)
                .sum::<f64>() / exposure_pathways.len() as f64
        };

        // Identify high-risk sources
        let high_risk_sources = data_sources.iter()
            .filter(|ds| matches!(ds.sensitivity, DataSensitivity::Restricted | DataSensitivity::HighlyRestricted))
            .map(|ds| ds.source_id.clone())
            .collect();

        Ok(DataExposureRisk {
            assessment_id: uuid::Uuid::new_v4().to_string(),
            model_id: model.model_id.clone(),
            data_sources,
            exposure_pathways,
            overall_exposure_risk,
            high_risk_sources,
            assessed_at: Utc::now(),
        })
    }

    /// Identify potential data sources for a model
    async fn identify_data_sources(&self, model: &DetectedAIModel) -> Vec<DataSourceMapping> {
        let mut sources = Vec::new();

        // In a real implementation, this would analyze:
        // - Network connections from the model's host
        // - File system access patterns
        // - Database connections
        // - API calls made by applications using the model

        // For now, create some sample mappings based on model type
        match model.model_type {
            AIModelType::LargeLanguageModel | AIModelType::GenerativeAI => {
                sources.push(DataSourceMapping {
                    source_id: "training_data".to_string(),
                    source_name: "Training Dataset".to_string(),
                    data_type: DataType::CustomerData,
                    sensitivity: DataSensitivity::Confidential,
                    access_pattern: AccessPattern {
                        frequency: AccessFrequency::Sporadic,
                        access_type: vec![AccessType::Read],
                        avg_data_per_access: 1_000_000_000, // 1GB
                        time_windows: vec!["any".to_string()],
                    },
                    data_volume: Some(100_000_000_000), // 100GB
                    last_accessed: Utc::now(),
                });
            }
            _ => {
                sources.push(DataSourceMapping {
                    source_id: "default_data".to_string(),
                    source_name: "Default Dataset".to_string(),
                    data_type: DataType::OperationalData,
                    sensitivity: DataSensitivity::Internal,
                    access_pattern: AccessPattern {
                        frequency: AccessFrequency::Daily,
                        access_type: vec![AccessType::Read],
                        avg_data_per_access: 10_000_000, // 10MB
                        time_windows: vec!["business_hours".to_string()],
                    },
                    data_volume: Some(1_000_000_000), // 1GB
                    last_accessed: Utc::now(),
                });
            }
        }

        sources
    }

    /// Analyze exposure pathways
    async fn analyze_exposure_pathways(&self, model: &DetectedAIModel, data_sources: &[DataSourceMapping]) -> Vec<ExposurePathway> {
        let mut pathways = Vec::new();

        // Check each data source for potential exposure
        for source in data_sources {
            // Model output leakage risk
            if matches!(source.data_type, 
                DataType::PersonalIdentifiableInformation | 
                DataType::CustomerData |
                DataType::TradeSecrets) {
                
                pathways.push(ExposurePathway {
                    pathway_id: uuid::Uuid::new_v4().to_string(),
                    description: format!("Potential leakage of {} through model outputs", source.source_name),
                    mechanism: ExposureMechanism::OutputLeakage,
                    likelihood: 0.6,
                    impact: source.sensitivity_to_impact(),
                    risk_score: 0.6 * source.sensitivity_to_impact(),
                    affected_sources: vec![source.source_id.clone()],
                });
            }

            // Memorization risk for large models
            if matches!(model.model_type, AIModelType::LargeLanguageModel) {
                pathways.push(ExposurePathway {
                    pathway_id: uuid::Uuid::new_v4().to_string(),
                    description: "Model may memorize training data and expose it".to_string(),
                    mechanism: ExposureMechanism::Memorization,
                    likelihood: 0.4,
                    impact: source.sensitivity_to_impact(),
                    risk_score: 0.4 * source.sensitivity_to_impact(),
                    affected_sources: vec![source.source_id.clone()],
                });
            }

            // Training data extraction risk
            if !model.registered {
                pathways.push(ExposurePathway {
                    pathway_id: uuid::Uuid::new_v4().to_string(),
                    description: "Unregistered model may enable training data extraction attacks".to_string(),
                    mechanism: ExposureMechanism::TrainingDataExtraction,
                    likelihood: 0.5,
                    impact: source.sensitivity_to_impact(),
                    risk_score: 0.5 * source.sensitivity_to_impact(),
                    affected_sources: vec![source.source_id.clone()],
                });
            }
        }

        pathways
    }

    /// Generate risk report for all discovered models
    pub async fn generate_risk_report(&self, models: &[DetectedAIModel]) -> RiskReport {
        let mut model_risks = Vec::new();

        for model in models {
            match self.assess_model_risk(model).await {
                Ok(risk) => model_risks.push(risk),
                Err(_) => continue,
            }
        }

        // Calculate statistics
        let total_models = model_risks.len();
        let by_level = |level: &RiskLevel| -> usize {
            model_risks.iter().filter(|r| &r.risk_level == level).count()
        };

        let summary = RiskSummary {
            total_models,
            critical_risk: by_level(&RiskLevel::Critical),
            high_risk: by_level(&RiskLevel::High),
            medium_risk: by_level(&RiskLevel::Medium),
            low_risk: by_level(&RiskLevel::Low),
            avg_risk_score: if total_models > 0 {
                model_risks.iter().map(|r| r.overall_score).sum::<f64>() / total_models as f64
            } else {
                0.0
            },
        };

        RiskReport {
            report_id: uuid::Uuid::new_v4().to_string(),
            generated_at: Utc::now(),
            summary,
            model_risks,
            recommendations: self.generate_overall_recommendations(&model_risks),
        }
    }

    /// Generate overall risk recommendations
    fn generate_overall_recommendations(&self, model_risks: &[AIRiskScore]) -> Vec<String> {
        let mut recommendations = Vec::new();

        let critical_count = model_risks.iter()
            .filter(|r| r.risk_level == RiskLevel::Critical)
            .count();

        if critical_count > 0 {
            recommendations.push(format!(
                "IMMEDIATE: {} critical-risk models require immediate attention and potential blocking",
                critical_count
            ));
        }

        let high_count = model_risks.iter()
            .filter(|r| r.risk_level == RiskLevel::High)
            .count();

        if high_count > 0 {
            recommendations.push(format!(
                "{} high-risk models should be reviewed and approved within 7 days",
                high_count
            ));
        }

        let unregistered = model_risks.iter()
            .filter(|r| {
                // Check if model is in cache but not registered (simplified check)
                // In real implementation, would check model metadata
                false
            })
            .count();

        if unregistered > 0 {
            recommendations.push(format!(
                "{} models are unregistered and should be registered for proper governance",
                unregistered
            ));
        }

        if recommendations.is_empty() {
            recommendations.push("All models are within acceptable risk levels. Continue monitoring.".to_string());
        }

        recommendations
    }
}

impl DataSensitivity {
    /// Convert sensitivity to impact score (0.0-1.0)
    fn sensitivity_to_impact(&self) -> f64 {
        match self {
            DataSensitivity::Public => 0.1,
            DataSensitivity::Internal => 0.3,
            DataSensitivity::Confidential => 0.5,
            DataSensitivity::Restricted => 0.7,
            DataSensitivity::HighlyRestricted => 0.9,
        }
    }
}

/// Risk report
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RiskReport {
    /// Report ID
    pub report_id: String,
    /// Generated at
    pub generated_at: DateTime<Utc>,
    /// Summary statistics
    pub summary: RiskSummary,
    /// Individual model risk scores
    pub model_risks: Vec<AIRiskScore>,
    /// Overall recommendations
    pub recommendations: Vec<String>,
}

/// Risk summary
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RiskSummary {
    /// Total models assessed
    pub total_models: usize,
    /// Critical risk count
    pub critical_risk: usize,
    /// High risk count
    pub high_risk: usize,
    /// Medium risk count
    pub medium_risk: usize,
    /// Low risk count
    pub low_risk: usize,
    /// Average risk score
    pub avg_risk_score: f64,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_risk_config() {
        let config = RiskConfig::default();
        assert!(config.enable_scoring);
        assert!(config.continuous_monitoring);
        assert_eq!(config.blocking_threshold, 0.85);
    }

    #[test]
    fn test_sensitivity_to_impact() {
        assert_eq!(DataSensitivity::Public.sensitivity_to_impact(), 0.1);
        assert_eq!(DataSensitivity::HighlyRestricted.sensitivity_to_impact(), 0.9);
    }
}