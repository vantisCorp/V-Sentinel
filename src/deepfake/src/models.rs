//! Data models for deepfake detection module

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Media content for analysis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MediaContent {
    /// Unique identifier
    pub id: String,
    /// Type of media
    pub media_type: MediaType,
    /// Raw media data
    pub data: Vec<u8>,
    /// Additional metadata
    pub metadata: HashMap<String, String>,
    /// Whether content is authenticated
    pub authenticated: bool,
    /// Authentication ID if present
    pub authentication_id: Option<String>,
    /// Creation timestamp
    pub created_at: DateTime<Utc>,
}

impl MediaContent {
    /// Create new media content
    pub fn new(media_type: MediaType, data: Vec<u8>) -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            media_type,
            data,
            metadata: HashMap::new(),
            authenticated: false,
            authentication_id: None,
            created_at: Utc::now(),
        }
    }

    /// Add metadata
    pub fn with_metadata(mut self, key: &str, value: &str) -> Self {
        self.metadata.insert(key.to_string(), value.to_string());
        self
    }

    /// Get file extension based on media type
    pub fn extension(&self) -> &str {
        self.metadata.get("extension").map(|s| s.as_str()).unwrap_or_else(|| {
            match self.media_type {
                MediaType::Image => "jpg",
                MediaType::Video => "mp4",
                MediaType::Audio => "mp3",
            }
        })
    }
}

/// Type of media content
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MediaType {
    /// Image content (JPEG, PNG, etc.)
    Image,
    /// Video content (MP4, AVI, etc.)
    Video,
    /// Audio content (MP3, WAV, etc.)
    Audio,
}

impl std::fmt::Display for MediaType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MediaType::Image => write!(f, "image"),
            MediaType::Video => write!(f, "video"),
            MediaType::Audio => write!(f, "audio"),
        }
    }
}

/// Type of deepfake detected
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DeepfakeType {
    /// Face swap deepfake
    FaceSwap,
    /// Lip sync deepfake
    LipSync,
    /// Voice cloning/synthesis
    VoiceClone,
    /// Full body synthesis
    BodySynthesis,
    /// Image manipulation (GANs, etc.)
    ImageManipulation,
    /// Video manipulation
    VideoManipulation,
    /// Audio manipulation
    AudioManipulation,
    /// Text-to-speech synthesis
    TextToSpeech,
    /// Real-time deepfake
    RealTime,
    /// Unknown/other type
    Unknown,
}

impl std::fmt::Display for DeepfakeType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DeepfakeType::FaceSwap => write!(f, "face_swap"),
            DeepfakeType::LipSync => write!(f, "lip_sync"),
            DeepfakeType::VoiceClone => write!(f, "voice_clone"),
            DeepfakeType::BodySynthesis => write!(f, "body_synthesis"),
            DeepfakeType::ImageManipulation => write!(f, "image_manipulation"),
            DeepfakeType::VideoManipulation => write!(f, "video_manipulation"),
            DeepfakeType::AudioManipulation => write!(f, "audio_manipulation"),
            DeepfakeType::TextToSpeech => write!(f, "text_to_speech"),
            DeepfakeType::RealTime => write!(f, "real_time"),
            DeepfakeType::Unknown => write!(f, "unknown"),
        }
    }
}

impl DeepfakeType {
    /// Get description of the deepfake type
    pub fn description(&self) -> &str {
        match self {
            DeepfakeType::FaceSwap => "AI-generated face replacement in images or video",
            DeepfakeType::LipSync => "AI-generated lip movements synchronized with audio",
            DeepfakeType::VoiceClone => "AI-synthesized voice mimicking a specific person",
            DeepfakeType::BodySynthesis => "AI-generated full body movements or appearance",
            DeepfakeType::ImageManipulation => "AI-generated or manipulated image content",
            DeepfakeType::VideoManipulation => "AI-generated or manipulated video content",
            DeepfakeType::AudioManipulation => "AI-generated or manipulated audio content",
            DeepfakeType::TextToSpeech => "AI-synthesized speech from text input",
            DeepfakeType::RealTime => "Live deepfake generation during streaming",
            DeepfakeType::Unknown => "Unknown type of deepfake manipulation",
        }
    }
}

/// Confidence level classification
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ConfidenceLevel {
    /// Very low confidence (0.0 - 0.25)
    VeryLow,
    /// Low confidence (0.25 - 0.5)
    Low,
    /// Medium confidence (0.5 - 0.75)
    Medium,
    /// High confidence (0.75 - 0.9)
    High,
    /// Very high confidence (0.9 - 1.0)
    VeryHigh,
}

impl From<f32> for ConfidenceLevel {
    fn from(confidence: f32) -> Self {
        if confidence >= 0.9 {
            ConfidenceLevel::VeryHigh
        } else if confidence >= 0.75 {
            ConfidenceLevel::High
        } else if confidence >= 0.5 {
            ConfidenceLevel::Medium
        } else if confidence >= 0.25 {
            ConfidenceLevel::Low
        } else {
            ConfidenceLevel::VeryLow
        }
    }
}

/// Media metadata extracted during analysis
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MediaMetadata {
    /// File format
    pub format: Option<String>,
    /// Duration in seconds (for video/audio)
    pub duration: Option<f64>,
    /// Width in pixels (for image/video)
    pub width: Option<u32>,
    /// Height in pixels (for image/video)
    pub height: Option<u32>,
    /// Frame rate (for video)
    pub frame_rate: Option<f32>,
    /// Bit rate
    pub bit_rate: Option<u64>,
    /// Sample rate (for audio)
    pub sample_rate: Option<u32>,
    /// Number of channels (for audio)
    pub channels: Option<u32>,
    /// Codec information
    pub codec: Option<String>,
    /// Creation time from metadata
    pub creation_time: Option<DateTime<Utc>>,
    /// Location information
    pub location: Option<String>,
    /// Device information
    pub device: Option<String>,
    /// Software used to create/edit
    pub software: Option<String>,
    /// EXIF data hash
    pub exif_hash: Option<String>,
    /// Custom metadata fields
    pub custom: HashMap<String, String>,
}

/// Face detection result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FaceDetection {
    /// Bounding box coordinates [x, y, width, height]
    pub bounding_box: [f32; 4],
    /// Confidence of face detection
    pub confidence: f32,
    /// Face embedding vector
    pub embedding: Option<Vec<f32>>,
    /// Estimated age
    pub age: Option<u32>,
    /// Estimated gender
    pub gender: Option<String>,
    /// Emotion detection
    pub emotion: Option<String>,
    /// Landmarks (key facial points)
    pub landmarks: Option<Vec<[f32; 2]>>,
    /// Whether face appears synthetic
    pub synthetic_probability: Option<f32>,
}

/// Audio analysis result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AudioAnalysis {
    /// Detected speakers
    pub speakers: Vec<SpeakerInfo>,
    /// Background noise level
    pub noise_level: f32,
    /// Voice characteristics
    pub voice_characteristics: VoiceCharacteristics,
    /// Anomalies detected
    pub anomalies: Vec<AudioAnomaly>,
    /// Synthesis probability
    pub synthesis_probability: f32,
}

/// Speaker information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpeakerInfo {
    /// Speaker identifier
    pub id: String,
    /// Speaking duration in seconds
    pub duration: f64,
    /// Voice print hash
    pub voice_print: Option<String>,
    /// Average pitch
    pub avg_pitch: Option<f32>,
}

/// Voice characteristics
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct VoiceCharacteristics {
    /// Average pitch in Hz
    pub avg_pitch: f32,
    /// Pitch variance
    pub pitch_variance: f32,
    /// Speaking rate (words per minute)
    pub speaking_rate: f32,
    /// Average volume
    pub avg_volume: f32,
    /// Voice clarity score
    pub clarity: f32,
}

/// Audio anomaly detection
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AudioAnomaly {
    /// Anomaly type
    pub anomaly_type: String,
    /// Timestamp in seconds
    pub timestamp: f64,
    /// Duration in seconds
    pub duration: f64,
    /// Description
    pub description: String,
    /// Severity (0.0 - 1.0)
    pub severity: f32,
}

/// Video frame analysis result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FrameAnalysis {
    /// Frame number
    pub frame_number: u64,
    /// Timestamp in seconds
    pub timestamp: f64,
    /// Detected faces
    pub faces: Vec<FaceDetection>,
    /// Scene change detected
    pub scene_change: bool,
    /// Manipulation indicators
    pub manipulation_indicators: Vec<String>,
    /// Artifacts detected
    pub artifacts: Vec<String>,
    /// Inconsistency score
    pub inconsistency_score: f32,
}

/// Detection indicators for deepfake analysis
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DetectionIndicators {
    /// Blurring artifacts
    pub blurring_artifacts: u32,
    /// Edge inconsistencies
    pub edge_inconsistencies: u32,
    /// Color inconsistencies
    pub color_inconsistencies: u32,
    /// Lighting inconsistencies
    pub lighting_inconsistencies: u32,
    /// Texture anomalies
    pub texture_anomalies: u32,
    /// Temporal inconsistencies (for video)
    pub temporal_inconsistencies: u32,
    /// Audio-video sync issues
    pub av_sync_issues: u32,
    /// Metadata inconsistencies
    pub metadata_inconsistencies: u32,
    /// Compression artifacts
    pub compression_artifacts: u32,
    /// GAN artifacts
    pub gan_artifacts: u32,
}

impl DetectionIndicators {
    /// Calculate overall indicator score
    pub fn calculate_score(&self) -> f32 {
        let total = self.blurring_artifacts
            + self.edge_inconsistencies
            + self.color_inconsistencies
            + self.lighting_inconsistencies
            + self.texture_anomalies
            + self.temporal_inconsistencies
            + self.av_sync_issues
            + self.metadata_inconsistencies
            + self.compression_artifacts
            + self.gan_artifacts;
        
        // Normalize to 0.0 - 1.0 range (assuming max ~100 indicators)
        (total as f32 / 100.0).min(1.0)
    }

    /// Check if any critical indicators are present
    pub fn has_critical_indicators(&self) -> bool {
        self.gan_artifacts > 5 || self.temporal_inconsistencies > 10
    }
}

/// Watermark configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Watermark {
    /// Watermark ID
    pub id: String,
    /// Watermark type
    pub watermark_type: WatermarkType,
    /// Watermark data
    pub data: Vec<u8>,
    /// Position for visible watermarks
    pub position: Option<WatermarkPosition>,
    /// Opacity for visible watermarks
    pub opacity: Option<f32>,
    /// Whether watermark is visible
    pub visible: bool,
    /// Creation timestamp
    pub created_at: DateTime<Utc>,
}

/// Type of watermark
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum WatermarkType {
    /// Visible watermark (overlay)
    Visible,
    /// Invisible watermark (steganography)
    Invisible,
    /// Frequency domain watermark
    FrequencyDomain,
    /// Blockchain-based watermark
    Blockchain,
    /// Digital signature
    DigitalSignature,
}

/// Watermark position
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum WatermarkPosition {
    TopLeft,
    TopRight,
    BottomLeft,
    BottomRight,
    Center,
    Tile,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_media_content_creation() {
        let media = MediaContent::new(MediaType::Image, vec![0u8; 100]);
        assert!(!media.id.is_empty());
        assert_eq!(media.media_type, MediaType::Image);
        assert_eq!(media.data.len(), 100);
    }

    #[test]
    fn test_media_type_display() {
        assert_eq!(MediaType::Image.to_string(), "image");
        assert_eq!(MediaType::Video.to_string(), "video");
        assert_eq!(MediaType::Audio.to_string(), "audio");
    }

    #[test]
    fn test_deepfake_type_display() {
        assert_eq!(DeepfakeType::FaceSwap.to_string(), "face_swap");
        assert_eq!(DeepfakeType::VoiceClone.to_string(), "voice_clone");
    }

    #[test]
    fn test_confidence_level_from_f32() {
        assert_eq!(ConfidenceLevel::from(0.95), ConfidenceLevel::VeryHigh);
        assert_eq!(ConfidenceLevel::from(0.8), ConfidenceLevel::High);
        assert_eq!(ConfidenceLevel::from(0.6), ConfidenceLevel::Medium);
        assert_eq!(ConfidenceLevel::from(0.3), ConfidenceLevel::Low);
        assert_eq!(ConfidenceLevel::from(0.1), ConfidenceLevel::VeryLow);
    }

    #[test]
    fn test_detection_indicators_score() {
        let indicators = DetectionIndicators {
            blurring_artifacts: 5,
            edge_inconsistencies: 3,
            gan_artifacts: 2,
            ..Default::default()
        };
        let score = indicators.calculate_score();
        assert!(score > 0.0 && score < 1.0);
    }

    #[test]
    fn test_detection_indicators_critical() {
        let critical = DetectionIndicators {
            gan_artifacts: 10,
            ..Default::default()
        };
        assert!(critical.has_critical_indicators());

        let normal = DetectionIndicators {
            gan_artifacts: 2,
            ..Default::default()
        };
        assert!(!normal.has_critical_indicators());
    }
}