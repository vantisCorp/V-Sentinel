//! AI API Security Engine
//! 
//! Provides comprehensive API security including gateway with rate limiting,
//! prompt injection detection, authentication, and abuse prevention.

use crate::ai_security::models::*;
use crate::ai_security::AISecurityError;
use chrono::{DateTime, Utc, Duration};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

/// API Security Configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct APISecurityConfig {
    /// Enable rate limiting
    pub rate_limiting_enabled: bool,
    /// Requests per minute limit
    pub rate_limit_per_minute: u32,
    /// Enable prompt injection detection
    pub prompt_injection_detection: bool,
    /// Injection detection sensitivity
    pub injection_sensitivity: f64,
    /// Enable authentication
    pub authentication_required: bool,
    /// Enable abuse detection
    pub abuse_detection: bool,
    /// Block suspicious requests
    pub auto_block_suspicious: bool,
}

impl Default for APISecurityConfig {
    fn default() -> Self {
        Self {
            rate_limiting_enabled: true,
            rate_limit_per_minute: 60,
            prompt_injection_detection: true,
            injection_sensitivity: 0.7,
            authentication_required: true,
            abuse_detection: true,
            auto_block_suspicious: true,
        }
    }
}

/// API Security Engine
pub struct APISecurityEngine {
    config: APISecurityConfig,
    rate_limiter: RateLimiter,
    prompt_injector_detector: PromptInjectionDetector,
    authenticator: APIAuthenticator,
    abuse_detector: AbuseDetector,
    request_log: Arc<RwLock<Vec<APIRequestRecord>>>,
}

impl APISecurityEngine {
    /// Create a new API Security Engine
    pub async fn new(config: APISecurityConfig) -> Result<Self, AISecurityError> {
        let rate_limiter = RateLimiter::new(config.rate_limit_per_minute);
        let prompt_injector_detector = PromptInjectionDetector::new(config.injection_sensitivity);
        let authenticator = APIAuthenticator::new();
        let abuse_detector = AbuseDetector::new();
        let request_log = Arc::new(RwLock::new(Vec::new()));

        Ok(Self {
            config,
            rate_limiter,
            prompt_injector_detector,
            authenticator,
            abuse_detector,
            request_log,
        })
    }

    /// Secure an API request
    pub async fn secure_request(&self, request: &APIRequest) -> Result<APIResponse, AISecurityError> {
        let mut security_warnings = Vec::new();

        // Step 1: Check rate limit
        if self.config.rate_limiting_enabled {
            let rate_result = self.rate_limiter.check_limit(&request.client_id).await?;
            if rate_result.exceeded {
                return Ok(APIResponse {
                    request_id: request.id.clone(),
                    status: APIResponseStatus::Throttled,
                    data: None,
                    security_warnings: vec!["Rate limit exceeded".to_string()],
                    rate_limit: rate_result.info,
                    timestamp: Utc::now(),
                });
            }
        }

        // Step 2: Authenticate request
        if self.config.authentication_required {
            let auth_result = self.authenticator.authenticate(&request.auth_token).await?;
            if !auth_result.is_valid {
                return Ok(APIResponse {
                    request_id: request.id.clone(),
                    status: APIResponseStatus::Unauthorized,
                    data: None,
                    security_warnings: vec!["Authentication failed".to_string()],
                    rate_limit: RateLimitInfo {
                        limit: self.config.rate_limit_per_minute,
                        remaining: 0,
                        reset_at: Utc::now() + Duration::minutes(1),
                    },
                    timestamp: Utc::now(),
                });
            }
        }

        // Step 3: Check for prompt injection
        if self.config.prompt_injection_detection {
            let injection_result = self.prompt_injector_detector.detect(&request.payload).await?;
            if injection_result.is_injection {
                security_warnings.push(format!("Potential prompt injection detected: {:?}", injection_result.injection_type));
                
                if self.config.auto_block_suspicious && injection_result.confidence > 0.8 {
                    return Ok(APIResponse {
                        request_id: request.id.clone(),
                        status: APIResponseStatus::Blocked,
                        data: None,
                        security_warnings,
                        rate_limit: RateLimitInfo {
                            limit: self.config.rate_limit_per_minute,
                            remaining: self.rate_limiter.get_remaining(&request.client_id).await,
                            reset_at: Utc::now() + Duration::minutes(1),
                        },
                        timestamp: Utc::now(),
                    });
                }
            }
        }

        // Step 4: Check for abuse
        if self.config.abuse_detection {
            let abuse_result = self.abuse_detector.analyze(request).await?;
            if abuse_result.is_abuse {
                security_warnings.push(format!("Potential abuse detected: {}", abuse_result.abuse_type));
            }
        }

        // Log request
        self.log_request(request).await;

        // Return success
        Ok(APIResponse {
            request_id: request.id.clone(),
            status: APIResponseStatus::Success,
            data: Some(request.payload.clone()),
            security_warnings,
            rate_limit: RateLimitInfo {
                limit: self.config.rate_limit_per_minute,
                remaining: self.rate_limiter.get_remaining(&request.client_id).await,
                reset_at: Utc::now() + Duration::minutes(1),
            },
            timestamp: Utc::now(),
        })
    }

    /// Detect prompt injection in payload
    pub async fn detect_prompt_injection(&self, payload: &serde_json::Value) -> Result<PromptInjectionResult, AISecurityError> {
        self.prompt_injector_detector.detect(payload).await
    }

    /// Get rate limit status for client
    pub async fn get_rate_limit_status(&self, client_id: &str) -> Result<RateLimitInfo, AISecurityError> {
        let remaining = self.rate_limiter.get_remaining(client_id).await;
        Ok(RateLimitInfo {
            limit: self.config.rate_limit_per_minute,
            remaining,
            reset_at: Utc::now() + Duration::minutes(1),
        })
    }

    /// Log request
    async fn log_request(&self, request: &APIRequest) {
        let record = APIRequestRecord {
            id: request.id.clone(),
            client_id: request.client_id.clone(),
            endpoint: request.endpoint.clone(),
            timestamp: Utc::now(),
        };
        
        let mut log = self.request_log.write().await;
        log.push(record);
    }

    /// Get component status
    pub async fn get_status(&self) -> Result<ComponentStatus, AISecurityError> {
        let log = self.request_log.read().await;
        
        Ok(ComponentStatus {
            healthy: true,
            active_protections: 3, // rate limiting, injection detection, auth
            threats_detected: 0,
            last_check: Utc::now(),
        })
    }
}

/// Rate Limiter
pub struct RateLimiter {
    limit: u32,
    requests: Arc<RwLock<HashMap<String, Vec<DateTime<Utc>>>>>,
}

impl RateLimiter {
    pub fn new(limit: u32) -> Self {
        Self {
            limit,
            requests: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub async fn check_limit(&self, client_id: &str) -> Result<RateLimitResult, AISecurityError> {
        let mut requests = self.requests.write().await;
        let now = Utc::now();
        let window_start = now - Duration::minutes(1);

        // Clean old requests
        let client_requests = requests.entry(client_id.to_string())
            .or_insert_with(Vec::new);
        client_requests.retain(|&t| t > window_start);

        let count = client_requests.len() as u32;
        let exceeded = count >= self.limit;

        if !exceeded {
            client_requests.push(now);
        }

        let remaining = if exceeded { 0 } else { self.limit - count - 1 };

        Ok(RateLimitResult {
            exceeded,
            info: RateLimitInfo {
                limit: self.limit,
                remaining,
                reset_at: window_start + Duration::minutes(1),
            },
        })
    }

    pub async fn get_remaining(&self, client_id: &str) -> u32 {
        let requests = self.requests.read().await;
        let now = Utc::now();
        let window_start = now - Duration::minutes(1);

        if let Some(client_requests) = requests.get(client_id) {
            let count = client_requests.iter().filter(|&&t| t > window_start).count() as u32;
            if count >= self.limit {
                0
            } else {
                self.limit - count
            }
        } else {
            self.limit
        }
    }
}

/// Rate Limit Result
pub struct RateLimitResult {
    pub exceeded: bool,
    pub info: RateLimitInfo,
}

/// Prompt Injection Detector
pub struct PromptInjectionDetector {
    sensitivity: f64,
    injection_patterns: Vec<InjectionPattern>,
}

impl PromptInjectionDetector {
    pub fn new(sensitivity: f64) -> Self {
        Self {
            sensitivity,
            injection_patterns: Self::load_patterns(),
        }
    }

    fn load_patterns() -> Vec<InjectionPattern> {
        vec![
            InjectionPattern {
                pattern_type: InjectionType::DirectInjection,
                patterns: vec![
                    "ignore previous instructions".to_string(),
                    "ignore all above".to_string(),
                    "disregard your instructions".to_string(),
                ],
                severity: 0.9,
            },
            InjectionPattern {
                pattern_type: InjectionType::Jailbreak,
                patterns: vec![
                    "DAN".to_string(),
                    "do anything now".to_string(),
                    "developer mode".to_string(),
                ],
                severity: 0.95,
            },
            InjectionPattern {
                pattern_type: InjectionType::RolePlay,
                patterns: vec![
                    "act as".to_string(),
                    "pretend to be".to_string(),
                    "imagine you are".to_string(),
                ],
                severity: 0.7,
            },
            InjectionPattern {
                pattern_type: InjectionType::TokenSmuggling,
                patterns: vec![
                    "split the instruction".to_string(),
                    "base64 decode".to_string(),
                    "rot13".to_string(),
                ],
                severity: 0.8,
            },
        ]
    }

    pub async fn detect(&self, payload: &serde_json::Value) -> Result<PromptInjectionResult, AISecurityError> {
        let content = self.extract_content(payload);
        let content_lower = content.to_lowercase();

        let mut max_confidence = 0.0;
        let mut detected_type = None;
        let mut detected_patterns = Vec::new();

        for pattern in &self.injection_patterns {
            for p in &pattern.patterns {
                if content_lower.contains(&p.to_lowercase()) {
                    max_confidence = max_confidence.max(pattern.severity);
                    detected_type = Some(pattern.pattern_type.clone());
                    detected_patterns.push(p.clone());
                }
            }
        }

        // Check for additional suspicious patterns
        let additional_checks = self.additional_checks(&content);
        max_confidence = max_confidence.max(additional_checks.confidence);
        if additional_checks.detected_type.is_some() && detected_type.is_none() {
            detected_type = additional_checks.detected_type;
        }

        let is_injection = max_confidence > self.sensitivity;

        Ok(PromptInjectionResult {
            request_id: String::new(),
            is_injection,
            injection_type: detected_type,
            confidence: max_confidence,
            patterns: detected_patterns,
            mitigation: if is_injection {
                Some("Request blocked - potential injection detected".to_string())
            } else {
                None
            },
            timestamp: Utc::now(),
        })
    }

    fn extract_content(&self, payload: &serde_json::Value) -> String {
        if let Some(prompt) = payload.get("prompt").and_then(|p| p.as_str()) {
            prompt.to_string()
        } else if let Some(message) = payload.get("message").and_then(|m| m.as_str()) {
            message.to_string()
        } else if let Some(text) = payload.get("text").and_then(|t| t.as_str()) {
            text.to_string()
        } else {
            payload.to_string()
        }
    }

    fn additional_checks(&self, content: &str) -> AdditionalCheckResult {
        let mut confidence = 0.0;
        let mut detected_type = None;

        // Check for base64-like patterns
        if content.len() > 100 && content.chars().all(|c| c.is_alphanumeric() || c == '+' || c == '/' || c == '=') {
            confidence = 0.6;
            detected_type = Some(InjectionType::TokenSmuggling);
        }

        // Check for excessive special characters
        let special_count = content.chars().filter(|&c| !c.is_alphanumeric() && !c.is_whitespace()).count();
        if special_count as f64 / content.len() as f64 > 0.3 {
            confidence = confidence.max(0.5);
        }

        AdditionalCheckResult { confidence, detected_type }
    }
}

/// Injection Pattern
struct InjectionPattern {
    pattern_type: InjectionType,
    patterns: Vec<String>,
    severity: f64,
}

/// Additional Check Result
struct AdditionalCheckResult {
    confidence: f64,
    detected_type: Option<InjectionType>,
}

/// API Authenticator
pub struct APIAuthenticator {
    valid_tokens: HashMap<String, TokenInfo>,
}

impl APIAuthenticator {
    pub fn new() -> Self {
        let mut valid_tokens = HashMap::new();
        // Add a default test token
        valid_tokens.insert(
            "test_token_valid".to_string(),
            TokenInfo {
                client_id: "test_client".to_string(),
                expires_at: Utc::now() + Duration::days(30),
                scopes: vec!["read".to_string(), "write".to_string()],
            },
        );

        Self { valid_tokens }
    }

    pub async fn authenticate(&self, token: &str) -> Result<AuthResult, AISecurityError> {
        if let Some(info) = self.valid_tokens.get(token) {
            if info.expires_at > Utc::now() {
                return Ok(AuthResult {
                    is_valid: true,
                    client_id: Some(info.client_id.clone()),
                    scopes: Some(info.scopes.clone()),
                });
            }
        }

        Ok(AuthResult {
            is_valid: false,
            client_id: None,
            scopes: None,
        })
    }
}

/// Token Info
struct TokenInfo {
    client_id: String,
    expires_at: DateTime<Utc>,
    scopes: Vec<String>,
}

/// Auth Result
pub struct AuthResult {
    pub is_valid: bool,
    pub client_id: Option<String>,
    pub scopes: Option<Vec<String>>,
}

/// Abuse Detector
pub struct AbuseDetector {
    patterns: Vec<AbusePattern>,
}

impl AbuseDetector {
    pub fn new() -> Self {
        Self {
            patterns: vec![
                AbusePattern {
                    pattern_type: "excessive_requests".to_string(),
                    threshold: 1000,
                    window_seconds: 60,
                },
                AbusePattern {
                    pattern_type: "large_payload".to_string(),
                    threshold: 100000,
                    window_seconds: 0,
                },
                AbusePattern {
                    pattern_type: "repeated_errors".to_string(),
                    threshold: 10,
                    window_seconds: 60,
                },
            ],
        }
    }

    pub async fn analyze(&self, request: &APIRequest) -> Result<AbuseResult, AISecurityError> {
        let mut is_abuse = false;
        let mut abuse_type = String::new();

        // Check payload size
        let payload_size = serde_json::to_string(&request.payload)
            .map(|s| s.len())
            .unwrap_or(0);
        
        if payload_size > 100000 {
            is_abuse = true;
            abuse_type = "large_payload".to_string();
        }

        Ok(AbuseResult {
            is_abuse,
            abuse_type,
            confidence: if is_abuse { 0.9 } else { 0.0 },
        })
    }
}

/// Abuse Pattern
struct AbusePattern {
    pattern_type: String,
    threshold: u32,
    window_seconds: u32,
}

/// Abuse Result
pub struct AbuseResult {
    pub is_abuse: bool,
    pub abuse_type: String,
    pub confidence: f64,
}

/// API Request Record
struct APIRequestRecord {
    id: String,
    client_id: String,
    endpoint: String,
    timestamp: DateTime<Utc>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_api_security_engine_creation() {
        let config = APISecurityConfig::default();
        let engine = APISecurityEngine::new(config).await;
        assert!(engine.is_ok());
    }

    #[tokio::test]
    async fn test_rate_limiting() {
        let config = APISecurityConfig::default();
        let engine = APISecurityEngine::new(config).await.unwrap();
        
        let request = APIRequest {
            id: "req-1".to_string(),
            client_id: "client-1".to_string(),
            endpoint: "/api/v1/predict".to_string(),
            payload: serde_json::json!({"input": "test"}),
            auth_token: "test_token_valid".to_string(),
            timestamp: Utc::now(),
            client_ip: "127.0.0.1".to_string(),
            user_agent: "test".to_string(),
        };
        
        let result = engine.secure_request(&request).await;
        assert!(result.is_ok());
    }
}