use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use chrono::{DateTime, Utc};
use regex::Regex;
use serde::{Deserialize, Serialize};
use crate::shadow_ai::{
    DetectedAIModel, AIModelType, RiskLevel, ShadowAIError
};

/// Configuration for AI model discovery
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiscoveryConfig {
    /// Network interfaces to monitor
    pub network_interfaces: Vec<String>,
    /// Ports to monitor for AI API calls
    pub monitored_ports: Vec<u16>,
    /// Directories to scan for AI model files
    pub scan_directories: Vec<String>,
    /// File extensions indicating AI models
    pub model_extensions: Vec<String>,
    /// Known AI service endpoints
    pub known_ai_endpoints: Vec<String>,
    /// Discovery interval in seconds
    pub scan_interval_secs: u64,
    /// Enable passive network monitoring
    pub passive_monitoring: bool,
    /// Enable active scanning
    pub active_scanning: bool,
}

impl Default for DiscoveryConfig {
    fn default() -> Self {
        Self {
            network_interfaces: vec!["eth0".to_string(), "wlan0".to_string()],
            monitored_ports: vec![443, 80, 8000, 8080, 8888],
            scan_directories: vec![
                "/opt/models".to_string(),
                "/home/*/models".to_string(),
                "/tmp/models".to_string(),
                "/workspace/models".to_string(),
            ],
            model_extensions: vec![
                "pkl".to_string(),
                "h5".to_string(),
                "onnx".to_string(),
                "pb".to_string(),
                "pt".to_string(),
                "pth".to_string(),
                "bin".to_string(),
                "safetensors".to_string(),
            ],
            known_ai_endpoints: vec![
                "api.openai.com".to_string(),
                "api.anthropic.com".to_string(),
                "api.cohere.ai".to_string(),
                "api.replicate.com".to_string(),
                "api.huggingface.co".to_string(),
                "platform.openai.com".to_string(),
                "claude.ai".to_string(),
            ],
            scan_interval_secs: 300,
            passive_monitoring: true,
            active_scanning: true,
        }
    }
}

/// Result of AI model discovery
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AIModelDiscovery {
    /// Unique identifier for the discovery
    pub discovery_id: String,
    /// Detected AI models
    pub detected_models: Vec<DetectedAIModel>,
    /// Network AI API calls detected
    pub api_calls: Vec<AIApiCall>,
    /// Discovery timestamp
    pub discovered_at: DateTime<Utc>,
    /// Discovery duration in seconds
    pub duration_secs: f64,
    /// Sources scanned
    pub sources_scanned: Vec<String>,
}

/// Detected AI API call to external service
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AIApiCall {
    /// Unique identifier
    pub id: String,
    /// API endpoint called
    pub endpoint: String,
    /// AI service provider
    pub provider: AIProvider,
    /// Source IP address
    pub source_ip: String,
    /// Destination IP address
    pub destination_ip: String,
    /// Timestamp
    pub timestamp: DateTime<Utc>,
    /// Request size in bytes
    pub request_size: u64,
    /// Response size in bytes
    pub response_size: u64,
    /// Model being accessed (if identifiable)
    pub model: Option<String>,
    /// Request type (chat, completion, embedding, etc.)
    pub request_type: AIRequestType,
    /// Risk level assessment
    pub risk_level: RiskLevel,
}

/// AI Service Provider
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum AIProvider {
    OpenAI,
    Anthropic,
    Cohere,
    HuggingFace,
    Replicate,
    Google,
    AWSBedrock,
    AzureOpenAI,
    Unknown,
}

/// Type of AI API request
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum AIRequestType {
    ChatCompletion,
    TextCompletion,
    Embedding,
    ImageGeneration,
    AudioTranscription,
    AudioTranslation,
    FineTuning,
    ModelList,
    Unknown,
}

/// AI Model fingerprint
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AIModelFingerprint {
    /// Unique fingerprint hash
    pub fingerprint: String,
    /// Model type
    pub model_type: AIModelType,
    /// Estimated model size in bytes
    pub estimated_size: u64,
    /// File hash (SHA-256)
    pub file_hash: String,
    /// Architecture/framework detected
    pub architecture: String,
    /// Framework version
    pub framework_version: Option<String>,
    /// Key characteristics
    pub characteristics: HashMap<String, String>,
    /// Confidence score (0.0-1.0)
    pub confidence: f64,
}

/// Network traffic analysis result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkTrafficAnalysis {
    /// Analysis ID
    pub analysis_id: String,
    /// Total AI API calls detected
    pub total_ai_calls: u64,
    /// Calls by provider
    pub calls_by_provider: HashMap<AIProvider, u64>,
    /// Calls by request type
    pub calls_by_type: HashMap<AIRequestType, u64>,
    /// Top endpoints
    pub top_endpoints: Vec<(String, u64)>,
    /// Data volume transferred
    pub total_data_volume_bytes: u64,
    /// Analysis period
    pub start_time: DateTime<Utc>,
    pub end_time: DateTime<Utc>,
}

/// Shadow AI Detector - Main detection system
pub struct ShadowAIDetector {
    config: DiscoveryConfig,
    /// Discovered models cache
    discovered_models: Arc<RwLock<HashMap<String, DetectedAIModel>>>,
    /// Network API calls cache
    api_calls: Arc<RwLock<Vec<AIApiCall>>>,
    /// Model fingerprints database
    fingerprints: Arc<RwLock<HashMap<String, AIModelFingerprint>>>,
}

impl ShadowAIDetector {
    /// Create new Shadow AI detector
    pub fn new(config: DiscoveryConfig) -> Self {
        Self {
            config,
            discovered_models: Arc::new(RwLock::new(HashMap::new())),
            api_calls: Arc::new(RwLock::new(Vec::new())),
            fingerprints: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Perform comprehensive AI model discovery
    pub async fn discover_ai_models(&self) -> Result<AIModelDiscovery, ShadowAIError> {
        let start_time = std::time::Instant::now();
        let discovery_id = uuid::Uuid::new_v4().to_string();
        let mut detected_models = Vec::new();
        let mut api_calls = Vec::new();
        let mut sources_scanned = Vec::new();

        // Scan for local AI model files
        if self.config.active_scanning {
            for directory in &self.config.scan_directories {
                match self.scan_directory_for_models(directory).await {
                    Ok(models) => {
                        detected_models.extend(models);
                        sources_scanned.push(format!("directory: {}", directory));
                    }
                    Err(e) => {
                        log::warn!("Failed to scan directory {}: {:?}", directory, e);
                    }
                }
            }
        }

        // Analyze network traffic for AI API calls
        if self.config.passive_monitoring {
            match self.analyze_network_traffic().await {
                Ok(calls) => {
                    api_calls.extend(calls);
                    sources_scanned.push("network_traffic".to_string());
                }
                Err(e) => {
                    log::warn!("Failed to analyze network traffic: {:?}", e);
                }
            }
        }

        // Update caches
        {
            let mut models_cache = self.discovered_models.write().await;
            for model in &detected_models {
                models_cache.insert(model.model_id.clone(), model.clone());
            }
        }

        {
            let mut api_cache = self.api_calls.write().await;
            api_cache.extend(api_calls.clone());
        }

        let duration_secs = start_time.elapsed().as_secs_f64();

        Ok(AIModelDiscovery {
            discovery_id,
            detected_models,
            api_calls,
            discovered_at: Utc::now(),
            duration_secs,
            sources_scanned,
        })
    }

    /// Scan directory for AI model files
    async fn scan_directory_for_models(&self, directory: &str) -> Result<Vec<DetectedAIModel>, ShadowAIError> {
        let mut models = Vec::new();

        // Expand home directory pattern if present
        let scan_path = if directory.contains('*') {
            // Handle glob patterns
            return self.scan_with_glob(directory).await;
        } else {
            directory.to_string()
        };

        if !std::path::Path::new(&scan_path).exists() {
            return Ok(models);
        }

        // Recursively scan directory
        let entries = std::fs::read_dir(&scan_path)
            .map_err(|e| ShadowAIError::DiscoveryError(format!("Failed to read directory {}: {}", scan_path, e)))?;

        for entry in entries {
            let entry = entry.map_err(|e| ShadowAIError::DiscoveryError(format!("Failed to read entry: {}", e)))?;
            let path = entry.path();

            if path.is_dir() {
                // Recursively scan subdirectories
                match self.scan_directory_for_models(path.to_str().unwrap()).await {
                    Ok(sub_models) => models.extend(sub_models),
                    Err(_) => continue,
                }
            } else if let Some(extension) = path.extension() {
                if let Some(ext) = extension.to_str() {
                    if self.config.model_extensions.contains(&ext.to_string()) {
                        // Found a potential model file
                        if let Ok(model) = self.analyze_model_file(&path).await {
                            models.push(model);
                        }
                    }
                }
            }
        }

        Ok(models)
    }

    /// Scan directory with glob patterns
    async fn scan_with_glob(&self, pattern: &str) -> Result<Vec<DetectedAIModel>, ShadowAIError> {
        let mut models = Vec::new();
        
        // Simple glob expansion for common patterns
        let expanded_dirs = if pattern.contains("~/") {
            vec![pattern.replace("~", &std::env::var("HOME").unwrap_or_else(|_| ".".to_string()))]
        } else if pattern.contains("/*/") {
            // Handle patterns like /home/*/models
            let base = pattern.split("/*/").next().unwrap();
            if let Ok(entries) = std::fs::read_dir(base) {
                let mut dirs = Vec::new();
                for entry in entries.flatten() {
                    if entry.path().is_dir() {
                        let suffix = pattern.split("/*/").nth(1).unwrap();
                        dirs.push(format!("{}/{}{}", base, entry.file_name().to_string_lossy(), suffix));
                    }
                }
                dirs
            } else {
                vec![]
            }
        } else {
            vec![pattern.to_string()]
        };

        for dir in expanded_dirs {
            match self.scan_directory_for_models(&dir).await {
                Ok(sub_models) => models.extend(sub_models),
                Err(_) => continue,
            }
        }

        Ok(models)
    }

    /// Analyze a model file to determine its properties
    async fn analyze_model_file(&self, path: &std::path::Path) -> Result<DetectedAIModel, ShadowAIError> {
        let file_path = path.to_str().ok_or_else(|| {
            ShadowAIError::DiscoveryError("Invalid file path".to_string())
        })?;

        let metadata = std::fs::metadata(path)
            .map_err(|e| ShadowAIError::DiscoveryError(format!("Failed to get metadata: {}", e)))?;

        let file_size = metadata.len();
        let modified_time = metadata.modified()
            .ok()
            .and_then(|t| DateTime::<Utc>::from(t).into())
            .unwrap_or_else(Utc::now);

        // Calculate file hash
        let file_hash = self.calculate_file_hash(path).await?;

        // Fingerprint the model
        let fingerprint = self.fingerprint_model(path).await?;

        // Determine model type from fingerprint
        let model_type = fingerprint.model_type.clone();

        // Assess risk based on size and type
        let risk_level = self.assess_model_risk(&model_type, file_size, file_path);

        Ok(DetectedAIModel {
            model_id: uuid::Uuid::new_v4().to_string(),
            model_name: path.file_stem()
                .and_then(|s| s.to_str())
                .unwrap_or("unknown")
                .to_string(),
            model_type,
            model_path: file_path.to_string(),
            model_size_bytes: file_size,
            file_hash,
            fingerprint: fingerprint.fingerprint,
            framework: fingerprint.architecture,
            version: fingerprint.framework_version,
            discovered_at: Utc::now(),
            last_modified: modified_time,
            risk_level,
            registered: false,
            owner: None,
            description: None,
        })
    }

    /// Calculate SHA-256 hash of a file
    async fn calculate_file_hash(&self, path: &std::path::Path) -> Result<String, ShadowAIError> {
        use std::io::Read;
        use sha2::{Sha256, Digest};

        let mut file = std::fs::File::open(path)
            .map_err(|e| ShadowAIError::DiscoveryError(format!("Failed to open file: {}", e)))?;

        let mut hasher = Sha256::new();
        let mut buffer = [0u8; 8192];

        loop {
            let n = file.read(&mut buffer)
                .map_err(|e| ShadowAIError::DiscoveryError(format!("Failed to read file: {}", e)))?;
            if n == 0 {
                break;
            }
            hasher.update(&buffer[..n]);
        }

        Ok(format!("{:x}", hasher.finalize()))
    }

    /// Fingerprint an AI model
    async fn fingerprint_model(&self, path: &std::path::Path) -> Result<AIModelFingerprint, ShadowAIError> {
        let file_path = path.to_str().ok_or_else(|| {
            ShadowAIError::DiscoveryError("Invalid file path".to_string())
        })?;

        // Read file header for fingerprinting
        let mut file = std::fs::File::open(path)
            .map_err(|e| ShadowAIError::DiscoveryError(format!("Failed to open file: {}", e)))?;

        let mut header = [0u8; 1024];
        let bytes_read = std::io::Read::read(&mut file, &mut header)
            .map_err(|e| ShadowAIError::DiscoveryError(format!("Failed to read file: {}", e)))?;

        // Analyze header to determine model type and framework
        let (model_type, architecture, framework_version) = self.analyze_model_header(&header[..bytes_read], file_path);

        // Calculate fingerprint hash
        let fingerprint = self.calculate_fingerprint_hash(&header[..bytes_read], path).await?;

        // Get file size
        let estimated_size = std::fs::metadata(path)
            .map(|m| m.len())
            .unwrap_or(0);

        let mut characteristics = HashMap::new();
        characteristics.insert("file_size".to_string(), estimated_size.to_string());
        characteristics.insert("header_bytes".to_string(), bytes_read.to_string());

        Ok(AIModelFingerprint {
            fingerprint,
            model_type,
            estimated_size,
            file_hash: self.calculate_file_hash(path).await?,
            architecture,
            framework_version,
            characteristics,
            confidence: 0.85, // Default confidence for file-based detection
        })
    }

    /// Analyze model header to determine type
    fn analyze_model_header(&self, header: &[u8], file_path: &str) -> (AIModelType, String, Option<String>) {
        let file_name = std::path::Path::new(file_path)
            .file_name()
            .and_then(|s| s.to_str())
            .unwrap_or("");

        // Try to identify by magic bytes/signatures
        if header.len() >= 4 {
            let magic = &header[..4.min(header.len())];
            
            // Check for common model signatures
            if magic == b"\x80\x02\x00\x00" || magic == b"\x80\x03\x00\x00" || magic == b"\x80\x04\x00\x00" {
                return (AIModelType::Unknown, "Python Pickle".to_string(), None);
            }
            
            // HDF5 format (common for Keras/TensorFlow)
            if magic == b"\x89\x48\x44\x46" {
                return (AIModelType::NeuralNetwork, "HDF5/Keras".to_string(), None);
            }
            
            // Protocol Buffers (TensorFlow SavedModel)
            if magic == [0x08] || magic == [0x0A] {
                return (AIModelType::NeuralNetwork, "TensorFlow SavedModel".to_string(), None);
            }
        }

        // Try to identify by file extension
        if let Some(extension) = std::path::Path::new(file_path).extension() {
            if let Some(ext) = extension.to_str() {
                match ext.to_lowercase().as_str() {
                    "onnx" => return (AIModelType::NeuralNetwork, "ONNX".to_string(), None),
                    "pb" => return (AIModelType::NeuralNetwork, "TensorFlow".to_string(), None),
                    "h5" => return (AIModelType::NeuralNetwork, "Keras/HDF5".to_string(), None),
                    "pkl" => return (AIModelType::Unknown, "Python Pickle".to_string(), None),
                    "pt" | "pth" => return (AIModelType::NeuralNetwork, "PyTorch".to_string(), None),
                    "bin" => return (AIModelType::LargeLanguageModel, "HuggingFace Transformers".to_string(), None),
                    "safetensors" => return (AIModelType::LargeLanguageModel, "Safetensors".to_string(), None),
                    _ => {}
                }
            }
        }

        // Default: Unknown
        (AIModelType::Unknown, "Unknown".to_string(), None)
    }

    /// Calculate fingerprint hash from header and metadata
    async fn calculate_fingerprint_hash(&self, header: &[u8], path: &std::path::Path) -> Result<String, ShadowAIError> {
        use sha2::{Sha256, Digest};
        
        let mut hasher = Sha256::new();
        hasher.update(header);
        
        // Include file size in fingerprint
        if let Ok(metadata) = std::fs::metadata(path) {
            hasher.update(metadata.len().to_be_bytes());
        }
        
        Ok(format!("{:x}", hasher.finalize()))
    }

    /// Assess risk level of a detected model
    fn assess_model_risk(&self, model_type: &AIModelType, size: u64, path: &str) -> RiskLevel {
        let mut risk_score = 0;

        // Risk by model type
        match model_type {
            AIModelType::LargeLanguageModel => risk_score += 3,
            AIModelType::GenerativeAI => risk_score += 2,
            AIModelType::NeuralNetwork => risk_score += 1,
            AIModelType::MachineLearning => risk_score += 1,
            AIModelType::ComputerVision => risk_score += 1,
            AIModelType::NaturalLanguageProcessing => risk_score += 2,
            AIModelType::Unknown => risk_score += 2,
        }

        // Risk by size (larger models often mean more powerful and potentially unauthorized)
        if size > 1_000_000_000 { // > 1GB
            risk_score += 2;
        } else if size > 100_000_000 { // > 100MB
            risk_score += 1;
        }

        // Risk by location (suspicious locations = higher risk)
        if path.contains("/tmp/") || path.contains("/cache/") {
            risk_score += 2;
        } else if path.contains("~/") && !path.contains("/models/") {
            risk_score += 1;
        }

        // Convert score to risk level
        match risk_score {
            0..=1 => RiskLevel::Low,
            2..=3 => RiskLevel::Medium,
            4..=5 => RiskLevel::High,
            _ => RiskLevel::Critical,
        }
    }

    /// Analyze network traffic for AI API calls
    async fn analyze_network_traffic(&self) -> Result<Vec<AIApiCall>, ShadowAIError> {
        let mut api_calls = Vec::new();

        // In a real implementation, this would analyze network packets
        // For now, we'll simulate detection based on system logs or other sources
        
        // Check for common log locations that might contain API call records
        let log_paths = [
            "/var/log/syslog",
            "/var/log/messages",
            "/var/log/access.log",
        ];

        for log_path in &log_paths {
            if let Ok(content) = std::fs::read_to_string(log_path) {
                let calls = self.scan_logs_for_api_calls(&content);
                api_calls.extend(calls);
            }
        }

        // Filter for known AI endpoints
        api_calls.retain(|call| {
            self.config.known_ai_endpoints.iter()
                .any(|endpoint| call.endpoint.contains(endpoint))
        });

        // Assess risk for each API call
        for call in &mut api_calls {
            call.risk_level = self.assess_api_call_risk(call);
        }

        Ok(api_calls)
    }

    /// Scan log content for AI API calls
    fn scan_logs_for_api_calls(&self, log_content: &str) -> Vec<AIApiCall> {
        let mut api_calls = Vec::new();

        // Common patterns for AI API calls
        let patterns = vec![
            (r"api\.openai\.com/v1/(\w+)", AIProvider::OpenAI),
            (r"api\.anthropic\.com/v1/(\w+)", AIProvider::Anthropic),
            (r"api\.cohere\.ai/v1/(\w+)", AIProvider::Cohere),
            (r"api\.huggingface\.co/(\w+)", AIProvider::HuggingFace),
        ];

        for (pattern, provider) in patterns {
            if let Ok(re) = Regex::new(pattern) {
                for cap in re.captures_iter(log_content) {
                    if let Some(endpoint) = cap.get(0) {
                        let request_type = self.infer_request_type(endpoint.as_str());
                        
                        api_calls.push(AIApiCall {
                            id: uuid::Uuid::new_v4().to_string(),
                            endpoint: endpoint.as_str().to_string(),
                            provider: provider.clone(),
                            source_ip: "unknown".to_string(),
                            destination_ip: "unknown".to_string(),
                            timestamp: Utc::now(),
                            request_size: 0,
                            response_size: 0,
                            model: None,
                            request_type,
                            risk_level: RiskLevel::Medium,
                        });
                    }
                }
            }
        }

        api_calls
    }

    /// Infer request type from endpoint path
    fn infer_request_type(&self, endpoint: &str) -> AIRequestType {
        if endpoint.contains("chat") {
            AIRequestType::ChatCompletion
        } else if endpoint.contains("completions") {
            AIRequestType::TextCompletion
        } else if endpoint.contains("embeddings") {
            AIRequestType::Embedding
        } else if endpoint.contains("images") || endpoint.contains("generations") {
            AIRequestType::ImageGeneration
        } else if endpoint.contains("audio") || endpoint.contains("transcri") || endpoint.contains("translat") {
            AIRequestType::AudioTranscription
        } else if endpoint.contains("fine-tune") {
            AIRequestType::FineTuning
        } else if endpoint.contains("models") {
            AIRequestType::ModelList
        } else {
            AIRequestType::Unknown
        }
    }

    /// Assess risk level for an API call
    fn assess_api_call_risk(&self, call: &AIApiCall) -> RiskLevel {
        let mut risk_score = 0;

        // Risk by request type
        match call.request_type {
            AIRequestType::ChatCompletion | AIRequestType::TextCompletion => risk_score += 1,
            AIRequestType::ImageGeneration => risk_score += 2,
            AIRequestType::FineTuning => risk_score += 3,
            AIRequestType::AudioTranscription | AIRequestType::AudioTranslation => risk_score += 1,
            _ => {}
        }

        // Risk by data volume
        if call.request_size > 10_000_000 { // > 10MB
            risk_score += 2;
        } else if call.request_size > 1_000_000 { // > 1MB
            risk_score += 1;
        }

        match risk_score {
            0 => RiskLevel::Low,
            1 => RiskLevel::Medium,
            2..=3 => RiskLevel::High,
            _ => RiskLevel::Critical,
        }
    }

    /// Get comprehensive network traffic analysis
    pub async fn get_network_analysis(&self) -> Result<NetworkTrafficAnalysis, ShadowAIError> {
        let api_calls = self.api_calls.read().await;

        let total_ai_calls = api_calls.len() as u64;
        let mut calls_by_provider: HashMap<AIProvider, u64> = HashMap::new();
        let mut calls_by_type: HashMap<AIRequestType, u64> = HashMap::new();
        let mut endpoint_counts: HashMap<String, u64> = HashMap::new();
        let mut total_data_volume = 0u64;

        let start_time = api_calls.first()
            .map(|c| c.timestamp)
            .unwrap_or_else(Utc::now);

        let end_time = api_calls.last()
            .map(|c| c.timestamp)
            .unwrap_or_else(Utc::now);

        for call in api_calls.iter() {
            *calls_by_provider.entry(call.provider.clone()).or_insert(0) += 1;
            *calls_by_type.entry(call.request_type.clone()).or_insert(0) += 1;
            *endpoint_counts.entry(call.endpoint.clone()).or_insert(0) += 1;
            total_data_volume += call.request_size + call.response_size;
        }

        let mut top_endpoints: Vec<(String, u64)> = endpoint_counts.into_iter().collect();
        top_endpoints.sort_by(|a, b| b.1.cmp(&a.1));
        top_endpoints.truncate(10);

        Ok(NetworkTrafficAnalysis {
            analysis_id: uuid::Uuid::new_v4().to_string(),
            total_ai_calls,
            calls_by_provider,
            calls_by_type,
            top_endpoints,
            total_data_volume_bytes: total_data_volume,
            start_time,
            end_time,
        })
    }

    /// Get all discovered models
    pub async fn get_discovered_models(&self) -> Vec<DetectedAIModel> {
        self.discovered_models.read().await.values().cloned().collect()
    }

    /// Get all detected API calls
    pub async fn get_api_calls(&self) -> Vec<AIApiCall> {
        self.api_calls.read().await.clone()
    }

    /// Clear discovered models cache
    pub async fn clear_cache(&self) {
        self.discovered_models.write().await.clear();
        self.api_calls.write().await.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_discovery_config() {
        let config = DiscoveryConfig::default();
        assert!(!config.network_interfaces.is_empty());
        assert!(!config.monitored_ports.is_empty());
        assert!(!config.model_extensions.is_empty());
    }

    #[test]
    fn test_infer_request_type() {
        let detector = ShadowAIDetector::new(DiscoveryConfig::default());
        
        assert_eq!(
            detector.infer_request_type("api.openai.com/v1/chat/completions"),
            AIRequestType::ChatCompletion
        );
        assert_eq!(
            detector.infer_request_type("api.openai.com/v1/completions"),
            AIRequestType::TextCompletion
        );
        assert_eq!(
            detector.infer_request_type("api.openai.com/v1/embeddings"),
            AIRequestType::Embedding
        );
    }

    #[test]
    fn test_assess_model_risk() {
        let detector = ShadowAIDetector::new(DiscoveryConfig::default());
        
        // Large LLM in suspicious location
        let risk = detector.assess_model_risk(&AIModelType::LargeLanguageModel, 2_000_000_000, "/tmp/model.bin");
        assert_eq!(risk, RiskLevel::Critical);
        
        // Small model in proper location
        let risk = detector.assess_model_risk(&AIModelType::MachineLearning, 10_000_000, "/opt/models/model.pkl");
        assert_eq!(risk, RiskLevel::Low);
    }
}