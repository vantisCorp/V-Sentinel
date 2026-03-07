//! Micro-Segmentation Module
//! 
//! Implements network, application, and data segmentation following
//! Zero Trust principles. Each segment has its own access policies.

use anyhow::{Result, anyhow};
use serde::{Serialize, Deserialize};
use std::collections::{HashMap, HashSet};
use tracing::{info, debug, warn};
use chrono::{DateTime, Utc};

use super::{Subject, Resource, SensitivityLevel};

/// Segmentation Engine
/// 
/// Manages segments and enforces segmentation policies
pub struct SegmentationEngine {
    segments: HashMap<String, Segment>,
    segment_policies: HashMap<String, SegmentPolicy>,
    subject_memberships: HashMap<String, HashSet<String>>,
    resource_memberships: HashMap<String, HashSet<String>>,
    traffic_rules: Vec<TrafficRule>,
}

/// Segment definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Segment {
    /// Segment ID
    pub id: String,
    /// Segment name
    pub name: String,
    /// Segment type
    pub segment_type: SegmentType,
    /// Description
    pub description: String,
    /// Sensitivity level
    pub sensitivity: SensitivityLevel,
    /// Members (subject or resource IDs)
    pub members: HashSet<String>,
    /// Allowed connections
    pub allowed_connections: HashSet<String>,
    /// Denied connections
    pub denied_connections: HashSet<String>,
    /// Created at
    pub created_at: DateTime<Utc>,
    /// Updated at
    pub updated_at: DateTime<Utc>,
    /// Is active
    pub is_active: bool,
}

/// Segment types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SegmentType {
    Network,
    Application,
    Data,
    Workload,
    Identity,
    Hybrid,
}

/// Segment policy
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SegmentPolicy {
    /// Policy ID
    pub id: String,
    /// Segment ID
    pub segment_id: String,
    /// Policy name
    pub name: String,
    /// Default action
    pub default_action: SegmentAction,
    /// Ingress rules
    pub ingress_rules: Vec<SegmentRule>,
    /// Egress rules
    pub egress_rules: Vec<SegmentRule>,
    /// Require encryption
    pub require_encryption: bool,
    /// Require authentication
    pub require_authentication: bool,
    /// Max concurrent connections
    pub max_concurrent_connections: Option<u32>,
    /// Rate limit (requests per minute)
    pub rate_limit: Option<u32>,
}

/// Segment rule
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SegmentRule {
    /// Rule ID
    pub id: String,
    /// Rule name
    pub name: String,
    /// Source segment
    pub source_segment: Option<String>,
    /// Destination segment
    pub dest_segment: Option<String>,
    /// Protocol
    pub protocol: Option<Protocol>,
    /// Port range
    pub port_range: Option<PortRange>,
    /// Action
    pub action: SegmentAction,
    /// Conditions
    pub conditions: Vec<RuleCondition>,
    /// Priority
    pub priority: u32,
    /// Enabled
    pub enabled: bool,
}

/// Segment actions
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum SegmentAction {
    Allow,
    Deny,
    Log,
    Alert,
    Challenge,
}

/// Protocol
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Protocol {
    TCP,
    UDP,
    ICMP,
    HTTP,
    HTTPS,
    GRPC,
    WebSocket,
    Custom(String),
}

/// Port range
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PortRange {
    pub start: u16,
    pub end: u16,
}

/// Rule condition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RuleCondition {
    pub field: String,
    pub operator: ConditionOperator,
    pub value: serde_json::Value,
}

/// Condition operators
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ConditionOperator {
    Equals,
    NotEquals,
    Contains,
    Matches,
    GreaterThan,
    LessThan,
}

/// Traffic rule for inter-segment communication
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrafficRule {
    /// Rule ID
    pub id: String,
    /// Source segment
    pub source_segment: String,
    /// Destination segment
    pub dest_segment: String,
    /// Allowed
    pub allowed: bool,
    /// Required trust level
    pub required_trust_level: f64,
    /// Time restrictions
    pub time_restrictions: Vec<TimeRestriction>,
}

/// Time restriction
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimeRestriction {
    pub start_hour: i32,
    pub end_hour: i32,
    pub days: Vec<chrono::Weekday>,
}

/// Access check result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SegmentationResult {
    pub allowed: bool,
    pub source_segment: String,
    pub dest_segment: String,
    pub matched_rules: Vec<String>,
    pub warnings: Vec<String>,
}

impl SegmentationEngine {
    /// Create a new Segmentation Engine
    pub fn new() -> Self {
        let mut engine = Self {
            segments: HashMap::new(),
            segment_policies: HashMap::new(),
            subject_memberships: HashMap::new(),
            resource_memberships: HashMap::new(),
            traffic_rules: Vec::new(),
        };
        
        // Add default segments
        engine.add_default_segments();
        engine
    }

    /// Add default segments
    fn add_default_segments(&mut self) {
        // DMZ segment
        let dmz = Segment {
            id: "dmz".to_string(),
            name: "DMZ".to_string(),
            segment_type: SegmentType::Network,
            description: "Demilitarized Zone - public-facing services".to_string(),
            sensitivity: SensitivityLevel::Public,
            members: HashSet::new(),
            allowed_connections: HashSet::new(),
            denied_connections: HashSet::new(),
            created_at: Utc::now(),
            updated_at: Utc::now(),
            is_active: true,
        };
        self.segments.insert(dmz.id.clone(), dmz);
        
        // Production segment
        let production = Segment {
            id: "production".to_string(),
            name: "Production".to_string(),
            segment_type: SegmentType::Hybrid,
            description: "Production environment".to_string(),
            sensitivity: SensitivityLevel::Confidential,
            members: HashSet::new(),
            allowed_connections: ["dmz".to_string()].iter().cloned().collect(),
            denied_connections: HashSet::new(),
            created_at: Utc::now(),
            updated_at: Utc::now(),
            is_active: true,
        };
        self.segments.insert(production.id.clone(), production);
        
        // Development segment
        let development = Segment {
            id: "development".to_string(),
            name: "Development".to_string(),
            segment_type: SegmentType::Hybrid,
            description: "Development environment".to_string(),
            sensitivity: SensitivityLevel::Internal,
            members: HashSet::new(),
            allowed_connections: HashSet::new(),
            denied_connections: ["production".to_string()].iter().cloned().collect(),
            created_at: Utc::now(),
            updated_at: Utc::now(),
            is_active: true,
        };
        self.segments.insert(development.id.clone(), development);
        
        // Data segment
        let data = Segment {
            id: "data".to_string(),
            name: "Data".to_string(),
            segment_type: SegmentType::Data,
            description: "Sensitive data storage".to_string(),
            sensitivity: SensitivityLevel::Restricted,
            members: HashSet::new(),
            allowed_connections: ["production".to_string()].iter().cloned().collect(),
            denied_connections: ["development".to_string(), "dmz".to_string()].iter().cloned().collect(),
            created_at: Utc::now(),
            updated_at: Utc::now(),
            is_active: true,
        };
        self.segments.insert(data.id.clone(), data);
        
        // Admin segment
        let admin = Segment {
            id: "admin".to_string(),
            name: "Admin".to_string(),
            segment_type: SegmentType::Identity,
            description: "Administrative access".to_string(),
            sensitivity: SensitivityLevel::TopSecret,
            members: HashSet::new(),
            allowed_connections: ["production".to_string(), "data".to_string()].iter().cloned().collect(),
            denied_connections: HashSet::new(),
            created_at: Utc::now(),
            updated_at: Utc::now(),
            is_active: true,
        };
        self.segments.insert(admin.id.clone(), admin);
        
        // Add default traffic rules
        self.traffic_rules = vec![
            TrafficRule {
                id: "dmz-to-prod".to_string(),
                source_segment: "dmz".to_string(),
                dest_segment: "production".to_string(),
                allowed: true,
                required_trust_level: 0.5,
                time_restrictions: vec![],
            },
            TrafficRule {
                id: "prod-to-data".to_string(),
                source_segment: "production".to_string(),
                dest_segment: "data".to_string(),
                allowed: true,
                required_trust_level: 0.7,
                time_restrictions: vec![],
            },
            TrafficRule {
                id: "admin-to-all".to_string(),
                source_segment: "admin".to_string(),
                dest_segment: "production".to_string(),
                allowed: true,
                required_trust_level: 0.9,
                time_restrictions: vec![TimeRestriction {
                    start_hour: 8,
                    end_hour: 18,
                    days: vec![chrono::Weekday::Mon, chrono::Weekday::Tue, 
                               chrono::Weekday::Wed, chrono::Weekday::Thu, 
                               chrono::Weekday::Fri],
                }],
            },
            TrafficRule {
                id: "dev-blocked-prod".to_string(),
                source_segment: "development".to_string(),
                dest_segment: "production".to_string(),
                allowed: false,
                required_trust_level: 1.0,
                time_restrictions: vec![],
            },
            TrafficRule {
                id: "dmz-blocked-data".to_string(),
                source_segment: "dmz".to_string(),
                dest_segment: "data".to_string(),
                allowed: false,
                required_trust_level: 1.0,
                time_restrictions: vec![],
            },
        ];
    }

    /// Create a new segment
    pub fn create_segment(&mut self, segment: Segment) -> Result<()> {
        if self.segments.contains_key(&segment.id) {
            return Err(anyhow!("Segment with ID {} already exists", segment.id));
        }
        
        info!("Creating segment: {} ({})", segment.name, segment.id);
        self.segments.insert(segment.id.clone(), segment);
        Ok(())
    }

    /// Delete a segment
    pub fn delete_segment(&mut self, segment_id: &str) -> Result<()> {
        if self.segments.remove(segment_id).is_none() {
            return Err(anyhow!("Segment {} not found", segment_id));
        }
        
        // Clean up memberships
        for memberships in self.subject_memberships.values_mut() {
            memberships.remove(segment_id);
        }
        for memberships in self.resource_memberships.values_mut() {
            memberships.remove(segment_id);
        }
        
        info!("Deleted segment: {}", segment_id);
        Ok(())
    }

    /// Add subject to segment
    pub fn add_subject_to_segment(&mut self, subject_id: &str, segment_id: &str) -> Result<()> {
        if !self.segments.contains_key(segment_id) {
            return Err(anyhow!("Segment {} not found", segment_id));
        }
        
        self.subject_memberships
            .entry(subject_id.to_string())
            .or_insert_with(HashSet::new)
            .insert(segment_id.to_string());
        
        // Also add to segment's member list
        if let Some(segment) = self.segments.get_mut(segment_id) {
            segment.members.insert(subject_id.to_string());
        }
        
        debug!("Added subject {} to segment {}", subject_id, segment_id);
        Ok(())
    }

    /// Add resource to segment
    pub fn add_resource_to_segment(&mut self, resource_id: &str, segment_id: &str) -> Result<()> {
        if !self.segments.contains_key(segment_id) {
            return Err(anyhow!("Segment {} not found", segment_id));
        }
        
        self.resource_memberships
            .entry(resource_id.to_string())
            .or_insert_with(HashSet::new)
            .insert(segment_id.to_string());
        
        // Also add to segment's member list
        if let Some(segment) = self.segments.get_mut(segment_id) {
            segment.members.insert(resource_id.to_string());
        }
        
        debug!("Added resource {} to segment {}", resource_id, segment_id);
        Ok(())
    }

    /// Get subject's segments
    pub fn get_subject_segments(&self, subject_id: &str) -> Vec<&Segment> {
        self.subject_memberships
            .get(subject_id)
            .map(|ids| {
                ids.iter()
                    .filter_map(|id| self.segments.get(id))
                    .collect()
            })
            .unwrap_or_default()
    }

    /// Get resource's segments
    pub fn get_resource_segments(&self, resource_id: &str) -> Vec<&Segment> {
        self.resource_memberships
            .get(resource_id)
            .map(|ids| {
                ids.iter()
                    .filter_map(|id| self.segments.get(id))
                    .collect()
            })
            .unwrap_or_default()
    }

    /// Check if access is allowed between subject and resource
    pub async fn check_access(&self, subject: &Subject, resource: &Resource) -> Result<bool> {
        let subject_segments = self.get_subject_segments(&subject.id);
        let resource_segments = self.get_resource_segments(&resource.id);
        
        // If no segments defined, use resource sensitivity for default segment
        if subject_segments.is_empty() && resource_segments.is_empty() {
            return Ok(true); // Default allow if no segmentation configured
        }
        
        // Check each subject segment against each resource segment
        for subject_seg in &subject_segments {
            for resource_seg in &resource_segments {
                let result = self.check_segment_access(subject_seg.id.as_str(), resource_seg.id.as_str()).await?;
                if result.allowed {
                    return Ok(true);
                }
            }
        }
        
        // Check if resource has a segment assigned
        if let Some(ref segment_id) = resource.segment {
            for subject_seg in &subject_segments {
                let result = self.check_segment_access(subject_seg.id.as_str(), segment_id).await?;
                if result.allowed {
                    return Ok(true);
                }
            }
        }
        
        // Check sensitivity-based default rules
        if subject_segments.is_empty() {
            // Unknown subject, apply sensitivity rules
            return Ok(resource.sensitivity <= SensitivityLevel::Internal);
        }
        
        if resource_segments.is_empty() {
            // Unknown resource, allow based on subject segment sensitivity
            return Ok(subject_segments.iter().all(|s| s.sensitivity >= SensitivityLevel::Internal));
        }
        
        Ok(false)
    }

    /// Check access between two segments
    pub async fn check_segment_access(&self, source_segment: &str, dest_segment: &str) -> Result<SegmentationResult> {
        let mut matched_rules = Vec::new();
        let mut warnings = Vec::new();
        
        // Same segment
        if source_segment == dest_segment {
            return Ok(SegmentationResult {
                allowed: true,
                source_segment: source_segment.to_string(),
                dest_segment: dest_segment.to_string(),
                matched_rules: vec!["same-segment".to_string()],
                warnings: vec![],
            });
        }
        
        // Check traffic rules
        for rule in &self.traffic_rules {
            if rule.source_segment == source_segment && rule.dest_segment == dest_segment {
                matched_rules.push(rule.id.clone());
                
                // Check time restrictions
                if !rule.time_restrictions.is_empty() {
                    let now = Utc::now();
                    let hour = now.hour() as i32;
                    let day = now.weekday();
                    
                    let within_time = rule.time_restrictions.iter().any(|r| {
                        hour >= r.start_hour && hour < r.end_hour && r.days.contains(&day)
                    });
                    
                    if !within_time && rule.allowed {
                        warnings.push(format!("Access allowed only during restricted hours"));
                    }
                }
                
                return Ok(SegmentationResult {
                    allowed: rule.allowed,
                    source_segment: source_segment.to_string(),
                    dest_segment: dest_segment.to_string(),
                    matched_rules,
                    warnings,
                });
            }
        }
        
        // Check segment allowed/denied connections
        if let Some(source_seg) = self.segments.get(source_segment) {
            if source_seg.denied_connections.contains(dest_segment) {
                return Ok(SegmentationResult {
                    allowed: false,
                    source_segment: source_segment.to_string(),
                    dest_segment: dest_segment.to_string(),
                    matched_rules: vec![format!("{}-denied-list", source_segment)],
                    warnings: vec!["Destination in denied list".to_string()],
                });
            }
            
            if source_seg.allowed_connections.contains(dest_segment) {
                return Ok(SegmentationResult {
                    allowed: true,
                    source_segment: source_segment.to_string(),
                    dest_segment: dest_segment.to_string(),
                    matched_rules: vec![format!("{}-allowed-list", source_segment)],
                    warnings,
                });
            }
        }
        
        // Default deny
        Ok(SegmentationResult {
            allowed: false,
            source_segment: source_segment.to_string(),
            dest_segment: dest_segment.to_string(),
            matched_rules: vec!["default-deny".to_string()],
            warnings: vec!["No matching rule found, defaulting to deny".to_string()],
        })
    }

    /// Add a traffic rule
    pub fn add_traffic_rule(&mut self, rule: TrafficRule) -> Result<()> {
        // Validate segments exist
        if !self.segments.contains_key(&rule.source_segment) {
            return Err(anyhow!("Source segment {} not found", rule.source_segment));
        }
        if !self.segments.contains_key(&rule.dest_segment) {
            return Err(anyhow!("Destination segment {} not found", rule.dest_segment));
        }
        
        info!("Adding traffic rule: {} -> {} (allowed: {})", 
              rule.source_segment, rule.dest_segment, rule.allowed);
        self.traffic_rules.push(rule);
        Ok(())
    }

    /// Get all segments
    pub fn list_segments(&self) -> Vec<&Segment> {
        self.segments.values().collect()
    }

    /// Get segment by ID
    pub fn get_segment(&self, segment_id: &str) -> Option<&Segment> {
        self.segments.get(segment_id)
    }

    /// Get segment count
    pub fn segment_count(&self) -> usize {
        self.segments.len()
    }
}

impl Default for SegmentationEngine {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_segmentation_engine_creation() {
        let engine = SegmentationEngine::new();
        assert!(!engine.segments.is_empty());
        assert!(engine.segments.contains_key("production"));
    }

    #[tokio::test]
    async fn test_segment_access_check() {
        let engine = SegmentationEngine::new();
        
        // DMZ to production should be allowed
        let result = engine.check_segment_access("dmz", "production").await.unwrap();
        assert!(result.allowed);
        
        // DMZ to data should be denied
        let result = engine.check_segment_access("dmz", "data").await.unwrap();
        assert!(!result.allowed);
        
        // Development to production should be denied
        let result = engine.check_segment_access("development", "production").await.unwrap();
        assert!(!result.allowed);
    }
}