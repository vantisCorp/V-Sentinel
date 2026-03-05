//! Trust scoring system for Zero Trust Architecture

use super::super::{DeviceInfo, NetworkInfo};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Trust score (0.0 to 1.0)
pub type TrustScore = f64;

/// Trust level classification
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TrustLevel {
    /// Very low trust - deny access
    VeryLow,
    /// Low trust - require additional verification
    Low,
    /// Medium trust - standard access
    Medium,
    /// High trust - enhanced access
    High,
    /// Very high trust - full access
    VeryHigh,
}

impl TrustLevel {
    /// Get trust level from score
    pub fn from_score(score: TrustScore) -> Self {
        match score {
            s if s >= 0.9 => TrustLevel::VeryHigh,
            s if s >= 0.75 => TrustLevel::High,
            s if s >= 0.5 => TrustLevel::Medium,
            s if s >= 0.25 => TrustLevel::Low,
            _ => TrustLevel::VeryLow,
        }
    }
    
    /// Get minimum score for this level
    pub fn min_score(&self) -> TrustScore {
        match self {
            TrustLevel::VeryHigh => 0.9,
            TrustLevel::High => 0.75,
            TrustLevel::Medium => 0.5,
            TrustLevel::Low => 0.25,
            TrustLevel::VeryLow => 0.0,
        }
    }
    
    /// Check if access is allowed at this trust level
    pub fn allows_access(&self) -> bool {
        !matches!(self, TrustLevel::VeryLow)
    }
}

/// Trust factor for scoring
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrustFactor {
    /// Factor name
    pub name: String,
    
    /// Factor weight (0.0 to 1.0)
    pub weight: f64,
    
    /// Factor score (0.0 to 1.0)
    pub score: f64,
    
    /// Factor description
    pub description: String,
}

impl TrustFactor {
    /// Create a new trust factor
    pub fn new(
        name: impl Into<String>,
        weight: f64,
        score: f64,
        description: impl Into<String>,
    ) -> Self {
        Self {
            name: name.into(),
            weight: weight.clamp(0.0, 1.0),
            score: score.clamp(0.0, 1.0),
            description: description.into(),
        }
    }
    
    /// Calculate weighted score
    pub fn weighted_score(&self) -> f64 {
        self.weight * self.score
    }
}

/// Trust score calculator
pub struct TrustCalculator {
    /// Factor weights
    weights: Weights,
}

/// Weights for different trust factors
#[derive(Debug, Clone)]
pub struct Weights {
    /// Identity verification weight (25%)
    pub identity: f64,
    /// Device health weight (20%)
    pub device: f64,
    /// Location weight (15%)
    pub location: f64,
    /// Behavior weight (20%)
    pub behavior: f64,
    /// Time weight (10%)
    pub time: f64,
    /// Risk intelligence weight (10%)
    pub risk: f64,
}

impl Default for Weights {
    fn default() -> Self {
        Self {
            identity: 0.25,
            device: 0.20,
            location: 0.15,
            behavior: 0.20,
            time: 0.10,
            risk: 0.10,
        }
    }
}

impl Default for TrustCalculator {
    fn default() -> Self {
        Self::new()
    }
}

impl TrustCalculator {
    /// Create a new trust calculator with default weights
    pub fn new() -> Self {
        Self {
            weights: Weights::default(),
        }
    }
    
    /// Create a new trust calculator with custom weights
    pub fn with_weights(weights: Weights) -> Self {
        Self { weights }
    }
    
    /// Calculate trust score for a request
    pub fn calculate(
        &self,
        context: &TrustContext,
    ) -> (TrustScore, Vec<TrustFactor>) {
        let mut factors = Vec::new();
        
        // Identity verification factor
        let identity_score = self.calculate_identity_score(context);
        factors.push(TrustFactor::new(
            "Identity Verification",
            self.weights.identity,
            identity_score,
            "Authentication strength and verification status",
        ));
        
        // Device health factor
        let device_score = self.calculate_device_score(context);
        factors.push(TrustFactor::new(
            "Device Health",
            self.weights.device,
            device_score,
            "Device security posture and trust status",
        ));
        
        // Location factor
        let location_score = self.calculate_location_score(context);
        factors.push(TrustFactor::new(
            "Location",
            self.weights.location,
            location_score,
            "Geographic and network location",
        ));
        
        // Behavior factor
        let behavior_score = self.calculate_behavior_score(context);
        factors.push(TrustFactor::new(
            "Behavior",
            self.weights.behavior,
            behavior_score,
            "Access patterns and behavioral anomalies",
        ));
        
        // Time factor
        let time_score = self.calculate_time_score(context);
        factors.push(TrustFactor::new(
            "Time",
            self.weights.time,
            time_score,
            "Access time and duration",
        ));
        
        // Risk intelligence factor
        let risk_score = self.calculate_risk_score(context);
        factors.push(TrustFactor::new(
            "Risk Intelligence",
            self.weights.risk,
            risk_score,
            "Threat intelligence and known compromises",
        ));
        
        // Calculate weighted total
        let total_score: f64 = factors.iter().map(|f| f.weighted_score()).sum();
        
        (total_score, factors)
    }
    
    /// Calculate identity verification score
    fn calculate_identity_score(&self, context: &TrustContext) -> f64 {
        let mut score = 0.5; // Base score
        
        // MFA bonus
        if context.has_mfa {
            score += 0.3;
        }
        
        // Strong authentication bonus
        if context.strong_auth {
            score += 0.1;
        }
        
        // Recent authentication bonus
        if let Ok(elapsed) = Utc::now().signed_duration_since(context.auth_time).num_minutes() {
            if elapsed < 30 {
                score += 0.1;
            } else if elapsed < 60 {
                score += 0.05;
            }
        }
        
        score.clamp(0.0, 1.0)
    }
    
    /// Calculate device health score
    fn calculate_device_score(&self, context: &TrustContext) -> f64 {
        let device = &context.device;
        
        let mut score = 0.0;
        
        // Security posture
        score += device.security_score * 0.6;
        
        // Trusted device bonus
        if device.is_trusted {
            score += 0.3;
        }
        
        // Recent device seen bonus
        if let Ok(elapsed) = Utc::now().signed_duration_since(device.last_seen).num_hours() {
            if elapsed < 24 {
                score += 0.1;
            } else if elapsed < 168 {
                score += 0.05;
            }
        }
        
        score.clamp(0.0, 1.0)
    }
    
    /// Calculate location score
    fn calculate_location_score(&self, context: &TrustContext) -> f64 {
        let network = &context.network;
        
        let mut score = 0.5;
        
        // Trusted network bonus
        if network.is_trusted {
            score += 0.3;
        }
        
        // Encrypted connection bonus
        if network.is_encrypted {
            score += 0.1;
        }
        
        // Corporate network bonus
        if matches!(network.network_type, crate::NetworkType::Corporate) {
            score += 0.1;
        }
        
        // Public network penalty
        if matches!(network.network_type, crate::NetworkType::Public) {
            score -= 0.2;
        }
        
        score.clamp(0.0, 1.0)
    }
    
    /// Calculate behavior score
    fn calculate_behavior_score(&self, context: &TrustContext) -> f64 {
        let mut score = 0.5;
        
        // No anomalies bonus
        if !context.has_anomaly {
            score += 0.3;
        }
        
        // Normal pattern bonus
        if context.normal_pattern {
            score += 0.2;
        }
        
        score.clamp(0.0, 1.0)
    }
    
    /// Calculate time score
    fn calculate_time_score(&self, context: &TrustContext) -> f64 {
        let mut score = 0.5;
        
        // Normal hours bonus
        if context.normal_hours {
            score += 0.3;
        }
        
        // Reasonable duration bonus
        if context.reasonable_duration {
            score += 0.2;
        }
        
        score.clamp(0.0, 1.0)
    }
    
    /// Calculate risk intelligence score
    fn calculate_risk_score(&self, context: &TrustContext) -> f64 {
        let mut score = 0.5;
        
        // No known threats bonus
        if !context.known_threat {
            score += 0.3;
        }
        
        // Clean IP reputation bonus
        if context.clean_reputation {
            score += 0.2;
        }
        
        score.clamp(0.0, 1.0)
    }
}

/// Context for trust score calculation
#[derive(Debug, Clone)]
pub struct TrustContext {
    /// Authentication timestamp
    pub auth_time: DateTime<Utc>,
    
    /// Has MFA been performed
    pub has_mfa: bool,
    
    /// Strong authentication (hardware key, etc.)
    pub strong_auth: bool,
    
    /// Device information
    pub device: DeviceInfo,
    
    /// Network information
    pub network: NetworkInfo,
    
    /// Has behavioral anomaly
    pub has_anomaly: bool,
    
    /// Normal access pattern
    pub normal_pattern: bool,
    
    /// Access during normal hours
    pub normal_hours: bool,
    
    /// Reasonable access duration
    pub reasonable_duration: bool,
    
    /// Known threat indicator
    pub known_threat: bool,
    
    /// Clean IP reputation
    pub clean_reputation: bool,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_trust_level_from_score() {
        assert_eq!(TrustLevel::from_score(0.95), TrustLevel::VeryHigh);
        assert_eq!(TrustLevel::from_score(0.80), TrustLevel::High);
        assert_eq!(TrustLevel::from_score(0.60), TrustLevel::Medium);
        assert_eq!(TrustLevel::from_score(0.40), TrustLevel::Low);
        assert_eq!(TrustLevel::from_score(0.10), TrustLevel::VeryLow);
    }

    #[test]
    fn test_trust_calculator() {
        let calculator = TrustCalculator::new();
        
        let context = TrustContext {
            auth_time: Utc::now(),
            has_mfa: true,
            strong_auth: false,
            device: DeviceInfo {
                device_id: "test-device".to_string(),
                device_type: crate::DeviceType::Laptop,
                os: "Windows".to_string(),
                os_version: "10".to_string(),
                security_score: 0.8,
                is_trusted: true,
                last_seen: Utc::now(),
            },
            network: NetworkInfo {
                ip_address: "192.168.1.1".to_string(),
                location: crate::LocationInfo {
                    country: "US".to_string(),
                    region: Some("CA".to_string()),
                    city: Some("San Francisco".to_string()),
                    latitude: Some(37.7749),
                    longitude: Some(-122.4194),
                },
                network_type: crate::NetworkType::Corporate,
                is_trusted: true,
                is_encrypted: true,
            },
            has_anomaly: false,
            normal_pattern: true,
            normal_hours: true,
            reasonable_duration: true,
            known_threat: false,
            clean_reputation: true,
        };
        
        let (score, factors) = calculator.calculate(&context);
        
        assert!(score > 0.0 && score <= 1.0);
        assert_eq!(factors.len(), 6);
        
        // Verify weights sum to 1.0
        let total_weight: f64 = factors.iter().map(|f| f.weight).sum();
        assert!((total_weight - 1.0).abs() < 0.001);
    }
}