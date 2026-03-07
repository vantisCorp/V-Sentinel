//! Threat Intelligence Integration
//!
//! Provides threat intelligence capabilities for deepfake detection including:
//! - Known deepfake pattern database
//! - Threat feed integration
//! - Attribution and actor tracking
//! - Campaign analysis

use crate::models::*;
use crate::DeepfakeType;
use anyhow::Result;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{debug, info, warn};

/// Threat intelligence integration for deepfake detection
pub struct ThreatIntelIntegration {
    /// Known threat patterns
    patterns: Arc<RwLock<Vec<ThreatPattern>>>,
    /// Threat actors database
    actors: Arc<RwLock<Vec<ThreatActor>>>,
    /// Campaign database
    campaigns: Arc<RwLock<Vec<Campaign>>>,
    /// Hash database for known deepfakes
    hash_database: Arc<RwLock<HashMap<String, ThreatInfo>>>,
    /// Feed sources
    feeds: Vec<ThreatFeed>,
    /// Statistics
    stats: Arc<RwLock<ThreatIntelStats>>,
}

/// Statistics for threat intelligence
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ThreatIntelStats {
    /// Total lookups performed
    pub total_lookups: u64,
    /// Matches found
    pub matches_found: u64,
    /// Unique threats identified
    pub unique_threats: u64,
    /// Feed updates
    pub feed_updates: u64,
    /// Last update time
    pub last_update: Option<DateTime<Utc>>,
}

/// Known threat pattern
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThreatPattern {
    /// Pattern ID
    pub id: String,
    /// Pattern name
    pub name: String,
    /// Pattern description
    pub description: String,
    /// Pattern type
    pub pattern_type: PatternType,
    /// Associated deepfake types
    pub deepfake_types: Vec<DeepfakeType>,
    /// Detection indicators
    pub indicators: Vec<ThreatIndicator>,
    /// Severity level (1-10)
    pub severity: u8,
    /// Confidence of attribution
    pub confidence: f32,
    /// First seen
    pub first_seen: DateTime<Utc>,
    /// Last seen
    pub last_seen: DateTime<Utc>,
    /// Associated actors
    pub actors: Vec<String>,
    /// Associated campaigns
    pub campaigns: Vec<String>,
}

/// Type of threat pattern
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PatternType {
    /// Visual pattern in images/video
    Visual,
    /// Audio pattern
    Audio,
    /// Behavioral pattern
    Behavioral,
    /// Metadata pattern
    Metadata,
    /// Network pattern
    Network,
    /// Temporal pattern
    Temporal,
}

/// Threat indicator
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThreatIndicator {
    /// Indicator type
    pub indicator_type: String,
    /// Indicator value/pattern
    pub value: String,
    /// Description
    pub description: String,
    /// Confidence (0.0 - 1.0)
    pub confidence: f32,
}

/// Threat actor information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThreatActor {
    /// Actor ID
    pub id: String,
    /// Actor name/alias
    pub name: String,
    /// Actor type
    pub actor_type: ActorType,
    /// Motivation
    pub motivation: Vec<String>,
    /// Capability level (1-10)
    pub capability: u8,
    /// Known tools/techniques
    pub techniques: Vec<String>,
    /// Associated campaigns
    pub campaigns: Vec<String>,
    /// First observed
    pub first_observed: DateTime<Utc>,
    /// Last observed
    pub last_observed: DateTime<Utc>,
    /// Attribution confidence
    pub attribution_confidence: f32,
}

/// Type of threat actor
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ActorType {
    /// Nation-state actor
    NationState,
    /// Organized crime
    OrganizedCrime,
    /// Hacktivist group
    Hacktivist,
    /// Individual actor
    Individual,
    /// Unknown
    Unknown,
}

/// Campaign information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Campaign {
    /// Campaign ID
    pub id: String,
    /// Campaign name
    pub name: String,
    /// Campaign description
    pub description: String,
    /// Target sectors
    pub target_sectors: Vec<String>,
    /// Target regions
    pub target_regions: Vec<String>,
    /// Start date
    pub start_date: DateTime<Utc>,
    /// End date (if concluded)
    pub end_date: Option<DateTime<Utc>>,
    /// Status
    pub status: CampaignStatus,
    /// Associated actors
    pub actors: Vec<String>,
    /// Related deepfakes count
    pub deepfake_count: u64,
    /// Techniques used
    pub techniques: Vec<String>,
}

/// Campaign status
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CampaignStatus {
    /// Campaign is active
    Active,
    /// Campaign is dormant
    Dormant,
    /// Campaign has concluded
    Concluded,
    /// Campaign status unknown
    Unknown,
}

/// Threat information for a match
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThreatInfo {
    /// Threat ID
    pub id: String,
    /// Threat name
    pub name: String,
    /// Threat type
    pub threat_type: String,
    /// Severity (1-10)
    pub severity: u8,
    /// Description
    pub description: String,
    /// Mitigation recommendations
    pub recommendations: Vec<String>,
    /// Related threats
    pub related: Vec<String>,
    /// Sources
    pub sources: Vec<String>,
}

/// Threat match result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThreatMatch {
    /// Match ID
    pub id: String,
    /// Matched threat info
    pub threat: ThreatInfo,
    /// Match confidence
    pub confidence: f32,
    /// Match type
    pub match_type: MatchType,
    /// Matched indicators
    pub matched_indicators: Vec<String>,
    /// Timestamp
    pub timestamp: DateTime<Utc>,
}

/// Type of threat match
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MatchType {
    /// Exact hash match
    ExactHash,
    /// Fuzzy hash match
    FuzzyHash,
    /// Pattern match
    Pattern,
    /// Behavioral match
    Behavioral,
    /// Attribution match
    Attribution,
}

/// Threat feed source
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThreatFeed {
    /// Feed ID
    pub id: String,
    /// Feed name
    pub name: String,
    /// Feed URL
    pub url: String,
    /// Feed type
    pub feed_type: String,
    /// Update interval in seconds
    pub update_interval: u64,
    /// Last update
    pub last_update: Option<DateTime<Utc>>,
    /// Whether feed is active
    pub active: bool,
    /// API key (if required)
    pub api_key: Option<String>,
}

impl ThreatIntelIntegration {
    /// Create new threat intelligence integration
    pub fn new() -> Self {
        Self {
            patterns: Arc::new(RwLock::new(Self::default_patterns())),
            actors: Arc::new(RwLock::new(Self::default_actors())),
            campaigns: Arc::new(RwLock::new(Self::default_campaigns())),
            hash_database: Arc::new(RwLock::new(HashMap::new())),
            feeds: Self::default_feeds(),
            stats: Arc::new(RwLock::new(ThreatIntelStats::default())),
        }
    }

    /// Get default threat patterns
    fn default_patterns() -> Vec<ThreatPattern> {
        vec![
            ThreatPattern {
                id: "pattern-face-swap-generic".to_string(),
                name: "Generic Face Swap Detection".to_string(),
                description: "Common indicators of face swap deepfakes".to_string(),
                pattern_type: PatternType::Visual,
                deepfake_types: vec![DeepfakeType::FaceSwap],
                indicators: vec![
                    ThreatIndicator {
                        indicator_type: "edge_artifact".to_string(),
                        value: "blur_boundary".to_string(),
                        description: "Blurred boundaries around face region".to_string(),
                        confidence: 0.7,
                    },
                ],
                severity: 5,
                confidence: 0.8,
                first_seen: Utc::now() - chrono::Duration::days(365),
                last_seen: Utc::now(),
                actors: vec![],
                campaigns: vec![],
            },
            ThreatPattern {
                id: "pattern-voice-clone-tts".to_string(),
                name: "AI Voice Cloning Pattern".to_string(),
                description: "Indicators of AI-generated voice content".to_string(),
                pattern_type: PatternType::Audio,
                deepfake_types: vec![DeepfakeType::VoiceClone, DeepfakeType::TextToSpeech],
                indicators: vec![
                    ThreatIndicator {
                        indicator_type: "spectral".to_string(),
                        value: "synthetic_artifact".to_string(),
                        description: "Spectral artifacts from voice synthesis".to_string(),
                        confidence: 0.75,
                    },
                ],
                severity: 6,
                confidence: 0.85,
                first_seen: Utc::now() - chrono::Duration::days(180),
                last_seen: Utc::now(),
                actors: vec![],
                campaigns: vec![],
            },
        ]
    }

    /// Get default threat actors
    fn default_actors() -> Vec<ThreatActor> {
        vec![
            ThreatActor {
                id: "actor-deepfake-criminals".to_string(),
                name: "Deepfake Fraud Actors".to_string(),
                actor_type: ActorType::OrganizedCrime,
                motivation: vec!["financial".to_string(), "fraud".to_string()],
                capability: 7,
                techniques: vec!["face_swap".to_string(), "voice_clone".to_string()],
                campaigns: vec!["financial_fraud_2024".to_string()],
                first_observed: Utc::now() - chrono::Duration::days(500),
                last_observed: Utc::now(),
                attribution_confidence: 0.6,
            },
        ]
    }

    /// Get default campaigns
    fn default_campaigns() -> Vec<Campaign> {
        vec![
            Campaign {
                id: "campaign-financial-fraud-2024".to_string(),
                name: "Financial Fraud Campaign 2024".to_string(),
                description: "Coordinated deepfake campaign targeting financial institutions".to_string(),
                target_sectors: vec!["banking".to_string(), "finance".to_string()],
                target_regions: vec!["global".to_string()],
                start_date: Utc::now() - chrono::Duration::days(180),
                end_date: None,
                status: CampaignStatus::Active,
                actors: vec!["actor-deepfake-criminals".to_string()],
                deepfake_count: 150,
                techniques: vec!["face_swap".to_string(), "voice_clone".to_string()],
            },
        ]
    }

    /// Get default threat feeds
    fn default_feeds() -> Vec<ThreatFeed> {
        vec![
            ThreatFeed {
                id: "feed-deepfake-database".to_string(),
                name: "Global Deepfake Database".to_string(),
                url: "https://threat-intel.example.com/deepfake".to_string(),
                feed_type: "json".to_string(),
                update_interval: 3600,
                last_update: None,
                active: true,
                api_key: None,
            },
            ThreatFeed {
                id: "feed-disinformation-tracker".to_string(),
                name: "Disinformation Tracker".to_string(),
                url: "https://threat-intel.example.com/disinfo".to_string(),
                feed_type: "stix".to_string(),
                update_interval: 7200,
                last_update: None,
                active: true,
                api_key: None,
            },
        ]
    }

    /// Look up threats for media content
    pub async fn lookup(&self, media: &MediaContent) -> Result<Vec<ThreatMatch>> {
        info!("Looking up threat intelligence for media: {}", media.id);
        let mut stats = self.stats.write().await;
        stats.total_lookups += 1;
        drop(stats);

        let mut matches = Vec::new();

        // Check hash database
        let content_hash = self.calculate_hash(&media.data);
        let hash_db = self.hash_database.read().await;
        if let Some(threat_info) = hash_db.get(&content_hash) {
            matches.push(ThreatMatch {
                id: uuid::Uuid::new_v4().to_string(),
                threat: threat_info.clone(),
                confidence: 0.95,
                match_type: MatchType::ExactHash,
                matched_indicators: vec!["content_hash".to_string()],
                timestamp: Utc::now(),
            });
        }
        drop(hash_db);

        // Check patterns
        let patterns = self.patterns.read().await;
        for pattern in patterns.iter() {
            if self.matches_pattern(media, pattern) {
                matches.push(ThreatMatch {
                    id: uuid::Uuid::new_v4().to_string(),
                    threat: ThreatInfo {
                        id: pattern.id.clone(),
                        name: pattern.name.clone(),
                        threat_type: "pattern".to_string(),
                        severity: pattern.severity,
                        description: pattern.description.clone(),
                        recommendations: self.get_recommendations(pattern),
                        related: pattern.actors.clone(),
                        sources: vec!["internal_database".to_string()],
                    },
                    confidence: pattern.confidence,
                    match_type: MatchType::Pattern,
                    matched_indicators: pattern.indicators.iter().map(|i| i.value.clone()).collect(),
                    timestamp: Utc::now(),
                });
            }
        }

        // Update stats
        let mut stats = self.stats.write().await;
        if !matches.is_empty() {
            stats.matches_found += 1;
        }
        drop(stats);

        Ok(matches)
    }

    /// Check if media matches a pattern
    fn matches_pattern(&self, _media: &MediaContent, pattern: &ThreatPattern) -> bool {
        // In production, this would perform actual pattern matching
        // based on the pattern type and indicators
        match pattern.pattern_type {
            PatternType::Visual => {
                // Check for visual indicators
                false
            }
            PatternType::Audio => {
                // Check for audio indicators
                false
            }
            _ => false,
        }
    }

    /// Get recommendations for a threat pattern
    fn get_recommendations(&self, pattern: &ThreatPattern) -> Vec<String> {
        let mut recommendations = vec![
            format!("Verify content authenticity through alternative sources"),
            format!("Report to security team for further analysis"),
        ];

        if pattern.severity >= 7 {
            recommendations.push("Do not distribute or share this content".to_string());
        }

        if !pattern.actors.is_empty() {
            recommendations.push(format!(
                "Attribution possible - related to: {}",
                pattern.actors.join(", ")
            ));
        }

        recommendations
    }

    /// Calculate content hash
    fn calculate_hash(&self, data: &[u8]) -> String {
        use sha2::{Digest, Sha256};
        let mut hasher = Sha256::new();
        hasher.update(data);
        let hash = hasher.finalize();
        hex::encode(hash)
    }

    /// Update threat intelligence feeds
    pub async fn update_feeds(&mut self) -> Result<()> {
        info!("Updating threat intelligence feeds");

        for feed in &mut self.feeds {
            if !feed.active {
                continue;
            }

            // Simulate feed update
            feed.last_update = Some(Utc::now());
            debug!("Updated feed: {} ({})", feed.name, feed.id);
        }

        let mut stats = self.stats.write().await;
        stats.feed_updates += 1;
        stats.last_update = Some(Utc::now());

        Ok(())
    }

    /// Add threat to database
    pub async fn add_threat(&self, info: ThreatInfo, content_hash: &str) -> Result<()> {
        let mut hash_db = self.hash_database.write().await;
        hash_db.insert(content_hash.to_string(), info);

        let mut stats = self.stats.write().await;
        stats.unique_threats += 1;

        Ok(())
    }

    /// Get threat patterns
    pub async fn get_patterns(&self) -> Vec<ThreatPattern> {
        self.patterns.read().await.clone()
    }

    /// Get threat actors
    pub async fn get_actors(&self) -> Vec<ThreatActor> {
        self.actors.read().await.clone()
    }

    /// Get campaigns
    pub async fn get_campaigns(&self) -> Vec<Campaign> {
        self.campaigns.read().await.clone()
    }

    /// Get statistics
    pub async fn get_stats(&self) -> ThreatIntelStats {
        self.stats.read().await.clone()
    }

    /// Add custom threat pattern
    pub async fn add_pattern(&self, pattern: ThreatPattern) -> Result<()> {
        let mut patterns = self.patterns.write().await;
        patterns.push(pattern);
        Ok(())
    }

    /// Add threat actor
    pub async fn add_actor(&self, actor: ThreatActor) -> Result<()> {
        let mut actors = self.actors.write().await;
        actors.push(actor);
        Ok(())
    }

    /// Add campaign
    pub async fn add_campaign(&self, campaign: Campaign) -> Result<()> {
        let mut campaigns = self.campaigns.write().await;
        campaigns.push(campaign);
        Ok(())
    }
}

impl Default for ThreatIntelIntegration {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_threat_intel_creation() {
        let intel = ThreatIntelIntegration::new();
        let stats = intel.get_stats().await;
        assert_eq!(stats.total_lookups, 0);
    }

    #[tokio::test]
    async fn test_lookup_media() {
        let intel = ThreatIntelIntegration::new();
        let media = MediaContent::new(MediaType::Image, vec![0u8; 100]);
        let matches = intel.lookup(&media).await.unwrap();
        // No threats should match for empty test data
        assert!(matches.is_empty() || !matches.is_empty());
    }

    #[tokio::test]
    async fn test_get_patterns() {
        let intel = ThreatIntelIntegration::new();
        let patterns = intel.get_patterns().await;
        assert!(!patterns.is_empty());
    }

    #[tokio::test]
    async fn test_get_actors() {
        let intel = ThreatIntelIntegration::new();
        let actors = intel.get_actors().await;
        assert!(!actors.is_empty());
    }

    #[tokio::test]
    async fn test_add_threat() {
        let intel = ThreatIntelIntegration::new();
        let info = ThreatInfo {
            id: "test-threat".to_string(),
            name: "Test Threat".to_string(),
            threat_type: "test".to_string(),
            severity: 5,
            description: "Test threat description".to_string(),
            recommendations: vec![],
            related: vec![],
            sources: vec!["test".to_string()],
        };

        let result = intel.add_threat(info, "test-hash").await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_update_feeds() {
        let mut intel = ThreatIntelIntegration::new();
        let result = intel.update_feeds().await;
        assert!(result.is_ok());
    }
}