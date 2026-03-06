//! AI Threat Defense Engine
//! 
//! Provides comprehensive threat detection, analysis, and automated response
//! for AI-specific threats including adversarial attacks, extraction, and inversion.

use crate::ai_security::models::*;
use crate::ai_security::AISecurityError;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

/// Threat Defense Configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThreatDefenseConfig {
    /// Enable automated response
    pub automated_response: bool,
    /// Enable threat detection
    pub threat_detection: bool,
    /// Detection sensitivity
    pub detection_sensitivity: f64,
    /// Maximum severity for auto-response
    pub auto_response_max_severity: Severity,
}

impl Default for ThreatDefenseConfig {
    fn default() -> Self {
        Self {
            automated_response: true,
            threat_detection: true,
            detection_sensitivity: 0.7,
            auto_response_max_severity: Severity::High,
        }
    }
}

/// Threat Defense Engine
pub struct ThreatDefenseEngine {
    config: ThreatDefenseConfig,
    threat_detector: ThreatDetector,
    response_engine: ResponseEngine,
    threat_store: Arc<RwLock<HashMap<String, AIThreat>>>,
    response_store: Arc<RwLock<HashMap<String, ThreatResponse>>>,
}

impl ThreatDefenseEngine {
    /// Create a new Threat Defense Engine
    pub async fn new(config: ThreatDefenseConfig) -> Result<Self, AISecurityError> {
        let threat_detector = ThreatDetector::new(config.detection_sensitivity);
        let response_engine = ResponseEngine::new(config.automated_response);
        let threat_store = Arc::new(RwLock::new(HashMap::new()));
        let response_store = Arc::new(RwLock::new(HashMap::new()));

        Ok(Self {
            config,
            threat_detector,
            response_engine,
            threat_store,
            response_store,
        })
    }

    /// Defend against an AI threat
    pub async fn defend_against(&self, threat: &AIThreat) -> Result<ThreatResponse, AISecurityError> {
        // Step 1: Store threat
        let mut threat_store = self.threat_store.write().await;
        threat_store.insert(threat.id.clone(), threat.clone());
        drop(threat_store);

        // Step 2: Analyze threat
        let analysis = self.threat_detector.analyze(threat).await?;

        // Step 3: Determine response actions
        let actions = self.response_engine.determine_actions(threat, &analysis).await?;

        // Step 4: Execute response
        let response = if threat.severity <= self.config.auto_response_max_severity {
            self.response_engine.execute_actions(&threat.id, &actions).await?
        } else {
            ThreatResponse {
                threat_id: threat.id.clone(),
                actions: vec![ResponseAction {
                    action_type: ResponseActionType::Escalated,
                    description: "Threat severity exceeds auto-response threshold - manual review required".to_string(),
                    result: ActionResult::Pending,
                    executed_at: Utc::now(),
                }],
                status: ResponseStatus::Escalated,
                recommendations: vec![
                    format!("Review threat {} manually", threat.id),
                    "Consider activating additional security measures".to_string(),
                ],
                responded_at: Utc::now(),
            }
        };

        // Step 5: Store response
        let mut response_store = self.response_store.write().await;
        response_store.insert(threat.id.clone(), response.clone());

        Ok(response)
    }

    /// Detect threats from indicators
    pub async fn detect_threat(&self, indicators: Vec<ThreatIndicator>) -> Result<Option<AIThreat>, AISecurityError> {
        if !self.config.threat_detection {
            return Ok(None);
        }

        self.threat_detector.detect_from_indicators(indicators).await
    }

    /// Get threat by ID
    pub async fn get_threat(&self, threat_id: &str) -> Option<AIThreat> {
        let store = self.threat_store.read().await;
        store.get(threat_id).cloned()
    }

    /// Get response by ID
    pub async fn get_response(&self, threat_id: &str) -> Option<ThreatResponse> {
        let store = self.response_store.read().await;
        store.get(threat_id).cloned()
    }

    /// Get component status
    pub async fn get_status(&self) -> Result<ComponentStatus, AISecurityError> {
        let threat_store = self.threat_store.read().await;
        let response_store = self.response_store.read().await;
        
        let threats_detected = threat_store.len() as u32;
        
        Ok(ComponentStatus {
            healthy: true,
            active_protections: response_store.len() as u32,
            threats_detected,
            last_check: Utc::now(),
        })
    }
}

/// Threat Detector
pub struct ThreatDetector {
    sensitivity: f64,
    threat_patterns: HashMap<AIThreatType, ThreatPattern>,
}

impl ThreatDetector {
    pub fn new(sensitivity: f64) -> Self {
        Self {
            sensitivity,
            threat_patterns: Self::load_threat_patterns(),
        }
    }

    fn load_threat_patterns() -> HashMap<AIThreatType, ThreatPattern> {
        let mut patterns = HashMap::new();

        patterns.insert(
            AIThreatType::AdversarialAttack,
            ThreatPattern {
                indicators: vec!["high_confidence_error".to_string(), "perturbation_pattern".to_string()],
                confidence_threshold: 0.8,
                severity: Severity::High,
            },
        );

        patterns.insert(
            AIThreatType::ModelExtraction,
            ThreatPattern {
                indicators: vec!["high_query_frequency".to_string(), "exhaustive_queries".to_string()],
                confidence_threshold: 0.7,
                severity: Severity::High,
            },
        );

        patterns.insert(
            AIThreatType::ModelInversion,
            ThreatPattern {
                indicators: vec!["targeted_queries".to_string(), "confidence_probing".to_string()],
                confidence_threshold: 0.75,
                severity: Severity::Medium,
            },
        );

        patterns.insert(
            AIThreatType::DataPoisoning,
            ThreatPattern {
                indicators: vec!["label_anomaly".to_string(), "data_distribution_shift".to_string()],
                confidence_threshold: 0.8,
                severity: Severity::Critical,
            },
        );

        patterns.insert(
            AIThreatType::PromptInjection,
            ThreatPattern {
                indicators: vec!["instruction_override".to_string(), "jailbreak_attempt".to_string()],
                confidence_threshold: 0.9,
                severity: Severity::High,
            },
        );

        patterns.insert(
            AIThreatType::EvasionAttack,
            ThreatPattern {
                indicators: vec!["adversarial_perturbation".to_string(), "input_manipulation".to_string()],
                confidence_threshold: 0.85,
                severity: Severity::High,
            },
        );

        patterns.insert(
            AIThreatType::BackdoorAttack,
            ThreatPattern {
                indicators: vec!["trigger_pattern".to_string(), "anomalous_behavior".to_string()],
                confidence_threshold: 0.9,
                severity: Severity::Critical,
            },
        );

        patterns.insert(
            AIThreatType::MembershipInference,
            ThreatPattern {
                indicators: vec!["membership_probing".to_string(), "training_data_inference".to_string()],
                confidence_threshold: 0.7,
                severity: Severity::Medium,
            },
        );

        patterns.insert(
            AIThreatType::AttributeInference,
            ThreatPattern {
                indicators: vec!["attribute_probing".to_string(), "sensitive_attribute_inference".to_string()],
                confidence_threshold: 0.75,
                severity: Severity::High,
            },
        );

        patterns.insert(
            AIThreatType::ModelStealing,
            ThreatPattern {
                indicators: vec!["model_replication".to_string(), "parameter_extraction".to_string()],
                confidence_threshold: 0.85,
                severity: Severity::High,
            },
        );

        patterns
    }

    pub async fn analyze(&self, threat: &AIThreat) -> Result<ThreatAnalysis, AISecurityError> {
        let pattern = self.threat_patterns.get(&threat.threat_type);
        
        let mut confidence = 0.0;
        if let Some(pattern) = pattern {
            confidence = threat.indicators
                .iter()
                .map(|indicator| indicator.confidence)
                .fold(0.0, |acc, x| acc.max(x));
        }

        let severity = if confidence > self.sensitivity {
            pattern.map(|p| p.severity.clone()).unwrap_or(Severity::Medium)
        } else {
            Severity::Low
        };

        Ok(ThreatAnalysis {
            confidence,
            severity,
            recommended_actions: self.get_recommended_actions(&threat.threat_type, &severity).await,
        })
    }

    pub async fn detect_from_indicators(&self, indicators: Vec<ThreatIndicator>) -> Result<Option<AIThreat>, AISecurityError> {
        let mut max_confidence = 0.0;
        let mut detected_type = None;

        for (threat_type, pattern) in &self.threat_patterns {
            let mut match_count = 0;
            let mut total_confidence = 0.0;

            for indicator in &indicators {
                if pattern.indicators.iter().any(|p| indicator.indicator_type.contains(p)) {
                    match_count += 1;
                    total_confidence += indicator.confidence;
                }
            }

            if match_count > 0 {
                let avg_confidence = total_confidence / match_count as f64;
                if avg_confidence > max_confidence && avg_confidence > self.sensitivity {
                    max_confidence = avg_confidence;
                    detected_type = Some(threat_type.clone());
                }
            }
        }

        if let Some(threat_type) = detected_type {
            let pattern = self.threat_patterns.get(&threat_type);
            let severity = pattern.map(|p| p.severity.clone()).unwrap_or(Severity::Medium);

            Ok(Some(AIThreat {
                id: format!("threat_{}", Utc::now().timestamp_millis()),
                threat_type,
                source: "detection_engine".to_string(),
                target: "ai_system".to_string(),
                severity,
                indicators,
                detected_at: Utc::now(),
            }))
        } else {
            Ok(None)
        }
    }

    async fn get_recommended_actions(&self, threat_type: &AIThreatType, severity: &Severity) -> Vec<ResponseActionType> {
        match (threat_type, severity) {
            (AIThreatType::AdversarialAttack, Severity::High | Severity::Critical) => {
                vec![ResponseActionType::Block, ResponseActionType::Alert, ResponseActionType::Mitigate]
            }
            (AIThreatType::ModelExtraction, Severity::High | Severity::Critical) => {
                vec![ResponseActionType::Block, ResponseActionType::Isolate, ResponseActionType::Alert]
            }
            (AIThreatType::DataPoisoning, Severity::Critical) => {
                vec![ResponseActionType::Quarantine, ResponseActionType::Alert, ResponseActionType::Rollback]
            }
            (AIThreatType::PromptInjection, Severity::High) => {
                vec![ResponseActionType::Block, ResponseActionType::Alert]
            }
            (AIThreatType::BackdoorAttack, Severity::Critical) => {
                vec![ResponseActionType::Quarantine, ResponseActionType::Rollback, ResponseActionType::Alert]
            }
            (_, _) => {
                vec![ResponseActionType::Alert, ResponseActionType::Investigate]
            }
        }
    }
}

/// Threat Pattern
struct ThreatPattern {
    indicators: Vec<String>,
    confidence_threshold: f64,
    severity: Severity,
}

/// Threat Analysis
struct ThreatAnalysis {
    confidence: f64,
    severity: Severity,
    recommended_actions: Vec<ResponseActionType>,
}

/// Response Engine
pub struct ResponseEngine {
    automated_response: bool,
}

impl ResponseEngine {
    pub fn new(automated_response: bool) -> Self {
        Self { automated_response }
    }

    pub async fn determine_actions(
        &self,
        threat: &AIThreat,
        analysis: &ThreatAnalysis,
    ) -> Result<Vec<ResponseAction>, AISecurityError> {
        let actions = analysis.recommended_actions
            .iter()
            .map(|action_type| ResponseAction {
                action_type: action_type.clone(),
                description: format!("Automatic response for {} threat", threat.threat_type),
                result: ActionResult::Pending,
                executed_at: Utc::now(),
            })
            .collect();

        Ok(actions)
    }

    pub async fn execute_actions(&self, threat_id: &str, actions: &[ResponseAction]) -> Result<ThreatResponse, AISecurityError> {
        let executed_actions = if self.automated_response {
            actions.iter()
                .map(|action| ResponseAction {
                    action_type: action.action_type.clone(),
                    description: action.description.clone(),
                    result: ActionResult::Success,
                    executed_at: Utc::now(),
                })
                .collect()
        } else {
            actions.iter()
                .map(|action| ResponseAction {
                    action_type: action.action_type.clone(),
                    description: action.description.clone(),
                    result: ActionResult::Pending,
                    executed_at: Utc::now(),
                })
                .collect()
        };

        Ok(ThreatResponse {
            threat_id: threat_id.to_string(),
            actions: executed_actions,
            status: if self.automated_response {
                ResponseStatus::Resolved
            } else {
                ResponseStatus::Monitoring
            },
            recommendations: vec![
                "Continue monitoring for related threats".to_string(),
                "Review security logs for additional indicators".to_string(),
            ],
            responded_at: Utc::now(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_threat_defense_engine_creation() {
        let config = ThreatDefenseConfig::default();
        let engine = ThreatDefenseEngine::new(config).await;
        assert!(engine.is_ok());
    }

    #[tokio::test]
    async fn test_defend_against_threat() {
        let config = ThreatDefenseConfig::default();
        let engine = ThreatDefenseEngine::new(config).await.unwrap();
        
        let threat = AIThreat {
            id: "threat-1".to_string(),
            threat_type: AIThreatType::AdversarialAttack,
            source: "external".to_string(),
            target: "model-1".to_string(),
            severity: Severity::High,
            indicators: vec![ThreatIndicator {
                indicator_type: "high_confidence_error".to_string(),
                value: "1.0".to_string(),
                confidence: 0.9,
                context: "input perturbation detected".to_string(),
            }],
            detected_at: Utc::now(),
        };
        
        let result = engine.defend_against(&threat).await;
        assert!(result.is_ok());
    }
}