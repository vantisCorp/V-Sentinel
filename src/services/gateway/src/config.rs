//! Gateway Configuration Module

use anyhow::{Result, anyhow};
use serde::{Serialize, Deserialize};
use std::path::PathBuf;
use std::collections::HashMap;

/// Gateway Configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GatewayConfig {
    /// Server address
    pub address: String,
    /// Server port
    pub port: u16,
    /// Gateway name
    pub name: String,
    /// Environment
    pub environment: Environment,
    /// PQC configuration
    pub pqc: super::pqc_gateway::GatewayPqcConfig,
    /// TLS configuration
    pub tls: Option<TlsConfig>,
    /// Rate limiting
    pub rate_limiting: RateLimitConfig,
    /// CORS configuration
    pub cors: CorsConfig,
    /// Upstream services
    pub upstreams: HashMap<String, UpstreamConfig>,
    /// Authentication
    pub auth: AuthConfig,
    /// Logging
    pub logging: LoggingConfig,
    /// Metrics
    pub metrics: MetricsConfig,
}

/// Environment
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Environment {
    Development,
    Staging,
    Production,
}

impl Default for GatewayConfig {
    fn default() -> Self {
        Self {
            address: "0.0.0.0".to_string(),
            port: 8080,
            name: "sentinel-gateway".to_string(),
            environment: Environment::Development,
            pqc: super::pqc_gateway::GatewayPqcConfig::default(),
            tls: None,
            rate_limiting: RateLimitConfig::default(),
            cors: CorsConfig::default(),
            upstreams: HashMap::new(),
            auth: AuthConfig::default(),
            logging: LoggingConfig::default(),
            metrics: MetricsConfig::default(),
        }
    }
}

/// TLS Configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TlsConfig {
    /// Enable TLS
    pub enabled: bool,
    /// Certificate path
    pub cert_path: PathBuf,
    /// Private key path
    pub key_path: PathBuf,
    /// CA certificate path (optional)
    pub ca_cert_path: Option<PathBuf>,
    /// Client authentication
    pub client_auth: Option<ClientAuthConfig>,
    /// Minimum TLS version
    pub min_tls_version: String,
    /// Cipher suites
    pub cipher_suites: Vec<String>,
}

/// Client Authentication Configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClientAuthConfig {
    /// Require client certificate
    pub required: bool,
    /// CA certificates
    pub ca_certs: Vec<PathBuf>,
}

/// Rate Limiting Configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RateLimitConfig {
    /// Enable rate limiting
    pub enabled: bool,
    /// Requests per second
    pub requests_per_second: u64,
    /// Burst size
    pub burst_size: u64,
}

impl Default for RateLimitConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            requests_per_second: 60,
            burst_size: 100,
        }
    }
}

/// CORS Configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CorsConfig {
    /// Enable CORS
    pub enabled: bool,
    /// Allowed origins
    pub allowed_origins: Vec<String>,
    /// Allowed methods
    pub allowed_methods: Vec<String>,
    /// Allowed headers
    pub allowed_headers: Vec<String>,
    /// Expose headers
    pub expose_headers: Vec<String>,
    /// Max age (seconds)
    pub max_age: Option<u64>,
    /// Allow credentials
    pub allow_credentials: bool,
}

impl Default for CorsConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            allowed_origins: vec!["*".to_string()],
            allowed_methods: vec!["GET".to_string(), "POST".to_string(), "PUT".to_string(), 
                                   "DELETE".to_string(), "OPTIONS".to_string()],
            allowed_headers: vec!["*".to_string()],
            expose_headers: vec![],
            max_age: Some(86400),
            allow_credentials: false,
        }
    }
}

/// Upstream Service Configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpstreamConfig {
    /// Service name
    pub name: String,
    /// Service URL
    pub url: String,
    /// Health check endpoint
    pub health_check: Option<String>,
    /// Timeout in seconds
    pub timeout_secs: u64,
    /// Maximum retries
    pub max_retries: u32,
    /// Circuit breaker threshold
    pub circuit_breaker_threshold: Option<u32>,
    /// Load balancing strategy
    pub load_balancing: LoadBalancingStrategy,
    /// Use PQC for upstream communication
    pub use_pqc: bool,
}

/// Load Balancing Strategy
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum LoadBalancingStrategy {
    RoundRobin,
    LeastConnections,
    Random,
    IpHash,
}

/// Authentication Configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthConfig {
    /// Enable authentication
    pub enabled: bool,
    /// Authentication method
    pub method: AuthMethod,
    /// JWT configuration
    pub jwt: Option<JwtConfig>,
    /// API key configuration
    pub api_keys: Option<ApiKeysConfig>,
    /// PQC authentication enabled
    pub enable_pqc_auth: bool,
}

impl Default for AuthConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            method: AuthMethod::None,
            jwt: None,
            api_keys: None,
            enable_pqc_auth: false,
        }
    }
}

/// Authentication Method
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AuthMethod {
    None,
    Jwt,
    ApiKey,
    OAuth2,
    PqcCertificate,
}

/// JWT Configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JwtConfig {
    /// Secret key
    pub secret: String,
    /// Issuer
    pub issuer: String,
    /// Audience
    pub audience: String,
    /// Token expiration (seconds)
    pub expiration_secs: u64,
}

/// API Keys Configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiKeysConfig {
    /// Header name
    pub header: String,
    /// Valid API keys
    pub keys: Vec<String>,
}

/// Logging Configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoggingConfig {
    /// Log level
    pub level: String,
    /// Log format
    pub format: LogFormat,
    /// Enable console logging
    pub console: bool,
    /// Enable file logging
    pub file: bool,
    /// Log file path
    pub file_path: Option<PathBuf>,
}

impl Default for LoggingConfig {
    fn default() -> Self {
        Self {
            level: "info".to_string(),
            format: LogFormat::Json,
            console: true,
            file: false,
            file_path: None,
        }
    }
}

/// Log Format
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum LogFormat {
    Json,
    Text,
    Pretty,
}

/// Metrics Configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetricsConfig {
    /// Enable metrics
    pub enabled: bool,
    /// Metrics endpoint
    pub endpoint: String,
    /// Include PQC metrics
    pub include_pqc_metrics: bool,
    /// Prometheus export
    pub prometheus: Option<PrometheusConfig>,
}

impl Default for MetricsConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            endpoint: "/metrics".to_string(),
            include_pqc_metrics: true,
            prometheus: None,
        }
    }
}

/// Prometheus Configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrometheusConfig {
    /// Enable Prometheus
    pub enabled: bool,
    /// Export endpoint
    pub endpoint: String,
    /// Namespace
    pub namespace: String,
}

impl GatewayConfig {
    /// Load configuration from file
    pub fn load_from_file(path: &str) -> Result<Self> {
        let content = std::fs::read_to_string(path)?;
        
        if path.ends_with(".toml") {
            self::load_from_toml(&content)
        } else if path.ends_with(".json") {
            self::load_from_json(&content)
        } else {
            Err(anyhow!("Unsupported config format"))
        }
    }
    
    /// Load configuration from TOML
    pub fn load_from_toml(content: &str) -> Result<Self> {
        toml::from_str(content).map_err(|e| anyhow!("Failed to parse TOML: {}", e))
    }
    
    /// Load configuration from JSON
    pub fn load_from_json(content: &str) -> Result<Self> {
        serde_json::from_str(content).map_err(|e| anyhow!("Failed to parse JSON: {}", e))
    }
    
    /// Validate configuration
    pub fn validate(&self) -> Result<()> {
        // Validate PQC configuration
        if self.pqc.enable_pqc {
            let kem_level = self.pqc.kem_algorithm.security_level();
            let sig_level = self.pqc.signature_algorithm.security_level();
            
            if kem_level < self.pqc.min_security_level {
                return Err(anyhow!(
                    "KEM algorithm security level ({}) is below minimum ({})",
                    kem_level, self.pqc.min_security_level
                ));
            }
        }
        
        // Validate TLS configuration
        if let Some(ref tls) = self.tls {
            if tls.enabled {
                if !tls.cert_path.exists() {
                    return Err(anyhow!("TLS certificate file not found: {:?}", tls.cert_path));
                }
                if !tls.key_path.exists() {
                    return Err(anyhow!("TLS key file not found: {:?}", tls.key_path));
                }
            }
        }
        
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_default_config() {
        let config = GatewayConfig::default();
        assert_eq!(config.port, 8080);
        assert_eq!(config.environment, Environment::Development);
    }
    
    #[test]
    fn test_config_validation() {
        let config = GatewayConfig::default();
        let result = config.validate();
        assert!(result.is_ok());
    }
}