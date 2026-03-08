//! Application-Level Segmentation
//! 
//! This module provides micro-segmentation at the application layer,
//! including service mesh integration, API gateway policies, and
//! container-level segmentation.

use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use chrono::{DateTime, Utc};

/// Application Segment Manager
pub struct ApplicationSegmenter {
    /// Application segments
    segments: HashMap<String, ApplicationSegment>,
    /// Service mesh configuration
    service_mesh: Option<ServiceMeshConfig>,
    /// API gateway policies
    api_policies: Vec<ApiGatewayPolicy>,
    /// Service dependencies graph
    dependencies: HashMap<String, Vec<ServiceDependency>>,
}

impl ApplicationSegmenter {
    /// Create a new application segmenter
    pub fn new() -> Self {
        Self {
            segments: HashMap::new(),
            service_mesh: None,
            api_policies: Vec::new(),
            dependencies: HashMap::new(),
        }
    }

    /// Register an application segment
    pub fn register_segment(&mut self, segment: ApplicationSegment) {
        self.segments.insert(segment.id.clone(), segment);
    }

    /// Configure service mesh
    pub fn configure_service_mesh(&mut self, config: ServiceMeshConfig) {
        self.service_mesh = Some(config);
    }

    /// Add API gateway policy
    pub fn add_api_policy(&mut self, policy: ApiGatewayPolicy) {
        self.api_policies.push(policy);
    }

    /// Register service dependency
    pub fn register_dependency(&mut self, service: &str, dependency: ServiceDependency) {
        self.dependencies
            .entry(service.to_string())
            .or_insert_with(Vec::new)
            .push(dependency);
    }

    /// Check if service-to-service communication is allowed
    pub fn is_communication_allowed(
        &self,
        source_service: &str,
        dest_service: &str,
        endpoint: &str,
        method: HttpMethod,
    ) -> CommunicationResult {
        // Get source segment
        let source_segment = self.find_segment_for_service(source_service);
        let dest_segment = self.find_segment_for_service(dest_service);

        // Check service dependencies
        if let Some(deps) = self.dependencies.get(source_service) {
            for dep in deps {
                if dep.target_service == dest_service {
                    // Check endpoint access
                    if dep.allowed_endpoints.is_empty() 
                        || dep.allowed_endpoints.iter().any(|e| endpoint.starts_with(e)) {
                        return CommunicationResult::Allowed {
                            reason: "Service dependency registered".to_string(),
                        };
                    }
                }
            }
        }

        // Check API policies
        for policy in &self.api_policies {
            if policy.applies_to(source_segment.as_deref(), dest_segment.as_deref()) {
                if policy.allows_endpoint(endpoint, method) {
                    return CommunicationResult::Allowed {
                        reason: "API policy allows access".to_string(),
                    };
                }
            }
        }

        // Check service mesh policies if configured
        if let Some(ref mesh) = self.service_mesh {
            if mesh.is_enabled {
                // Check mesh authorization policies
                for auth_policy in &mesh.authorization_policies {
                    if auth_policy.allows(source_service, dest_service, endpoint) {
                        return CommunicationResult::Allowed {
                            reason: "Service mesh policy allows".to_string(),
                        };
                    }
                }
            }
        }

        CommunicationResult::Denied {
            reason: "No policy allows this communication".to_string(),
            risk_score: 0.8,
        }
    }

    /// Find segment for a service
    fn find_segment_for_service(&self, service: &str) -> Option<String> {
        for (id, segment) in &self.segments {
            if segment.services.contains(service) {
                return Some(id.clone());
            }
        }
        None
    }

    /// Get all services in a segment
    pub fn get_segment_services(&self, segment_id: &str) -> Vec<&str> {
        self.segments
            .get(segment_id)
            .map(|s| s.services.iter().map(|s| s.as_str()).collect())
            .unwrap_or_default()
    }

    /// Get service dependencies
    pub fn get_dependencies(&self, service: &str) -> Vec<&ServiceDependency> {
        self.dependencies.get(service).map(|d| d.iter().collect()).unwrap_or_default()
    }

    /// Validate service mesh configuration
    pub fn validate_mesh_config(&self) -> Result<(), SegmentationError> {
        let mesh = self.service_mesh.as_ref()
            .ok_or_else(|| SegmentationError::MeshNotConfigured)?;

        // Validate mutual TLS is enabled
        if !mesh.mtls_enabled {
            return Err(SegmentationError::MeshConfigError(
                "Mutual TLS must be enabled for Zero Trust".to_string()
            ));
        }

        // Validate that all services have authorization policies
        for segment in self.segments.values() {
            for service in &segment.services {
                let has_policy = mesh.authorization_policies.iter()
                    .any(|p| p.source_selector.matches(service));
                
                if !has_policy {
                    return Err(SegmentationError::MeshConfigError(format!(
                        "Service {} has no authorization policy", service
                    )));
                }
            }
        }

        Ok(())
    }
}

impl Default for ApplicationSegmenter {
    fn default() -> Self {
        Self::new()
    }
}

/// Application segment definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApplicationSegment {
    /// Segment ID
    pub id: String,
    /// Segment name
    pub name: String,
    /// Services in this segment
    pub services: HashSet<String>,
    /// Environment (production, staging, development)
    pub environment: Environment,
    /// Security level
    pub security_level: SecurityLevel,
    /// Data classification
    pub data_classification: DataClassification,
    /// Allowed inbound services
    pub allowed_inbound: HashSet<String>,
    /// Allowed outbound services
    pub allowed_outbound: HashSet<String>,
    /// Rate limits
    pub rate_limits: Vec<RateLimit>,
    /// Created at
    pub created_at: DateTime<Utc>,
    /// Updated at
    pub updated_at: DateTime<Utc>,
}

impl ApplicationSegment {
    /// Create a new application segment
    pub fn new(id: impl Into<String>, name: impl Into<String>) -> Self {
        let now = Utc::now();
        Self {
            id: id.into(),
            name: name.into(),
            services: HashSet::new(),
            environment: Environment::Production,
            security_level: SecurityLevel::Standard,
            data_classification: DataClassification::Internal,
            allowed_inbound: HashSet::new(),
            allowed_outbound: HashSet::new(),
            rate_limits: Vec::new(),
            created_at: now,
            updated_at: now,
        }
    }

    /// Add service to segment
    pub fn with_service(mut self, service: impl Into<String>) -> Self {
        self.services.insert(service.into());
        self
    }

    /// Set environment
    pub fn with_environment(mut self, env: Environment) -> Self {
        self.environment = env;
        self
    }

    /// Set security level
    pub fn with_security_level(mut self, level: SecurityLevel) -> Self {
        self.security_level = level;
        self
    }

    /// Add allowed inbound service
    pub fn allow_inbound(mut self, service: impl Into<String>) -> Self {
        self.allowed_inbound.insert(service.into());
        self
    }

    /// Add allowed outbound service
    pub fn allow_outbound(mut self, service: impl Into<String>) -> Self {
        self.allowed_outbound.insert(service.into());
        self
    }
}

/// Environment type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Environment {
    Production,
    Staging,
    Development,
    Testing,
    Sandbox,
}

/// Security level
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SecurityLevel {
    Public,
    Standard,
    Confidential,
    Restricted,
    TopSecret,
}

/// Data classification
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DataClassification {
    Public,
    Internal,
    Confidential,
    Restricted,
    PII,
    PHI, // Protected Health Information
    PCI, // Payment Card Industry
}

/// Service dependency definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceDependency {
    /// Source service
    pub source_service: String,
    /// Target service
    pub target_service: String,
    /// Allowed endpoints
    pub allowed_endpoints: Vec<String>,
    /// Required permissions
    pub required_permissions: Vec<String>,
    /// Is this dependency critical
    pub is_critical: bool,
    /// Latency SLA in milliseconds
    pub latency_sla_ms: Option<u32>,
}

/// Service mesh configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceMeshConfig {
    /// Is mesh enabled
    pub is_enabled: bool,
    /// Mesh type
    pub mesh_type: MeshType,
    /// Mutual TLS enabled
    pub mtls_enabled: bool,
    /// mTLS mode
    pub mtls_mode: MtlsMode,
    /// Authorization policies
    pub authorization_policies: Vec<MeshAuthorizationPolicy>,
    /// Traffic policies
    pub traffic_policies: Vec<TrafficPolicy>,
    /// Observability settings
    pub observability: ObservabilityConfig,
}

impl Default for ServiceMeshConfig {
    fn default() -> Self {
        Self {
            is_enabled: true,
            mesh_type: MeshType::Istio,
            mtls_enabled: true,
            mtls_mode: MtlsMode::Strict,
            authorization_policies: Vec::new(),
            traffic_policies: Vec::new(),
            observability: ObservabilityConfig::default(),
        }
    }
}

/// Service mesh types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MeshType {
    Istio,
    Linkerd,
    ConsulConnect,
    Kuma,
    AppMesh,
    Custom(String),
}

/// mTLS mode
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MtlsMode {
    /// Strict - all traffic must use mTLS
    Strict,
    /// Permissive - mTLS optional
    Permissive,
}

/// Mesh authorization policy
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MeshAuthorizationPolicy {
    /// Policy name
    pub name: String,
    /// Source selector
    pub source_selector: ServiceSelector,
    /// Destination selector
    pub dest_selector: ServiceSelector,
    /// Allowed operations
    pub allowed_operations: Vec<AllowedOperation>,
    /// Conditions
    pub conditions: Vec<PolicyCondition>,
}

impl MeshAuthorizationPolicy {
    /// Check if policy allows the communication
    pub fn allows(&self, source: &str, dest: &str, endpoint: &str) -> bool {
        if !self.source_selector.matches(source) {
            return false;
        }
        if !self.dest_selector.matches(dest) {
            return false;
        }
        
        self.allowed_operations.iter().any(|op| {
            op.paths.iter().any(|p| endpoint.starts_with(p))
        })
    }
}

/// Service selector
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ServiceSelector {
    All,
    Service(String),
    Label(String, String),
    Namespace(String),
}

impl ServiceSelector {
    /// Check if selector matches a service
    pub fn matches(&self, service: &str) -> bool {
        match self {
            ServiceSelector::All => true,
            ServiceSelector::Service(s) => s == service,
            ServiceSelector::Label(_, _) => true, // Would need label lookup
            ServiceSelector::Namespace(_) => true, // Would need namespace lookup
        }
    }
}

/// Allowed operation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AllowedOperation {
    /// HTTP methods
    pub methods: Vec<HttpMethod>,
    /// Path patterns
    pub paths: Vec<String>,
}

/// HTTP method
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum HttpMethod {
    GET,
    POST,
    PUT,
    PATCH,
    DELETE,
    HEAD,
    OPTIONS,
    ANY,
}

/// Policy condition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PolicyCondition {
    /// Condition key
    pub key: String,
    /// Condition values
    pub values: Vec<String>,
    /// Condition type
    pub condition_type: ConditionType,
}

/// Condition types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ConditionType {
    Exact,
    Prefix,
    Suffix,
    Regex,
    Present,
    Absent,
}

/// Traffic policy
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrafficPolicy {
    /// Policy name
    pub name: String,
    /// Target services
    pub target_services: Vec<String>,
    /// Load balancing
    pub load_balancing: LoadBalancing,
    /// Connection pool
    pub connection_pool: ConnectionPool,
    /// Outlier detection
    pub outlier_detection: OutlierDetection,
    /// TLS settings
    pub tls_settings: TlsSettings,
}

/// Load balancing configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoadBalancing {
    pub policy: LbPolicy,
    pub healthy_panic_threshold: f64,
    pub locality_lb_policy: Option<LbPolicy>,
}

/// Load balancing policy
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum LbPolicy {
    RoundRobin,
    LeastConnections,
    Random,
    ConsistentHash(ConsistentHashConfig),
}

/// Consistent hash configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConsistentHashConfig {
    pub http_header: Option<String>,
    pub http_cookie: Option<String>,
    pub use_source_ip: bool,
    pub minimum_ring_size: u32,
}

/// Connection pool configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConnectionPool {
    pub tcp_max_connections: u32,
    pub tcp_connect_timeout_ms: u32,
    pub http_max_pending_requests: u32,
    pub http_max_requests: u32,
    pub http_max_requests_per_connection: u32,
    pub http_idle_timeout_ms: u32,
}

impl Default for ConnectionPool {
    fn default() -> Self {
        Self {
            tcp_max_connections: 100,
            tcp_connect_timeout_ms: 5000,
            http_max_pending_requests: 1000,
            http_max_requests: 1000,
            http_max_requests_per_connection: 100,
            http_idle_timeout_ms: 300000,
        }
    }
}

/// Outlier detection configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OutlierDetection {
    pub enabled: bool,
    pub consecutive_5xx_errors: u32,
    pub interval_ms: u32,
    pub base_ejection_time_ms: u32,
    pub max_ejection_percent: u32,
}

impl Default for OutlierDetection {
    fn default() -> Self {
        Self {
            enabled: true,
            consecutive_5xx_errors: 5,
            interval_ms: 30000,
            base_ejection_time_ms: 30000,
            max_ejection_percent: 50,
        }
    }
}

/// TLS settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TlsSettings {
    pub mode: TlsMode,
    pub client_certificate: Option<String>,
    pub server_certificate: Option<String>,
    pub verify_spki: Vec<String>,
    pub verify_subject_alt_names: Vec<String>,
}

/// TLS mode
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TlsMode {
    Disable,
    Simple,
    Mutual,
    IstioMutual,
}

/// Observability configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ObservabilityConfig {
    pub tracing_enabled: bool,
    pub tracing_sampling_rate: f64,
    pub metrics_enabled: bool,
    pub access_logging: bool,
    pub access_log_format: String,
}

impl Default for ObservabilityConfig {
    fn default() -> Self {
        Self {
            tracing_enabled: true,
            tracing_sampling_rate: 1.0,
            metrics_enabled: true,
            access_logging: true,
            access_log_format: "JSON".to_string(),
        }
    }
}

/// API Gateway Policy
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiGatewayPolicy {
    /// Policy name
    pub name: String,
    /// Source segment selector
    pub source_segment: Option<String>,
    /// Destination segment selector
    pub dest_segment: Option<String>,
    /// API patterns
    pub api_patterns: Vec<ApiPattern>,
    /// Rate limits
    pub rate_limits: Vec<RateLimit>,
    /// Authentication required
    pub auth_required: bool,
    /// CORS settings
    pub cors: Option<CorsConfig>,
}

impl ApiGatewayPolicy {
    /// Check if policy applies
    pub fn applies_to(&self, source: Option<&str>, dest: Option<&str>) -> bool {
        let source_match = self.source_segment.as_ref()
            .map(|s| source.map(|src| src == s).unwrap_or(false))
            .unwrap_or(true);
        
        let dest_match = self.dest_segment.as_ref()
            .map(|d| dest.map(|dst| dst == d).unwrap_or(false))
            .unwrap_or(true);
        
        source_match && dest_match
    }

    /// Check if endpoint is allowed
    pub fn allows_endpoint(&self, endpoint: &str, method: HttpMethod) -> bool {
        self.api_patterns.iter().any(|pattern| {
            pattern.matches(endpoint, method)
        })
    }
}

/// API pattern
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiPattern {
    /// Path pattern (supports wildcards)
    pub path: String,
    /// Allowed methods
    pub methods: Vec<HttpMethod>,
    /// Required scopes
    pub required_scopes: Vec<String>,
}

impl ApiPattern {
    /// Check if pattern matches endpoint and method
    pub fn matches(&self, endpoint: &str, method: HttpMethod) -> bool {
        let method_match = self.methods.contains(&HttpMethod::ANY) 
            || self.methods.contains(&method);
        
        let path_match = if self.path.contains('*') {
            // Simple wildcard matching
            let prefix = self.path.trim_end_matches('*');
            endpoint.starts_with(prefix)
        } else {
            endpoint == self.path
        };
        
        method_match && path_match
    }
}

/// Rate limit configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RateLimit {
    /// Limit name
    pub name: String,
    /// Requests per second
    pub requests_per_second: Option<u32>,
    /// Requests per minute
    pub requests_per_minute: Option<u32>,
    /// Requests per hour
    pub requests_per_hour: Option<u32>,
    /// Burst size
    pub burst: Option<u32>,
    /// Key type
    pub key_type: RateLimitKey,
}

/// Rate limit key type
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RateLimitKey {
    IP,
    User,
    APIKey,
    Service,
    Custom(String),
}

/// CORS configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CorsConfig {
    pub allowed_origins: Vec<String>,
    pub allowed_methods: Vec<String>,
    pub allowed_headers: Vec<String>,
    pub exposed_headers: Vec<String>,
    pub allow_credentials: bool,
    pub max_age: u32,
}

/// Communication result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CommunicationResult {
    Allowed { reason: String },
    Denied { reason: String, risk_score: f64 },
}

impl CommunicationResult {
    pub fn is_allowed(&self) -> bool {
        matches!(self, CommunicationResult::Allowed { .. })
    }
}

/// Segmentation error
#[derive(Debug, thiserror::Error)]
pub enum SegmentationError {
    #[error("Service mesh not configured")]
    MeshNotConfigured,
    #[error("Mesh configuration error: {0}")]
    MeshConfigError(String),
    #[error("Segment not found: {0}")]
    SegmentNotFound(String),
    #[error("Invalid configuration: {0}")]
    InvalidConfig(String),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_application_segment_creation() {
        let segment = ApplicationSegment::new("payment", "Payment Services")
            .with_service("payment-api")
            .with_service("payment-processor")
            .with_environment(Environment::Production)
            .with_security_level(SecurityLevel::Restricted);

        assert_eq!(segment.id, "payment");
        assert!(segment.services.contains("payment-api"));
        assert_eq!(segment.environment, Environment::Production);
    }

    #[test]
    fn test_service_dependency() {
        let mut segmenter = ApplicationSegmenter::new();

        segmenter.register_dependency("web-frontend", ServiceDependency {
            source_service: "web-frontend".to_string(),
            target_service: "api-gateway".to_string(),
            allowed_endpoints: vec!["/api/v1/".to_string()],
            required_permissions: vec!["api:read".to_string()],
            is_critical: true,
            latency_sla_ms: Some(100),
        });

        let deps = segmenter.get_dependencies("web-frontend");
        assert_eq!(deps.len(), 1);
        assert_eq!(deps[0].target_service, "api-gateway");
    }

    #[test]
    fn test_communication_allowed() {
        let mut segmenter = ApplicationSegmenter::new();

        segmenter.register_dependency("frontend", ServiceDependency {
            source_service: "frontend".to_string(),
            target_service: "backend".to_string(),
            allowed_endpoints: vec!["/api/".to_string()],
            required_permissions: vec![],
            is_critical: false,
            latency_sla_ms: None,
        });

        let result = segmenter.is_communication_allowed(
            "frontend",
            "backend",
            "/api/users",
            HttpMethod::GET,
        );

        assert!(result.is_allowed());
    }

    #[test]
    fn test_service_mesh_config() {
        let config = ServiceMeshConfig::default();
        assert!(config.is_enabled);
        assert!(config.mtls_enabled);
        assert_eq!(config.mtls_mode, MtlsMode::Strict);
    }

    #[test]
    fn test_api_pattern_matching() {
        let pattern = ApiPattern {
            path: "/api/v1/*".to_string(),
            methods: vec![HttpMethod::GET, HttpMethod::POST],
            required_scopes: vec![],
        };

        assert!(pattern.matches("/api/v1/users", HttpMethod::GET));
        assert!(pattern.matches("/api/v1/orders", HttpMethod::POST));
        assert!(!pattern.matches("/api/v1/users", HttpMethod::DELETE));
        assert!(!pattern.matches("/api/v2/users", HttpMethod::GET));
    }

    #[test]
    fn test_rate_limit_defaults() {
        let segment = ApplicationSegment::new("test", "Test");
        assert!(segment.rate_limits.is_empty());
    }
}