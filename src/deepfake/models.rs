use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Media type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum MediaType {
    Video,
    Audio,
    Image,
    Text,
}

/// Deepfake detection result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeepfakeDetectionResult {
    /// Whether content is likely a deepfake
    pub is_deepfake: bool,
    /// Confidence score (0.0-1.0)
    pub confidence: f64,
    /// Media type detected
    pub media_type: MediaType,
    /// Detection method used
    pub detection_method: DetectionMethod,
    /// Detection timestamp
    pub detected_at: DateTime<Utc>,
    /// Specific analysis results
    pub analysis_details: AnalysisDetails,
    /// Manipulation indicators
    pub manipulation_indicators: Vec<ManipulationIndicator>,
}

/// Detection method
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum DetectionMethod {
    /// Neural network based detection
    NeuralNetwork,
    /// Frequency domain analysis
    FrequencyAnalysis,
    /// Metadata analysis
    MetadataAnalysis,
    /// Forensic watermark detection
    ForensicWatermark,
    /// Artifact detection
    ArtifactDetection,
    /// Ensemble of multiple methods
    Ensemble,
    /// Hybrid approach
    Hybrid,
}

/// Analysis details
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnalysisDetails {
    /// Video-specific details
    pub video_details: Option<VideoAnalysisDetails>,
    /// Audio-specific details
    pub audio_details: Option<AudioAnalysisDetails>,
    /// Image-specific details
    pub image_details: Option<ImageAnalysisDetails>,
    /// Text-specific details
    pub text_details: Option<TextAnalysisDetails>,
}

/// Video analysis details
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VideoAnalysisDetails {
    /// Face swap detection confidence
    pub face_swap_confidence: Option<f64>,
    /// Lip sync analysis score
    pub lip_sync_score: Option<f64>,
    /// Frame consistency score
    pub frame_consistency_score: Option<f64>,
    /// Deepfake technique identified
    pub technique: Option<DeepfakeTechnique>,
    /// Affected time segments (start, end seconds)
    pub affected_segments: Vec<(f64, f64)>,
}

/// Audio analysis details
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AudioAnalysisDetails {
    /// Voice cloning detection confidence
    pub voice_cloning_confidence: Option<f64>,
    /// Synthetic voice indicators
    pub synthetic_indicators: Vec<String>,
    /// Spectral anomalies detected
    pub spectral_anomalies: Vec<String>,
    /// Voice conversion technique
    pub conversion_technique: Option<String>,
}

/// Image analysis details
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImageAnalysisDetails {
    /// GAN generation detection
    pub gan_detection_confidence: Option<f64>,
    /// Editing artifacts detected
    pub editing_artifacts: Vec<String>,
    /// Manipulation regions
    pub manipulated_regions: Vec<ManipulatedRegion>,
    /// AI generation model type
    pub ai_model_type: Option<String>,
}

/// Text analysis details
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TextAnalysisDetails {
    /// LLM generation confidence
    pub llm_generation_confidence: Option<f64>,
    /// Watermark detection result
    pub watermark_detected: bool,
    /// Text generation model identified
    pub generation_model: Option<String>,
    /// Style anomalies
    pub style_anomalies: Vec<String>,
}

/// Deepfake technique
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum DeepfakeTechnique {
    FaceSwap,
    FaceReenactment,
    VoiceCloning,
    LipSyncGeneration,
    FullBodySynthesis,
    NeuralRendering,
    StyleTransfer,
    Unknown,
}

/// Manipulated region in media
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ManipulatedRegion {
    /// Region identifier
    pub region_id: String,
    /// Bounding box (x, y, width, height)
    pub bounding_box: (u32, u32, u32, u32),
    /// Manipulation confidence
    pub confidence: f64,
    /// Type of manipulation
    pub manipulation_type: ManipulationType,
}

/// Manipulation type
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ManipulationType {
    FaceReplacement,
    FaceModification,
    ObjectRemoval,
    ObjectAddition,
    BackgroundReplacement,
    ColorAdjustment,
    StyleTransfer,
    TextReplacement,
    Unknown,
}

/// Manipulation indicator
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ManipulationIndicator {
    /// Indicator type
    pub indicator_type: String,
    /// Description
    pub description: String,
    /// Severity (0.0-1.0)
    pub severity: f64,
    /// Location in media (if applicable)
    pub location: Option<String>,
}

/// Forensics analysis result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ForensicsAnalysis {
    /// Analysis ID
    pub analysis_id: String,
    /// File analyzed
    pub file_path: String,
    /// Media type
    pub media_type: MediaType,
    /// Metadata extracted
    pub metadata: MetadataExtraction,
    /// Content verification result
    pub content_verification: ContentVerification,
    /// Origin tracking information
    pub origin_tracking: Option<OriginTracking>,
    /// Analysis timestamp
    pub analyzed_at: DateTime<Utc>,
}

/// Metadata extraction result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetadataExtraction {
    /// Technical metadata
    pub technical: TechnicalMetadata,
    /// Creation metadata
    pub creation: CreationMetadata,
    /// Device metadata
    pub device: Option<DeviceMetadata>,
    /// Location metadata
    pub location: Option<LocationMetadata>,
    /// Custom metadata
    pub custom: HashMap<String, String>,
    /// Metadata integrity check
    pub integrity: IntegrityCheck,
}

/// Technical metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TechnicalMetadata {
    /// File format
    pub format: String,
    /// File size in bytes
    pub size: u64,
    /// Duration (for audio/video)
    pub duration: Option<f64>,
    /// Dimensions (for images/video)
    pub dimensions: Option<(u32, u32)>,
    /// Codec information
    pub codec: Option<String>,
    /// Bitrate
    pub bitrate: Option<u64>,
    /// Frame rate (for video)
    pub frame_rate: Option<f64>,
}

/// Creation metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreationMetadata {
    /// Creation timestamp
    pub created_at: Option<DateTime<Utc>>,
    /// Modified timestamp
    pub modified_at: Option<DateTime<Utc>>,
    /// Software used
    pub software: Option<String>,
    /// Software version
    pub software_version: Option<String>,
    /// Creator
    pub creator: Option<String>,
}

/// Device metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeviceMetadata {
    /// Device manufacturer
    pub manufacturer: Option<String>,
    /// Device model
    pub model: Option<String>,
    /// Camera make
    pub camera_make: Option<String>,
    /// Camera model
    pub camera_model: Option<String>,
    /// Serial number
    pub serial_number: Option<String>,
}

/// Location metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LocationMetadata {
    /// GPS coordinates
    pub gps_coordinates: Option<(f64, f64)>,
    /// Altitude
    pub altitude: Option<f64>,
    /// Location name
    pub location_name: Option<String>,
    /// Country
    pub country: Option<String>,
    /// Region
    pub region: Option<String>,
}

/// Integrity check result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IntegrityCheck {
    /// Hash of original file
    pub hash: String,
    /// Hash algorithm used
    pub algorithm: String,
    /// Whether metadata is consistent
    pub consistent: bool,
    /// Inconsistencies found
    pub inconsistencies: Vec<String>,
}

/// Content verification result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContentVerification {
    /// Content hash
    pub content_hash: String,
    /// Whether content is verified
    pub verified: bool,
    /// Verification timestamp
    pub verified_at: Option<DateTime<Utc>>,
    /// Verifier identity
    pub verified_by: Option<String>,
    /// Verification chain
    pub verification_chain: Vec<VerificationEntry>,
}

/// Verification entry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VerificationEntry {
    /// Entry timestamp
    pub timestamp: DateTime<Utc>,
    /// Verifier
    pub verifier: String,
    /// Verification method
    pub method: String,
    /// Result
    pub result: bool,
}

/// Origin tracking information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OriginTracking {
    /// Origin ID
    pub origin_id: String,
    /// Original source
    pub source: String,
    /// Creation date
    pub created_at: DateTime<Utc>,
    /// Provenance chain
    pub provenance_chain: Vec<ProvenanceEntry>,
    /// Ownership history
    pub ownership: Vec<OwnershipEntry>,
}

/// Provenance entry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProvenanceEntry {
    /// Entry timestamp
    pub timestamp: DateTime<Utc>,
    /// Entity
    pub entity: String,
    /// Action performed
    pub action: String,
    /// Additional details
    pub details: HashMap<String, String>,
}

/// Ownership entry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OwnershipEntry {
    /// Owner
    pub owner: String,
    /// Ownership start
    pub from: DateTime<Utc>,
    /// Ownership end
    pub to: Option<DateTime<Utc>>,
    /// Transfer reason
    pub reason: Option<String>,
}

/// Content authentication status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContentAuthStatus {
    /// Whether content is authenticated
    pub authenticated: bool,
    /// Authenticity level
    pub authenticity_level: AuthenticityLevel,
    /// Verified by
    pub verified_by: Option<String>,
    /// Verified at
    pub verified_at: Option<DateTime<Utc>>,
    /// Signature validity
    pub signature_valid: Option<bool>,
}

/// Authenticity level
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum AuthenticityLevel {
    /// Verified as authentic
    Authentic,
    /// Likely authentic
    LikelyAuthentic,
    /// Unknown authenticity
    Unknown,
    /// Likely manipulated
    LikelyManipulated,
    /// Confirmed manipulated
    Manipulated,
}

/// Digital signature
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DigitalSignature {
    /// Signature ID
    pub signature_id: String,
    /// Signature algorithm
    pub algorithm: String,
    /// Signature value
    pub signature_value: String,
    /// Signed content hash
    pub content_hash: String,
    /// Signer identity
    pub signer: String,
    /// Signing timestamp
    pub signed_at: DateTime<Utc>,
    /// Certificate chain
    pub certificate_chain: Vec<String>,
}

/// Content provenance
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContentProvenance {
    /// Provenance ID
    pub provenance_id: String,
    /// Content ID
    pub content_id: String,
    /// Creation information
    pub creation: ProvenanceCreation,
    /// Modification history
    pub modifications: Vec<ProvenanceModification>,
    /// Verification chain
    pub verification_chain: Vec<VerificationEntry>,
    /// Current status
    pub status: ProvenanceStatus,
}

/// Provenance creation information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProvenanceCreation {
    /// Creator
    pub creator: String,
    /// Creation timestamp
    pub created_at: DateTime<Utc>,
    /// Creation tool
    pub tool: Option<String>,
    /// Creation parameters
    pub parameters: HashMap<String, String>,
}

/// Provenance modification entry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProvenanceModification {
    /// Modifier
    pub modifier: String,
    /// Modification timestamp
    pub modified_at: DateTime<Utc>,
    /// Modification type
    pub modification_type: String,
    /// Description
    pub description: String,
    /// Tools used
    pub tools: Vec<String>,
}

/// Provenance status
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ProvenanceStatus {
    /// Original content
    Original,
    /// Modified content
    Modified,
    /// Derivative work
    Derivative,
    /// Synthetic/AI-generated
    Synthetic,
    /// Unknown
    Unknown,
}

/// Blockchain provenance entry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlockchainProvenance {
    /// Blockchain ID
    pub blockchain_id: String,
    /// Transaction hash
    pub transaction_hash: String,
    /// Block number
    pub block_number: u64,
    /// Timestamp
    pub timestamp: DateTime<Utc>,
    /// Network
    pub network: String,
    /// Smart contract address
    pub contract_address: Option<String>,
    /// Additional chain data
    pub chain_data: HashMap<String, String>,
}

/// Deepfake incident
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeepfakeIncident {
    /// Incident ID
    pub incident_id: String,
    /// Incident timestamp
    pub timestamp: DateTime<Utc>,
    /// Media type
    pub media_type: MediaType,
    /// Source file
    pub source_file: String,
    /// Detection confidence
    pub confidence: f64,
    /// Incident severity
    pub severity: IncidentSeverity,
    /// Affected entities
    pub affected_entities: Vec<String>,
    /// Threat intelligence
    pub threat_intel: Option<ThreatIntelligence>,
    /// Response status
    pub response_status: ResponseStatus,
}

/// Incident severity
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum IncidentSeverity {
    Low,
    Medium,
    High,
    Critical,
}

/// Threat intelligence information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThreatIntelligence {
    /// Campaign ID
    pub campaign_id: Option<String>,
    /// Threat actor
    pub threat_actor: Option<String>,
    /// Attack technique
    pub attack_technique: Option<String>,
    /// Indicators of compromise
    pub iocs: Vec<String>,
    /// Related incidents
    pub related_incidents: Vec<String>,
}

/// Response status
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ResponseStatus {
    /// Incident detected
    Detected,
    /// Investigation in progress
    Investigating,
    /// Incident contained
    Contained,
    /// Incident remediated
    Remediated,
    /// False positive
    FalsePositive,
    /// Closed
    Closed,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_media_type() {
        assert_eq!(MediaType::Video, MediaType::Video);
        assert_eq!(MediaType::Audio, MediaType::Audio);
    }

    #[test]
    fn test_deepfake_detection_result() {
        let result = DeepfakeDetectionResult {
            is_deepfake: false,
            confidence: 0.95,
            media_type: MediaType::Video,
            detection_method: DetectionMethod::NeuralNetwork,
            detected_at: Utc::now(),
            analysis_details: AnalysisDetails {
                video_details: None,
                audio_details: None,
                image_details: None,
                text_details: None,
            },
            manipulation_indicators: Vec::new(),
        };

        assert_eq!(result.media_type, MediaType::Video);
        assert!(!result.is_deepfake);
    }

    #[test]
    fn test_authenticity_level() {
        assert!(AuthenticityLevel::Authentic != AuthenticityLevel::Manipulated);
    }
}