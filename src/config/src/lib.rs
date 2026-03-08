//! SENTINEL Configuration Module
//! 
//! This module provides production-ready configuration management
//! with support for multiple formats, validation, and hot-reloading.

use anyhow::{Result, anyhow};
use tracing::{info, debug, warn, error};
use std::sync::Arc;
use std::collections::HashMap;
use tokio::sync::RwLock;
use serde::{Serialize, Deserialize};
use std::path::{Path, PathBuf};
use std::fs;
use notify::{Watcher, RecursiveMode, watcher, DebouncedEvent, RecommendedWatcher};
use std::time::Duration;

pub mod pqc_validator;

/// Configuration Manager
pub struct ConfigManager {
    config: Arc<RwLock<Config>>,
    watchers: Arc<RwLock<Vec<RecommendedWatcher>>>,
    validators: Arc<RwLock<Vec<Box<dyn ConfigValidator>>>>,
    reload_callbacks: Arc<RwLock<Vec<Box<dyn ReloadCallback>>>>,
}

/// Main Configuration Structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    /// Core configuration
    pub core: CoreConfig,
    /// AI configuration
    pub ai: AIConfig,
    /// Gaming configuration
    pub gaming: GamingConfig,
    /// Quantum configuration
    pub quantum: QuantumConfig,
    /// Network configuration
    pub network: NetworkConfig,
    /// Logging configuration
    pub logging: LoggingConfig,
    /// Security configuration
    pub security: SecurityConfig,
    /// Performance configuration
    pub performance: PerformanceConfig,
}

/// Core Configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreConfig {
    /// Application name
    pub app_name: String,
    /// Application version
    pub version: String,
    /// Environment (development, staging, production)
    pub environment: Environment,
    /// Debug mode
    pub debug: bool,
    /// Data directory
    pub data_dir: PathBuf,
    /// Cache directory
    pub cache_dir: PathBuf,
    /// Log directory
    pub log_dir: PathBuf,
    /// Maximum concurrent operations
    pub max_concurrent_ops: usize,
    /// Operation timeout in seconds
    pub operation_timeout_secs: u64,
}

/// AI Configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AIConfig {
    /// Model path
    pub model_path: PathBuf,
    /// Model type
    pub model_type: String,
    /// Batch size
    pub batch_size: usize,
    /// Inference threads
    pub inference_threads: usize,
    /// Enable NPU offloading
    pub enable_npu: bool,
    /// Enable GPU acceleration
    pub enable_gpu: bool,
    /// Cache size in MB
    pub cache_size_mb: usize,
    /// Prediction timeout in milliseconds
    pub prediction_timeout_ms: u64,
}

/// Gaming Configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GamingConfig {
    /// Enable gaming mode
    pub enabled: bool,
    /// Auto-detect games
    pub auto_detect: bool,
    /// Zero-scan mode
    pub zero_scan_mode: bool,
    /// Anti-DDoS protection
    pub anti_ddos: bool,
    /// RAM defolding
    pub ram_defolding: bool,
    /// AI overclocking
    pub ai_overclocking: bool,
    /// Trusted handshake
    pub trusted_handshake: bool,
    /// Game list
    pub game_list: Vec<String>,
}

/// Quantum Configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuantumConfig {
    /// Enable quantum cryptography
    pub enabled: bool,
    /// KEM algorithm
    pub kem_algorithm: String,
    /// Signature algorithm
    pub signature_algorithm: String,
    /// Hybrid mode
    pub hybrid_mode: bool,
    /// Key rotation interval in hours
    pub key_rotation_hours: u64,
}

/// Network Configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkConfig {
    /// Listen address
    pub listen_address: String,
    /// Listen port
    pub listen_port: u16,
    /// Enable TLS
    pub enable_tls: bool,
    /// TLS certificate path
    pub tls_cert_path: Option<PathBuf>,
    /// TLS key path
    pub tls_key_path: Option<PathBuf>,
    /// Max connections
    pub max_connections: usize,
    /// Connection timeout in seconds
    pub connection_timeout_secs: u64,
    /// Enable rate limiting
    pub enable_rate_limiting: bool,
    /// Rate limit requests per minute
    pub rate_limit_rpm: u64,
    
    // PQC-specific fields
    /// Enable Post-Quantum Cryptography
    pub enable_pqc: bool,
    /// PQC KEM algorithm (e.g., "CrystalsKyber768", "CrystalsKyber512", "CrystalsKyber1024")
    pub pqc_kem_algorithm: Option<String>,
    /// PQC signature algorithm (e.g., "CrystalsDilithium2", "CrystalsDilithium3", "CrystalsDilithium5")
    pub pqc_signature_algorithm: Option<String>,
    /// Enable hybrid mode (PQC + classical algorithms)
    pub pqc_hybrid_mode: bool,
    /// Allow fallback to classical algorithms if PQC fails
    pub pqc_fallback_to_classical: bool,
    /// PQC certificate path (Dilithium/FALCON certificates)
    pub pqc_cert_path: Option<PathBuf>,
    /// PQC private key path
    pub pqc_key_path: Option<PathBuf>,
    /// Minimum PQC security level (1-5)
    pub pqc_min_security_level: u8,
}

/// PQC KEM Algorithm Types
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PqcKemAlgorithm {
    /// CRYSTALS-Kyber 512 (NIST Level 1)
    CrystalsKyber512,
    /// CRYSTALS-Kyber 768 (NIST Level 3)
    CrystalsKyber768,
    /// CRYSTALS-Kyber 1024 (NIST Level 5)
    CrystalsKyber1024,
}

/// PQC Signature Algorithm Types
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PqcSignatureAlgorithm {
    /// CRYSTALS-Dilithium 2 (NIST Level 1)
    CrystalsDilithium2,
    /// CRYSTALS-Dilithium 3 (NIST Level 3)
    CrystalsDilithium3,
    /// CRYSTALS-Dilithium 5 (NIST Level 5)
    CrystalsDilithium5,
    /// FALCON 512 (NIST Level 1)
    Falcon512,
    /// FALCON 1024 (NIST Level 5)
    Falcon1024,
    /// SPHINCS+ SHA2 128f
    SphincsPlusSha2128f,
    /// SPHINCS+ SHA2 192f
    SphincsPlusSha2192f,
    /// SPHINCS+ SHA2 256f
    SphincsPlusSha2256f,
}

/// Logging Configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoggingConfig {
    /// Log level
    pub level: LogLevel,
    /// Log format
    pub format: LogFormat,
    /// Enable console logging
    pub console: bool,
    /// Enable file logging
    pub file: bool,
    /// Log file path
    pub file_path: Option<PathBuf>,
    /// Max log file size in MB
    pub max_file_size_mb: usize,
    /// Max log files
    pub max_files: usize,
    /// Enable structured logging
    pub structured: bool,
}

/// Security Configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityConfig {
    /// Enable secure boot
    pub enable_secure_boot: bool,
    /// Enable immutable partition
    pub enable_immutable_partition: bool,
    /// Enable hardware attestation
    pub enable_hardware_attestation: bool,
    /// Enable TPM
    pub enable_tpm: bool,
    /// TPM device path
    pub tpm_device_path: Option<PathBuf>,
    /// Enable secure enclave
    pub enable_secure_enclave: bool,
    /// Audit logging
    pub audit_logging: bool,
    /// Audit log path
    pub audit_log_path: Option<PathBuf>,
}

/// Performance Configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceConfig {
    /// CPU limit percentage
    pub cpu_limit_percent: f64,
    /// Memory limit in MB
    pub memory_limit_mb: usize,
    /// Enable performance profiling
    pub enable_profiling: bool,
    /// Profile interval in seconds
    pub profile_interval_secs: u64,
    /// Enable metrics collection
    pub enable_metrics: bool,
    /// Metrics export interval in seconds
    pub metrics_interval_secs: u64,
    /// Enable caching
    pub enable_caching: bool,
    /// Cache TTL in seconds
    pub cache_ttl_secs: u64,
}

/// Environment
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Environment {
    Development,
    Staging,
    Production,
}

/// Log Level
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum LogLevel {
    Trace,
    Debug,
    Info,
    Warn,
    Error,
}

/// Log Format
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum LogFormat {
    Text,
    Json,
    Pretty,
}

/// Config Validator trait
pub trait ConfigValidator: Send + Sync {
    fn validate(&self, config: &Config) -> Result<()>;
    fn name(&self) -> &str;
}

/// Reload Callback trait
pub trait ReloadCallback: Send + Sync {
    fn on_reload(&self, old_config: &Config, new_config: &Config) -> Result<()>;
    fn name(&self) -> &str;
}

impl ConfigManager {
    /// Create a new configuration manager
    pub fn new() -> Result<Self> {
        info!("Creating Configuration Manager...");
        
        Ok(Self {
            config: Arc::new(RwLock::new(Config::default())),
            watchers: Arc::new(RwLock::new(Vec::new())),
            validators: Arc::new(RwLock::new(Vec::new())),
            reload_callbacks: Arc::new(RwLock::new(Vec::new())),
        })
    }
    
    /// Load configuration from file
    pub async fn load_from_file<P: AsRef<Path>>(&self, path: P) -> Result<()> {
        let path = path.as_ref();
        info!("Loading configuration from: {:?}", path);
        
        let extension = path
            .extension()
            .and_then(|e| e.to_str())
            .ok_or_else(|| anyhow!("Invalid config file extension"))?;
        
        let content = fs::read_to_string(path)
            .map_err(|e| anyhow!("Failed to read config file: {}", e))?;
        
        let config = match extension {
            "toml" => self.load_toml(&content)?,
            "yaml" | "yml" => self.load_yaml(&content)?,
            "json" => self.load_json(&content)?,
            _ => return Err(anyhow!("Unsupported config format: {}", extension)),
        };
        
        // Validate configuration
        self.validate_config(&config).await?;
        
        // Store old config for callbacks
        let old_config = self.config.read().await.clone();
        
        // Update configuration
        *self.config.write().await = config;
        
        // Call reload callbacks
        self.call_reload_callbacks(&old_config, &self.config.read().await).await?;
        
        info!("Configuration loaded successfully");
        
        Ok(())
    }
    
    /// Load configuration from environment variables
    pub async fn load_from_env(&self) -> Result<()> {
        info!("Loading configuration from environment variables...");
        
        let mut config = self.config.read().await.clone();
        
        // Override with environment variables
        if let Ok(env) = std::env::var("SENTINEL_ENVIRONMENT") {
            config.core.environment = match env.to_lowercase().as_str() {
                "development" => Environment::Development,
                "staging" => Environment::Staging,
                "production" => Environment::Production,
                _ => return Err(anyhow!("Invalid environment: {}", env)),
            };
        }
        
        if let Ok(debug) = std::env::var("SENTINEL_DEBUG") {
            config.core.debug = debug.parse().unwrap_or(false);
        }
        
        if let Ok(level) = std::env::var("SENTINEL_LOG_LEVEL") {
            config.logging.level = match level.to_lowercase().as_str() {
                "trace" => LogLevel::Trace,
                "debug" => LogLevel::Debug,
                "info" => LogLevel::Info,
                "warn" => LogLevel::Warn,
                "error" => LogLevel::Error,
                _ => return Err(anyhow!("Invalid log level: {}", level)),
            };
        }
        
        // Validate configuration
        self.validate_config(&config).await?;
        
        *self.config.write().await = config;
        
        info!("Configuration loaded from environment variables");
        
        Ok(())
    }
    
    /// Get current configuration
    pub async fn get_config(&self) -> Config {
        self.config.read().await.clone()
    }
    
    /// Get specific configuration section
    pub async fn get_core_config(&self) -> CoreConfig {
        self.config.read().await.core.clone()
    }
    
    pub async fn get_ai_config(&self) -> AIConfig {
        self.config.read().await.ai.clone()
    }
    
    pub async fn get_gaming_config(&self) -> GamingConfig {
        self.config.read().await.gaming.clone()
    }
    
    pub async fn get_quantum_config(&self) -> QuantumConfig {
        self.config.read().await.quantum.clone()
    }
    
    pub async fn get_network_config(&self) -> NetworkConfig {
        self.config.read().await.network.clone()
    }
    
    pub async fn get_logging_config(&self) -> LoggingConfig {
        self.config.read().await.logging.clone()
    }
    
    pub async fn get_security_config(&self) -> SecurityConfig {
        self.config.read().await.security.clone()
    }
    
    pub async fn get_performance_config(&self) -> PerformanceConfig {
        self.config.read().await.performance.clone()
    }
    
    /// Add configuration validator
    pub async fn add_validator(&self, validator: Box<dyn ConfigValidator>) {
        self.validators.write().await.push(validator);
    }
    
    /// Add reload callback
    pub async fn add_reload_callback(&self, callback: Box<dyn ReloadCallback>) {
        self.reload_callbacks.write().await.push(callback);
    }
    
    /// Watch configuration file for changes
    pub async fn watch_file<P: AsRef<Path>>(&self, path: P) -> Result<()> {
        let path = path.as_ref().to_path_buf();
        info!("Watching configuration file: {:?}", path);
        
        let (tx, rx) = std::sync::mpsc::channel();
        let mut watcher = watcher(tx, Duration::from_secs(1))
            .map_err(|e| anyhow!("Failed to create watcher: {}", e))?;
        
        watcher.watch(&path, RecursiveMode::NonRecursive)
            .map_err(|e| anyhow!("Failed to watch file: {}", e))?;
        
        let config_manager = self.clone();
        tokio::spawn(async move {
            while let Ok(event) = rx.recv() {
                match event {
                    DebouncedEvent::Write(_) | DebouncedEvent::Create(_) => {
                        debug!("Configuration file changed, reloading...");
                        if let Err(e) = config_manager.load_from_file(&path).await {
                            error!("Failed to reload configuration: {}", e);
                        }
                    }
                    _ => {}
                }
            }
        });
        
        self.watchers.write().await.push(watcher);
        
        info!("Configuration file watcher started");
        
        Ok(())
    }
    
    /// Save configuration to file
    pub async fn save_to_file<P: AsRef<Path>>(&self, path: P) -> Result<()> {
        let path = path.as_ref();
        info!("Saving configuration to: {:?}", path);
        
        let config = self.config.read().await.clone();
        
        let extension = path
            .extension()
            .and_then(|e| e.to_str())
            .ok_or_else(|| anyhow!("Invalid config file extension"))?;
        
        let content = match extension {
            "toml" => self.save_toml(&config)?,
            "yaml" | "yml" => self.save_yaml(&config)?,
            "json" => self.save_json(&config)?,
            _ => return Err(anyhow!("Unsupported config format: {}", extension)),
        };
        
        fs::write(path, content)
            .map_err(|e| anyhow!("Failed to write config file: {}", e))?;
        
        info!("Configuration saved successfully");
        
        Ok(())
    }
    
    fn load_toml(&self, content: &str) -> Result<Config> {
        toml::from_str(content).map_err(|e| anyhow!("Failed to parse TOML: {}", e))
    }
    
    fn load_yaml(&self, content: &str) -> Result<Config> {
        serde_yaml::from_str(content).map_err(|e| anyhow!("Failed to parse YAML: {}", e))
    }
    
    fn load_json(&self, content: &str) -> Result<Config> {
        serde_json::from_str(content).map_err(|e| anyhow!("Failed to parse JSON: {}", e))
    }
    
    fn save_toml(&self, config: &Config) -> Result<String> {
        toml::to_string_pretty(config).map_err(|e| anyhow!("Failed to serialize TOML: {}", e))
    }
    
    fn save_yaml(&self, config: &Config) -> Result<String> {
        serde_yaml::to_string(config).map_err(|e| anyhow!("Failed to serialize YAML: {}", e))
    }
    
    fn save_json(&self, config: &Config) -> Result<String> {
        serde_json::to_string_pretty(config).map_err(|e| anyhow!("Failed to serialize JSON: {}", e))
    }
    
    async fn validate_config(&self, config: &Config) -> Result<()> {
        let validators = self.validators.read().await;
        
        for validator in validators.iter() {
            if let Err(e) = validator.validate(config) {
                return Err(anyhow!("Validation failed for '{}': {}", validator.name(), e));
            }
        }
        
        Ok(())
    }
    
    async fn call_reload_callbacks(&self, old_config: &Config, new_config: &Config) -> Result<()> {
        let callbacks = self.reload_callbacks.read().await;
        
        for callback in callbacks.iter() {
            if let Err(e) = callback.on_reload(old_config, new_config) {
                warn!("Reload callback '{}' failed: {}", callback.name(), e);
            }
        }
        
        Ok(())
    }
}

impl Clone for ConfigManager {
    fn clone(&self) -> Self {
        Self {
            config: Arc::clone(&self.config),
            watchers: Arc::clone(&self.watchers),
            validators: Arc::clone(&self.validators),
            reload_callbacks: Arc::clone(&self.reload_callbacks),
        }
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            core: CoreConfig::default(),
            ai: AIConfig::default(),
            gaming: GamingConfig::default(),
            quantum: QuantumConfig::default(),
            network: NetworkConfig::default(),
            logging: LoggingConfig::default(),
            security: SecurityConfig::default(),
            performance: PerformanceConfig::default(),
        }
    }
}

impl Default for CoreConfig {
    fn default() -> Self {
        Self {
            app_name: "SENTINEL".to_string(),
            version: "0.1.0".to_string(),
            environment: Environment::Development,
            debug: true,
            data_dir: PathBuf::from("/var/lib/sentinel"),
            cache_dir: PathBuf::from("/var/cache/sentinel"),
            log_dir: PathBuf::from("/var/log/sentinel"),
            max_concurrent_ops: 100,
            operation_timeout_secs: 30,
        }
    }
}

impl Default for AIConfig {
    fn default() -> Self {
        Self {
            model_path: PathBuf::from("/var/lib/sentinel/models"),
            model_type: "neural_network".to_string(),
            batch_size: 32,
            inference_threads: 4,
            enable_npu: false,
            enable_gpu: false,
            cache_size_mb: 1024,
            prediction_timeout_ms: 1000,
        }
    }
}

impl Default for GamingConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            auto_detect: true,
            zero_scan_mode: true,
            anti_ddos: true,
            ram_defolding: true,
            ai_overclocking: true,
            trusted_handshake: true,
            game_list: vec![],
        }
    }
}

impl Default for QuantumConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            kem_algorithm: "crystals_kyber768".to_string(),
            signature_algorithm: "crystals_dilithium3".to_string(),
            hybrid_mode: true,
            key_rotation_hours: 24,
        }
    }
}

impl Default for NetworkConfig {
    fn default() -> Self {
        Self {
            listen_address: "0.0.0.0".to_string(),
            listen_port: 8080,
            enable_tls: false,
            tls_cert_path: None,
            tls_key_path: None,
            max_connections: 1000,
            connection_timeout_secs: 30,
            enable_rate_limiting: true,
            rate_limit_rpm: 60,
            // PQC defaults
            enable_pqc: true,
            pqc_kem_algorithm: Some("CrystalsKyber768".to_string()),
            pqc_signature_algorithm: Some("CrystalsDilithium3".to_string()),
            pqc_hybrid_mode: true,
            pqc_fallback_to_classical: true,
            pqc_cert_path: None,
            pqc_key_path: None,
            pqc_min_security_level: 3,
        }
    }
}

impl Default for LoggingConfig {
    fn default() -> Self {
        Self {
            level: LogLevel::Info,
            format: LogFormat::Pretty,
            console: true,
            file: true,
            file_path: Some(PathBuf::from("/var/log/sentinel/sentinel.log")),
            max_file_size_mb: 100,
            max_files: 10,
            structured: false,
        }
    }
}

impl Default for SecurityConfig {
    fn default() -> Self {
        Self {
            enable_secure_boot: true,
            enable_immutable_partition: true,
            enable_hardware_attestation: true,
            enable_tpm: true,
            tpm_device_path: Some(PathBuf::from("/dev/tpm0")),
            enable_secure_enclave: false,
            audit_logging: true,
            audit_log_path: Some(PathBuf::from("/var/log/sentinel/audit.log")),
        }
    }
}

impl Default for PerformanceConfig {
    fn default() -> Self {
        Self {
            cpu_limit_percent: 80.0,
            memory_limit_mb: 2048,
            enable_profiling: false,
            profile_interval_secs: 60,
            enable_metrics: true,
            metrics_interval_secs: 30,
            enable_caching: true,
            cache_ttl_secs: 3600,
        }
    }
}

/// Initialize config module
pub fn init() -> Result<()> {
    info!("Configuration Module initialized");
    Ok(())
}

// Re-export PQC validator types
pub use pqc_validator::{
    PqcConfigValidator,
    PqcValidationResult,
    VALID_KEM_ALGORITHMS,
    VALID_SIGNATURE_ALGORITHMS,
    KEM_SECURITY_LEVELS,
    SIGNATURE_SECURITY_LEVELS,
};

#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test]
    async fn test_config_manager_creation() {
        let manager = ConfigManager::new().unwrap();
        let config = manager.get_config().await;
        assert_eq!(config.core.app_name, "SENTINEL");
    }
    
    #[tokio::test]
    async fn test_default_config() {
        let config = Config::default();
        assert_eq!(config.core.app_name, "SENTINEL");
        assert_eq!(config.core.environment, Environment::Development);
        assert_eq!(config.logging.level, LogLevel::Info);
    }
    
    #[tokio::test]
    async fn test_load_from_json() {
        let manager = ConfigManager::new().unwrap();
        
        let json_config = r#"{
            "core": {
                "app_name": "TestApp",
                "version": "1.0.0",
                "environment": "Production",
                "debug": false
            }
        }"#;
        
        let config = manager.load_json(json_config).unwrap();
        assert_eq!(config.core.app_name, "TestApp");
        assert_eq!(config.core.environment, Environment::Production);
    }
    
    #[tokio::test]
    async fn test_save_to_json() {
        let manager = ConfigManager::new().unwrap();
        let config = Config::default();
        
        let json = manager.save_json(&config).unwrap();
        assert!(json.contains("SENTINEL"));
    }
    
    #[tokio::test]
    async fn test_get_config_sections() {
        let manager = ConfigManager::new().unwrap();
        
        let core = manager.get_core_config().await;
        assert_eq!(core.app_name, "SENTINEL");
        
        let ai = manager.get_ai_config().await;
        assert_eq!(ai.model_type, "neural_network");
        
        let gaming = manager.get_gaming_config().await;
        assert!(gaming.enabled);
    }
}