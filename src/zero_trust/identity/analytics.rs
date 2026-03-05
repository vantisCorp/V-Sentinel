//! Identity Analytics Module
//! 
//! This module provides identity analytics including:
//! - Access pattern analysis
//! - Anomaly detection
//! - Risk scoring
//! - Compliance reporting

use serde::{Deserialize, Serialize};
use std::collections::{HashMap, VecDeque};
use std::sync::Arc;
use tokio::sync::RwLock;
use chrono::{DateTime, Utc, Duration};
use std::f64;

/// Identity Analytics Manager
pub struct IdentityAnalyticsManager {
    /// Access event log
    access_events: Arc<RwLock<VecDeque<AccessEvent>>>,
    /// User profiles
    user_profiles: Arc<RwLock<HashMap<String, UserProfile>>>,
    /// Anomaly detector
    anomaly_detector: Arc<AnomalyDetector>,
    /// Risk scores
    risk_scores: Arc<RwLock<HashMap<String, RiskScore>>>,
    /// Analytics configuration
    config: AnalyticsConfig,
}

impl IdentityAnalyticsManager {
    /// Create a new analytics manager
    pub fn new(config: AnalyticsConfig) -> Self {
        Self {
            access_events: Arc::new(RwLock::new(VecDeque::with_capacity(config.max_events))),
            user_profiles: Arc::new(RwLock::new(HashMap::new())),
            anomaly_detector: Arc::new(AnomalyDetector::new(config.clone())),
            risk_scores: Arc::new(RwLock::new(HashMap::new())),
            config,
        }
    }

    /// Record an access event
    pub async fn record_access_event(&self, event: AccessEvent) {
        let mut events = self.access_events.write().await;
        events.push_back(event.clone());
        
        // Trim old events
        while events.len() > self.config.max_events {
            events.pop_front();
        }
        
        drop(events);

        // Update user profile
        self.update_user_profile(&event).await;

        // Detect anomalies
        if let Some(anomaly) = self.anomaly_detector.detect(&event).await {
            self.handle_anomaly(&event, &anomaly).await;
        }

        // Update risk score
        self.update_risk_score(&event).await;
    }

    /// Update user profile
    async fn update_user_profile(&self, event: &AccessEvent) {
        let mut profiles = self.user_profiles.write().await;
        
        let profile = profiles.entry(event.user_id.clone()).or_insert_with(|| {
            UserProfile::new(event.user_id.clone())
        });

        profile.update_from_event(event);
    }

    /// Handle detected anomaly
    async fn handle_anomaly(&self, event: &AccessEvent, anomaly: &Anomaly) {
        // Log anomaly
        tracing::warn!(
            "Anomaly detected for user {}: {:?} - Score: {}",
            event.user_id,
            anomaly.anomaly_type,
            anomaly.score
        );

        // Update user risk score
        let mut scores = self.risk_scores.write().await;
        let risk = scores.entry(event.user_id.clone()).or_insert_with(|| {
            RiskScore::new(event.user_id.clone())
        });

        risk.add_anomaly(anomaly);
    }

    /// Update risk score
    async fn update_risk_score(&self, event: &AccessEvent) {
        let mut scores = self.risk_scores.write().await;
        let risk = scores.entry(event.user_id.clone()).or_insert_with(|| {
            RiskScore::new(event.user_id.clone())
        });

        risk.update_from_event(event, &self.config);
    }

    /// Get user risk score
    pub async fn get_risk_score(&self, user_id: &str) -> Option<RiskScore> {
        self.risk_scores.read().await.get(user_id).cloned()
    }

    /// Get user profile
    pub async fn get_user_profile(&self, user_id: &str) -> Option<UserProfile> {
        self.user_profiles.read().await.get(user_id).cloned()
    }

    /// Get access events for a user
    pub async fn get_user_events(
        &self,
        user_id: &str,
        limit: usize,
    ) -> Vec<AccessEvent> {
        self.access_events.read().await
            .iter()
            .filter(|e| e.user_id == user_id)
            .rev()
            .take(limit)
            .cloned()
            .collect()
    }

    /// Get high-risk users
    pub async fn get_high_risk_users(&self, threshold: f64, limit: usize) -> Vec<RiskScore> {
        self.risk_scores.read().await
            .values()
            .filter(|r| r.overall_score >= threshold)
            .cloned()
            .take(limit)
            .collect()
    }

    /// Get recent anomalies
    pub async fn get_recent_anomalies(&self, limit: usize) -> Vec<Anomaly> {
        self.anomaly_detector.get_recent_anomalies(limit).await
    }

    /// Generate access pattern report
    pub async fn generate_access_pattern_report(
        &self,
        user_id: &str,
        days: u32,
    ) -> AccessPatternReport {
        let since = Utc::now() - Duration::days(days as i64);
        
        let user_events = self.access_events.read().await
            .iter()
            .filter(|e| e.user_id == user_id && e.timestamp >= since)
            .cloned()
            .collect::<Vec<_>>();

        let mut hourly_distribution = HashMap::new();
        let mut daily_distribution = HashMap::new();
        let mut resource_counts = HashMap::new();
        let mut location_counts = HashMap::new();

        for event in &user_events {
            let hour = event.timestamp.hour();
            *hourly_distribution.entry(hour).or_insert(0) += 1;

            let date = event.timestamp.date_naive();
            *daily_distribution.entry(date).or_insert(0) += 1;

            *resource_counts.entry(event.resource.clone()).or_insert(0) += 1;

            if let Some(ref loc) = event.location {
                *location_counts.entry(loc.clone()).or_insert(0) += 1;
            }
        }

        AccessPatternReport {
            user_id: user_id.to_string(),
            period_start: since,
            period_end: Utc::now(),
            total_events: user_events.len(),
            hourly_distribution,
            daily_distribution,
            most_accessed_resources: resource_counts,
            most_used_locations: location_counts,
            unique_resources: resource_counts.len(),
            unique_locations: location_counts.len(),
        }
    }

    /// Generate compliance report
    pub async fn generate_compliance_report(
        &self,
        compliance_type: ComplianceType,
        days: u32,
    ) -> ComplianceReport {
        let since = Utc::now() - Duration::days(days as i64);
        
        let events = self.access_events.read().await
            .iter()
            .filter(|e| e.timestamp >= since)
            .cloned()
            .collect::<Vec<_>>();

        let mut user_access_counts = HashMap::new();
        let mut resource_access_counts = HashMap::new();
        let mut failed_access_attempts = 0;
        let mut privileged_access_count = 0;

        for event in &events {
            *user_access_counts.entry(event.user_id.clone()).or_insert(0) += 1;
            *resource_access_counts.entry(event.resource.clone()).or_insert(0) += 1;

            if !event.success {
                failed_access_attempts += 1;
            }

            if event.is_privileged {
                privileged_access_count += 1;
            }
        }

        let unique_users = user_access_counts.len();
        let unique_resources = resource_access_counts.len();
        let success_rate = if events.is_empty() {
            0.0
        } else {
            1.0 - (failed_access_attempts as f64 / events.len() as f64)
        };

        ComplianceReport {
            compliance_type,
            period_start: since,
            period_end: Utc::now(),
            total_events: events.len(),
            unique_users,
            unique_resources,
            failed_access_attempts,
            privileged_access_count,
            success_rate,
            riskiest_users: self.get_high_risk_users(0.7, 10).await,
            recommendations: self.generate_compliance_recommendations(
                compliance_type,
                success_rate,
                &user_access_counts,
            ),
        }
    }

    /// Generate compliance recommendations
    fn generate_compliance_recommendations(
        &self,
        _compliance_type: ComplianceType,
        success_rate: f64,
        _user_access_counts: &HashMap<String, u64>,
    ) -> Vec<String> {
        let mut recommendations = Vec::new();

        if success_rate < 0.95 {
            recommendations.push("High rate of failed access attempts detected. Review security policies.".to_string());
        }

        recommendations.push("Regular access reviews recommended.".to_string());
        recommendations.push("Implement automated access recertification.".to_string());

        recommendations
    }

    /// Get analytics summary
    pub async fn get_summary(&self) -> AnalyticsSummary {
        let events = self.access_events.read().await;
        let profiles = self.user_profiles.read().await;
        let risks = self.risk_scores.read().await;

        let total_users = profiles.len();
        let high_risk_count = risks.values().filter(|r| r.overall_score >= 0.7).count();
        let medium_risk_count = risks.values().filter(|r| r.overall_score >= 0.4 && r.overall_score < 0.7).count();

        let mut resource_access_count = HashMap::new();
        for event in events.iter() {
            *resource_access_count.entry(event.resource.clone()).or_insert(0) += 1;
        }

        let most_accessed_resources = resource_access_count
            .into_iter()
            .collect::<Vec<_>>()
            .into_iter()
            .map(|(r, c)| (r, c))
            .collect();

        AnalyticsSummary {
            total_events: events.len(),
            total_users,
            high_risk_users: high_risk_count,
            medium_risk_users: medium_risk_count,
            low_risk_users: total_users - high_risk_count - medium_risk_count,
            most_accessed_resources,
            active_sessions: profiles.values().filter(|p| p.is_active()).count(),
        }
    }
}

impl Default for IdentityAnalyticsManager {
    fn default() -> Self {
        Self::new(AnalyticsConfig::default())
    }
}

/// Analytics Configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnalyticsConfig {
    /// Maximum events to retain
    pub max_events: usize,
    /// Anomaly detection threshold
    pub anomaly_threshold: f64,
    /// Risk score decay rate (0.0-1.0 per hour)
    pub risk_decay_rate: f64,
    /// Pattern analysis window in hours
    pub pattern_window_hours: u32,
}

impl Default for AnalyticsConfig {
    fn default() -> Self {
        Self {
            max_events: 100000,
            anomaly_threshold: 0.7,
            risk_decay_rate: 0.05,
            pattern_window_hours: 24,
        }
    }
}

/// Access event
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccessEvent {
    /// Event ID
    pub event_id: String,
    /// User ID
    pub user_id: String,
    /// Resource accessed
    pub resource: String,
    /// Action performed
    pub action: String,
    /// Timestamp
    pub timestamp: DateTime<Utc>,
    /// Was successful
    pub success: bool,
    /// Is privileged access
    pub is_privileged: bool,
    /// IP address
    pub ip_address: String,
    /// Location
    pub location: Option<String>,
    /// Device
    pub device: Option<String>,
    /// User agent
    pub user_agent: Option<String>,
    /// MFA used
    pub mfa_used: bool,
    /// Risk score at time of access
    pub risk_score: f64,
}

/// User profile
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserProfile {
    /// User ID
    pub user_id: String,
    /// Last access time
    pub last_access: Option<DateTime<Utc>>,
    /// Total access count
    pub total_access_count: u64,
    /// Failed access count
    pub failed_access_count: u64,
    /// Privileged access count
    pub privileged_access_count: u64,
    /// Unique resources accessed
    pub unique_resources: HashSet<String>,
    /// Unique locations
    pub unique_locations: HashSet<String>,
    /// Unique devices
    pub unique_devices: HashSet<String>,
    /// Access pattern
    pub access_pattern: AccessPattern,
    /// Created at
    pub created_at: DateTime<Utc>,
}

impl UserProfile {
    /// Create a new user profile
    fn new(user_id: impl Into<String>) -> Self {
        Self {
            user_id: user_id.into(),
            last_access: None,
            total_access_count: 0,
            failed_access_count: 0,
            privileged_access_count: 0,
            unique_resources: HashSet::new(),
            unique_locations: HashSet::new(),
            unique_devices: HashSet::new(),
            access_pattern: AccessPattern::default(),
            created_at: Utc::now(),
        }
    }

    /// Update from event
    fn update_from_event(&mut self, event: &AccessEvent) {
        self.last_access = Some(event.timestamp);
        self.total_access_count += 1;
        
        if !event.success {
            self.failed_access_count += 1;
        }

        if event.is_privileged {
            self.privileged_access_count += 1;
        }

        self.unique_resources.insert(event.resource.clone());
        
        if let Some(ref loc) = event.location {
            self.unique_locations.insert(loc.clone());
        }

        if let Some(ref device) = event.device {
            self.unique_devices.insert(device.clone());
        }
    }

    /// Check if user is active
    pub fn is_active(&self) -> bool {
        if let Some(last) = self.last_access {
            (Utc::now() - last).num_hours() < 24
        } else {
            false
        }
    }
}

/// Access pattern
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccessPattern {
    /// Typical access hours
    pub typical_hours: Vec<u32>,
    /// Typical access days (0 = Sunday)
    pub typical_days: Vec<u32>,
    /// Average daily access count
    pub avg_daily_access: f64,
    /// Session duration average in minutes
    pub avg_session_duration: u32,
}

impl Default for AccessPattern {
    fn default() -> Self {
        Self {
            typical_hours: vec![9, 10, 11, 14, 15, 16, 17],
            typical_days: vec![1, 2, 3, 4, 5],
            avg_daily_access: 10.0,
            avg_session_duration: 480,
        }
    }
}

/// Risk score
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RiskScore {
    /// User ID
    pub user_id: String,
    /// Overall risk score (0.0-1.0)
    pub overall_score: f64,
    /// Access pattern risk
    pub access_pattern_risk: f64,
    /// Behavioral risk
    pub behavioral_risk: f64,
    /// Location risk
    pub location_risk: f64,
    /// Device risk
    pub device_risk: f64,
    /// Anomaly count
    pub anomaly_count: u32,
    /// Last updated
    pub last_updated: DateTime<Utc>,
}

impl RiskScore {
    /// Create a new risk score
    fn new(user_id: impl Into<String>) -> Self {
        Self {
            user_id: user_id.into(),
            overall_score: 0.5,
            access_pattern_risk: 0.5,
            behavioral_risk: 0.5,
            location_risk: 0.5,
            device_risk: 0.5,
            anomaly_count: 0,
            last_updated: Utc::now(),
        }
    }

    /// Add anomaly
    fn add_anomaly(&mut self, anomaly: &Anomaly) {
        self.anomaly_count += 1;
        self.overall_score = (self.overall_score + anomaly.score) / 2.0;
        self.last_updated = Utc::now();
    }

    /// Update from event
    fn update_from_event(&mut self, _event: &AccessEvent, _config: &AnalyticsConfig) {
        // Decay risk score over time
        let hours_since_update = (Utc::now() - self.last_updated).num_hours() as f64;
        let decay = _config.risk_decay_rate.powf(hours_since_update);
        self.overall_score = self.overall_score * decay;
        
        self.last_updated = Utc::now();
    }

    /// Get risk level
    pub fn risk_level(&self) -> RiskLevel {
        if self.overall_score >= 0.7 {
            RiskLevel::High
        } else if self.overall_score >= 0.4 {
            RiskLevel::Medium
        } else {
            RiskLevel::Low
        }
    }
}

/// Risk level
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum RiskLevel {
    Low,
    Medium,
    High,
    Critical,
}

/// Anomaly detector
pub struct AnomalyDetector {
    /// Recent anomalies
    anomalies: Arc<RwLock<VecDeque<Anomaly>>>,
    /// Configuration
    config: AnalyticsConfig,
    /// Access patterns
    access_patterns: Arc<RwLock<HashMap<String, Vec<AccessEvent>>>>,
}

impl AnomalyDetector {
    /// Create a new anomaly detector
    fn new(config: AnalyticsConfig) -> Self {
        Self {
            anomalies: Arc::new(RwLock::new(VecDeque::with_capacity(1000))),
            config,
            access_patterns: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Detect anomaly in event
    async fn detect(&self, event: &AccessEvent) -> Option<Anomaly> {
        let mut anomalies = Vec::new();

        // Check for unusual time access
        if let Some(time_anomaly) = self.check_time_anomaly(event).await {
            anomalies.push(time_anomaly);
        }

        // Check for unusual location
        if let Some(location_anomaly) = self.check_location_anomaly(event).await {
            anomalies.push(location_anomaly);
        }

        // Check for unusual device
        if let Some(device_anomaly) = self.check_device_anomaly(event).await {
            anomalies.push(device_anomaly);
        }

        // Return the highest scoring anomaly
        anomalies.into_iter()
            .max_by(|a, b| a.score.partial_cmp(&b.score).unwrap())
    }

    /// Check for time-based anomaly
    async fn check_time_anomaly(&self, event: &AccessEvent) -> Option<Anomaly> {
        let patterns = self.access_patterns.read().await;
        
        if let Some(user_events) = patterns.get(&event.user_id) {
            let hour = event.timestamp.hour();
            
            // Check if this is an unusual hour for this user
            let hour_counts: std::collections::HashMap<u32, u32> = user_events
                .iter()
                .map(|e| e.timestamp.hour())
                .fold(std::collections::HashMap::new(), |mut acc, h| {
                    *acc.entry(h).or_insert(0) += 1;
                    acc
                });

            let total: u32 = hour_counts.values().sum();
            let hour_count = *hour_counts.get(&hour).unwrap_or(&0);
            
            if total > 10 && hour_count == 0 {
                return Some(Anomaly {
                    id: uuid::Uuid::new_v4().to_string(),
                    user_id: event.user_id.clone(),
                    anomaly_type: AnomalyType::UnusualTime,
                    score: 0.6,
                    description: format!("Access at unusual hour: {}", hour),
                    detected_at: event.timestamp,
                    resolved: false,
                });
            }
        }

        None
    }

    /// Check for location-based anomaly
    async fn check_location_anomaly(&self, event: &AccessEvent) -> Option<Anomaly> {
        let patterns = self.access_patterns.read().await;
        
        if let Some(ref location) = event.location {
            if let Some(user_events) = patterns.get(&event.user_id) {
                let locations: std::collections::HashMap<String, u32> = user_events
                    .iter()
                    .filter_map(|e| e.location.as_ref())
                    .cloned()
                    .fold(std::collections::HashMap::new(), |mut acc, loc| {
                        *acc.entry(loc).or_insert(0) += 1;
                        acc
                    });

                if !locations.is_empty() && !locations.contains_key(location) {
                    return Some(Anomaly {
                        id: uuid::Uuid::new_v4().to_string(),
                        user_id: event.user_id.clone(),
                        anomaly_type: AnomalyType::UnusualLocation,
                        score: 0.7,
                        description: format!("Access from unusual location: {}", location),
                        detected_at: event.timestamp,
                        resolved: false,
                    });
                }
            }
        }

        None
    }

    /// Check for device-based anomaly
    async fn check_device_anomaly(&self, event: &AccessEvent) -> Option<Anomaly> {
        let patterns = self.access_patterns.read().await;
        
        if let Some(ref device) = event.device {
            if let Some(user_events) = patterns.get(&event.user_id) {
                let devices: std::collections::HashMap<String, u32> = user_events
                    .iter()
                    .filter_map(|e| e.device.as_ref())
                    .cloned()
                    .fold(std::collections::HashMap::new(), |mut acc, dev| {
                        *acc.entry(dev).or_insert(0) += 1;
                        acc
                    });

                if !devices.is_empty() && !devices.contains_key(device) {
                    return Some(Anomaly {
                        id: uuid::Uuid::new_v4().to_string(),
                        user_id: event.user_id.clone(),
                        anomaly_type: AnomalyType::UnusualDevice,
                        score: 0.65,
                        description: format!("Access from unusual device: {}", device),
                        detected_at: event.timestamp,
                        resolved: false,
                    });
                }
            }
        }

        None
    }

    /// Get recent anomalies
    pub async fn get_recent_anomalies(&self, limit: usize) -> Vec<Anomaly> {
        self.anomalies.read().await
            .iter()
            .rev()
            .take(limit)
            .cloned()
            .collect()
    }

    /// Add event to patterns
    pub async fn add_to_patterns(&self, event: AccessEvent) {
        let mut patterns = self.access_patterns.write().await;
        patterns
            .entry(event.user_id.clone())
            .or_insert_with(Vec::new)
            .push(event);
    }
}

/// Anomaly
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Anomaly {
    /// Anomaly ID
    pub id: String,
    /// User ID
    pub user_id: String,
    /// Anomaly type
    pub anomaly_type: AnomalyType,
    /// Anomaly score (0.0-1.0)
    pub score: f64,
    /// Description
    pub description: String,
    /// Detected at
    pub detected_at: DateTime<Utc>,
    /// Is resolved
    pub resolved: bool,
}

/// Anomaly type
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AnomalyType {
    UnusualTime,
    UnusualLocation,
    UnusualDevice,
    ExcessiveAccess,
    FailedAttempts,
    PrivilegeEscalation,
    DataExfiltration,
}

/// Access pattern report
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccessPatternReport {
    pub user_id: String,
    pub period_start: DateTime<Utc>,
    pub period_end: DateTime<Utc>,
    pub total_events: usize,
    pub hourly_distribution: HashMap<u32, u64>,
    pub daily_distribution: HashMap<chrono::NaiveDate, u64>,
    pub most_accessed_resources: HashMap<String, u64>,
    pub most_used_locations: HashMap<String, u64>,
    pub unique_resources: usize,
    pub unique_locations: usize,
}

/// Compliance report
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComplianceReport {
    pub compliance_type: ComplianceType,
    pub period_start: DateTime<Utc>,
    pub period_end: DateTime<Utc>,
    pub total_events: usize,
    pub unique_users: usize,
    pub unique_resources: usize,
    pub failed_access_attempts: u64,
    pub privileged_access_count: u64,
    pub success_rate: f64,
    pub riskiest_users: Vec<RiskScore>,
    pub recommendations: Vec<String>,
}

/// Compliance type
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ComplianceType {
    SOX,
    GDPR,
    HIPAA,
    PCI,
    ISO27001,
    NIST80053,
    SOC2,
    Custom(String),
}

/// Analytics summary
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnalyticsSummary {
    pub total_events: usize,
    pub total_users: usize,
    pub high_risk_users: usize,
    pub medium_risk_users: usize,
    pub low_risk_users: usize,
    pub most_accessed_resources: Vec<(String, u64)>,
    pub active_sessions: usize,
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_analytics() -> IdentityAnalyticsManager {
        IdentityAnalyticsManager::new(AnalyticsConfig::default())
    }

    #[tokio::test]
    async fn test_record_access_event() {
        let analytics = create_test_analytics();

        let event = AccessEvent {
            event_id: uuid::Uuid::new_v4().to_string(),
            user_id: "user1".to_string(),
            resource: "/api/data".to_string(),
            action: "read".to_string(),
            timestamp: Utc::now(),
            success: true,
            is_privileged: false,
            ip_address: "10.0.0.1".to_string(),
            location: Some("US".to_string()),
            device: Some("laptop-1".to_string()),
            user_agent: Some("Mozilla/5.0".to_string()),
            mfa_used: true,
            risk_score: 0.2,
        };

        analytics.record_access_event(event).await;
    }

    #[tokio::test]
    async fn test_user_profile() {
        let analytics = create_test_analytics();

        let event = AccessEvent {
            event_id: uuid::Uuid::new_v4().to_string(),
            user_id: "user1".to_string(),
            resource: "/api/data".to_string(),
            action: "read".to_string(),
            timestamp: Utc::now(),
            success: true,
            is_privileged: false,
            ip_address: "10.0.0.1".to_string(),
            location: Some("US".to_string()),
            device: Some("laptop-1".to_string()),
            user_agent: Some("Mozilla/5.0".to_string()),
            mfa_used: true,
            risk_score: 0.2,
        };

        analytics.record_access_event(event).await;

        let profile = analytics.get_user_profile("user1").await;
        assert!(profile.is_some());
        assert_eq!(profile.unwrap().total_access_count, 1);
    }

    #[tokio::test]
    async fn test_risk_score() {
        let analytics = create_test_analytics();

        let event = AccessEvent {
            event_id: uuid::Uuid::new_v4().to_string(),
            user_id: "user1".to_string(),
            resource: "/api/data".to_string(),
            action: "read".to_string(),
            timestamp: Utc::now(),
            success: true,
            is_privileged: false,
            ip_address: "10.0.0.1".to_string(),
            location: Some("US".to_string()),
            device: Some("laptop-1".to_string()),
            user_agent: Some("Mozilla/5.0".to_string()),
            mfa_used: true,
            risk_score: 0.2,
        };

        analytics.record_access_event(event).await;

        let risk = analytics.get_risk_score("user1").await;
        assert!(risk.is_some());
    }

    #[tokio::test]
    async fn test_access_pattern_report() {
        let analytics = create_test_analytics();

        let event = AccessEvent {
            event_id: uuid::Uuid::new_v4().to_string(),
            user_id: "user1".to_string(),
            resource: "/api/data".to_string(),
            action: "read".to_string(),
            timestamp: Utc::now(),
            success: true,
            is_privileged: false,
            ip_address: "10.0.0.1".to_string(),
            location: Some("US".to_string()),
            device: Some("laptop-1".to_string()),
            user_agent: Some("Mozilla/5.0".to_string()),
            mfa_used: true,
            risk_score: 0.2,
        };

        analytics.record_access_event(event).await;

        let report = analytics.generate_access_pattern_report("user1", 7).await;
        assert_eq!(report.user_id, "user1");
        assert!(report.total_events >= 1);
    }

    #[tokio::test]
    async fn test_analytics_summary() {
        let analytics = create_test_analytics();

        let event = AccessEvent {
            event_id: uuid::Uuid::new_v4().to_string(),
            user_id: "user1".to_string(),
            resource: "/api/data".to_string(),
            action: "read".to_string(),
            timestamp: Utc::now(),
            success: true,
            is_privileged: false,
            ip_address: "10.0.0.1".to_string(),
            location: Some("US".to_string()),
            device: Some("laptop-1".to_string()),
            user_agent: Some("Mozilla/5.0".to_string()),
            mfa_used: true,
            risk_score: 0.2,
        };

        analytics.record_access_event(event).await;

        let summary = analytics.get_summary().await;
        assert!(summary.total_events >= 1);
        assert!(summary.total_users >= 1);
    }
}