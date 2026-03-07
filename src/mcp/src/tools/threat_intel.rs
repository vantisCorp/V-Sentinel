//! Threat intelligence tool for AI-driven threat analysis

use crate::error::MCPResult;
use crate::tools::ToolTrait;
use crate::types::{Tool, ToolResult};
use serde::{Deserialize, Serialize};

/// Threat intelligence query parameters
#[derive(Debug, Deserialize)]
pub struct ThreatIntelParams {
    /// IOC (Indicator of Compromise) to lookup
    #[serde(default)]
    pub ioc: Option<String>,
    
    /// IOC type
    #[serde(default)]
    pub ioc_type: Option<String>,
    
    /// Threat actor name
    #[serde(default)]
    pub threat_actor: Option<String>,
    
    /// Malware family
    #[serde(default)]
    pub malware_family: Option<String>,
}

/// Threat intelligence data
#[derive(Debug, Serialize)]
pub struct ThreatIntel {
    /// IOC value
    pub ioc: String,
    
    /// IOC type (ip, domain, hash, url)
    pub ioc_type: String,
    
    /// Threat actor (if known)
    pub threat_actor: Option<String>,
    
    /// Malware family (if known)
    pub malware_family: Option<String>,
    
    /// First seen timestamp
    pub first_seen: String,
    
    /// Last seen timestamp
    pub last_seen: String,
    
    /// Confidence score (0-100)
    pub confidence: u8,
    
    /// Severity (critical, high, medium, low)
    pub severity: String,
    
    /// Additional context
    pub context: Vec<String>,
}

/// Threat intelligence tool
pub struct ThreatIntelTool;

#[async_trait::async_trait]
impl ToolTrait for ThreatIntelTool {
    fn get_tool(&self) -> Tool {
        Tool {
            name: "sentinel_threat_intel".to_string(),
            description: "Query threat intelligence database for IOCs, threat actors, and malware families".to_string(),
            input_schema: serde_json::json!({
                "type": "object",
                "properties": {
                    "ioc": {
                        "type": "string",
                        "description": "Indicator of Compromise (IP, domain, hash, URL)"
                    },
                    "ioc_type": {
                        "type": "string",
                        "description": "Type of IOC",
                        "enum": ["ip", "domain", "hash", "url"]
                    },
                    "threat_actor": {
                        "type": "string",
                        "description": "Threat actor name"
                    },
                    "malware_family": {
                        "type": "string",
                        "description": "Malware family name"
                    }
                },
                "required": []
            }),
            requires_auth: true,
        }
    }
    
    async fn execute(&self, params: serde_json::Value) -> MCPResult<ToolResult> {
        let params: ThreatIntelParams = serde_json::from_value(params)
            .map_err(|e| crate::error::MCPError::InvalidParameters(format!("Invalid parameters: {}", e)))?;
        
        let threat_intel = self.mock_query_threat_intel(&params).await?;
        
        let result = serde_json::json!({
            "threat_intel": threat_intel,
            "total": threat_intel.len()
        });
        
        Ok(ToolResult::success(result))
    }
}

impl ThreatIntelTool {
    async fn mock_query_threat_intel(&self, params: &ThreatIntelParams) -> Result<Vec<ThreatIntel>, crate::error::MCPError> {
        let threat_intel = vec![
            ThreatIntel {
                ioc: "192.168.1.100".to_string(),
                ioc_type: "ip".to_string(),
                threat_actor: Some("APT29".to_string()),
                malware_family: Some("Emotet".to_string()),
                first_seen: "2026-02-01T00:00:00Z".to_string(),
                last_seen: "2026-03-06T10:00:00Z".to_string(),
                confidence: 95,
                severity: "critical".to_string(),
                context: vec![
                    "Known C2 server for APT29".to_string(),
                    "Associated with Emotet botnet".to_string(),
                    "Active in financial sector attacks".to_string(),
                ],
            },
            ThreatIntel {
                ioc: "malicious-domain.com".to_string(),
                ioc_type: "domain".to_string(),
                threat_actor: Some("Lazarus Group".to_string()),
                malware_family: None,
                first_seen: "2026-01-15T00:00:00Z".to_string(),
                last_seen: "2026-03-05T18:00:00Z".to_string(),
                confidence: 88,
                severity: "high".to_string(),
                context: vec![
                    "Phishing infrastructure".to_string(),
                    "Used in credential harvesting".to_string(),
                    "Associated with North Korean state-sponsored activity".to_string(),
                ],
            },
            ThreatIntel {
                ioc: "a1b2c3d4e5f6...".to_string(),
                ioc_type: "hash".to_string(),
                threat_actor: None,
                malware_family: Some("TrickBot".to_string()),
                first_seen: "2026-02-20T00:00:00Z".to_string(),
                last_seen: "2026-03-04T12:00:00Z".to_string(),
                confidence: 92,
                severity: "high".to_string(),
                context: vec![
                    "TrickBot payload hash".to_string(),
                    "Banking trojan variant".to_string(),
                    "Spreads via phishing emails".to_string(),
                ],
            },
        ];
        
        Ok(threat_intel)
    }
}