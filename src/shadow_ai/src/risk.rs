//! Risk Assessment Module
//!
//! Evaluates risks associated with AI deployments and usage.

use anyhow::Result;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tracing::{debug, info};

use super::{AIUsageEvent, DataClassification, DiscoveredAI};

/// Risk Assessment Engine
///
/// Calculates and tracks risks for AI models and usage
pub struct RiskAssessment {
    factors: Vec<RiskFactor>,
    risk_history: HashMap<String, Vec<RiskScore>>,
    thresholds: RiskThresholds,
}

/// Risk factor
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RiskFactor {
    pub id: String,
    pub name: String,
    pub description: String,
    pub weight: f64,
    pub factor_type: RiskFactorType,
    pub scoring: RiskScoring,
}

/// Risk factor types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RiskFactorType {
    DataExposure,
    UnauthorizedAccess,
    ComplianceViolation,
    DataLeakage,
    ModelReliability,
    ProviderTrust,
    SecurityPosture,
    UsagePattern,
    DataSensitivity,
    AccessControl,
}

/// Risk scoring configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RiskScoring {
    pub scoring_type: ScoringType,
    pub thresholds: Vec<(f64, f64)>, // (input_threshold, risk_score)
}

/// Scoring type
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ScoringType {
    Linear,
    Exponential,
    Step,
    Custom(String),
}

/// Risk score result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RiskScore {
    pub id: String,
    pub model_id: String,
    pub overall_score: f64,
    pub risk_level: RiskLevel,
    pub factor_scores: HashMap<String, f64>,
    pub risk_indicators: Vec<RiskIndicator>,
    pub recommendations: Vec<String>,
    pub timestamp: DateTime<Utc>,
}

/// Risk level
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, PartialOrd)]
pub enum RiskLevel {
    Minimal = 1,
    Low = 2,
    Medium = 3,
    High = 4,
    Critical = 5,
}

/// Risk indicator
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RiskIndicator {
    pub indicator_type: String,
    pub severity: f64,
    pub description: String,
    pub remediation: Option<String>,
}

/// Risk thresholds configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RiskThresholds {
    pub minimal: f64,
    pub low: f64,
    pub medium: f64,
    pub high: f64,
    pub critical: f64,
}

impl Default for RiskThresholds {
    fn default() -> Self {
        Self {
            minimal: 0.0,
            low: 0.2,
            medium: 0.4,
            high: 0.6,
            critical: 0.8,
        }
    }
}

impl RiskAssessment {
    /// Create a new Risk Assessment engine
    pub fn new() -> Self {
        let mut engine = Self {
            factors: Vec::new(),
            risk_history: HashMap::new(),
            thresholds: RiskThresholds::default(),
        };

        engine.add_default_factors();
        engine
    }

    /// Add default risk factors
    fn add_default_factors(&mut self) {
        let factors = vec![
            RiskFactor {
                id: "data-exposure".to_string(),
                name: "Data Exposure Risk".to_string(),
                description: "Risk of sensitive data being exposed to AI".to_string(),
                weight: 0.25,
                factor_type: RiskFactorType::DataExposure,
                scoring: RiskScoring {
                    scoring_type: ScoringType::Step,
                    thresholds: vec![
                        (0.0, 0.1), // No sensitive data
                        (0.3, 0.3), // Internal data
                        (0.5, 0.5), // Confidential data
                        (0.7, 0.8), // Restricted data
                        (0.9, 1.0), // Top secret data
                    ],
                },
            },
            RiskFactor {
                id: "provider-trust".to_string(),
                name: "Provider Trust Level".to_string(),
                description: "Trust level of the AI provider".to_string(),
                weight: 0.20,
                factor_type: RiskFactorType::ProviderTrust,
                scoring: RiskScoring {
                    scoring_type: ScoringType::Step,
                    thresholds: vec![
                        (1.0, 0.1), // Internal
                        (0.9, 0.2), // Enterprise agreement
                        (0.7, 0.4), // Known provider
                        (0.5, 0.6), // Unknown provider
                        (0.0, 0.9), // Untrusted
                    ],
                },
            },
            RiskFactor {
                id: "unauthorized-access".to_string(),
                name: "Unauthorized Access Risk".to_string(),
                description: "Risk of unauthorized users accessing AI".to_string(),
                weight: 0.15,
                factor_type: RiskFactorType::UnauthorizedAccess,
                scoring: RiskScoring {
                    scoring_type: ScoringType::Linear,
                    thresholds: vec![],
                },
            },
            RiskFactor {
                id: "compliance-violation".to_string(),
                name: "Compliance Violation Risk".to_string(),
                description: "Risk of violating compliance requirements".to_string(),
                weight: 0.20,
                factor_type: RiskFactorType::ComplianceViolation,
                scoring: RiskScoring {
                    scoring_type: ScoringType::Step,
                    thresholds: vec![(0.0, 0.1), (0.5, 0.5), (1.0, 0.9)],
                },
            },
            RiskFactor {
                id: "data-leakage".to_string(),
                name: "Data Leakage Risk".to_string(),
                description: "Risk of data being leaked through AI responses".to_string(),
                weight: 0.20,
                factor_type: RiskFactorType::DataLeakage,
                scoring: RiskScoring {
                    scoring_type: ScoringType::Linear,
                    thresholds: vec![],
                },
            },
        ];

        self.factors = factors;
    }

    /// Calculate risk for a discovered AI
    pub async fn calculate(&self, discovered: &DiscoveredAI) -> Result<(f64, RiskLevel)> {
        info!("Calculating risk for AI: {}", discovered.id);

        let mut total_score = 0.0;
        let mut total_weight = 0.0;

        // Calculate data exposure risk
        let data_risk = self.calculate_data_risk(discovered);
        total_score += data_risk * self.get_factor_weight("data-exposure");
        total_weight += self.get_factor_weight("data-exposure");

        // Calculate provider trust risk
        let provider_risk = self.calculate_provider_risk(discovered);
        total_score += provider_risk * self.get_factor_weight("provider-trust");
        total_weight += self.get_factor_weight("provider-trust");

        // Factor in risk indicators
        let indicator_penalty = discovered.risk_indicators.len() as f64 * 0.05;
        total_score += indicator_penalty;

        // Normalize score
        let final_score = if total_weight > 0.0 {
            (total_score / total_weight).min(1.0)
        } else {
            0.5
        };

        let risk_level = self.score_to_level(final_score);

        debug!(
            "Risk score for {}: {} ({:?})",
            discovered.id, final_score, risk_level
        );

        Ok((final_score, risk_level))
    }

    /// Calculate data risk
    fn calculate_data_risk(&self, discovered: &DiscoveredAI) -> f64 {
        let data_types = &discovered.details.data_types;

        // Check for sensitive data types
        let sensitive_types = ["pii", "phi", "financial", "credentials", "secrets"];
        let has_sensitive = data_types.iter().any(|dt| {
            sensitive_types
                .iter()
                .any(|st| dt.to_lowercase().contains(st))
        });

        if has_sensitive {
            0.8
        } else if !data_types.is_empty() {
            0.4
        } else {
            0.2
        }
    }

    /// Calculate provider risk
    fn calculate_provider_risk(&self, discovered: &DiscoveredAI) -> f64 {
        if let Some(ref provider) = discovered.details.provider {
            match provider.to_lowercase().as_str() {
                "internal" | "self-hosted" => 0.1,
                "openai" | "anthropic" => 0.3,
                "azure" | "aws" | "gcp" => 0.35,
                _ => 0.6,
            }
        } else {
            0.7 // Unknown provider
        }
    }

    /// Get factor weight
    fn get_factor_weight(&self, factor_id: &str) -> f64 {
        self.factors
            .iter()
            .find(|f| f.id == factor_id)
            .map(|f| f.weight)
            .unwrap_or(0.0)
    }

    /// Convert score to risk level
    fn score_to_level(&self, score: f64) -> RiskLevel {
        if score >= self.thresholds.critical {
            RiskLevel::Critical
        } else if score >= self.thresholds.high {
            RiskLevel::High
        } else if score >= self.thresholds.medium {
            RiskLevel::Medium
        } else if score >= self.thresholds.low {
            RiskLevel::Low
        } else {
            RiskLevel::Minimal
        }
    }

    /// Analyze usage event for risk
    pub async fn analyze_usage(&self, event: &AIUsageEvent) -> Result<f64> {
        let mut risk_score = 0.0;

        // Check data classification
        let classification_risk = match event.data_classification {
            DataClassification::Public => 0.1,
            DataClassification::Internal => 0.2,
            DataClassification::Confidential => 0.5,
            DataClassification::Restricted => 0.8,
            DataClassification::Sensitive => 0.9,
        };
        risk_score += classification_risk * 0.4;

        // Check risk indicators
        let indicator_risk = event.risk_indicators.len() as f64 * 0.1;
        risk_score += indicator_risk * 0.3;

        // Check request size (large requests may indicate data exfiltration)
        let size_risk = if event.request_size > 100_000 {
            0.3
        } else if event.request_size > 10_000 {
            0.1
        } else {
            0.0
        };
        risk_score += size_risk * 0.3;

        Ok(risk_score.min(1.0))
    }

    /// Get detailed risk assessment
    pub async fn get_detailed_assessment(&self, discovered: &DiscoveredAI) -> Result<RiskScore> {
        let (overall_score, risk_level) = self.calculate(discovered).await?;

        // Calculate individual factor scores
        let mut factor_scores = HashMap::new();
        factor_scores.insert(
            "data_exposure".to_string(),
            self.calculate_data_risk(discovered),
        );
        factor_scores.insert(
            "provider_trust".to_string(),
            self.calculate_provider_risk(discovered),
        );

        // Generate risk indicators
        let mut risk_indicators = Vec::new();

        if !discovered.risk_indicators.is_empty() {
            risk_indicators.push(RiskIndicator {
                indicator_type: "detected_indicators".to_string(),
                severity: 0.7,
                description: format!(
                    "Detected risk indicators: {}",
                    discovered.risk_indicators.join(", ")
                ),
                remediation: Some("Review and address detected risk indicators".to_string()),
            });
        }

        if discovered.details.api_key_masked.is_some() {
            risk_indicators.push(RiskIndicator {
                indicator_type: "api_key_exposed".to_string(),
                severity: 0.8,
                description: "API key detected in configuration".to_string(),
                remediation: Some("Rotate API key and store securely".to_string()),
            });
        }

        // Generate recommendations
        let mut recommendations = Vec::new();

        if risk_level >= RiskLevel::High {
            recommendations.push("Consider blocking this AI until approved".to_string());
        }
        if risk_level >= RiskLevel::Medium {
            recommendations.push("Request formal approval for this AI".to_string());
        }
        if !discovered.details.users.is_empty() {
            recommendations.push("Review user access to this AI".to_string());
        }

        Ok(RiskScore {
            id: uuid::Uuid::new_v4().to_string(),
            model_id: discovered.id.clone(),
            overall_score,
            risk_level,
            factor_scores,
            risk_indicators,
            recommendations,
            timestamp: Utc::now(),
        })
    }

    /// Add custom risk factor
    pub fn add_factor(&mut self, factor: RiskFactor) -> Result<()> {
        info!("Adding risk factor: {}", factor.name);
        self.factors.push(factor);
        Ok(())
    }

    /// Get risk factors
    pub fn get_factors(&self) -> &[RiskFactor] {
        &self.factors
    }

    /// Record risk score in history
    pub fn record_score(&mut self, score: &RiskScore) {
        self.risk_history
            .entry(score.model_id.clone())
            .or_insert_with(Vec::new)
            .push(score.clone());
    }

    /// Get risk history for a model
    pub fn get_risk_history(&self, model_id: &str) -> Option<&[RiskScore]> {
        self.risk_history.get(model_id).map(|v| v.as_slice())
    }
}

impl Default for RiskAssessment {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_risk_assessment_creation() {
        let assessment = RiskAssessment::new();
        assert!(!assessment.factors.is_empty());
    }

    #[test]
    fn test_risk_level_conversion() {
        let assessment = RiskAssessment::new();

        assert_eq!(assessment.score_to_level(0.1), RiskLevel::Minimal);
        assert_eq!(assessment.score_to_level(0.3), RiskLevel::Low);
        assert_eq!(assessment.score_to_level(0.5), RiskLevel::Medium);
        assert_eq!(assessment.score_to_level(0.7), RiskLevel::High);
        assert_eq!(assessment.score_to_level(0.9), RiskLevel::Critical);
    }
}
