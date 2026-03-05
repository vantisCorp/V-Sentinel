//! Network micro-segmentation for Zero Trust Architecture

use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};

/// Network segmenter for micro-segmentation
pub struct NetworkSegmenter {
    /// Defined segments
    segments: HashMap<String, NetworkSegment>,
    
    /// Segment policies
    policies: Vec<SegmentPolicy>,
}

impl NetworkSegmenter {
    /// Create a new network segmenter
    pub fn new() -> Self {
        Self {
            segments: HashMap::new(),
            policies: Vec::new(),
        }
    }
    
    /// Add a segment
    pub fn add_segment(&mut self, segment: NetworkSegment) {
        self.segments.insert(segment.id.clone(), segment);
    }
    
    /// Add a policy
    pub fn add_policy(&mut self, policy: SegmentPolicy) {
        self.policies.push(policy);
    }
    
    /// Get segment by ID
    pub fn get_segment(&self, id: &str) -> Option<&NetworkSegment> {
        self.segments.get(id)
    }
    
    /// Check if traffic is allowed between segments
    pub fn is_traffic_allowed(
        &self,
        source_segment: &str,
        dest_segment: &str,
        port: u16,
        protocol: Protocol,
    ) -> bool {
        // Find applicable policies
        for policy in &self.policies {
            if policy.applies_to(source_segment, dest_segment) {
                return policy.allows(port, protocol);
            }
        }
        
        // Default deny
        false
    }
    
    /// Get all segments
    pub fn list_segments(&self) -> Vec<&NetworkSegment> {
        self.segments.values().collect()
    }
}

impl Default for NetworkSegmenter {
    fn default() -> Self {
        Self::new()
    }
}

/// Network segment definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkSegment {
    /// Segment ID
    pub id: String,
    
    /// Segment name
    pub name: String,
    
    /// Segment type
    pub segment_type: SegmentType,
    
    /// CIDR ranges
    pub cidrs: Vec<String>,
    
    /// Allowed ports
    pub allowed_ports: Vec<u16>,
    
    /// Allowed protocols
    pub allowed_protocols: Vec<Protocol>,
    
    /// Parent segment (for hierarchical segmentation)
    pub parent: Option<String>,
    
    /// Tags for policy matching
    pub tags: HashSet<String>,
}

impl NetworkSegment {
    /// Create a new segment
    pub fn new(id: impl Into<String>, name: impl Into<String>, segment_type: SegmentType) -> Self {
        Self {
            id: id.into(),
            name: name.into(),
            segment_type,
            cidrs: Vec::new(),
            allowed_ports: Vec::new(),
            allowed_protocols: Vec::new(),
            parent: None,
            tags: HashSet::new(),
        }
    }
    
    /// Add CIDR range
    pub fn with_cidr(mut self, cidr: impl Into<String>) -> Self {
        self.cidrs.push(cidr.into());
        self
    }
    
    /// Add allowed port
    pub fn with_port(mut self, port: u16) -> Self {
        self.allowed_ports.push(port);
        self
    }
    
    /// Add tag
    pub fn with_tag(mut self, tag: impl Into<String>) -> Self {
        self.tags.insert(tag.into());
        self
    }
}

/// Segment type classification
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SegmentType {
    /// Corporate network
    Corporate,
    /// Production environment
    Production,
    /// Development environment
    Development,
    /// DMZ
    Dmz,
    /// Data center
    DataCenter,
    /// Cloud
    Cloud,
    /// IoT devices
    IoT,
    /// Guest network
    Guest,
    /// Management network
    Management,
}

/// Segment policy
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SegmentPolicy {
    /// Policy name
    pub name: String,
    
    /// Source segment selector
    pub source_selector: SegmentSelector,
    
    /// Destination segment selector
    pub dest_selector: SegmentSelector,
    
    /// Allowed ports
    pub allowed_ports: Vec<u16>,
    
    /// Allowed protocols
    pub allowed_protocols: Vec<Protocol>,
    
    /// Action
    pub action: PolicyAction,
}

impl SegmentPolicy {
    /// Check if policy applies to source/destination
    pub fn applies_to(&self, source: &str, dest: &str) -> bool {
        self.source_selector.matches(source) && self.dest_selector.matches(dest)
    }
    
    /// Check if port/protocol is allowed
    pub fn allows(&self, port: u16, protocol: Protocol) -> bool {
        let port_allowed = self.allowed_ports.is_empty() || self.allowed_ports.contains(&port);
        let proto_allowed = self.allowed_protocols.is_empty() || self.allowed_protocols.contains(&protocol);
        
        port_allowed && proto_allowed && matches!(self.action, PolicyAction::Allow)
    }
}

/// Segment selector
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SegmentSelector {
    /// Match all segments
    Any,
    /// Match specific segment
    Id(String),
    /// Match segments with tag
    Tag(String),
}

impl SegmentSelector {
    /// Check if selector matches segment
    pub fn matches(&self, segment_id: &str) -> bool {
        match self {
            SegmentSelector::Any => true,
            SegmentSelector::Id(id) => id == segment_id,
            SegmentSelector::Tag(_) => true, // Would need segment tags to check
        }
    }
}

/// Network protocol
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Protocol {
    TCP,
    UDP,
    ICMP,
    Any,
}

/// Policy action
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PolicyAction {
    Allow,
    Deny,
    Log,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_segment_creation() {
        let segment = NetworkSegment::new("prod-1", "Production Segment 1", SegmentType::Production)
            .with_cidr("10.0.1.0/24")
            .with_port(443)
            .with_tag("production");
        
        assert_eq!(segment.id, "prod-1");
        assert_eq!(segment.cidrs.len(), 1);
        assert!(segment.tags.contains("production"));
    }

    #[test]
    fn test_segmenter() {
        let mut segmenter = NetworkSegmenter::new();
        
        let prod = NetworkSegment::new("prod", "Production", SegmentType::Production)
            .with_cidr("10.0.1.0/24");
        
        let dev = NetworkSegment::new("dev", "Development", SegmentType::Development)
            .with_cidr("10.0.2.0/24");
        
        segmenter.add_segment(prod);
        segmenter.add_segment(dev);
        
        // Add policy to allow HTTPS from dev to prod
        let policy = SegmentPolicy {
            name: "dev-to-prod-https".to_string(),
            source_selector: SegmentSelector::Id("dev".to_string()),
            dest_selector: SegmentSelector::Id("prod".to_string()),
            allowed_ports: vec![443],
            allowed_protocols: vec![Protocol::TCP],
            action: PolicyAction::Allow,
        };
        
        segmenter.add_policy(policy);
        
        // Test traffic check
        assert!(segmenter.is_traffic_allowed("dev", "prod", 443, Protocol::TCP));
        assert!(!segmenter.is_traffic_allowed("dev", "prod", 80, Protocol::TCP));
    }
}