//! Enhanced Security Hardening Measures
//! 
//! This module provides comprehensive security hardening capabilities:
//! - Input validation and sanitization
//! - Output encoding
//! - SQL injection prevention
//! - XSS prevention
//! - CSRF protection
//! - Security headers
//! - Rate limiting
//! - IP whitelisting/blacklisting

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::sync::RwLock;
use tracing::{debug, error, info, warn};

/// Security level
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum SecurityLevel {
    Low,
    Medium,
    High,
    Critical,
}

/// Input validation rule
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationRule {
    pub rule_id: String,
    pub name: String,
    pub pattern: String,
    pub description: String,
    pub severity: SecurityLevel,
}

/// Sanitization result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SanitizationResult {
    pub original: String,
    pub sanitized: String,
    pub violations: Vec<SecurityViolation>,
    pub is_safe: bool,
}

/// Security violation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityViolation {
    pub violation_id: String,
    pub violation_type: ViolationType,
    pub severity: SecurityLevel,
    pub description: String,
    pub location: String,
    pub recommendation: String,
}

/// Violation types
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ViolationType {
    SQLInjection,
    XSS,
    CSRF,
    CommandInjection,
    PathTraversal,
    XXE,
    SSRF,
    InsecureDeserialization,
    WeakCryptography,
    InformationDisclosure,
}

/// Security headers
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityHeaders {
    pub content_security_policy: Option<String>,
    pub x_frame_options: Option<String>,
    pub x_content_type_options: Option<String>,
    pub x_xss_protection: Option<String>,
    pub strict_transport_security: Option<String>,
    pub referrer_policy: Option<String>,
    pub permissions_policy: Option<String>,
}

/// Rate limit rule
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RateLimitRule {
    pub rule_id: String,
    pub identifier: String,
    pub max_requests: u64,
    pub window: Duration,
    pub block_duration: Option<Duration>,
}

/// IP list type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum IPListType {
    Whitelist,
    Blacklist,
}

/// Enhanced Security Hardening Manager
pub struct EnhancedSecurityHardeningManager {
    validation_rules: Arc<RwLock<Vec<ValidationRule>>>,
    ip_whitelist: Arc<RwLock<HashSet<String>>>,
    ip_blacklist: Arc<RwLock<HashSet<String>>>,
    rate_limits: Arc<RwLock<HashMap<String, RateLimitRule>>>,
    rate_limit_counters: Arc<RwLock<HashMap<String, (u64, Instant)>>>,
    security_headers: Arc<RwLock<SecurityHeaders>>,
    statistics: Arc<RwLock<HardeningStatistics>>,
}

/// Hardening statistics
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct HardeningStatistics {
    pub total_validations: u64,
    pub violations_detected: u64,
    pub inputs_sanitized: u64,
    pub sql_injection_attempts: u64,
    pub xss_attempts: u64,
    pub csrf_attempts: u64,
    pub rate_limit_blocks: u64,
    pub ip_whitelist_blocks: u64,
    pub ip_blacklist_blocks: u64,
}

impl EnhancedSecurityHardeningManager {
    /// Create a new enhanced security hardening manager
    pub fn new() -> Self {
        Self {
            validation_rules: Arc::new(RwLock::new(Vec::new())),
            ip_whitelist: Arc::new(RwLock::new(HashSet::new())),
            ip_blacklist: Arc::new(RwLock::new(HashSet::new())),
            rate_limits: Arc::new(RwLock::new(HashMap::new())),
            rate_limit_counters: Arc::new(RwLock::new(HashMap::new())),
            security_headers: Arc::new(RwLock::new(SecurityHeaders::default())),
            statistics: Arc::new(RwLock::new(HardeningStatistics::default())),
        }
    }

    /// Initialize the enhanced security hardening manager
    pub async fn initialize(&self) -> Result<()> {
        info!("Initializing Enhanced Security Hardening Manager");
        
        // Load default validation rules
        self.load_default_validation_rules().await?;
        
        // Start rate limit cleanup
        self.start_rate_limit_cleanup().await?;
        
        info!("Enhanced Security Hardening Manager initialized successfully");
        Ok(())
    }

    /// Validate input
    pub async fn validate_input(&self, input: &str, input_type: &str) -> Result<Vec<SecurityViolation>> {
        let mut violations = Vec::new();
        
        // Check against validation rules
        let rules = self.validation_rules.read().await;
        for rule in rules.iter() {
            if self.matches_pattern(input, &rule.pattern) {
                violations.push(SecurityViolation {
                    violation_id: uuid::Uuid::new_v4().to_string(),
                    violation_type: self.determine_violation_type(&rule.name),
                    severity: rule.severity,
                    description: rule.description.clone(),
                    location: input_type.to_string(),
                    recommendation: format!("Input violates rule: {}", rule.name),
                });
            }
        }
        
        // Update statistics
        {
            let mut stats = self.statistics.write().await;
            stats.total_validations += 1;
            stats.violations_detected += violations.len() as u64;
        }
        
        Ok(violations)
    }

    /// Sanitize input
    pub async fn sanitize_input(&self, input: &str) -> Result<SanitizationResult> {
        let violations = self.validate_input(input, "input").await?;
        
        let mut sanitized = input.to_string();
        
        // Remove dangerous characters
        sanitized = sanitized.replace("'", "''"); // SQL injection prevention
        sanitized = sanitized.replace("<", "&lt;"); // XSS prevention
        sanitized = sanitized.replace(">", "&gt;");
        sanitized = sanitized.replace("&quot;", "&quot;");
        sanitized = sanitized.replace("&", "&amp;");
        
        // Remove path traversal attempts
        sanitized = sanitized.replace("../", "");
        sanitized = sanitized.replace("..\&quot;, "");
        
        let is_safe = violations.is_empty();
        
        // Update statistics
        {
            let mut stats = self.statistics.write().await;
            stats.inputs_sanitized += 1;
        }
        
        Ok(SanitizationResult {
            original: input.to_string(),
            sanitized,
            violations,
            is_safe,
        })
    }

    /// Check SQL injection
    pub async fn check_sql_injection(&self, input: &str) -> Result<bool> {
        let sql_patterns = vec![
            "' OR '1'='1",
            "' OR 1=1",
            "'; DROP TABLE",
            "UNION SELECT",
            "1; EXEC",
            "xp_cmdshell",
            "sp_executesql",
        ];
        
        let input_upper = input.to_uppercase();
        for pattern in sql_patterns {
            if input_upper.contains(&pattern.to_uppercase()) {
                warn!("SQL injection attempt detected: {}", input);
                
                let mut stats = self.statistics.write().await;
                stats.sql_injection_attempts += 1;
                
                return Ok(true);
            }
        }
        
        Ok(false)
    }

    /// Check XSS
    pub async fn check_xss(&self, input: &str) -> Result<bool> {
        let xss_patterns = vec![
            "<script>",
            "javascript:",
            "onerror=",
            "onload=",
            "onmouseover=",
            "eval(",
            "document.cookie",
            "alert(",
            "fromCharCode",
        ];
        
        let input_lower = input.to_lowercase();
        for pattern in xss_patterns {
            if input_lower.contains(&pattern.to_lowercase()) {
                warn!("XSS attempt detected: {}", input);
                
                let mut stats = self.statistics.write().await;
                stats.xss_attempts += 1;
                
                return Ok(true);
            }
        }
        
        Ok(false)
    }

    /// Generate CSRF token
    pub async fn generate_csrf_token(&self) -> Result<String> {
        let token = format!("csrf_{}", uuid::Uuid::new_v4());
        Ok(token)
    }

    /// Validate CSRF token
    pub async fn validate_csrf_token(&self, token: &str) -> Result<bool> {
        // In a real implementation, this would validate against stored tokens
        let is_valid = token.starts_with("csrf_") && token.len() > 10;
        
        if !is_valid {
            warn!("CSRF token validation failed: {}", token);
            
            let mut stats = self.statistics.write().await;
            stats.csrf_attempts += 1;
        }
        
        Ok(is_valid)
    }

    /// Get security headers
    pub async fn get_security_headers(&self) -> SecurityHeaders {
        self.security_headers.read().await.clone()
    }

    /// Set security headers
    pub async fn set_security_headers(&self, headers: SecurityHeaders) {
        let mut security_headers = self.security_headers.write().await;
        *security_headers = headers;
    }

    /// Add IP to whitelist
    pub async fn add_ip_to_whitelist(&self, ip: String) -> Result<()> {
        let mut whitelist = self.ip_whitelist.write().await;
        whitelist.insert(ip);
        
        info!("Added IP to whitelist: {}", ip);
        Ok(())
    }

    /// Add IP to blacklist
    pub async fn add_ip_to_blacklist(&self, ip: String) -> Result<()> {
        let mut blacklist = self.ip_blacklist.write().await;
        blacklist.insert(ip);
        
        info!("Added IP to blacklist: {}", ip);
        Ok(())
    }

    /// Check IP access
    pub async fn check_ip_access(&self, ip: &str) -> Result<bool> {
        let whitelist = self.ip_whitelist.read().await;
        let blacklist = self.ip_blacklist.read().await;
        
        // Check blacklist first
        if blacklist.contains(ip) {
            warn!("IP blocked (blacklist): {}", ip);
            
            let mut stats = self.statistics.write().await;
            stats.ip_blacklist_blocks += 1;
            
            return Ok(false);
        }
        
        // Check whitelist
        if !whitelist.is_empty() && !whitelist.contains(ip) {
            warn!("IP blocked (not in whitelist): {}", ip);
            
            let mut stats = self.statistics.write().await;
            stats.ip_whitelist_blocks += 1;
            
            return Ok(false);
        }
        
        Ok(true)
    }

    /// Add rate limit rule
    pub async fn add_rate_limit_rule(&self, rule: RateLimitRule) -> Result<()> {
        let mut rate_limits = self.rate_limits.write().await;
        rate_limits.insert(rule.identifier.clone(), rule);
        
        info!("Added rate limit rule: {}", rule.identifier);
        Ok(())
    }

    /// Check rate limit
    pub async fn check_rate_limit(&self, identifier: &str) -> Result<bool> {
        let rate_limits = self.rate_limits.read().await;
        
        if let Some(rule) = rate_limits.get(identifier) {
            let mut counters = self.rate_limit_counters.write().await;
            let now = Instant::now();
            
            // Get or create counter
            let (count, last_reset) = counters.entry(identifier.to_string())
                .or_insert((0, now));
            
            // Reset if window has passed
            if now.duration_since(*last_reset)? > rule.window {
                *count = 0;
                *last_reset = now;
            }
            
            // Check if limit exceeded
            if *count >= rule.max_requests {
                warn!("Rate limit exceeded for: {}", identifier);
                
                let mut stats = self.statistics.write().await;
                stats.rate_limit_blocks += 1;
                
                return Ok(false);
            }
            
            // Increment counter
            *count += 1;
        }
        
        Ok(true)
    }

    /// Get hardening statistics
    pub async fn get_statistics(&self) -> HardeningStatistics {
        self.statistics.read().await.clone()
    }

    /// Load default validation rules
    async fn load_default_validation_rules(&self) -> Result<()> {
        let rules = vec![
            ValidationRule {
                rule_id: "sql-injection-1".to_string(),
                name: "SQL Injection Pattern 1".to_string(),
                pattern: "' OR '1'='1".to_string(),
                description: "Common SQL injection pattern".to_string(),
                severity: SecurityLevel::Critical,
            },
            ValidationRule {
                rule_id: "xss-1".to_string(),
                name: "XSS Pattern 1".to_string(),
                pattern: "<script".to_string(),
                description: "Script tag injection".to_string(),
                severity: SecurityLevel::Critical,
            },
            ValidationRule {
                rule_id: "path-traversal-1".to_string(),
                name: "Path Traversal Pattern 1".to_string(),
                pattern: "../".to_string(),
                description: "Directory traversal attempt".to_string(),
                severity: SecurityLevel::High,
            },
        ];
        
        let mut rules_store = self.validation_rules.write().await;
        *rules_store = rules;
        
        info!("Loaded {} default validation rules", rules.len());
        Ok(())
    }

    /// Match pattern
    fn matches_pattern(&self, input: &str, pattern: &str) -> bool {
        input.contains(pattern)
    }

    /// Determine violation type
    fn determine_violation_type(&self, rule_name: &str) -> ViolationType {
        if rule_name.contains("SQL") {
            ViolationType::SQLInjection
        } else if rule_name.contains("XSS") {
            ViolationType::XSS
        } else if rule_name.contains("Path") {
            ViolationType::PathTraversal
        } else {
            ViolationType::InformationDisclosure
        }
    }

    /// Start rate limit cleanup task
    async fn start_rate_limit_cleanup(&self) -> Result<()> {
        let rate_limits = Arc::clone(&self.rate_limits);
        let rate_limit_counters = Arc::clone(&self.rate_limit_counters);
        
        tokio::spawn(async move {
            let mut timer = interval(Duration::from_secs(60));
            
            loop {
                timer.tick().await;
                
                let limits = rate_limits.read().await;
                let mut counters = rate_limit_counters.write().await;
                let now = Instant::now();
                
                for (identifier, rule) in limits.iter() {
                    if let Some((_, last_reset)) = counters.get_mut(identifier) {
                        if now.duration_since(*last_reset).unwrap_or(Duration::ZERO) > rule.window {
                            counters.remove(identifier);
                        }
                    }
                }
            }
        });
        
        Ok(())
    }
}

impl Default for SecurityHeaders {
    fn default() -> Self {
        Self {
            content_security_policy: Some("default-src 'self'".to_string()),
            x_frame_options: Some("DENY".to_string()),
            x_content_type_options: Some("nosniff".to_string()),
            x_xss_protection: Some("1; mode=block".to_string()),
            strict_transport_security: Some("max-age=31536000; includeSubDomains".to_string()),
            referrer_policy: Some("strict-origin-when-cross-origin".to_string()),
            permissions_policy: Some("geolocation=(), microphone=()".to_string()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_hardening_initialization() {
        let manager = EnhancedSecurityHardeningManager::new();
        assert!(manager.initialize().await.is_ok());
    }

    #[tokio::test]
    async fn test_validate_input() {
        let manager = EnhancedSecurityHardeningManager::new();
        manager.initialize().await.unwrap();
        
        let violations = manager.validate_input("normal input", "test").await.unwrap();
        assert_eq!(violations.len(), 0);
    }

    #[tokio::test]
    async fn test_sanitize_input() {
        let manager = EnhancedSecurityHardeningManager::new();
        manager.initialize().await.unwrap();
        
        let result = manager.sanitize_input("<script>alert('xss')</script>").await.unwrap();
        assert!(!result.sanitized.contains("<script>"));
        assert!(!result.is_safe);
    }

    #[tokio::test]
    async fn test_check_sql_injection() {
        let manager = EnhancedSecurityHardeningManager::new();
        manager.initialize().await.unwrap();
        
        let is_injection = manager.check_sql_injection("SELECT * FROM users WHERE id = '1' OR '1'='1'").await.unwrap();
        assert!(is_injection);
    }

    #[tokio::test]
    async fn test_check_xss() {
        let manager = EnhancedSecurityHardeningManager::new();
        manager.initialize().await.unwrap();
        
        let is_xss = manager.check_xss("<script>alert('xss')</script>").await.unwrap();
        assert!(is_xss);
    }

    #[tokio::test]
    async fn test_csrf_token() {
        let manager = EnhancedSecurityHardeningManager::new();
        manager.initialize().await.unwrap();
        
        let token = manager.generate_csrf_token().await.unwrap();
        assert!(token.starts_with("csrf_"));
        
        let is_valid = manager.validate_csrf_token(&token).await.unwrap();
        assert!(is_valid);
    }

    #[tokio::test]
    async fn test_ip_access() {
        let manager = EnhancedSecurityHardeningManager::new();
        manager.initialize().await.unwrap();
        
        manager.add_ip_to_whitelist("192.168.1.1".to_string()).await.unwrap();
        
        let allowed = manager.check_ip_access("192.168.1.1").await.unwrap();
        assert!(allowed);
        
        let blocked = manager.check_ip_access("192.168.1.2").await.unwrap();
        assert!(!blocked);
    }

    #[tokio::test]
    async fn test_rate_limit() {
        let manager = EnhancedSecurityHardeningManager::new();
        manager.initialize().await.unwrap();
        
        let rule = RateLimitRule {
            rule_id: "test-rule".to_string(),
            identifier: "test".to_string(),
            max_requests: 5,
            window: Duration::from_secs(60),
            block_duration: None,
        };
        
        manager.add_rate_limit_rule(rule).await.unwrap();
        
        for _ in 0..5 {
            assert!(manager.check_rate_limit("test").await.unwrap());
        }
        
        assert!(!manager.check_rate_limit("test").await.unwrap());
    }
}