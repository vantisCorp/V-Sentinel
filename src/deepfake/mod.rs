//! Deepfake Detection and Media Forensics Module
//!
//! This module provides comprehensive capabilities for detecting AI-generated content,
//! analyzing media files, and authenticating digital content.

pub mod detection;
pub mod forensics;
pub mod authentication;
pub mod integration;
pub mod models;

// Re-export main types from models
pub use models::{
    MediaType, DeepfakeDetectionResult, ForensicsAnalysis,
    ContentAuthStatus, AuthenticityLevel, ManipulationType,
};

// Re-export detection types
pub use detection::{
    DeepfakeDetector, DetectionConfig, VideoAnalysisResult,
    AudioAnalysisResult, ImageAnalysisResult, TextAnalysisResult,
};

// Re-export forensics types
pub use forensics::{
    MediaForensicsEngine, ForensicsConfig, MetadataExtraction,
    ContentVerification, OriginTracking,
};

// Re-export authentication types
pub use authentication::{
    ContentAuthenticator, WatermarkConfig, DigitalSignature,
    ContentProvenance, BlockchainProvenance,
};

// Re-export integration types
pub use integration::{
    ThreatIntegrator, IncidentResponse, DeepfakeMonitoring,
    AlertConfig, ResponseWorkflow,
};

/// Error type for Deepfake operations
#[derive(Debug, Clone, thiserror::Error)]
pub enum DeepfakeError {
    #[error("Detection error: {0}")]
    DetectionError(String),
    
    #[error("Forensics error: {0}")]
    ForensicsError(String),
    
    #[error("Authentication error: {0}")]
    AuthenticationError(String),
    
    #[error("Integration error: {0}")]
    IntegrationError(String),
    
    #[error("File processing error: {0}")]
    FileProcessingError(String),
    
    #[error("Analysis error: {0}")]
    AnalysisError(String),
    
    #[error("Configuration error: {0}")]
    ConfigError(String),
}

/// Configuration for Deepfake Detection module
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct DeepfakeConfig {
    /// Detection settings
    pub detection: DetectionSettings,
    /// Forensics settings
    pub forensics: ForensicsSettings,
    /// Authentication settings
    pub authentication: AuthenticationSettings,
    /// Integration settings
    pub integration: IntegrationSettings,
}

impl Default for DeepfakeConfig {
    fn default() -> Self {
        Self {
            detection: DetectionSettings::default(),
            forensics: ForensicsSettings::default(),
            authentication: AuthenticationSettings::default(),
            integration: IntegrationSettings::default(),
        }
    }
}

/// Detection settings
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct DetectionSettings {
    /// Enable video detection
    pub enable_video_detection: bool,
    /// Enable audio detection
    pub enable_audio_detection: bool,
    /// Enable image detection
    pub enable_image_detection: bool,
    /// Enable text watermark detection
    pub enable_text_detection: bool,
    /// Detection threshold (0.0-1.0)
    pub detection_threshold: f64,
    /// Use GPU acceleration if available
    pub use_gpu: bool,
}

impl Default for DetectionSettings {
    fn default() -> Self {
        Self {
            enable_video_detection: true,
            enable_audio_detection: true,
            enable_image_detection: true,
            enable_text_detection: true,
            detection_threshold: 0.7,
            use_gpu: true,
        }
    }
}

/// Forensics settings
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ForensicsSettings {
    /// Enable metadata extraction
    pub enable_metadata_extraction: bool,
    /// Enable content verification
    pub enable_content_verification: bool,
    /// Enable origin tracking
    pub enable_origin_tracking: bool,
    /// Hash algorithm for content verification
    pub hash_algorithm: HashAlgorithm,
}

impl Default for ForensicsSettings {
    fn default() -> Self {
        Self {
            enable_metadata_extraction: true,
            enable_content_verification: true,
            enable_origin_tracking: true,
            hash_algorithm: HashAlgorithm::SHA256,
        }
    }
}

/// Authentication settings
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct AuthenticationSettings {
    /// Enable digital watermarking
    pub enable_watermarking: bool,
    /// Enable content signing
    pub enable_signing: bool,
    /// Enable blockchain provenance
    pub enable_blockchain: bool,
    /// Watermark strength
    pub watermark_strength: u8,
}

impl Default for AuthenticationSettings {
    fn default() -> Self {
        Self {
            enable_watermarking: true,
            enable_signing: true,
            enable_blockchain: false,
            watermark_strength: 50,
        }
    }
}

/// Integration settings
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct IntegrationSettings {
    /// Enable threat intelligence integration
    pub enable_threat_intel: bool,
    /// Enable incident response workflows
    pub enable_incident_response: bool,
    /// Enable monitoring
    pub enable_monitoring: bool,
    /// Alert on detection
    pub alert_on_detection: bool,
}

impl Default for IntegrationSettings {
    fn default() -> Self {
        Self {
            enable_threat_intel: true,
            enable_incident_response: true,
            enable_monitoring: true,
            alert_on_detection: true,
        }
    }
}

/// Hash algorithm options
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum HashAlgorithm {
    MD5,
    SHA1,
    SHA256,
    SHA512,
    BLAKE2b,
}

impl HashAlgorithm {
    pub fn as_str(&self) -> &'static str {
        match self {
            HashAlgorithm::MD5 => "md5",
            HashAlgorithm::SHA1 => "sha1",
            HashAlgorithm::SHA256 => "sha256",
            HashAlgorithm::SHA512 => "sha512",
            HashAlgorithm::BLAKE2b => "blake2b",
        }
    }
}

/// Main Deepfake Detection Manager - integrates all components
pub struct DeepfakeManager {
    config: DeepfakeConfig,
    detector: detection::DeepfakeDetector,
    forensics: forensics::MediaForensicsEngine,
    authenticator: authentication::ContentAuthenticator,
    integrator: integration::ThreatIntegrator,
}

impl DeepfakeManager {
    /// Create a new Deepfake Manager
    pub fn new(config: DeepfakeConfig) -> Self {
        Self {
            detector: detection::DeepfakeDetector::new(
                detection::DetectionConfig::default()
            ),
            forensics: forensics::MediaForensicsEngine::new(
                forensics::ForensicsConfig::default()
            ),
            authenticator: authentication::ContentAuthenticator::new(
                authentication::WatermarkConfig::default()
            ),
            integrator: integration::ThreatIntegrator::new(
                integration::AlertConfig::default()
            ),
            config,
        }
    }

    /// Analyze a media file for deepfakes and perform forensics
    pub async fn analyze_media_file(&self, file_path: &str) -> Result<MediaAnalysisReport, DeepfakeError> {
        // Detect media type
        let media_type = self.detect_media_type(file_path)?;

        // Perform deepfake detection
        let detection_result = self.detector.detect_deepfake(file_path, &media_type).await?;

        // Perform forensics analysis
        let forensics_result = self.forensics.analyze_file(file_path, &media_type).await?;

        // Verify authenticity if watermark present
        let auth_status = if self.config.authentication.enable_signing {
            self.authenticator.verify_content(file_path).await?
        } else {
            ContentAuthStatus {
                authenticated: false,
                authenticity_level: AuthenticityLevel::Unknown,
                verified_by: None,
                verified_at: None,
                signature_valid: None,
            }
        };

        Ok(MediaAnalysisReport {
            file_path: file_path.to_string(),
            media_type,
            detection_result,
            forensics_result,
            auth_status,
            analyzed_at: chrono::Utc::now(),
        })
    }

    /// Detect media type from file
    fn detect_media_type(&self, file_path: &str) -> Result<MediaType, DeepfakeError> {
        let extension = std::path::Path::new(file_path)
            .extension()
            .and_then(|e| e.to_str())
            .ok_or_else(|| DeepfakeError::FileProcessingError(
                "Could not determine file extension".to_string()
            ))?;

        match extension.to_lowercase().as_str() {
            "mp4" | "avi" | "mov" | "mkv" | "webm" => Ok(MediaType::Video),
            "mp3" | "wav" | "flac" | "aac" | "ogg" | "m4a" => Ok(MediaType::Audio),
            "jpg" | "jpeg" | "png" | "gif" | "bmp" | "tiff" | "webp" => Ok(MediaType::Image),
            "txt" | "doc" | "docx" | "pdf" => Ok(MediaType::Text),
            _ => Err(DeepfakeError::FileProcessingError(
                format!("Unsupported file type: {}", extension)
            )),
        }
    }

    /// Get detection statistics
    pub async fn get_detection_stats(&self) -> DetectionStatistics {
        self.detector.get_statistics().await
    }

    /// Get monitoring alerts
    pub async fn get_monitoring_alerts(&self) -> Vec<DeepfakeAlert> {
        self.integrator.get_alerts().await
    }
}

/// Media analysis report
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MediaAnalysisReport {
    /// File path
    pub file_path: String,
    /// Media type
    pub media_type: MediaType,
    /// Detection result
    pub detection_result: DeepfakeDetectionResult,
    /// Forensics result
    pub forensics_result: ForensicsAnalysis,
    /// Authentication status
    pub auth_status: ContentAuthStatus,
    /// Analysis timestamp
    pub analyzed_at: chrono::DateTime<chrono::Utc>,
}

/// Detection statistics
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct DetectionStatistics {
    /// Total files analyzed
    pub total_analyzed: u64,
    /// Deepfakes detected
    pub deepfakes_detected: u64,
    /// False positives
    pub false_positives: u64,
    /// True positives
    pub true_positives: u64,
    /// Detection rate
    pub detection_rate: f64,
}

/// Deepfake alert
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct DeepfakeAlert {
    /// Alert ID
    pub alert_id: String,
    /// Timestamp
    pub timestamp: chrono::DateTime<chrono::Utc>,
    /// Severity
    pub severity: AlertSeverity,
    /// Media type
    pub media_type: MediaType,
    /// File path
    pub file_path: Option<String>,
    /// Detection confidence
    pub confidence: f64,
    /// Alert message
    pub message: String,
}

/// Alert severity
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum AlertSeverity {
    Low,
    Medium,
    High,
    Critical,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config() {
        let config = DeepfakeConfig::default();
        assert!(config.detection.enable_video_detection);
        assert!(config.forensics.enable_metadata_extraction);
        assert!(config.authentication.enable_watermarking);
        assert!(config.integration.enable_threat_intel);
    }

    #[test]
    fn test_hash_algorithm() {
        assert_eq!(HashAlgorithm::SHA256.as_str(), "sha256");
        assert_eq!(HashAlgorithm::SHA512.as_str(), "sha512");
    }
}