//! Continuous authentication manager for Zero Trust Architecture

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Continuous authentication manager
pub struct ContinuousAuthManager {
    /// Active sessions
    sessions: HashMap<String, SessionInfo>,
    
    /// Authentication configuration
    config: AuthConfig,
}

impl ContinuousAuthManager {
    /// Create a new continuous authentication manager
    pub fn new(config: AuthConfig) -> Self {
        Self {
            sessions: HashMap::new(),
            config,
        }
    }
    
    /// Create with default configuration
    pub fn with_defaults() -> Self {
        Self::new(AuthConfig::default())
    }
    
    /// Start a new session
    pub fn start_session(
        &mut self,
        subject: &str,
        device_id: &str,
    ) -> SessionInfo {
        let session = SessionInfo {
            session_id: uuid::Uuid::new_v4().to_string(),
            subject: subject.to_string(),
            device_id: device_id.to_string(),
            start_time: Utc::now(),
            last_activity: Utc::now(),
            auth_state: AuthState::Active,
            mfa_performed: false,
            risk_score: 0.0,
            activity_count: 0,
            anomaly_count: 0,
            last_mfa_time: None,
            behavioral_baseline: BehavioralBaseline::default(),
        };
        
        self.sessions.insert(session.session_id.clone(), session.clone());
        session
    }
    
    /// Record session activity
    pub fn record_activity(&mut self, session_id: &str) -> Result<(), AuthError> {
        if let Some(session) = self.sessions.get_mut(session_id) {
            session.last_activity = Utc::now();
            session.activity_count += 1;
            
            // Check if re-authentication needed
            self.check_reauth_needed(session);
            
            Ok(())
        } else {
            Err(AuthError::SessionNotFound)
        }
    }
    
    /// Get session info
    pub fn get_session(&self, session_id: &str) -> Option<&SessionInfo> {
        self.sessions.get(session_id)
    }
    
    /// End session
    pub fn end_session(&mut self, session_id: &str) -> Option<SessionInfo> {
        self.sessions.remove(session_id)
    }
    
    /// Update risk score
    pub fn update_risk_score(&mut self, session_id: &str, score: f64) -> Result<(), AuthError> {
        if let Some(session) = self.sessions.get_mut(session_id) {
            session.risk_score = score;
            
            // Check if session should be terminated due to high risk
            if score >= self.config.max_risk_score {
                session.auth_state = AuthState::Terminated;
                return Err(AuthError::HighRiskDetected);
            }
            
            Ok(())
        } else {
            Err(AuthError::SessionNotFound)
        }
    }
    
    /// Record anomaly
    pub fn record_anomaly(&mut self, session_id: &str, anomaly_type: &str) -> Result<(), AuthError> {
        if let Some(session) = self.sessions.get_mut(session_id) {
            session.anomaly_count += 1;
            
            // Terminate if too many anomalies
            if session.anomaly_count >= self.config.max_anomalies {
                session.auth_state = AuthState::Terminated;
                return Err(AuthError::TooManyAnomalies);
            }
            
            Ok(())
        } else {
            Err(AuthError::SessionNotFound)
        }
    }
    
    /// Record MFA completion
    pub fn record_mfa(&mut self, session_id: &str) -> Result<(), AuthError> {
        if let Some(session) = self.sessions.get_mut(session_id) {
            session.mfa_performed = true;
            session.last_mfa_time = Some(Utc::now());
            session.auth_state = AuthState::Active;
            Ok(())
        } else {
            Err(AuthError::SessionNotFound)
        }
    }
    
    /// Require MFA for session
    pub fn require_mfa(&mut self, session_id: &str) -> Result<(), AuthError> {
        if let Some(session) = self.sessions.get_mut(session_id) {
            session.auth_state = AuthState::MfaRequired;
            Ok(())
        } else {
            Err(AuthError::SessionNotFound)
        }
    }
    
    /// Check if re-authentication is needed
    fn check_reauth_needed(&self, session: &mut SessionInfo) {
        let elapsed = Utc::now()
            .signed_duration_since(session.last_mfa_time.unwrap_or(session.start_time))
            .num_minutes();
        
        if elapsed > self.config.mfa_reauth_interval_minutes as i64 {
            session.auth_state = AuthState::MfaRequired;
        }
    }
    
    /// Clean up expired sessions
    pub fn cleanup_expired(&mut self) -> usize {
        let now = Utc::now();
        let max_age = chrono::Duration::hours(self.config.max_session_age_hours as i64);
        
        let expired: Vec<String> = self.sessions
            .iter()
            .filter(|(_, session)| {
                now.signed_duration_since(session.last_activity) > max_age
            })
            .map(|(id, _)| id.clone())
            .collect();
        
        let count = expired.len();
        for id in expired {
            self.sessions.remove(&id);
        }
        count
    }
}

/// Authentication configuration
#[derive(Debug, Clone)]
pub struct AuthConfig {
    /// Maximum session age in hours
    pub max_session_age_hours: u32,
    
    /// Maximum risk score before termination
    pub max_risk_score: f64,
    
    /// Maximum anomalies before termination
    pub max_anomalies: u32,
    
    /// MFA re-authentication interval in minutes
    pub mfa_reauth_interval_minutes: u32,
    
    /// Maximum concurrent sessions per user
    pub max_concurrent_sessions: u32,
}

impl Default for AuthConfig {
    fn default() -> Self {
        Self {
            max_session_age_hours: 8,
            max_risk_score: 0.8,
            max_anomalies: 5,
            mfa_reauth_interval_minutes: 60,
            max_concurrent_sessions: 5,
        }
    }
}

/// Session information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionInfo {
    /// Session identifier
    pub session_id: String,
    
    /// User subject
    pub subject: String,
    
    /// Device ID
    pub device_id: String,
    
    /// Session start time
    pub start_time: DateTime<Utc>,
    
    /// Last activity timestamp
    pub last_activity: DateTime<Utc>,
    
    /// Authentication state
    pub auth_state: AuthState,
    
    /// Has MFA been performed
    pub mfa_performed: bool,
    
    /// Current risk score
    pub risk_score: f64,
    
    /// Activity count
    pub activity_count: u32,
    
    /// Anomaly count
    pub anomaly_count: u32,
    
    /// Last MFA timestamp
    pub last_mfa_time: Option<DateTime<Utc>>,
    
    /// Behavioral baseline
    pub behavioral_baseline: BehavioralBaseline,
}

/// Authentication state
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AuthState {
    /// Session is active
    Active,
    /// MFA required
    MfaRequired,
    /// Step-up authentication required
    StepUpRequired,
    /// Session is suspended
    Suspended,
    /// Session is terminated
    Terminated,
}

/// Behavioral baseline for user
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct BehavioralBaseline {
    /// Typical login hours
    pub typical_hours: Vec<u8>,
    
    /// Typical locations
    pub typical_locations: Vec<String>,
    
    /// Typical devices
    pub typical_devices: Vec<String>,
    
    /// Typical resources accessed
    pub typical_resources: Vec<String>,
    
    /// Average typing speed (WPM)
    pub typing_speed: Option<f64>,
    
    /// Mouse movement patterns
    pub mouse_patterns: Option<MousePatterns>,
}

/// Mouse movement patterns
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MousePatterns {
    /// Average movement speed
    pub avg_speed: f64,
    
    /// Movement variability
    pub variability: f64,
    
    /// Click timing pattern
    pub click_pattern: Vec<f64>,
}

impl Default for MousePatterns {
    fn default() -> Self {
        Self {
            avg_speed: 500.0,
            variability: 0.3,
            click_pattern: Vec::new(),
        }
    }
}

/// Authentication error types
#[derive(Debug, Clone, thiserror::Error)]
pub enum AuthError {
    #[error("Session not found")]
    SessionNotFound,
    
    #[error("High risk detected")]
    HighRiskDetected,
    
    #[error("Too many anomalies")]
    TooManyAnomalies,
    
    #[error("Session expired")]
    SessionExpired,
    
    #[error("MFA required")]
    MfaRequired,
    
    #[error("Authentication failed: {0}")]
    AuthFailed(String),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_start_session() {
        let mut manager = ContinuousAuthManager::with_defaults();
        let session = manager.start_session("user@example.com", "device-123");
        
        assert!(!session.session_id.is_empty());
        assert_eq!(session.subject, "user@example.com");
        assert_eq!(session.auth_state, AuthState::Active);
    }

    #[test]
    fn test_record_activity() {
        let mut manager = ContinuousAuthManager::with_defaults();
        let session = manager.start_session("user@example.com", "device-123");
        
        let result = manager.record_activity(&session.session_id);
        assert!(result.is_ok());
        
        let updated = manager.get_session(&session.session_id).unwrap();
        assert_eq!(updated.activity_count, 1);
    }

    #[test]
    fn test_update_risk_score() {
        let mut manager = ContinuousAuthManager::with_defaults();
        let session = manager.start_session("user@example.com", "device-123");
        
        let result = manager.update_risk_score(&session.session_id, 0.5);
        assert!(result.is_ok());
        
        let updated = manager.get_session(&session.session_id).unwrap();
        assert!((updated.risk_score - 0.5).abs() < 0.001);
    }

    #[test]
    fn test_high_risk_termination() {
        let mut manager = ContinuousAuthManager::with_defaults();
        let session = manager.start_session("user@example.com", "device-123");
        
        let result = manager.update_risk_score(&session.session_id, 0.9);
        assert!(matches!(result, Err(AuthError::HighRiskDetected)));
        
        let updated = manager.get_session(&session.session_id).unwrap();
        assert_eq!(updated.auth_state, AuthState::Terminated);
    }

    #[test]
    fn test_end_session() {
        let mut manager = ContinuousAuthManager::with_defaults();
        let session = manager.start_session("user@example.com", "device-123");
        
        let ended = manager.end_session(&session.session_id);
        assert!(ended.is_some());
        
        assert!(manager.get_session(&session.session_id).is_none());
    }
}