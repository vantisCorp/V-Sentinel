//! Data Segmentation Controls
//! 
//! This module provides data-level segmentation including:
//! - Data classification and labeling
//! - Attribute-based access control
//! - Encryption boundaries
//! - Data loss prevention integration

use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use chrono::{DateTime, Utc};
use regex::Regex;

/// Data Segment Manager
pub struct DataSegmenter {
    /// Data classifications
    classifications: HashMap<String, DataClassification>,
    /// Data segments
    segments: HashMap<String, DataSegment>,
    /// Access policies
    policies: Vec<DataAccessPolicy>,
    /// Encryption zones
    encryption_zones: HashMap<String, EncryptionZone>,
    /// DLP rules
    dlp_rules: Vec<DlpRule>,
}

impl DataSegmenter {
    /// Create a new data segmenter
    pub fn new() -> Self {
        Self {
            classifications: HashMap::new(),
            segments: HashMap::new(),
            policies: Vec::new(),
            encryption_zones: HashMap::new(),
            dlp_rules: Vec::new(),
        }
    }

    /// Register a data classification
    pub fn register_classification(&mut self, classification: DataClassification) {
        self.classifications.insert(classification.id.clone(), classification);
    }

    /// Create a data segment
    pub fn create_segment(&mut self, segment: DataSegment) {
        self.segments.insert(segment.id.clone(), segment);
    }

    /// Add access policy
    pub fn add_policy(&mut self, policy: DataAccessPolicy) {
        self.policies.push(policy);
    }

    /// Create encryption zone
    pub fn create_encryption_zone(&mut self, zone: EncryptionZone) {
        self.encryption_zones.insert(zone.id.clone(), zone);
    }

    /// Add DLP rule
    pub fn add_dlp_rule(&mut self, rule: DlpRule) {
        self.dlp_rules.push(rule);
    }

    /// Check data access
    pub fn check_access(
        &self,
        user_id: &str,
        user_attributes: &UserAttributes,
        data_segment: &str,
        operation: DataOperation,
    ) -> DataAccessResult {
        // Get the data segment
        let segment = match self.segments.get(data_segment) {
            Some(s) => s,
            None => return DataAccessResult::Denied {
                reason: "Data segment not found".to_string(),
                violations: vec![],
            },
        };

        let mut violations = Vec::new();

        // Check classification requirements
        let classification = self.classifications.get(&segment.classification_id);
        if let Some(class) = classification {
            // Check clearance level
            if user_attributes.clearance_level < class.required_clearance {
                violations.push(AccessViolation::InsufficientClearance {
                    required: class.required_clearance,
                    actual: user_attributes.clearance_level,
                });
            }

            // Check required roles
            for role in &class.required_roles {
                if !user_attributes.roles.contains(role) {
                    violations.push(AccessViolation::MissingRole {
                        role: role.clone(),
                    });
                }
            }

            // Check required attributes
            for (key, value) in &class.required_attributes {
                match user_attributes.attributes.get(key) {
                    Some(user_value) if user_value == value => {}
                    _ => violations.push(AccessViolation::MissingAttribute {
                        attribute: key.clone(),
                        required_value: value.clone(),
                    }),
                }
            }
        }

        // Check access policies
        let mut policy_allowed = false;
        for policy in &self.policies {
            if policy.applies_to(user_attributes, data_segment) {
                if policy.allows_operation(operation) {
                    policy_allowed = true;
                } else {
                    violations.push(AccessViolation::PolicyDenied {
                        policy: policy.name.clone(),
                    });
                }
            }
        }

        // Check encryption zone requirements
        if let Some(zone_id) = &segment.encryption_zone_id {
            if let Some(zone) = self.encryption_zones.get(zone_id) {
                if !zone.is_user_authorized(user_id) {
                    violations.push(AccessViolation::NotAuthorizedForZone {
                        zone: zone_id.clone(),
                    });
                }
            }
        }

        // Determine result
        if violations.is_empty() && policy_allowed {
            DataAccessResult::Allowed {
                classification: segment.classification_id.clone(),
                encryption_required: segment.encryption_required,
                audit_required: segment.audit_required,
            }
        } else if !violations.is_empty() {
            DataAccessResult::Denied {
                reason: "Access violations detected".to_string(),
                violations,
            }
        } else {
            DataAccessResult::Denied {
                reason: "No matching policy allows this operation".to_string(),
                violations: vec![],
            }
        }
    }

    /// Scan data for DLP violations
    pub fn scan_for_dlp(
        &self,
        content: &str,
        context: &DlpContext,
    ) -> DlpScanResult {
        let mut matches = Vec::new();
        let mut total_risk_score = 0.0;

        for rule in &self.dlp_rules {
            if rule.is_enabled && rule.applies_to_context(context) {
                let rule_matches = rule.scan(content);
                for m in rule_matches {
                    total_risk_score += rule.risk_weight;
                    matches.push(m);
                }
            }
        }

        DlpScanResult {
            matches,
            risk_score: total_risk_score.min(1.0),
            action: if total_risk_score > 0.7 {
                DlpAction::Block
            } else if total_risk_score > 0.4 {
                DlpAction::Warn
            } else {
                DlpAction::Log
            },
        }
    }

    /// Get classification by ID
    pub fn get_classification(&self, id: &str) -> Option<&DataClassification> {
        self.classifications.get(id)
    }

    /// Get segment by ID
    pub fn get_segment(&self, id: &str) -> Option<&DataSegment> {
        self.segments.get(id)
    }
}

impl Default for DataSegmenter {
    fn default() -> Self {
        Self::new()
    }
}

/// Data classification definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataClassification {
    /// Classification ID
    pub id: String,
    /// Classification name
    pub name: String,
    /// Description
    pub description: String,
    /// Classification level (higher = more sensitive)
    pub level: u32,
    /// Required clearance level to access
    pub required_clearance: u32,
    /// Required roles
    pub required_roles: HashSet<String>,
    /// Required attributes
    pub required_attributes: HashMap<String, String>,
    /// Retention policy
    pub retention_policy: Option<RetentionPolicy>,
    /// Encryption required
    pub encryption_required: bool,
    /// Audit all access
    pub audit_all_access: bool,
    /// Data types in this classification
    pub data_types: Vec<DataType>,
}

impl DataClassification {
    /// Create a new classification
    pub fn new(id: impl Into<String>, name: impl Into<String>, level: u32) -> Self {
        Self {
            id: id.into(),
            name: name.into(),
            description: String::new(),
            level,
            required_clearance: level,
            required_roles: HashSet::new(),
            required_attributes: HashMap::new(),
            retention_policy: None,
            encryption_required: level >= 2,
            audit_all_access: level >= 3,
            data_types: Vec::new(),
        }
    }

    /// Add required role
    pub fn with_required_role(mut self, role: impl Into<String>) -> Self {
        self.required_roles.insert(role.into());
        self
    }

    /// Add required attribute
    pub fn with_required_attribute(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.required_attributes.insert(key.into(), value.into());
        self
    }
}

/// Data type classification
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DataType {
    PII,           // Personal Identifiable Information
    PHI,           // Protected Health Information
    Financial,     // Financial data
    PaymentCard,   // Payment card data (PCI)
    Credentials,   // User credentials
    Intellectual,  // Intellectual property
    Operational,   // Operational data
    Public,        // Public data
    Custom(String),
}

/// Retention policy
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RetentionPolicy {
    /// Retention period in days
    pub retention_days: u32,
    /// Archive after days
    pub archive_after_days: Option<u32>,
    /// Delete after days
    pub delete_after_days: Option<u32>,
    /// Legal hold enabled
    pub legal_hold: bool,
}

/// Data segment definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataSegment {
    /// Segment ID
    pub id: String,
    /// Segment name
    pub name: String,
    /// Classification ID
    pub classification_id: String,
    /// Data sources
    pub data_sources: Vec<DataSource>,
    /// Encryption zone ID
    pub encryption_zone_id: Option<String>,
    /// Encryption required
    pub encryption_required: bool,
    /// Audit required
    pub audit_required: bool,
    /// Data residency requirements
    pub residency_requirements: Vec<String>,
    /// Allowed operations
    pub allowed_operations: HashSet<DataOperation>,
    /// Created at
    pub created_at: DateTime<Utc>,
}

impl DataSegment {
    /// Create a new data segment
    pub fn new(
        id: impl Into<String>,
        name: impl Into<String>,
        classification_id: impl Into<String>,
    ) -> Self {
        Self {
            id: id.into(),
            name: name.into(),
            classification_id: classification_id.into(),
            data_sources: Vec::new(),
            encryption_zone_id: None,
            encryption_required: false,
            audit_required: true,
            residency_requirements: Vec::new(),
            allowed_operations: HashSet::new(),
            created_at: Utc::now(),
        }
    }

    /// Add data source
    pub fn with_data_source(mut self, source: DataSource) -> Self {
        self.data_sources.push(source);
        self
    }

    /// Set encryption zone
    pub fn with_encryption_zone(mut self, zone_id: impl Into<String>) -> Self {
        self.encryption_zone_id = Some(zone_id.into());
        self.encryption_required = true;
        self
    }

    /// Allow operation
    pub fn allow_operation(mut self, operation: DataOperation) -> Self {
        self.allowed_operations.insert(operation);
        self
    }
}

/// Data source definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataSource {
    /// Source ID
    pub id: String,
    /// Source name
    pub name: String,
    /// Source type
    pub source_type: DataSourceType,
    /// Location/endpoint
    pub location: String,
    /// Schema reference
    pub schema: Option<String>,
    /// Field-level classifications
    pub field_classifications: HashMap<String, String>,
}

/// Data source type
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DataSourceType {
    Database,
    FileStore,
    Api,
    Stream,
    Cache,
    DataWarehouse,
    LakeHouse,
}

/// Data operation
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum DataOperation {
    Read,
    Write,
    Update,
    Delete,
    Export,
    Share,
    Annotate,
    Archive,
}

/// User attributes for access decisions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserAttributes {
    /// User ID
    pub user_id: String,
    /// Clearance level
    pub clearance_level: u32,
    /// Roles
    pub roles: HashSet<String>,
    /// Custom attributes
    pub attributes: HashMap<String, String>,
    /// Department
    pub department: Option<String>,
    /// Location
    pub location: Option<String>,
    /// Groups
    pub groups: HashSet<String>,
}

impl UserAttributes {
    /// Create new user attributes
    pub fn new(user_id: impl Into<String>) -> Self {
        Self {
            user_id: user_id.into(),
            clearance_level: 0,
            roles: HashSet::new(),
            attributes: HashMap::new(),
            department: None,
            location: None,
            groups: HashSet::new(),
        }
    }

    /// Set clearance level
    pub fn with_clearance(mut self, level: u32) -> Self {
        self.clearance_level = level;
        self
    }

    /// Add role
    pub fn with_role(mut self, role: impl Into<String>) -> Self {
        self.roles.insert(role.into());
        self
    }

    /// Add attribute
    pub fn with_attribute(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.attributes.insert(key.into(), value.into());
        self
    }
}

/// Data access policy
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataAccessPolicy {
    /// Policy name
    pub name: String,
    /// Description
    pub description: String,
    /// User selector
    pub user_selector: UserSelector,
    /// Data segment selector
    pub data_selector: DataSelector,
    /// Allowed operations
    pub allowed_operations: HashSet<DataOperation>,
    /// Time constraints
    pub time_constraints: Option<TimeConstraints>,
    /// Purpose constraints
    pub purpose_constraints: Vec<String>,
    /// Conditions
    pub conditions: Vec<AccessCondition>,
}

impl DataAccessPolicy {
    /// Check if policy applies to user and data
    pub fn applies_to(&self, user: &UserAttributes, data_segment: &str) -> bool {
        self.user_selector.matches(user) 
            && self.data_selector.matches(data_segment)
    }

    /// Check if operation is allowed
    pub fn allows_operation(&self, operation: DataOperation) -> bool {
        self.allowed_operations.contains(&operation)
    }
}

/// User selector for policies
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum UserSelector {
    All,
    User(String),
    Role(String),
    Group(String),
    Department(String),
    Attribute { key: String, value: String },
    Clearance { min: u32, max: Option<u32> },
}

impl UserSelector {
    /// Check if selector matches user
    pub fn matches(&self, user: &UserAttributes) -> bool {
        match self {
            UserSelector::All => true,
            UserSelector::User(id) => &user.user_id == id,
            UserSelector::Role(role) => user.roles.contains(role),
            UserSelector::Group(group) => user.groups.contains(group),
            UserSelector::Department(dept) => user.department.as_ref() == Some(dept),
            UserSelector::Attribute { key, value } => user.attributes.get(key) == Some(value),
            UserSelector::Clearance { min, max } => {
                user.clearance_level >= *min 
                    && max.map_or(true, |m| user.clearance_level <= m)
            }
        }
    }
}

/// Data selector for policies
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DataSelector {
    All,
    Segment(String),
    Classification(String),
    DataSource(String),
    Tag(String),
}

impl DataSelector {
    /// Check if selector matches segment
    pub fn matches(&self, segment_id: &str) -> bool {
        match self {
            DataSelector::All => true,
            DataSelector::Segment(id) => id == segment_id,
            _ => true, // Would need more context for other selectors
        }
    }
}

/// Time constraints
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimeConstraints {
    /// Allowed hours (start, end) in 24h format
    pub allowed_hours: Option<(u32, u32)>,
    /// Allowed days of week (0 = Sunday)
    pub allowed_days: Option<Vec<u32>>,
    /// Allowed timezone
    pub timezone: Option<String>,
    /// Max session duration in minutes
    pub max_duration_minutes: Option<u32>,
}

/// Access condition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccessCondition {
    /// Condition type
    pub condition_type: AccessConditionType,
    /// Parameters
    pub parameters: HashMap<String, String>,
}

/// Access condition types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AccessConditionType {
    MfaRequired,
    ApprovalRequired { approvers: Vec<String> },
    BreakGlass,
    TimeLimit { minutes: u32 },
    IpRestriction { allowed_cidrs: Vec<String> },
    DeviceCompliance,
}

/// Data access result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DataAccessResult {
    Allowed {
        classification: String,
        encryption_required: bool,
        audit_required: bool,
    },
    Denied {
        reason: String,
        violations: Vec<AccessViolation>,
    },
}

impl DataAccessResult {
    pub fn is_allowed(&self) -> bool {
        matches!(self, DataAccessResult::Allowed { .. })
    }
}

/// Access violation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AccessViolation {
    InsufficientClearance { required: u32, actual: u32 },
    MissingRole { role: String },
    MissingAttribute { attribute: String, required_value: String },
    PolicyDenied { policy: String },
    NotAuthorizedForZone { zone: String },
    TimeConstraintViolation,
    PurposeViolation { purpose: String },
}

/// Encryption zone definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EncryptionZone {
    /// Zone ID
    pub id: String,
    /// Zone name
    pub name: String,
    /// Encryption algorithm
    pub algorithm: EncryptionAlgorithm,
    /// Key management
    pub key_management: KeyManagement,
    /// Authorized users
    pub authorized_users: HashSet<String>,
    /// Authorized services
    pub authorized_services: HashSet<String>,
    /// Key rotation policy
    pub key_rotation: KeyRotationPolicy,
}

impl EncryptionZone {
    /// Create a new encryption zone
    pub fn new(id: impl Into<String>, name: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            name: name.into(),
            algorithm: EncryptionAlgorithm::Aes256Gcm,
            key_management: KeyManagement::Managed,
            authorized_users: HashSet::new(),
            authorized_services: HashSet::new(),
            key_rotation: KeyRotationPolicy::default(),
        }
    }

    /// Check if user is authorized
    pub fn is_user_authorized(&self, user_id: &str) -> bool {
        self.authorized_users.contains(user_id)
    }

    /// Authorize user
    pub fn authorize_user(mut self, user_id: impl Into<String>) -> Self {
        self.authorized_users.insert(user_id.into());
        self
    }
}

/// Encryption algorithm
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EncryptionAlgorithm {
    Aes256Gcm,
    Aes256Cbc,
    ChaCha20Poly1305,
    Rsa4096,
    Hybrid {
        symmetric: Box<EncryptionAlgorithm>,
        asymmetric: Box<EncryptionAlgorithm>,
    },
}

/// Key management type
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum KeyManagement {
    Managed,
    CustomerManaged { key_id: String },
    Hsm { hsm_id: String },
    External { endpoint: String },
}

/// Key rotation policy
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KeyRotationPolicy {
    /// Rotation enabled
    pub enabled: bool,
    /// Rotation interval in days
    pub interval_days: u32,
    /// Auto-rotate
    pub auto_rotate: bool,
    /// Retire old keys after days
    pub retire_after_days: u32,
}

impl Default for KeyRotationPolicy {
    fn default() -> Self {
        Self {
            enabled: true,
            interval_days: 90,
            auto_rotate: true,
            retire_after_days: 30,
        }
    }
}

/// DLP Rule
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DlpRule {
    /// Rule ID
    pub id: String,
    /// Rule name
    pub name: String,
    /// Description
    pub description: String,
    /// Is enabled
    pub is_enabled: bool,
    /// Rule type
    pub rule_type: DlpRuleType,
    /// Risk weight (0.0-1.0)
    pub risk_weight: f64,
    /// Action
    pub action: DlpAction,
    /// Exceptions
    pub exceptions: Vec<DlpException>,
}

impl DlpRule {
    /// Check if rule applies to context
    pub fn applies_to_context(&self, context: &DlpContext) -> bool {
        // Check exceptions first
        for exception in &self.exceptions {
            if exception.matches(context) {
                return false;
            }
        }
        true
    }

    /// Scan content for matches
    pub fn scan(&self, content: &str) -> Vec<DlpMatch> {
        let mut matches = Vec::new();

        match &self.rule_type {
            DlpRuleType::Regex { pattern } => {
                if let Ok(re) = Regex::new(pattern) {
                    for cap in re.find_iter(content) {
                        matches.push(DlpMatch {
                            rule_id: self.id.clone(),
                            matched_text: cap.as_str().to_string(),
                            start_pos: cap.start(),
                            end_pos: cap.end(),
                            context: Self::get_context(content, cap.start(), cap.end()),
                        });
                    }
                }
            }
            DlpRuleType::Keyword { keywords, case_sensitive } => {
                for keyword in keywords {
                    let search = if *case_sensitive {
                        keyword.clone()
                    } else {
                        keyword.to_lowercase()
                    };
                    let search_content = if *case_sensitive {
                        content.to_string()
                    } else {
                        content.to_lowercase()
                    };

                    let mut start = 0;
                    while let Some(pos) = search_content[start..].find(&search) {
                        let abs_start = start + pos;
                        let abs_end = abs_start + keyword.len();
                        matches.push(DlpMatch {
                            rule_id: self.id.clone(),
                            matched_text: content[abs_start..abs_end].to_string(),
                            start_pos: abs_start,
                            end_pos: abs_end,
                            context: Self::get_context(content, abs_start, abs_end),
                        });
                        start = abs_end;
                    }
                }
            }
            DlpRuleType::Pattern { pattern_type } => {
                let pattern = Self::get_pattern_for_type(pattern_type);
                if let Ok(re) = Regex::new(&pattern) {
                    for cap in re.find_iter(content) {
                        matches.push(DlpMatch {
                            rule_id: self.id.clone(),
                            matched_text: cap.as_str().to_string(),
                            start_pos: cap.start(),
                            end_pos: cap.end(),
                            context: Self::get_context(content, cap.start(), cap.end()),
                        });
                    }
                }
            }
        }

        matches
    }

    /// Get regex pattern for built-in types
    fn get_pattern_for_type(pattern_type: &DlpPattern) -> String {
        match pattern_type {
            DlpPattern::CreditCard => r"\b(?:\d{4}[-\s]?){3}\d{4}\b".to_string(),
            DlpPattern::SSN => r"\b\d{3}[-\s]?\d{2}[-\s]?\d{4}\b".to_string(),
            DlpPattern::Email => r"\b[A-Za-z0-9._%+-]+@[A-Za-z0-9.-]+\.[A-Z|a-z]{2,}\b".to_string(),
            DlpPattern::PhoneNumber => r"\b(?:\+?1[-.\s]?)?\(?[0-9]{3}\)?[-.\s]?[0-9]{3}[-.\s]?[0-9]{4}\b".to_string(),
            DlpPattern::IpAddress => r"\b(?:\d{1,3}\.){3}\d{1,3}\b".to_string(),
            DlpPattern::ApiKey => r"\b[a-zA-Z0-9]{32,}\b".to_string(),
            DlpPattern::Custom(p) => p.clone(),
        }
    }

    /// Get context around a match
    fn get_context(content: &str, start: usize, end: usize) -> String {
        let context_before = 20;
        let context_after = 20;
        let ctx_start = start.saturating_sub(context_before);
        let ctx_end = (end + context_after).min(content.len());
        content[ctx_start..ctx_end].to_string()
    }
}

/// DLP rule types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DlpRuleType {
    Regex { pattern: String },
    Keyword { keywords: Vec<String>, case_sensitive: bool },
    Pattern { pattern_type: DlpPattern },
}

/// DLP pattern types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DlpPattern {
    CreditCard,
    SSN,
    Email,
    PhoneNumber,
    IpAddress,
    ApiKey,
    Custom(String),
}

/// DLP action
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DlpAction {
    Log,
    Warn,
    Block,
    Quarantine,
}

/// DLP exception
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DlpException {
    pub exception_type: DlpExceptionType,
}

impl DlpException {
    pub fn matches(&self, _context: &DlpContext) -> bool {
        match &self.exception_type {
            DlpExceptionType::AuthorizedUser(_) => false, // Would check context
            DlpExceptionType::AuthorizedDestination(_) => false,
            DlpExceptionType::TimeWindow { .. } => false,
        }
    }
}

/// DLP exception types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DlpExceptionType {
    AuthorizedUser(String),
    AuthorizedDestination(String),
    TimeWindow { start_hour: u32, end_hour: u32 },
}

/// DLP context
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DlpContext {
    pub user_id: String,
    pub source: String,
    pub destination: String,
    pub operation: DataOperation,
}

/// DLP match
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DlpMatch {
    pub rule_id: String,
    pub matched_text: String,
    pub start_pos: usize,
    pub end_pos: usize,
    pub context: String,
}

/// DLP scan result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DlpScanResult {
    pub matches: Vec<DlpMatch>,
    pub risk_score: f64,
    pub action: DlpAction,
}

impl DlpScanResult {
    pub fn has_violations(&self) -> bool {
        !self.matches.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_data_classification() {
        let classification = DataClassification::new("confidential", "Confidential", 3)
            .with_required_role("data_reader")
            .with_required_attribute("department", "finance");

        assert_eq!(classification.level, 3);
        assert!(classification.required_roles.contains("data_reader"));
        assert!(classification.encryption_required);
    }

    #[test]
    fn test_user_attributes() {
        let user = UserAttributes::new("user1")
            .with_clearance(3)
            .with_role("admin")
            .with_attribute("department", "engineering");

        assert_eq!(user.clearance_level, 3);
        assert!(user.roles.contains("admin"));
    }

    #[test]
    fn test_data_segment() {
        let segment = DataSegment::new("customer-data", "Customer Data", "confidential")
            .with_encryption_zone("zone-1")
            .allow_operation(DataOperation::Read);

        assert_eq!(segment.classification_id, "confidential");
        assert!(segment.encryption_required);
        assert!(segment.allowed_operations.contains(&DataOperation::Read));
    }

    #[test]
    fn test_data_access_check() {
        let mut segmenter = DataSegmenter::new();

        // Add classification
        segmenter.register_classification(
            DataClassification::new("restricted", "Restricted", 4)
                .with_required_role("data_admin")
        );

        // Add segment
        segmenter.create_segment(
            DataSegment::new("secrets", "Secrets", "restricted")
        );

        // User without required role
        let user = UserAttributes::new("user1").with_clearance(4);
        let result = segmenter.check_access("user1", &user, "secrets", DataOperation::Read);
        
        assert!(!result.is_allowed());
    }

    #[test]
    fn test_dlp_credit_card_detection() {
        let rule = DlpRule {
            id: "cc-rule".to_string(),
            name: "Credit Card Detection".to_string(),
            description: "Detects credit card numbers".to_string(),
            is_enabled: true,
            rule_type: DlpRuleType::Pattern { 
                pattern_type: DlpPattern::CreditCard 
            },
            risk_weight: 0.8,
            action: DlpAction::Block,
            exceptions: vec![],
        };

        let content = "My card number is 4532-1234-5678-9010 and it's valid.";
        let matches = rule.scan(content);

        assert!(!matches.is_empty());
    }

    #[test]
    fn test_dlp_email_detection() {
        let rule = DlpRule {
            id: "email-rule".to_string(),
            name: "Email Detection".to_string(),
            description: "Detects email addresses".to_string(),
            is_enabled: true,
            rule_type: DlpRuleType::Pattern { 
                pattern_type: DlpPattern::Email 
            },
            risk_weight: 0.5,
            action: DlpAction::Warn,
            exceptions: vec![],
        };

        let content = "Contact us at support@example.com for help.";
        let matches = rule.scan(content);

        assert!(!matches.is_empty());
        assert!(matches[0].matched_text.contains("@"));
    }

    #[test]
    fn test_encryption_zone() {
        let zone = EncryptionZone::new("zone-1", "Primary Zone")
            .authorize_user("user1");

        assert!(zone.is_user_authorized("user1"));
        assert!(!zone.is_user_authorized("user2"));
    }

    #[test]
    fn test_user_selector_matching() {
        let user = UserAttributes::new("user1")
            .with_role("admin")
            .with_clearance(3);

        assert!(UserSelector::All.matches(&user));
        assert!(UserSelector::User("user1".to_string()).matches(&user));
        assert!(UserSelector::Role("admin".to_string()).matches(&user));
        assert!(UserSelector::Clearance { min: 2, max: None }.matches(&user));
        assert!(!UserSelector::Clearance { min: 4, max: None }.matches(&user));
    }
}