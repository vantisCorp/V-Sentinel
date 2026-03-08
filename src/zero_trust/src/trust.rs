//! Trust Engine Module
//! 
//! Implements dynamic trust scoring based on multiple factors including:
//! - Device trustworthiness
//! - User behavior analysis
//! - Location and time patterns
//! - Risk indicators

use anyhow::Result;
use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use tracing::{info, debug};
use chrono::{DateTime, Utc, Duration, Timelike, Datelike};

use super::{AccessRequest, Subject, DeviceInfo, RiskIndicator};

/// Trust Engine
/// 
/// Calculates and manages trust scores for access decisions
pub struct TrustEngine {
    factors: Vec<Box<dyn TrustFactorEvaluator + Send + Sync>>,
    statistics: TrustStatistics,
    trust_history: HashMap<String, Vec<TrustScoreSnapshot>>,
}

/// Trust score result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrustScore {
    /// Overall trust score (0.0 - 1.0)
    pub score: f64,
    /// Individual factor scores
    pub factors: HashMap<String, FactorScore>,
    /// Risk adjustments
    pub risk_adjustments: Vec<RiskAdjustment>,
    /// Timestamp
    pub timestamp: DateTime<Utc>,
    /// Confidence level
    pub confidence: f64,
}

/// Individual factor score
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FactorScore {
    pub factor_name: String,
    pub score: f64,
    pub weight: f64,
    pub details: HashMap<String, serde_json::Value>,
}

/// Risk adjustment to trust score
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RiskAdjustment {
    pub risk_type: String,
    pub adjustment: f64,
    pub reason: String,
}

/// Trust score snapshot for history
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrustScoreSnapshot {
    pub score: f64,
    pub timestamp: DateTime<Utc>,
    pub context: String,
}

/// Trust factor definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrustFactor {
    pub name: String,
    pub weight: f64,
    pub description: String,
    pub enabled: bool,
}

/// Trust statistics
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TrustStatistics {
    pub total_requests: u64,
    pub allowed_requests: u64,
    pub denied_requests: u64,
    pub challenged_requests: u64,
    pub total_trust_score: f64,
    pub low_trust_count: u64,
    pub high_trust_count: u64,
}

/// Trait for trust factor evaluators
#[async_trait::async_trait]
pub trait TrustFactorEvaluator {
    fn name(&self) -> &str;
    fn weight(&self) -> f64;
    async fn evaluate(&self, request: &AccessRequest) -> Result<FactorScore>;
}

/// Device Trust Factor
pub struct DeviceTrustFactor {
    weight: f64,
}

/// Behavior Trust Factor
pub struct BehaviorTrustFactor {
    weight: f64,
    behavior_history: HashMap<String, BehaviorProfile>,
}

/// Location Trust Factor
pub struct LocationTrustFactor {
    weight: f64,
    known_locations: HashMap<String, Vec<KnownLocation>>,
}

/// Time Trust Factor
pub struct TimeTrustFactor {
    weight: f64,
    work_hours: (i32, i32), // Start hour, end hour (0-23)
}

/// Session Trust Factor
pub struct SessionTrustFactor {
    weight: f64,
}

/// Behavior profile for a user
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BehaviorProfile {
    pub user_id: String,
    pub typical_access_times: Vec<i32>,
    pub typical_resources: Vec<String>,
    pub typical_actions: Vec<String>,
    pub average_session_duration: i64,
    pub anomaly_score: f64,
}

/// Known location
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KnownLocation {
    pub country: String,
    pub region: String,
    pub city: String,
    pub latitude: f64,
    pub longitude: f64,
    pub radius_km: f64,
    pub last_visit: DateTime<Utc>,
    pub visit_count: u64,
    pub is_trusted: bool,
}

impl TrustEngine {
    /// Create a new Trust Engine
    pub fn new() -> Self {
        let mut engine = Self {
            factors: Vec::new(),
            statistics: TrustStatistics::default(),
            trust_history: HashMap::new(),
        };
        
        // Add default trust factors
        engine.add_factor(Box::new(DeviceTrustFactor::new()));
        engine.add_factor(Box::new(BehaviorTrustFactor::new()));
        engine.add_factor(Box::new(LocationTrustFactor::new()));
        engine.add_factor(Box::new(TimeTrustFactor::new()));
        engine.add_factor(Box::new(SessionTrustFactor::new()));
        
        engine
    }

    /// Add a trust factor
    pub fn add_factor(&mut self, factor: Box<dyn TrustFactorEvaluator + Send + Sync>) {
        self.factors.push(factor);
    }

    /// Calculate trust score for a request
    pub async fn calculate_score(&self, request: &AccessRequest) -> Result<f64> {
        info!("Calculating trust score for request {}", request.id);
        
        let mut total_weight = 0.0;
        let mut weighted_score = 0.0;
        let mut factors = HashMap::new();
        
        // Evaluate each trust factor
        for factor in &self.factors {
            let factor_score = factor.evaluate(request).await?;
            weighted_score += factor_score.score * factor_score.weight;
            total_weight += factor_score.weight;
            factors.insert(factor_score.factor_name.clone(), factor_score);
        }
        
        // Calculate base score
        let base_score = if total_weight > 0.0 {
            weighted_score / total_weight
        } else {
            0.5 // Default neutral score
        };
        
        // Apply risk adjustments
        let risk_adjustment = self.calculate_risk_adjustment(&request.context.risk_indicators);
        let final_score = (base_score + risk_adjustment).clamp(0.0, 1.0);
        
        debug!("Trust score: base={}, risk_adj={}, final={}", base_score, risk_adjustment, final_score);
        
        Ok(final_score)
    }

    /// Calculate risk adjustment based on risk indicators
    fn calculate_risk_adjustment(&self, indicators: &[RiskIndicator]) -> f64 {
        let mut adjustment = 0.0;
        
        for indicator in indicators {
            let penalty = match indicator.indicator_type {
                super::RiskIndicatorType::AnomalousLocation => -0.15,
                super::RiskIndicatorType::AnomalousTime => -0.10,
                super::RiskIndicatorType::AnomalousBehavior => -0.20,
                super::RiskIndicatorType::CompromisedCredentials => -0.50,
                super::RiskIndicatorType::UnmanagedDevice => -0.25,
                super::RiskIndicatorType::VulnerableDevice => -0.15,
                super::RiskIndicatorType::ThreatIntelligenceMatch => -0.40,
                super::RiskIndicatorType::ImpossibleTravel => -0.35,
                super::RiskIndicatorType::BruteForceAttempt => -0.45,
                super::RiskIndicatorType::CredentialStuffing => -0.40,
            };
            
            adjustment += penalty * indicator.severity;
        }
        
        adjustment
    }

    /// Record a trust decision
    pub fn record_decision(&mut self, score: f64, allowed: bool) {
        self.statistics.total_requests += 1;
        self.statistics.total_trust_score += score;
        
        if allowed {
            self.statistics.allowed_requests += 1;
        } else if score < 0.3 {
            self.statistics.denied_requests += 1;
            self.statistics.low_trust_count += 1;
        } else {
            self.statistics.challenged_requests += 1;
        }
        
        if score > 0.8 {
            self.statistics.high_trust_count += 1;
        }
    }

    /// Get average trust score
    pub fn average_trust_score(&self) -> f64 {
        if self.statistics.total_requests == 0 {
            return 0.5;
        }
        self.statistics.total_trust_score / self.statistics.total_requests as f64
    }

    /// Get total requests
    pub fn total_requests(&self) -> u64 {
        self.statistics.total_requests
    }

    /// Get allowed requests count
    pub fn allowed_requests(&self) -> u64 {
        self.statistics.allowed_requests
    }

    /// Get denied requests count
    pub fn denied_requests(&self) -> u64 {
        self.statistics.denied_requests
    }

    /// Get challenged requests count
    pub fn challenged_requests(&self) -> u64 {
        self.statistics.challenged_requests
    }
}

impl Default for TrustEngine {
    fn default() -> Self {
        Self::new()
    }
}

// Device Trust Factor Implementation
impl DeviceTrustFactor {
    pub fn new() -> Self {
        Self { weight: 0.25 }
    }
}

#[async_trait::async_trait]
impl TrustFactorEvaluator for DeviceTrustFactor {
    fn name(&self) -> &str {
        "device"
    }
    
    fn weight(&self) -> f64 {
        self.weight
    }
    
    async fn evaluate(&self, request: &AccessRequest) -> Result<FactorScore> {
        let mut score: f64 = 0.5; // Base score for unknown device
        let mut details = HashMap::new();
        
        if let Some(ref device) = request.subject.device {
            // Managed device bonus
            if device.is_managed {
                score += 0.2;
                details.insert("managed".to_string(), serde_json::json!(true));
            }
            
            // Compliant device bonus
            if device.is_compliant {
                score += 0.15;
                details.insert("compliant".to_string(), serde_json::json!(true));
            }
            
            // Device type consideration
            match device.device_type {
                super::DeviceType::Desktop | super::DeviceType::Server => score += 0.05,
                super::DeviceType::Mobile | super::DeviceType::Tablet => score += 0.02,
                super::DeviceType::IoT => score -= 0.1,
                super::DeviceType::Unknown => score -= 0.15,
            }
            
            // Device risk level
            score -= device.risk_level * 0.3;
            
            // Last seen recency
            let hours_since_seen = (Utc::now() - device.last_seen).num_hours() as f64;
            if hours_since_seen < 24.0 {
                score += 0.05;
            } else if hours_since_seen > 720.0 { // 30 days
                score -= 0.1;
            }
            
            details.insert("device_id".to_string(), serde_json::json!(device.device_id));
            details.insert("device_type".to_string(), serde_json::json!(format!("{:?}", device.device_type)));
        }
        
        Ok(FactorScore {
            factor_name: self.name().to_string(),
            score: score.clamp(0.0, 1.0),
            weight: self.weight,
            details,
        })
    }
}

// Behavior Trust Factor Implementation
impl BehaviorTrustFactor {
    pub fn new() -> Self {
        Self {
            weight: 0.20,
            behavior_history: HashMap::new(),
        }
    }
}

#[async_trait::async_trait]
impl TrustFactorEvaluator for BehaviorTrustFactor {
    fn name(&self) -> &str {
        "behavior"
    }
    
    fn weight(&self) -> f64 {
        self.weight
    }
    
    async fn evaluate(&self, request: &AccessRequest) -> Result<FactorScore> {
        let mut score: f64 = 0.5;
        let mut details = HashMap::new();
        
        // Check if user has behavior profile
        if let Some(profile) = self.behavior_history.get(&request.subject.id) {
            // Check resource access pattern
            let resource_known = profile.typical_resources.contains(&request.resource.id);
            if resource_known {
                score += 0.15;
            } else {
                score -= 0.1;
            }
            
            // Check action pattern
            let action_known = profile.typical_actions.iter().any(|a| {
                format!("{:?}", request.action.action_type).contains(a)
            });
            if action_known {
                score += 0.1;
            }
            
            // Use anomaly score from profile
            score -= profile.anomaly_score * 0.2;
            
            details.insert("has_profile".to_string(), serde_json::json!(true));
            details.insert("anomaly_score".to_string(), serde_json::json!(profile.anomaly_score));
        } else {
            // New user, neutral score with slight penalty
            score -= 0.05;
            details.insert("has_profile".to_string(), serde_json::json!(false));
        }
        
        Ok(FactorScore {
            factor_name: self.name().to_string(),
            score: score.clamp(0.0, 1.0),
            weight: self.weight,
            details,
        })
    }
}

// Location Trust Factor Implementation
impl LocationTrustFactor {
    pub fn new() -> Self {
        Self {
            weight: 0.15,
            known_locations: HashMap::new(),
        }
    }
}

#[async_trait::async_trait]
impl TrustFactorEvaluator for LocationTrustFactor {
    fn name(&self) -> &str {
        "location"
    }
    
    fn weight(&self) -> f64 {
        self.weight
    }
    
    async fn evaluate(&self, request: &AccessRequest) -> Result<FactorScore> {
        let mut score: f64 = 0.5;
        let mut details = HashMap::new();
        
        if let Some(ref geo) = request.context.geo_location {
            // Check if location is known for this user
            if let Some(locations) = self.known_locations.get(&request.subject.id) {
                let mut found_known = false;
                
                for known in locations {
                    let distance = calculate_distance(
                        geo.latitude, geo.longitude,
                        known.latitude, known.longitude
                    );
                    
                    if distance <= known.radius_km {
                        found_known = true;
                        if known.is_trusted {
                            score += 0.25;
                        } else {
                            score += 0.1;
                        }
                        details.insert("location_type".to_string(), serde_json::json!("known"));
                        break;
                    }
                }
                
                if !found_known {
                    score -= 0.15;
                    details.insert("location_type".to_string(), serde_json::json!("unknown"));
                }
            }
            
            details.insert("country".to_string(), serde_json::json!(geo.country));
            details.insert("city".to_string(), serde_json::json!(geo.city));
        }
        
        Ok(FactorScore {
            factor_name: self.name().to_string(),
            score: score.clamp(0.0, 1.0),
            weight: self.weight,
            details,
        })
    }
}

// Time Trust Factor Implementation
impl TimeTrustFactor {
    pub fn new() -> Self {
        Self {
            weight: 0.15,
            work_hours: (8, 18), // 8 AM to 6 PM
        }
    }
}

#[async_trait::async_trait]
impl TrustFactorEvaluator for TimeTrustFactor {
    fn name(&self) -> &str {
        "time"
    }
    
    fn weight(&self) -> f64 {
        self.weight
    }
    
    async fn evaluate(&self, request: &AccessRequest) -> Result<FactorScore> {
        let mut score: f64 = 0.5;
        let mut details = HashMap::new();
        
        let hour = request.context.access_time.hour() as i32;
        let is_work_hours = hour >= self.work_hours.0 && hour < self.work_hours.1;
        
        if is_work_hours {
            score += 0.15;
            details.insert("work_hours".to_string(), serde_json::json!(true));
        } else {
            score -= 0.05;
            details.insert("work_hours".to_string(), serde_json::json!(false));
        }
        
        // Weekend penalty
        let weekday = request.context.access_time.weekday();
        if weekday == chrono::Weekday::Sat || weekday == chrono::Weekday::Sun {
            score -= 0.05;
            details.insert("weekend".to_string(), serde_json::json!(true));
        }
        
        details.insert("hour".to_string(), serde_json::json!(hour));
        
        Ok(FactorScore {
            factor_name: self.name().to_string(),
            score: score.clamp(0.0, 1.0),
            weight: self.weight,
            details,
        })
    }
}

// Session Trust Factor Implementation
impl SessionTrustFactor {
    pub fn new() -> Self {
        Self { weight: 0.25 }
    }
}

#[async_trait::async_trait]
impl TrustFactorEvaluator for SessionTrustFactor {
    fn name(&self) -> &str {
        "session"
    }
    
    fn weight(&self) -> f64 {
        self.weight
    }
    
    async fn evaluate(&self, request: &AccessRequest) -> Result<FactorScore> {
        let mut score: f64 = 0.5;
        let mut details = HashMap::new();
        
        // Check session maturity
        if request.context.session_id.is_some() {
            // Existing session
            let request_count = request.context.session_request_count;
            
            if request_count > 0 {
                score += 0.1; // Bonus for established session
                details.insert("session_state".to_string(), serde_json::json!("established"));
            } else {
                details.insert("session_state".to_string(), serde_json::json!("new"));
            }
            
            details.insert("request_count".to_string(), serde_json::json!(request_count));
        } else {
            // No session - first request
            details.insert("session_state".to_string(), serde_json::json!("none"));
        }
        
        Ok(FactorScore {
            factor_name: self.name().to_string(),
            score: score.clamp(0.0, 1.0),
            weight: self.weight,
            details,
        })
    }
}

/// Calculate distance between two geographic points using Haversine formula
fn calculate_distance(lat1: f64, lon1: f64, lat2: f64, lon2: f64) -> f64 {
    const EARTH_RADIUS_KM: f64 = 6371.0;
    
    let lat1_rad = lat1.to_radians();
    let lat2_rad = lat2.to_radians();
    let delta_lat = (lat2 - lat1).to_radians();
    let delta_lon = (lon2 - lon1).to_radians();
    
    let a = (delta_lat / 2.0).sin().powi(2) +
            lat1_rad.cos() * lat2_rad.cos() * (delta_lon / 2.0).sin().powi(2);
    let c = 2.0 * a.sqrt().atan2((1.0 - a).sqrt());
    
    EARTH_RADIUS_KM * c
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_trust_engine_creation() {
        let engine = TrustEngine::new();
        assert!(!engine.factors.is_empty());
    }

    #[test]
    fn test_distance_calculation() {
        // New York to Los Angeles ~3944 km
        let distance = calculate_distance(40.7128, -74.0060, 34.0522, -118.2437);
        assert!(distance > 3900.0 && distance < 4000.0);
    }
}