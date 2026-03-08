//! Adaptive Authentication Module
//! 
//! This module provides risk-based authentication flows that adapt
//! security requirements based on contextual risk assessment.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use chrono::{DateTime, Utc, Duration};

use super::mfa::{MfaManager, MfaMethod, MfaVerificationResult};
use super::biometrics::{BiometricsManager, BiometricAnalysisResult, RiskLevel};
use super::continuous::{ContinuousAuthManager, AuthState};
use crate::trust::trust_score::{TrustCalculator, TrustScore};

/// Adaptive Authentication Configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdaptiveAuthConfig {
    /// Minimum trust score required for password-only auth
    pub password_only_threshold: f64,
    /// Minimum trust score required for single-factor MFA
    pub single_factor_threshold: f64,
    /// Minimum trust score for step-up auth
    pub step_up_threshold: f64,
    /// Maximum session duration in minutes
    pub max_session_duration: u32,
    /// Risk score threshold for requiring MFA
    pub mfa_risk_threshold: f64,
    /// Enable continuous authentication
    pub enable_continuous_auth: bool,
    /// Enable behavioral biometrics
    pub enable_biometrics: bool,
    /// Re-authentication interval for high-risk sessions (minutes)
    pub high_risk_reauth_interval: u32,
    /// Re-authentication interval for medium-risk sessions (minutes)
    pub medium_risk_reauth_interval: u32,
}

impl Default for AdaptiveAuthConfig {
    fn default() -> Self {
        Self {
            password_only_threshold: 0.9,
            single_factor_threshold: 0.7,
            step_up_threshold: 0.5,
            max_session_duration: 480, // 8 hours
            mfa_risk_threshold: 0.6,
            enable_continuous_auth: true,
            enable_biometrics: true,
            high_risk_reauth_interval: 15,
            medium_risk_reauth_interval: 60,
        }
    }
}

/// Authentication requirement level
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum AuthRequirement {
    /// Password-only authentication sufficient
    PasswordOnly,
    /// Single factor MFA required
    SingleFactorMfa(MfaMethod),
    /// Multiple MFA factors required
    MultiFactorMfa(Vec<MfaMethod>),
    /// Step-up authentication required
    StepUpAuth(MfaMethod),
    /// Full re-authentication required
    ReAuthenticate,
    /// Access denied
    Denied(String),
}

/// Adaptive authentication decision
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdaptiveAuthDecision {
    /// User ID
    pub user_id: String,
    /// Session ID
    pub session_id: String,
    /// Required authentication level
    pub auth_requirement: AuthRequirement,
    /// Current trust score
    pub trust_score: f64,
    /// Risk level
    pub risk_level: RiskLevel,
    /// Reason for the decision
    pub reason: String,
    /// Recommended MFA methods
    pub recommended_methods: Vec<MfaMethod>,
    /// Timestamp
    pub timestamp: DateTime<Utc>,
    /// Expires at (for step-up auth)
    pub expires_at: Option<DateTime<Utc>>,
}

/// Authentication context for risk assessment
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthContext {
    /// User ID
    pub user_id: String,
    /// Session ID
    pub session_id: String,
    /// Current trust score
    pub trust_score: TrustScore,
    /// Device fingerprint
    pub device_fingerprint: Option<String>,
    /// IP address
    pub ip_address: String,
    /// Geographic location (country code)
    pub geo_location: Option<String>,
    /// Time of access
    pub access_time: DateTime<Utc>,
    /// Resource being accessed
    pub resource: String,
    /// Action being performed
    pub action: String,
    /// Previous authentication history
    pub auth_history: Vec<AuthEvent>,
    /// Behavioral analysis result
    pub biometric_result: Option<BiometricAnalysisResult>,
}

/// Authentication event
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthEvent {
    pub timestamp: DateTime<Utc>,
    pub event_type: AuthEventType,
    pub method: Option<MfaMethod>,
    pub success: bool,
    pub risk_score: f64,
    pub location: Option<String>,
}

/// Authentication event types
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum AuthEventType {
    Login,
    Logout,
    MfaChallenge,
    MfaSuccess,
    MfaFailure,
    SessionTimeout,
    StepUpAuth,
    AccessDenied,
    AnomalyDetected,
}

/// Risk factor for adaptive authentication
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RiskFactor {
    pub name: String,
    pub weight: f64,
    pub score: f64,
    pub description: String,
}

/// Adaptive Authentication Manager
pub struct AdaptiveAuthManager {
    config: AdaptiveAuthConfig,
    mfa_manager: Arc<MfaManager>,
    biometrics_manager: Arc<BiometricsManager>,
    continuous_auth_manager: Arc<ContinuousAuthManager>,
    trust_calculator: Arc<TrustCalculator>,
    auth_history: Arc<RwLock<HashMap<String, Vec<AuthEvent>>>>,
    session_requirements: Arc<RwLock<HashMap<String, AuthRequirement>>>,
}

impl AdaptiveAuthManager {
    /// Create a new Adaptive Authentication Manager
    pub fn new(
        config: AdaptiveAuthConfig,
        mfa_manager: Arc<MfaManager>,
        biometrics_manager: Arc<BiometricsManager>,
        continuous_auth_manager: Arc<ContinuousAuthManager>,
        trust_calculator: Arc<TrustCalculator>,
    ) -> Self {
        Self {
            config,
            mfa_manager,
            biometrics_manager,
            continuous_auth_manager,
            trust_calculator,
            auth_history: Arc::new(RwLock::new(HashMap::new())),
            session_requirements: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Evaluate authentication requirements
    pub async fn evaluate_auth_requirements(
        &self,
        context: &AuthContext,
    ) -> AdaptiveAuthDecision {
        // Calculate risk factors
        let risk_factors = self.calculate_risk_factors(context).await;
        
        // Calculate overall risk score
        let overall_risk = self.calculate_overall_risk(&risk_factors);
        
        // Determine risk level
        let risk_level = self.determine_risk_level(overall_risk);
        
        // Determine authentication requirement
        let auth_requirement = self.determine_auth_requirement(
            context.trust_score.score(),
            overall_risk,
            &risk_factors,
            &context.auth_history,
        );
        
        // Get recommended MFA methods
        let recommended_methods = self.recommend_mfa_methods(&auth_requirement, &risk_factors);
        
        // Determine reason
        let reason = self.generate_decision_reason(&risk_factors, &auth_requirement);
        
        // Check if step-up auth has expiration
        let expires_at = match &auth_requirement {
            AuthRequirement::StepUpAuth(_) => Some(Utc::now() + Duration::minutes(5)),
            _ => None,
        };

        AdaptiveAuthDecision {
            user_id: context.user_id.clone(),
            session_id: context.session_id.clone(),
            auth_requirement,
            trust_score: context.trust_score.score(),
            risk_level,
            reason,
            recommended_methods,
            timestamp: Utc::now(),
            expires_at,
        }
    }

    /// Calculate risk factors
    async fn calculate_risk_factors(&self, context: &AuthContext) -> Vec<RiskFactor> {
        let mut factors = Vec::new();

        // Trust score factor
        factors.push(RiskFactor {
            name: "trust_score".to_string(),
            weight: 0.25,
            score: context.trust_score.score(),
            description: format!("Trust score: {:.2}", context.trust_score.score()),
        });

        // Biometric risk factor
        if let Some(ref biometric) = context.biometric_result {
            factors.push(RiskFactor {
                name: "biometrics".to_string(),
                weight: 0.20,
                score: biometric.overall_score,
                description: format!(
                    "Biometric match: {:.2}, Anomalies: {}",
                    biometric.overall_score,
                    biometric.anomalies.len()
                ),
            });
        }

        // Geographic risk factor
        if let Some(ref geo) = context.geo_location {
            let geo_score = self.evaluate_geo_risk(geo, &context.auth_history);
            factors.push(RiskFactor {
                name: "geographic".to_string(),
                weight: 0.15,
                score: geo_score,
                description: format!("Location risk: {} ({:.2})", geo, geo_score),
            });
        }

        // Time-based risk factor
        let time_score = self.evaluate_time_risk(context.access_time, &context.auth_history);
        factors.push(RiskFactor {
            name: "time".to_string(),
            weight: 0.10,
            score: time_score,
            description: format!("Time-based risk: {:.2}", time_score),
        });

        // Device risk factor
        let device_score = if let Some(ref _device) = context.device_fingerprint {
            0.8 // Known device
        } else {
            0.4 // Unknown device
        };
        factors.push(RiskFactor {
            name: "device".to_string(),
            weight: 0.15,
            score: device_score,
            description: if context.device_fingerprint.is_some() {
                "Known device".to_string()
            } else {
                "Unknown device".to_string()
            },
        });

        // Authentication history risk factor
        let history_score = self.evaluate_history_risk(&context.auth_history);
        factors.push(RiskFactor {
            name: "auth_history".to_string(),
            weight: 0.15,
            score: history_score,
            description: format!("Auth history risk: {:.2}", history_score),
        });

        factors
    }

    /// Calculate overall risk score
    fn calculate_overall_risk(&self, factors: &[RiskFactor]) -> f64 {
        let total_weight: f64 = factors.iter().map(|f| f.weight).sum();
        let weighted_score: f64 = factors.iter()
            .map(|f| f.score * f.weight)
            .sum();
        
        if total_weight > 0.0 {
            weighted_score / total_weight
        } else {
            0.5 // Neutral risk
        }
    }

    /// Determine risk level from score
    fn determine_risk_level(&self, score: f64) -> RiskLevel {
        if score >= 0.8 {
            RiskLevel::Low
        } else if score >= 0.6 {
            RiskLevel::Medium
        } else if score >= 0.4 {
            RiskLevel::High
        } else {
            RiskLevel::Critical
        }
    }

    /// Determine authentication requirement
    fn determine_auth_requirement(
        &self,
        trust_score: f64,
        risk_score: f64,
        factors: &[RiskFactor],
        _auth_history: &[AuthEvent],
    ) -> AuthRequirement {
        // Check for critical risk factors
        for factor in factors {
            if factor.score < 0.3 {
                match factor.name.as_str() {
                    "geographic" => {
                        return AuthRequirement::StepUpAuth(MfaMethod::Push);
                    }
                    "device" => {
                        return AuthRequirement::MultiFactorMfa(vec![
                            MfaMethod::Totp,
                            MfaMethod::Push,
                        ]);
                    }
                    "biometrics" => {
                        return AuthRequirement::StepUpAuth(MfaMethod::Fido2);
                    }
                    _ => {}
                }
            }
        }

        // General risk-based decision
        if trust_score >= self.config.password_only_threshold && risk_score >= 0.8 {
            AuthRequirement::PasswordOnly
        } else if trust_score >= self.config.single_factor_threshold || risk_score >= 0.6 {
            if risk_score < 0.5 {
                AuthRequirement::MultiFactorMfa(vec![MfaMethod::Totp, MfaMethod::Push])
            } else {
                AuthRequirement::SingleFactorMfa(MfaMethod::Totp)
            }
        } else if trust_score >= self.config.step_up_threshold {
            AuthRequirement::StepUpAuth(MfaMethod::Totp)
        } else {
            AuthRequirement::Denied("Risk level too high for authentication".to_string())
        }
    }

    /// Recommend MFA methods based on context
    fn recommend_mfa_methods(
        &self,
        requirement: &AuthRequirement,
        _factors: &[RiskFactor],
    ) -> Vec<MfaMethod> {
        match requirement {
            AuthRequirement::PasswordOnly => vec![],
            AuthRequirement::SingleFactorMfa(method) => {
                vec![method.clone(), MfaMethod::Push]
            }
            AuthRequirement::MultiFactorMfa(methods) => methods.clone(),
            AuthRequirement::StepUpAuth(method) => {
                vec![method.clone(), MfaMethod::Fido2]
            }
            AuthRequirement::ReAuthenticate => vec![MfaMethod::Totp, MfaMethod::Fido2],
            AuthRequirement::Denied(_) => vec![],
        }
    }

    /// Generate decision reason
    fn generate_decision_reason(
        &self,
        factors: &[RiskFactor],
        requirement: &AuthRequirement,
    ) -> String {
        let mut reasons = Vec::new();

        for factor in factors {
            if factor.score < 0.5 {
                reasons.push(format!("Low {} score ({:.2})", factor.name, factor.score));
            }
        }

        let req_desc = match requirement {
            AuthRequirement::PasswordOnly => "Password authentication sufficient".to_string(),
            AuthRequirement::SingleFactorMfa(_) => "Single-factor MFA required".to_string(),
            AuthRequirement::MultiFactorMfa(methods) => {
                format!("Multi-factor MFA required: {} factors", methods.len())
            }
            AuthRequirement::StepUpAuth(_) => "Step-up authentication required".to_string(),
            AuthRequirement::ReAuthenticate => "Full re-authentication required".to_string(),
            AuthRequirement::Denied(reason) => format!("Access denied: {}", reason),
        };

        if reasons.is_empty() {
            req_desc
        } else {
            format!("{} due to: {}", req_desc, reasons.join(", "))
        }
    }

    /// Evaluate geographic risk
    fn evaluate_geo_risk(&self, location: &str, history: &[AuthEvent]) -> f64 {
        // Check if location is in recent history
        let recent_locations: Vec<_> = history.iter()
            .filter_map(|e| e.location.clone())
            .collect();

        if recent_locations.iter().any(|l| l == location) {
            0.9 // Known location
        } else {
            0.5 // New location
        }
    }

    /// Evaluate time-based risk
    fn evaluate_time_risk(&self, access_time: DateTime<Utc>, _history: &[AuthEvent]) -> f64 {
        let hour = access_time.hour();
        
        // Business hours (9 AM - 6 PM) are lower risk
        if hour >= 9 && hour <= 18 {
            0.9
        } else if hour >= 7 && hour <= 22 {
            0.7 // Extended business hours
        } else {
            0.5 // After hours
        }
    }

    /// Evaluate authentication history risk
    fn evaluate_history_risk(&self, history: &[AuthEvent]) -> f64 {
        if history.is_empty() {
            return 0.5; // Neutral for new users
        }

        // Check recent failure rate
        let recent: Vec<_> = history.iter()
            .filter(|e| e.timestamp > Utc::now() - Duration::hours(24))
            .collect();

        if recent.is_empty() {
            return 0.7;
        }

        let failures = recent.iter().filter(|e| !e.success).count();
        let failure_rate = failures as f64 / recent.len() as f64;

        1.0 - (failure_rate * 0.5)
    }

    /// Record authentication event
    pub async fn record_auth_event(
        &self,
        user_id: &str,
        event: AuthEvent,
    ) {
        let mut history = self.auth_history.write().await;
        history
            .entry(user_id.to_string())
            .or_insert_with(Vec::new)
            .push(event);
    }

    /// Get authentication history
    pub async fn get_auth_history(&self, user_id: &str) -> Vec<AuthEvent> {
        self.auth_history.read().await
            .get(user_id)
            .cloned()
            .unwrap_or_default()
    }

    /// Check if re-authentication is required
    pub async fn check_reauth_required(
        &self,
        session_id: &str,
        current_risk_level: &RiskLevel,
    ) -> bool {
        let requirements = self.session_requirements.read().await;
        
        if let Some(requirement) = requirements.get(session_id) {
            match (requirement, current_risk_level) {
                (AuthRequirement::SingleFactorMfa(_), RiskLevel::High) => true,
                (AuthRequirement::PasswordOnly, RiskLevel::Medium) => true,
                (AuthRequirement::PasswordOnly, RiskLevel::High) => true,
                (AuthRequirement::PasswordOnly, RiskLevel::Critical) => true,
                _ => false,
            }
        } else {
            false
        }
    }

    /// Update session authentication requirement
    pub async fn update_session_requirement(
        &self,
        session_id: &str,
        requirement: AuthRequirement,
    ) {
        self.session_requirements.write().await
            .insert(session_id.to_string(), requirement);
    }

    /// Clear session authentication requirement
    pub async fn clear_session_requirement(&self, session_id: &str) {
        self.session_requirements.write().await.remove(session_id);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_managers() -> (Arc<MfaManager>, Arc<BiometricsManager>, Arc<ContinuousAuthManager>, Arc<TrustCalculator>) {
        let mfa = Arc::new(MfaManager::new(Default::default()));
        let biometrics = Arc::new(BiometricsManager::new(Default::default()));
        let continuous = Arc::new(ContinuousAuthManager::new(Default::default()));
        let trust = Arc::new(TrustCalculator::new(Default::default()));
        (mfa, biometrics, continuous, trust)
    }

    #[tokio::test]
    async fn test_adaptive_auth_evaluation() {
        let (mfa, biometrics, continuous, trust) = create_test_managers();
        let manager = AdaptiveAuthManager::new(
            AdaptiveAuthConfig::default(),
            mfa,
            biometrics,
            continuous,
            trust,
        );

        let context = AuthContext {
            user_id: "user1".to_string(),
            session_id: "session1".to_string(),
            trust_score: TrustScore::new(0.85),
            device_fingerprint: Some("fp123".to_string()),
            ip_address: "192.168.1.1".to_string(),
            geo_location: Some("US".to_string()),
            access_time: Utc::now(),
            resource: "/api/data".to_string(),
            action: "read".to_string(),
            auth_history: vec![],
            biometric_result: None,
        };

        let decision = manager.evaluate_auth_requirements(&context).await;
        
        assert_eq!(decision.user_id, "user1");
        assert!(decision.trust_score >= 0.0);
    }

    #[tokio::test]
    async fn test_risk_factor_calculation() {
        let (mfa, biometrics, continuous, trust) = create_test_managers();
        let manager = AdaptiveAuthManager::new(
            AdaptiveAuthConfig::default(),
            mfa,
            biometrics,
            continuous,
            trust,
        );

        let context = AuthContext {
            user_id: "user1".to_string(),
            session_id: "session1".to_string(),
            trust_score: TrustScore::new(0.7),
            device_fingerprint: None,
            ip_address: "10.0.0.1".to_string(),
            geo_location: Some("CN".to_string()),
            access_time: Utc::now(),
            resource: "/admin/users".to_string(),
            action: "delete".to_string(),
            auth_history: vec![],
            biometric_result: Some(BiometricAnalysisResult {
                user_id: "user1".to_string(),
                overall_score: 0.6,
                risk_level: RiskLevel::Medium,
                keystroke_score: Some(0.65),
                mouse_score: Some(0.55),
                device_match_score: Some(0.6),
                activity_score: None,
                anomalies: vec!["Unusual typing pattern".to_string()],
                recommendations: vec![],
                timestamp: Utc::now(),
            }),
        };

        let decision = manager.evaluate_auth_requirements(&context).await;
        
        // Should require more authentication due to risk factors
        assert!(matches!(
            decision.auth_requirement,
            AuthRequirement::SingleFactorMfa(_) | 
            AuthRequirement::MultiFactorMfa(_) |
            AuthRequirement::StepUpAuth(_)
        ));
    }

    #[tokio::test]
    async fn test_auth_history_recording() {
        let (mfa, biometrics, continuous, trust) = create_test_managers();
        let manager = AdaptiveAuthManager::new(
            AdaptiveAuthConfig::default(),
            mfa,
            biometrics,
            continuous,
            trust,
        );

        let event = AuthEvent {
            timestamp: Utc::now(),
            event_type: AuthEventType::MfaSuccess,
            method: Some(MfaMethod::Totp),
            success: true,
            risk_score: 0.1,
            location: Some("US".to_string()),
        };

        manager.record_auth_event("user1", event).await;
        
        let history = manager.get_auth_history("user1").await;
        assert_eq!(history.len(), 1);
    }

    #[tokio::test]
    async fn test_session_requirement_management() {
        let (mfa, biometrics, continuous, trust) = create_test_managers();
        let manager = AdaptiveAuthManager::new(
            AdaptiveAuthConfig::default(),
            mfa,
            biometrics,
            continuous,
            trust,
        );

        manager.update_session_requirement("session1", AuthRequirement::SingleFactorMfa(MfaMethod::Totp)).await;
        
        assert!(manager.check_reauth_required("session1", &RiskLevel::High).await);
        assert!(!manager.check_reauth_required("session1", &RiskLevel::Low).await);

        manager.clear_session_requirement("session1").await;
        assert!(!manager.check_reauth_required("session1", &RiskLevel::High).await);
    }
}