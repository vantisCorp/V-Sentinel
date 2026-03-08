//! Behavioral Biometrics for Continuous Authentication
//! 
//! This module provides continuous authentication through behavioral analysis:
//! - Keystroke dynamics (typing patterns)
//! - Mouse movement patterns
//! - Device fingerprinting
//! - Activity pattern analysis

use serde::{Deserialize, Serialize};
use std::collections::{HashMap, VecDeque};
use std::sync::Arc;
use tokio::sync::RwLock;
use chrono::{DateTime, Utc, Duration};
use std::time::Instant;

/// Behavioral Biometrics Configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BiometricsConfig {
    /// Minimum samples required for baseline creation
    pub min_baseline_samples: usize,
    /// Maximum baseline samples to keep
    pub max_baseline_samples: usize,
    /// Score threshold for anomaly detection (0.0-1.0)
    pub anomaly_threshold: f64,
    /// Score threshold for high risk (0.0-1.0)
    pub high_risk_threshold: f64,
    /// Baseline decay rate per day (0.0-1.0)
    pub baseline_decay_rate: f64,
    /// Session analysis interval in seconds
    pub session_analysis_interval: u64,
    /// Maximum session length in minutes
    pub max_session_length: u64,
}

impl Default for BiometricsConfig {
    fn default() -> Self {
        Self {
            min_baseline_samples: 20,
            max_baseline_samples: 100,
            anomaly_threshold: 0.7,
            high_risk_threshold: 0.5,
            baseline_decay_rate: 0.01,
            session_analysis_interval: 30,
            max_session_length: 480, // 8 hours
        }
    }
}

/// Risk score level
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum RiskLevel {
    Low,
    Medium,
    High,
    Critical,
}

/// Keystroke event
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KeystrokeEvent {
    pub key_code: u32,
    pub timestamp: DateTime<Utc>,
    pub press_duration_ms: u32,
    pub flight_time_ms: u32, // Time since previous keystroke
    pub key_position: Option<(u32, u32)>, // Keyboard position estimation
}

/// Mouse event
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MouseEvent {
    pub event_type: MouseEventType,
    pub timestamp: DateTime<Utc>,
    pub x: f64,
    pub y: f64,
    pub pressure: Option<f64>, // Pressure for touchpads
    pub velocity_x: f64,
    pub velocity_y: f64,
    pub acceleration_x: f64,
    pub acceleration_y: f64,
}

/// Mouse event types
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum MouseEventType {
    Move,
    Click,
    Scroll,
    Drag,
    Hover,
}

/// Device fingerprint data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeviceFingerprint {
    pub user_agent: String,
    pub screen_resolution: String,
    pub screen_color_depth: u16,
    pub timezone_offset: i32,
    pub language: String,
    pub platform: String,
    pub hardware_concurrency: u32,
    pub device_memory: Option<u32>,
    pub touch_support: bool,
    pub canvas_fingerprint: String,
    pub webgl_fingerprint: String,
    pub audio_fingerprint: String,
    pub fonts: Vec<String>,
    pub plugins: Vec<String>,
    pub timezone: String,
    pub battery_level: Option<f32>,
    pub network_type: Option<String>,
}

/// Behavioral baseline
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BehavioralBaseline {
    pub user_id: String,
    pub keystroke_baseline: Option<KeystrokeBaseline>,
    pub mouse_baseline: Option<MouseBaseline>,
    pub device_fingerprint: DeviceFingerprint,
    pub activity_patterns: ActivityPatterns,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub sample_count: usize,
}

/// Keystroke baseline statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KeystrokeBaseline {
    pub mean_press_duration: f64,
    pub std_press_duration: f64,
    pub mean_flight_time: f64,
    pub std_flight_time: f64,
    pub digraph_timing: HashMap<(char, char), DigraphStats>,
    pub typing_speed_wpm: f64,
    pub error_rate: f64,
}

/// Digraph timing statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DigraphStats {
    pub mean_time: f64,
    pub std_time: f64,
    pub sample_count: usize,
}

/// Mouse baseline statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MouseBaseline {
    pub mean_velocity: f64,
    pub std_velocity: f64,
    pub mean_acceleration: f64,
    pub std_acceleration: f64,
    pub mean_click_duration: f64,
    pub std_click_duration: f64,
    pub movement_smoothness: f64,
    pub cursor_path_variance: f64,
}

/// Activity patterns
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActivityPatterns {
    pub active_hours: Vec<(u32, u32)>, // Start and end hours of typical activity
    pub typical_session_duration: Duration,
    pub keyboard_idle_threshold: Duration,
    pub mouse_idle_threshold: Duration,
    pub activity_frequency: HashMap<String, f64>,
}

/// Biometric analysis result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BiometricAnalysisResult {
    pub user_id: String,
    pub overall_score: f64, // 0.0 (no match) to 1.0 (perfect match)
    pub risk_level: RiskLevel,
    pub keystroke_score: Option<f64>,
    pub mouse_score: Option<f64>,
    pub device_match_score: Option<f64>,
    pub activity_score: Option<f64>,
    pub anomalies: Vec<String>,
    pub recommendations: Vec<String>,
    pub timestamp: DateTime<Utc>,
}

/// Session biometric data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionBiometrics {
    pub session_id: String,
    pub user_id: String,
    pub keystrokes: Vec<KeystrokeEvent>,
    pub mouse_events: VecDeque<MouseEvent>,
    pub start_time: DateTime<Utc>,
    pub last_activity: DateTime<Utc>,
    pub risk_scores: VecDeque<BiometricAnalysisResult>,
}

/// Behavioral Biometrics Manager
pub struct BiometricsManager {
    config: BiometricsConfig,
    baselines: Arc<RwLock<HashMap<String, BehavioralBaseline>>>,
    sessions: Arc<RwLock<HashMap<String, SessionBiometrics>>>,
    device_history: Arc<RwLock<HashMap<String, Vec<DeviceFingerprint>>>>,
}

impl BiometricsManager {
    /// Create a new Biometrics Manager
    pub fn new(config: BiometricsConfig) -> Self {
        Self {
            config,
            baselines: Arc::new(RwLock::new(HashMap::new())),
            sessions: Arc::new(RwLock::new(HashMap::new())),
            device_history: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    // ==================== Baseline Management ====================

    /// Create or update behavioral baseline
    pub async fn update_baseline(
        &self,
        user_id: &str,
        device_fingerprint: DeviceFingerprint,
        keystrokes: &[KeystrokeEvent],
        mouse_events: &[MouseEvent],
    ) -> Result<(), BiometricsError> {
        let mut baselines = self.baselines.write().await;
        
        let baseline = baselines.entry(user_id.to_string()).or_insert_with(|| {
            BehavioralBaseline {
                user_id: user_id.to_string(),
                keystroke_baseline: None,
                mouse_baseline: None,
                device_fingerprint: device_fingerprint.clone(),
                activity_patterns: ActivityPatterns::default(),
                created_at: Utc::now(),
                updated_at: Utc::now(),
                sample_count: 0,
            }
        });

        // Update device fingerprint
        baseline.device_fingerprint = device_fingerprint;

        // Update keystroke baseline if enough samples
        if keystrokes.len() >= self.config.min_baseline_samples {
            let keystroke_baseline = self.calculate_keystroke_baseline(keystrokes);
            
            // Merge with existing baseline if present
            baseline.keystroke_baseline = Some(match &baseline.keystroke_baseline {
                Some(existing) => self.merge_keystroke_baselines(existing, &keystroke_baseline),
                None => keystroke_baseline,
            });
        }

        // Update mouse baseline if enough samples
        if mouse_events.len() >= self.config.min_baseline_samples {
            let mouse_baseline = self.calculate_mouse_baseline(mouse_events);
            
            baseline.mouse_baseline = Some(match &baseline.mouse_baseline {
                Some(existing) => self.merge_mouse_baselines(existing, &mouse_baseline),
                None => mouse_baseline,
            });
        }

        baseline.updated_at = Utc::now();
        baseline.sample_count += 1;

        // Decay baseline if too many samples
        if baseline.sample_count > self.config.max_baseline_samples {
            baseline.sample_count = self.config.max_baseline_samples;
        }

        Ok(())
    }

    /// Get user baseline
    pub async fn get_baseline(&self, user_id: &str) -> Option<BehavioralBaseline> {
        self.baselines.read().await.get(user_id).cloned()
    }

    /// Calculate keystroke baseline from events
    fn calculate_keystroke_baseline(&self, keystrokes: &[KeystrokeEvent]) -> KeystrokeBaseline {
        let press_durations: Vec<f64> = keystrokes.iter()
            .map(|k| k.press_duration_ms as f64)
            .collect();
        
        let flight_times: Vec<f64> = keystrokes.iter()
            .map(|k| k.flight_time_ms as f64)
            .collect();

        let mean_press = mean(&press_durations);
        let std_press = std_dev(&press_durations, mean_press);
        let mean_flight = mean(&flight_times);
        let std_flight = std_dev(&flight_times, mean_flight);

        // Calculate digraph timings
        let mut digraph_timing: HashMap<(char, char), Vec<f64>> = HashMap::new();
        for window in keystrokes.windows(2) {
            let c1 = std::char::from_u32(window[0].key_code).unwrap_or(' ');
            let c2 = std::char::from_u32(window[1].key_code).unwrap_or(' ');
            digraph_timing
                .entry((c1, c2))
                .or_insert_with(Vec::new)
                .push(window[1].flight_time_ms as f64);
        }

        let digraph_stats: HashMap<(char, char), DigraphStats> = digraph_timing
            .into_iter()
            .map(|(digraph, times)| {
                let mean_time = mean(&times);
                let std_time = std_dev(&times, mean_time);
                (digraph, DigraphStats {
                    mean_time,
                    std_time,
                    sample_count: times.len(),
                })
            })
            .collect();

        // Calculate typing speed (words per minute)
        let total_chars = keystrokes.len();
        let time_span_minutes = (keystrokes.last().unwrap().timestamp - keystrokes.first().unwrap().timestamp).num_seconds() as f64 / 60.0;
        let typing_speed = if time_span_minutes > 0.0 {
            (total_chars as f64 / 5.0) / time_span_minutes
        } else {
            0.0
        };

        KeystrokeBaseline {
            mean_press_duration: mean_press,
            std_press_duration: std_press,
            mean_flight_time: mean_flight,
            std_flight_time: std_flight,
            digraph_timing: digraph_stats,
            typing_speed_wpm: typing_speed,
            error_rate: 0.05, // Placeholder - would need actual error tracking
        }
    }

    /// Calculate mouse baseline from events
    fn calculate_mouse_baseline(&self, mouse_events: &[MouseEvent]) -> MouseBaseline {
        let velocities: Vec<f64> = mouse_events.iter()
            .map(|e| (e.velocity_x.hypot(e.velocity_y)).abs())
            .collect();
        
        let accelerations: Vec<f64> = mouse_events.iter()
            .map(|e| (e.acceleration_x.hypot(e.acceleration_y)).abs())
            .collect();

        let click_durations: Vec<f64> = mouse_events.iter()
            .filter(|e| e.event_type == MouseEventType::Click)
            .map(|e| 100.0) // Placeholder - would need actual click duration
            .collect();

        let mean_velocity = mean(&velocities);
        let std_velocity = std_dev(&velocities, mean_velocity);
        let mean_acceleration = mean(&accelerations);
        let std_acceleration = std_dev(&accelerations, mean_acceleration);

        // Calculate movement smoothness (lower is smoother)
        let mut total_jerk = 0.0f64;
        for window in mouse_events.windows(3) {
            let v1 = (window[0].velocity_x, window[0].velocity_y);
            let v2 = (window[1].velocity_x, window[1].velocity_y);
            let v3 = (window[2].velocity_x, window[2].velocity_y);
            
            let jerk1 = ((v2.0 - v1.0).powi(2) + (v2.1 - v1.1).powi(2)).sqrt();
            let jerk2 = ((v3.0 - v2.0).powi(2) + (v3.1 - v2.1).powi(2)).sqrt();
            total_jerk += jerk1.abs_diff(jerk2);
        }
        let smoothness = if mouse_events.len() > 2 {
            total_jerk / (mouse_events.len() - 2) as f64
        } else {
            0.0
        };

        MouseBaseline {
            mean_velocity,
            std_velocity,
            mean_acceleration,
            std_acceleration,
            mean_click_duration: mean(&click_durations),
            std_click_duration: std_dev(&click_durations, mean(&click_durations)),
            movement_smoothness: smoothness,
            cursor_path_variance: 0.0, // Would need path calculation
        }
    }

    /// Merge keystroke baselines (weighted average)
    fn merge_keystroke_baselines(
        &self,
        existing: &KeystrokeBaseline,
        new: &KeystrokeBaseline,
    ) -> KeystrokeBaseline {
        let weight = (1.0 - self.config.baseline_decay_rate).clamp(0.0, 1.0);
        
        KeystrokeBaseline {
            mean_press_duration: existing.mean_press_duration * weight + new.mean_press_duration * (1.0 - weight),
            std_press_duration: existing.std_press_duration * weight + new.std_press_duration * (1.0 - weight),
            mean_flight_time: existing.mean_flight_time * weight + new.mean_flight_time * (1.0 - weight),
            std_flight_time: existing.std_flight_time * weight + new.std_flight_time * (1.0 - weight),
            digraph_timing: new.digraph_timing.clone(), // Simplified - would merge properly
            typing_speed_wpm: existing.typing_speed_wpm * weight + new.typing_speed_wpm * (1.0 - weight),
            error_rate: existing.error_rate * weight + new.error_rate * (1.0 - weight),
        }
    }

    /// Merge mouse baselines
    fn merge_mouse_baselines(&self, existing: &MouseBaseline, new: &MouseBaseline) -> MouseBaseline {
        let weight = (1.0 - self.config.baseline_decay_rate).clamp(0.0, 1.0);
        
        MouseBaseline {
            mean_velocity: existing.mean_velocity * weight + new.mean_velocity * (1.0 - weight),
            std_velocity: existing.std_velocity * weight + new.std_velocity * (1.0 - weight),
            mean_acceleration: existing.mean_acceleration * weight + new.mean_acceleration * (1.0 - weight),
            std_acceleration: existing.std_acceleration * weight + new.std_acceleration * (1.0 - weight),
            mean_click_duration: existing.mean_click_duration * weight + new.mean_click_duration * (1.0 - weight),
            std_click_duration: existing.std_click_duration * weight + new.std_click_duration * (1.0 - weight),
            movement_smoothness: existing.movement_smoothness * weight + new.movement_smoothness * (1.0 - weight),
            cursor_path_variance: 0.0,
        }
    }

    // ==================== Analysis ====================

    /// Analyze biometric data against baseline
    pub async fn analyze_biometrics(
        &self,
        user_id: &str,
        device_fingerprint: Option<&DeviceFingerprint>,
        keystrokes: &[KeystrokeEvent],
        mouse_events: &[MouseEvent],
    ) -> Result<BiometricAnalysisResult, BiometricsError> {
        let baseline = self.baselines.read().await;
        let baseline = baseline.get(user_id)
            .ok_or_else(|| BiometricsError::NoBaseline("No baseline found for user".to_string()))?;

        let mut scores = Vec::new();
        let mut anomalies = Vec::new();
        let mut recommendations = Vec::new();

        // Analyze keystrokes
        let keystroke_score = if let Some(ks_baseline) = &baseline.keystroke_baseline {
            if !keystrokes.is_empty() {
                let score = self.analyze_keystrokes(keystrokes, ks_baseline);
                scores.push(score);
                if score < self.config.high_risk_threshold {
                    anomalies.push("Unusual typing pattern detected".to_string());
                    recommendations.push("Consider re-authentication".to_string());
                }
                Some(score)
            } else {
                None
            }
        } else {
            None
        };

        // Analyze mouse movements
        let mouse_score = if let Some(ms_baseline) = &baseline.mouse_baseline {
            if !mouse_events.is_empty() {
                let score = self.analyze_mouse(mouse_events, ms_baseline);
                scores.push(score);
                if score < self.config.high_risk_threshold {
                    anomalies.push("Unusual mouse movement detected".to_string());
                }
                Some(score)
            } else {
                None
            }
        } else {
            None
        };

        // Analyze device fingerprint
        let device_score = if let Some(device_fp) = device_fingerprint {
            let score = self.compare_device_fingerprint(&baseline.device_fingerprint, device_fp);
            scores.push(score);
            if score < self.config.high_risk_threshold {
                anomalies.push("Device fingerprint mismatch".to_string());
                recommendations.push("New device detected - additional verification required".to_string());
            }
            Some(score)
        } else {
            None
        };

        // Calculate overall score
        let overall_score = if scores.is_empty() {
            0.5 // Neutral if no data
        } else {
            mean(&scores)
        };

        // Determine risk level
        let risk_level = if overall_score >= self.config.anomaly_threshold {
            RiskLevel::Low
        } else if overall_score >= self.config.high_risk_threshold {
            RiskLevel::Medium
        } else {
            RiskLevel::High
        };

        Ok(BiometricAnalysisResult {
            user_id: user_id.to_string(),
            overall_score,
            risk_level,
            keystroke_score,
            mouse_score,
            device_match_score: device_score,
            activity_score: None,
            anomalies,
            recommendations,
            timestamp: Utc::now(),
        })
    }

    /// Analyze keystrokes against baseline
    fn analyze_keystrokes(&self, keystrokes: &[KeystrokeEvent], baseline: &KeystrokeBaseline) -> f64 {
        let press_durations: Vec<f64> = keystrokes.iter()
            .map(|k| k.press_duration_ms as f64)
            .collect();
        
        let flight_times: Vec<f64> = keystrokes.iter()
            .map(|k| k.flight_time_ms as f64)
            .collect();

        let mean_press = mean(&press_durations);
        let mean_flight = mean(&flight_times);

        // Calculate Z-scores
        let press_z_score = (mean_press - baseline.mean_press_duration).abs() / baseline.std_press_duration.max(1.0);
        let flight_z_score = (mean_flight - baseline.mean_flight_time).abs() / baseline.std_flight_time.max(1.0);

        // Convert Z-scores to similarity score (0-1)
        let press_score = 1.0 / (1.0 + press_z_score);
        let flight_score = 1.0 / (1.0 + flight_z_score);

        (press_score + flight_score) / 2.0
    }

    /// Analyze mouse movements against baseline
    fn analyze_mouse(&self, mouse_events: &[MouseEvent], baseline: &MouseBaseline) -> f64 {
        let velocities: Vec<f64> = mouse_events.iter()
            .map(|e| (e.velocity_x.hypot(e.velocity_y)).abs())
            .collect();
        
        let accelerations: Vec<f64> = mouse_events.iter()
            .map(|e| (e.acceleration_x.hypot(e.acceleration_y)).abs())
            .collect();

        let mean_velocity = mean(&velocities);
        let mean_acceleration = mean(&accelerations);

        let velocity_z_score = (mean_velocity - baseline.mean_velocity).abs() / baseline.std_velocity.max(1.0);
        let acceleration_z_score = (mean_acceleration - baseline.mean_acceleration).abs() / baseline.std_acceleration.max(1.0);

        let velocity_score = 1.0 / (1.0 + velocity_z_score);
        let acceleration_score = 1.0 / (1.0 + acceleration_z_score);

        (velocity_score + acceleration_score) / 2.0
    }

    /// Compare device fingerprints
    fn compare_device_fingerprint(&self, baseline: &DeviceFingerprint, current: &DeviceFingerprint) -> f64 {
        let mut matches = 0;
        let mut total = 0;

        // Compare critical attributes
        if baseline.screen_resolution == current.screen_resolution { matches += 1; }
        total += 1;

        if baseline.screen_color_depth == current.screen_color_depth { matches += 1; }
        total += 1;

        if baseline.timezone_offset == current.timezone_offset { matches += 1; }
        total += 1;

        if baseline.language == current.language { matches += 1; }
        total += 1;

        if baseline.platform == current.platform { matches += 1; }
        total += 1;

        if baseline.hardware_concurrency == current.hardware_concurrency { matches += 1; }
        total += 1;

        if baseline.touch_support == current.touch_support { matches += 1; }
        total += 1;

        // Compare fingerprints (simplified - would use proper comparison)
        if baseline.canvas_fingerprint == current.canvas_fingerprint { matches += 1; }
        total += 1;

        // Calculate similarity
        if total == 0 {
            0.5
        } else {
            matches as f64 / total as f64
        }
    }

    // ==================== Session Management ====================

    /// Start a new biometric session
    pub async fn start_session(&self, user_id: &str, session_id: &str) -> Result<(), BiometricsError> {
        let session = SessionBiometrics {
            session_id: session_id.to_string(),
            user_id: user_id.to_string(),
            keystrokes: Vec::new(),
            mouse_events: VecDeque::with_capacity(1000),
            start_time: Utc::now(),
            last_activity: Utc::now(),
            risk_scores: VecDeque::with_capacity(100),
        };

        self.sessions.write().await.insert(session_id.to_string(), session);
        Ok(())
    }

    /// Record a keystroke event
    pub async fn record_keystroke(&self, session_id: &str, event: KeystrokeEvent) -> Result<(), BiometricsError> {
        let mut sessions = self.sessions.write().await;
        if let Some(session) = sessions.get_mut(session_id) {
            session.keystrokes.push(event);
            session.last_activity = Utc::now();
            Ok(())
        } else {
            Err(BiometricsError::SessionNotFound("Session not found".to_string()))
        }
    }

    /// Record a mouse event
    pub async fn record_mouse_event(&self, session_id: &str, event: MouseEvent) -> Result<(), BiometricsError> {
        let mut sessions = self.sessions.write().await;
        if let Some(session) = sessions.get_mut(session_id) {
            session.mouse_events.push_back(event);
            session.last_activity = Utc::now();
            
            // Keep only recent events (last 1000)
            if session.mouse_events.len() > 1000 {
                session.mouse_events.pop_front();
            }
            Ok(())
        } else {
            Err(BiometricsError::SessionNotFound("Session not found".to_string()))
        }
    }

    /// End a session and analyze
    pub async fn end_session(&self, session_id: &str) -> Result<BiometricAnalysisResult, BiometricsError> {
        let sessions = self.sessions.read().await;
        let session = sessions.get(session_id)
            .ok_or_else(|| BiometricsError::SessionNotFound("Session not found".to_string()))?;

        // Get baseline
        let baseline_opt = self.baselines.read().await.get(&session.user_id).cloned();

        if let Some(baseline) = baseline_opt {
            drop(sessions);
            
            let result = self.analyze_biometrics(
                &session.user_id,
                Some(&baseline.device_fingerprint),
                &session.keystrokes,
                &session.mouse_events.iter().cloned().collect::<Vec<_>>(),
            ).await?;

            // Remove session
            self.sessions.write().await.remove(session_id);
            
            Ok(result)
        } else {
            Err(BiometricsError::NoBaseline("No baseline found".to_string()))
        }
    }

    /// Get session risk history
    pub async fn get_session_risk_history(&self, session_id: &str) -> Vec<BiometricAnalysisResult> {
        let sessions = self.sessions.read().await;
        if let Some(session) = sessions.get(session_id) {
            session.risk_scores.iter().cloned().collect()
        } else {
            Vec::new()
        }
    }
}

/// Biometrics Error types
#[derive(Debug, thiserror::Error)]
pub enum BiometricsError {
    #[error("No baseline: {0}")]
    NoBaseline(String),
    #[error("Session not found: {0}")]
    SessionNotFound(String),
    #[error("Invalid data: {0}")]
    InvalidData(String),
    #[error("Internal error: {0}")]
    Internal(String),
}

/// Calculate mean of a slice
fn mean(values: &[f64]) -> f64 {
    if values.is_empty() {
        return 0.0;
    }
    values.iter().sum::<f64>() / values.len() as f64
}

/// Calculate standard deviation
fn std_dev(values: &[f64], mean: f64) -> f64 {
    if values.is_empty() {
        return 0.0;
    }
    let variance = values.iter()
        .map(|&x| (x - mean).powi(2))
        .sum::<f64>() / values.len() as f64;
    variance.sqrt()
}

impl Default for ActivityPatterns {
    fn default() -> Self {
        Self {
            active_hours: vec![(9, 17)], // 9 AM - 5 PM default
            typical_session_duration: Duration::hours(8),
            keyboard_idle_threshold: Duration::seconds(300),
            mouse_idle_threshold: Duration::seconds(180),
            activity_frequency: HashMap::new(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_keystroke_baseline_calculation() {
        let manager = BiometricsManager::new(BiometricsConfig::default());
        
        let keystrokes = vec![
            KeystrokeEvent {
                key_code: 65,
                timestamp: Utc::now(),
                press_duration_ms: 100,
                flight_time_ms: 50,
                key_position: None,
            },
            KeystrokeEvent {
                key_code: 66,
                timestamp: Utc::now() + Duration::milliseconds(150),
                press_duration_ms: 95,
                flight_time_ms: 150,
                key_position: None,
            },
        ];

        let baseline = manager.calculate_keystroke_baseline(&keystrokes);
        assert!(baseline.mean_press_duration > 0.0);
        assert!(baseline.mean_flight_time > 0.0);
    }

    #[test]
    fn test_mouse_baseline_calculation() {
        let manager = BiometricsManager::new(BiometricsConfig::default());
        
        let mouse_events = vec![
            MouseEvent {
                event_type: MouseEventType::Move,
                timestamp: Utc::now(),
                x: 100.0,
                y: 100.0,
                pressure: None,
                velocity_x: 5.0,
                velocity_y: 3.0,
                acceleration_x: 0.5,
                acceleration_y: 0.3,
            },
        ];

        let baseline = manager.calculate_mouse_baseline(&mouse_events);
        assert!(baseline.mean_velocity > 0.0);
    }

    #[test]
    fn test_mean_and_std_dev() {
        let values = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        let mean_val = mean(&values);
        let std = std_dev(&values, mean_val);
        
        assert!((mean_val - 3.0).abs() < 0.001);
        assert!((std - 1.414).abs() < 0.01);
    }

    #[tokio::test]
    async fn test_session_management() {
        let manager = BiometricsManager::new(BiometricsConfig::default());
        
        // Start session
        manager.start_session("user1", "session1").await.unwrap();
        
        // Record events
        let keystroke = KeystrokeEvent {
            key_code: 65,
            timestamp: Utc::now(),
            press_duration_ms: 100,
            flight_time_ms: 50,
            key_position: None,
        };
        manager.record_keystroke("session1", keystroke).await.unwrap();
        
        let mouse_event = MouseEvent {
            event_type: MouseEventType::Move,
            timestamp: Utc::now(),
            x: 100.0,
            y: 100.0,
            pressure: None,
            velocity_x: 5.0,
            velocity_y: 3.0,
            acceleration_x: 0.5,
            acceleration_y: 0.3,
        };
        manager.record_mouse_event("session1", mouse_event).await.unwrap();
        
        // Session should exist
        let sessions = manager.sessions.read().await;
        assert!(sessions.contains_key("session1"));
    }

    #[test]
    fn test_device_fingerprint_comparison() {
        let manager = BiometricsManager::new(BiometricsConfig::default());
        
        let baseline = DeviceFingerprint {
            user_agent: "Mozilla/5.0".to_string(),
            screen_resolution: "1920x1080".to_string(),
            screen_color_depth: 24,
            timezone_offset: 3600,
            language: "en-US".to_string(),
            platform: "Win32".to_string(),
            hardware_concurrency: 8,
            device_memory: Some(16),
            touch_support: false,
            canvas_fingerprint: "abc123".to_string(),
            webgl_fingerprint: "def456".to_string(),
            audio_fingerprint: "ghi789".to_string(),
            fonts: vec!["Arial".to_string()],
            plugins: vec![],
            timezone: "Europe/London".to_string(),
            battery_level: None,
            network_type: None,
        };
        
        let current = baseline.clone();
        
        let score = manager.compare_device_fingerprint(&baseline, &current);
        assert_eq!(score, 1.0);
    }
}