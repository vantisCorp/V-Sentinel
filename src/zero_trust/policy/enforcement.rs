//! Policy Enforcement Points (PEP) for Zero Trust
//!
//! Provides enforcement points for API gateway, service mesh, and database access.

use super::super::{ZeroTrustContext, AccessResult, AccessDecision};
use super::{PolicyEngine, Policy};
use std::sync::Arc;
use tokio::sync::RwLock;
use std::collections::HashMap;
use std::time::Duration;
use chrono::{DateTime, Utc};

/// Policy Enforcement Point Manager
pub struct EnforcementPointManager {
    /// Policy engine
    policy_engine: Arc<PolicyEngine>,
    
    /// API gateway enforcement point
    api_gateway: Arc<ApiGatewayEnforcer>,
    
    /// Service mesh enforcement point
    service_mesh: Arc<ServiceMeshEnforcer>,
    
    /// Database enforcement point
    database: Arc<DatabaseEnforcer>,
    
    /// Enforcement configuration
    config: EnforcementConfig,
    
    /// Enforcement statistics
    stats: Arc<RwLock<EnforcementStats>>,
}

impl EnforcementPointManager {
    /// Create a new enforcement point manager
    pub fn new(policy_engine: Arc<PolicyEngine>, config: EnforcementConfig) -> Self {
        let api_gateway = Arc::new(ApiGatewayEnforcer::new(
            policy_engine.clone(),
            config.api_gateway.clone(),
        ));
        
        let service_mesh = Arc::new(ServiceMeshEnforcer::new(
            policy_engine.clone(),
            config.service_mesh.clone(),
        ));
        
        let database = Arc::new(DatabaseEnforcer::new(
            policy_engine.clone(),
            config.database.clone(),
        ));
        
        Self {
            policy_engine,
            api_gateway,
            service_mesh,
            database,
            config,
            stats: Arc::new(RwLock::new(EnforcementStats::default())),
        }
    }
    
    /// Enforce policy for API gateway request
    pub async fn enforce_api_gateway(
        &self,
        context: &ZeroTrustContext,
        request: &ApiRequest,
    ) -> EnforcementResult<ApiEnforcementDecision> {
        let mut stats = self.stats.write().await;
        stats.total_requests += 1;
        
        match self.api_gateway.enforce(context, request).await {
            Ok(decision) => {
                match decision.decision {
                    AccessDecision::Allow => stats.allowed_requests += 1,
                    AccessDecision::Deny => stats.denied_requests += 1,
                    AccessDecision::RequireMfa => stats.mfa_required += 1,
                    AccessDecision::RequireStepUp => stats.step_up_required += 1,
                    _ => {}
                }
                Ok(decision)
            }
            Err(e) => {
                stats.errors += 1;
                Err(e)
            }
        }
    }
    
    /// Enforce policy for service mesh request
    pub async fn enforce_service_mesh(
        &self,
        context: &ZeroTrustContext,
        request: &MeshRequest,
    ) -> EnforcementResult<MeshEnforcementDecision> {
        let mut stats = self.stats.write().await;
        stats.total_requests += 1;
        
        match self.service_mesh.enforce(context, request).await {
            Ok(decision) => {
                match decision.decision {
                    AccessDecision::Allow => stats.allowed_requests += 1,
                    AccessDecision::Deny => stats.denied_requests += 1,
                    _ => {}
                }
                Ok(decision)
            }
            Err(e) => {
                stats.errors += 1;
                Err(e)
            }
        }
    }
    
    /// Enforce policy for database access
    pub async fn enforce_database(
        &self,
        context: &ZeroTrustContext,
        request: &DatabaseRequest,
    ) -> EnforcementResult<DatabaseEnforcementDecision> {
        let mut stats = self.stats.write().await;
        stats.total_requests += 1;
        
        match self.database.enforce(context, request).await {
            Ok(decision) => {
                match decision.decision {
                    AccessDecision::Allow => stats.allowed_requests += 1,
                    AccessDecision::Deny => stats.denied_requests += 1,
                    _ => {}
                }
                Ok(decision)
            }
            Err(e) => {
                stats.errors += 1;
                Err(e)
            }
        }
    }
    
    /// Get enforcement statistics
    pub async fn get_stats(&self) -> EnforcementStats {
        self.stats.read().await.clone()
    }
    
    /// Reset enforcement statistics
    pub async fn reset_stats(&self) {
        let mut stats = self.stats.write().await;
        *stats = EnforcementStats::default();
    }
}

/// API Gateway Enforcement Point
pub struct ApiGatewayEnforcer {
    policy_engine: Arc<PolicyEngine>,
    config: ApiGatewayConfig,
    rate_limiter: Arc<RwLock<RateLimiter>>,
}

impl ApiGatewayEnforcer {
    pub fn new(policy_engine: Arc<PolicyEngine>, config: ApiGatewayConfig) -> Self {
        Self {
            policy_engine,
            config,
            rate_limiter: Arc::new(RwLock::new(RateLimiter::new())),
        }
    }
    
    pub async fn enforce(
        &self,
        context: &ZeroTrustContext,
        request: &ApiRequest,
    ) -> EnforcementResult<ApiEnforcementDecision> {
        // Check rate limiting
        if !self.check_rate_limit(context, request).await {
            return Ok(ApiEnforcementDecision {
                decision: AccessDecision::Deny,
                reason: "Rate limit exceeded".to_string(),
                headers: self.get_rate_limit_headers().await,
                retry_after: Some(Duration::from_secs(60)),
            });
        }
        
        // Check IP whitelist/blacklist
        if self.config.ip_blacklist.contains(&context.network.ip_address) {
            return Ok(ApiEnforcementDecision {
                decision: AccessDecision::Deny,
                reason: "IP address is blacklisted".to_string(),
                headers: HashMap::new(),
                retry_after: None,
            });
        }
        
        // Evaluate policy
        let trust_score = request.trust_score.unwrap_or(0.5);
        let access_result = self.policy_engine.evaluate(
            context,
            trust_score,
            trust_score.into(),
        );
        
        let mut headers = HashMap::new();
        headers.insert(
            "X-Trust-Score".to_string(),
            trust_score.to_string(),
        );
        
        Ok(ApiEnforcementDecision {
            decision: access_result.decision,
            reason: access_result.reason,
            headers,
            retry_after: None,
        })
    }
    
    async fn check_rate_limit(&self, context: &ZeroTrustContext, request: &ApiRequest) -> bool {
        let mut rate_limiter = self.rate_limiter.write().await;
        rate_limiter.check(&context.subject, &request.path, self.config.rate_limit)
    }
    
    async fn get_rate_limit_headers(&self) -> HashMap<String, String> {
        let mut headers = HashMap::new();
        let rate_limiter = self.rate_limiter.read().await;
        
        headers.insert(
            "X-RateLimit-Limit".to_string(),
            self.config.rate_limit.to_string(),
        );
        
        headers
    }
}

/// Service Mesh Enforcement Point
pub struct ServiceMeshEnforcer {
    policy_engine: Arc<PolicyEngine>,
    config: ServiceMeshConfig,
}

impl ServiceMeshEnforcer {
    pub fn new(policy_engine: Arc<PolicyEngine>, config: ServiceMeshConfig) -> Self {
        Self {
            policy_engine,
            config,
        }
    }
    
    pub async fn enforce(
        &self,
        context: &ZeroTrustContext,
        request: &MeshRequest,
    ) -> EnforcementResult<MeshEnforcementDecision> {
        // Check service-to-service authentication
        if self.config.require_mtls && !context.network.is_encrypted {
            return Ok(MeshEnforcementDecision {
                decision: AccessDecision::Deny,
                reason: "mTLS required".to_string(),
                mutual_tls_required: true,
            });
        }
        
        // Check service whitelist
        if !self.config.allowed_services.contains(&request.destination_service) {
            return Ok(MeshEnforcementDecision {
                decision: AccessDecision::Deny,
                reason: "Service not in allowed list".to_string(),
                mutual_tls_required: false,
            });
        }
        
        // Evaluate policy
        let trust_score = request.trust_score.unwrap_or(0.5);
        let access_result = self.policy_engine.evaluate(
            context,
            trust_score,
            trust_score.into(),
        );
        
        Ok(MeshEnforcementDecision {
            decision: access_result.decision,
            reason: access_result.reason,
            mutual_tls_required: self.config.require_mtls,
        })
    }
}

/// Database Enforcement Point
pub struct DatabaseEnforcer {
    policy_engine: Arc<PolicyEngine>,
    config: DatabaseConfig,
}

impl DatabaseEnforcer {
    pub fn new(policy_engine: Arc<PolicyEngine>, config: DatabaseConfig) -> Self {
        Self {
            policy_engine,
            config,
        }
    }
    
    pub async fn enforce(
        &self,
        context: &ZeroTrustContext,
        request: &DatabaseRequest,
    ) -> EnforcementResult<DatabaseEnforcementDecision> {
        // Check database access rules
        if !self.config.allowed_databases.contains(&request.database_name) {
            return Ok(DatabaseEnforcementDecision {
                decision: AccessDecision::Deny,
                reason: "Database not allowed for user".to_string(),
                query_modification: None,
                row_limit: None,
            });
        }
        
        // Check query type restrictions
        if request.query_type == QueryType::Drop || request.query_type == QueryType::Delete {
            // Special handling for destructive operations
            if context.device.security_score < 0.8 {
                return Ok(DatabaseEnforcementDecision {
                    decision: AccessDecision::Deny,
                    reason: "Insufficient security score for destructive operations".to_string(),
                    query_modification: None,
                    row_limit: None,
                });
            }
        }
        
        // Evaluate policy
        let trust_score = request.trust_score.unwrap_or(0.5);
        let access_result = self.policy_engine.evaluate(
            context,
            trust_score,
            trust_score.into(),
        );
        
        let decision = match access_result.decision {
            AccessDecision::Allow => {
                // Apply query modification if needed
                let row_limit = if trust_score < 0.75 {
                    Some(1000) // Limit rows for lower trust
                } else {
                    None
                };
                
                DatabaseEnforcementDecision {
                    decision: AccessDecision::Allow,
                    reason: access_result.reason,
                    query_modification: None,
                    row_limit,
                }
            }
            decision => DatabaseEnforcementDecision {
                decision,
                reason: access_result.reason,
                query_modification: None,
                row_limit: None,
            },
        };
        
        Ok(decision)
    }
}

/// Rate Limiter
pub struct RateLimiter {
    request_counts: HashMap<String, HashMap<String, Vec<DateTime<Utc>>>>,
}

impl RateLimiter {
    pub fn new() -> Self {
        Self {
            request_counts: HashMap::new(),
        }
    }
    
    pub fn check(&mut self, subject: &str, path: &str, limit: u32) -> bool {
        let now = Utc::now();
        let window_start = now - chrono::Duration::minutes(1);
        
        let user_requests = self.request_counts
            .entry(subject.to_string())
            .or_insert_with(HashMap::new);
        
        let path_requests = user_requests
            .entry(path.to_string())
            .or_insert_with(Vec::new);
        
        // Remove old requests
        path_requests.retain(|t| *t > window_start);
        
        // Check limit
        if path_requests.len() >= limit as usize {
            return false;
        }
        
        // Add current request
        path_requests.push(now);
        true
    }
}

/// API Gateway Configuration
#[derive(Debug, Clone)]
pub struct ApiGatewayConfig {
    pub rate_limit: u32,
    pub ip_blacklist: Vec<String>,
    pub ip_whitelist: Vec<String>,
    pub enable_cors: bool,
}

impl Default for ApiGatewayConfig {
    fn default() -> Self {
        Self {
            rate_limit: 100,
            ip_blacklist: Vec::new(),
            ip_whitelist: Vec::new(),
            enable_cors: true,
        }
    }
}

/// Service Mesh Configuration
#[derive(Debug, Clone)]
pub struct ServiceMeshConfig {
    pub require_mtls: bool,
    pub allowed_services: Vec<String>,
    pub timeout: Duration,
}

impl Default for ServiceMeshConfig {
    fn default() -> Self {
        Self {
            require_mtls: true,
            allowed_services: vec![
                "auth-service".to_string(),
                "data-service".to_string(),
                "api-service".to_string(),
            ],
            timeout: Duration::from_secs(30),
        }
    }
}

/// Database Configuration
#[derive(Debug, Clone)]
pub struct DatabaseConfig {
    pub allowed_databases: Vec<String>,
    pub enable_query_modification: bool,
    pub max_row_limit: Option<usize>,
}

impl Default for DatabaseConfig {
    fn default() -> Self {
        Self {
            allowed_databases: vec!["production".to_string()],
            enable_query_modification: true,
            max_row_limit: Some(10000),
        }
    }
}

/// Enforcement Configuration
#[derive(Debug, Clone)]
pub struct EnforcementConfig {
    pub api_gateway: ApiGatewayConfig,
    pub service_mesh: ServiceMeshConfig,
    pub database: DatabaseConfig,
}

impl Default for EnforcementConfig {
    fn default() -> Self {
        Self {
            api_gateway: ApiGatewayConfig::default(),
            service_mesh: ServiceMeshConfig::default(),
            database: DatabaseConfig::default(),
        }
    }
}

/// Enforcement Statistics
#[derive(Debug, Clone, Default)]
pub struct EnforcementStats {
    pub total_requests: u64,
    pub allowed_requests: u64,
    pub denied_requests: u64,
    pub mfa_required: u64,
    pub step_up_required: u64,
    pub errors: u64,
}

/// API Request
#[derive(Debug, Clone)]
pub struct ApiRequest {
    pub method: String,
    pub path: String,
    pub headers: HashMap<String, String>,
    pub query_params: HashMap<String, String>,
    pub body: Option<String>,
    pub trust_score: Option<f64>,
}

/// Mesh Request
#[derive(Debug, Clone)]
pub struct MeshRequest {
    pub source_service: String,
    pub destination_service: String,
    pub method: String,
    pub path: String,
    pub trust_score: Option<f64>,
}

/// Database Request
#[derive(Debug, Clone)]
pub struct DatabaseRequest {
    pub database_name: String,
    pub table_name: String,
    pub query_type: QueryType,
    pub query: String,
    pub trust_score: Option<f64>,
}

/// Query Type
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum QueryType {
    Select,
    Insert,
    Update,
    Delete,
    Drop,
    Create,
    Alter,
}

/// API Enforcement Decision
#[derive(Debug, Clone)]
pub struct ApiEnforcementDecision {
    pub decision: AccessDecision,
    pub reason: String,
    pub headers: HashMap<String, String>,
    pub retry_after: Option<Duration>,
}

/// Mesh Enforcement Decision
#[derive(Debug, Clone)]
pub struct MeshEnforcementDecision {
    pub decision: AccessDecision,
    pub reason: String,
    pub mutual_tls_required: bool,
}

/// Database Enforcement Decision
#[derive(Debug, Clone)]
pub struct DatabaseEnforcementDecision {
    pub decision: AccessDecision,
    pub reason: String,
    pub query_modification: Option<String>,
    pub row_limit: Option<usize>,
}

/// Enforcement Result
pub type EnforcementResult<T> = Result<T, EnforcementError>;

/// Enforcement Error
#[derive(Debug, thiserror::Error)]
pub enum EnforcementError {
    #[error("Policy evaluation error: {0}")]
    PolicyEvaluation(String),
    
    #[error("Rate limit error: {0}")]
    RateLimitError(String),
    
    #[error("Configuration error: {0}")]
    ConfigurationError(String),
    
    #[error("Internal error: {0}")]
    InternalError(String),
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{DeviceInfo, NetworkInfo, LocationInfo, DeviceType, NetworkType, TrustLevel};

    fn create_test_context() -> ZeroTrustContext {
        ZeroTrustContext {
            request_id: uuid::Uuid::new_v4(),
            subject: "user@example.com".to_string(),
            resource: "/api/data".to_string(),
            action: "read".to_string(),
            device: DeviceInfo {
                device_id: "device-123".to_string(),
                device_type: DeviceType::Laptop,
                os: "Windows".to_string(),
                os_version: "10".to_string(),
                security_score: 0.8,
                is_trusted: true,
                last_seen: Utc::now(),
            },
            network: NetworkInfo {
                ip_address: "192.168.1.100".to_string(),
                location: LocationInfo {
                    country: "US".to_string(),
                    region: Some("CA".to_string()),
                    city: Some("San Francisco".to_string()),
                    latitude: Some(37.7749),
                    longitude: Some(-122.4194),
                },
                network_type: NetworkType::Corporate,
                is_trusted: true,
                is_encrypted: true,
            },
            timestamp: Utc::now(),
            context: serde_json::json!({}),
        }
    }

    #[tokio::test]
    async fn test_enforcement_manager_creation() {
        let policy_engine = Arc::new(PolicyEngine::new());
        let config = EnforcementConfig::default();
        let manager = EnforcementPointManager::new(policy_engine, config);
        
        let stats = manager.get_stats().await;
        assert_eq!(stats.total_requests, 0);
    }

    #[tokio::test]
    async fn test_api_gateway_enforcement() {
        let policy_engine = Arc::new(PolicyEngine::new());
        let enforcer = ApiGatewayEnforcer::new(
            policy_engine,
            ApiGatewayConfig::default(),
        );
        
        let context = create_test_context();
        let request = ApiRequest {
            method: "GET".to_string(),
            path: "/api/test".to_string(),
            headers: HashMap::new(),
            query_params: HashMap::new(),
            body: None,
            trust_score: Some(0.8),
        };
        
        let result = enforcer.enforce(&context, &request).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_rate_limiting() {
        let mut rate_limiter = RateLimiter::new();
        
        for i in 0..100 {
            assert!(rate_limiter.check("user1", "/api/test", 100));
        }
        
        // Should be limited
        assert!(!rate_limiter.check("user1", "/api/test", 100));
    }

    #[tokio::test]
    async fn test_service_mesh_enforcement() {
        let policy_engine = Arc::new(PolicyEngine::new());
        let enforcer = ServiceMeshEnforcer::new(
            policy_engine,
            ServiceMeshConfig::default(),
        );
        
        let context = create_test_context();
        let request = MeshRequest {
            source_service: "api-service".to_string(),
            destination_service: "data-service".to_string(),
            method: "GET".to_string(),
            path: "/data/1".to_string(),
            trust_score: Some(0.8),
        };
        
        let result = enforcer.enforce(&context, &request).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_database_enforcement() {
        let policy_engine = Arc::new(PolicyEngine::new());
        let enforcer = DatabaseEnforcer::new(
            policy_engine,
            DatabaseConfig::default(),
        );
        
        let context = create_test_context();
        let request = DatabaseRequest {
            database_name: "production".to_string(),
            table_name: "users".to_string(),
            query_type: QueryType::Select,
            query: "SELECT * FROM users".to_string(),
            trust_score: Some(0.8),
        };
        
        let result = enforcer.enforce(&context, &request).await;
        assert!(result.is_ok());
    }
}