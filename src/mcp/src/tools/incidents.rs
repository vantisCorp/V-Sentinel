//! Incident management tool for AI-driven incident response

use crate::error::MCPResult;
use crate::tools::ToolTrait;
use crate::types::{Tool, ToolResult};
use serde::{Deserialize, Serialize};

/// Incident search parameters
#[derive(Debug, Deserialize)]
pub struct IncidentsParams {
    /// Incident ID
    #[serde(default)]
    pub incident_id: Option<String>,

    /// Filter by severity
    #[serde(default)]
    pub severity: Option<String>,

    /// Filter by status
    #[serde(default)]
    pub status: Option<String>,

    /// Maximum number of results
    #[serde(default = "default_limit")]
    pub limit: usize,
}

fn default_limit() -> usize {
    50
}

/// Incident information
#[derive(Debug, Serialize)]
pub struct Incident {
    /// Incident ID
    pub id: String,

    /// Incident name
    pub name: String,

    /// Description
    pub description: String,

    /// Severity (critical, high, medium, low)
    pub severity: String,

    /// Status (new, in_progress, resolved, closed)
    pub status: String,

    /// Creation timestamp
    pub created_timestamp: String,

    /// Assigned user
    pub assigned_to: Option<String>,

    /// Associated detections
    pub detection_ids: Vec<String>,

    /// Incident tags
    pub tags: Vec<String>,
}

/// Incident management tool
pub struct IncidentsTool;

#[async_trait::async_trait]
impl ToolTrait for IncidentsTool {
    fn get_tool(&self) -> Tool {
        Tool {
            name: "sentinel_list_incidents".to_string(),
            description:
                "List and manage security incidents with filtering and search capabilities"
                    .to_string(),
            input_schema: serde_json::json!({
                "type": "object",
                "properties": {
                    "incident_id": {
                        "type": "string",
                        "description": "Specific incident ID to lookup"
                    },
                    "severity": {
                        "type": "string",
                        "description": "Filter by severity",
                        "enum": ["critical", "high", "medium", "low"]
                    },
                    "status": {
                        "type": "string",
                        "description": "Filter by status",
                        "enum": ["new", "in_progress", "resolved", "closed"]
                    },
                    "limit": {
                        "type": "integer",
                        "description": "Maximum number of results",
                        "default": 50,
                        "maximum": 500
                    }
                },
                "required": []
            }),
            requires_auth: true,
        }
    }

    async fn execute(&self, params: serde_json::Value) -> MCPResult<ToolResult> {
        let params: IncidentsParams = serde_json::from_value(params).map_err(|e| {
            crate::error::MCPError::InvalidParameters(format!("Invalid parameters: {}", e))
        })?;

        let incidents = self.mock_list_incidents(&params).await?;

        let result = serde_json::json!({
            "incidents": incidents,
            "total": incidents.len()
        });

        Ok(ToolResult::success(result))
    }
}

impl IncidentsTool {
    async fn mock_list_incidents(
        &self,
        params: &IncidentsParams,
    ) -> Result<Vec<Incident>, crate::error::MCPError> {
        let incidents = vec![
            Incident {
                id: "inc_001".to_string(),
                name: "Malware Outbreak - Emotet".to_string(),
                description: "Multiple hosts infected with Emotet trojan via phishing email"
                    .to_string(),
                severity: "critical".to_string(),
                status: "in_progress".to_string(),
                created_timestamp: "2026-03-06T08:00:00Z".to_string(),
                assigned_to: Some("SOC Team Lead".to_string()),
                detection_ids: vec!["det_001".to_string(), "det_002".to_string()],
                tags: vec![
                    "malware".to_string(),
                    "phishing".to_string(),
                    "emotet".to_string(),
                ],
            },
            Incident {
                id: "inc_002".to_string(),
                name: "Unauthorized Access Attempt".to_string(),
                description: "Failed login attempts from external IP address".to_string(),
                severity: "high".to_string(),
                status: "new".to_string(),
                created_timestamp: "2026-03-06T09:15:00Z".to_string(),
                assigned_to: None,
                detection_ids: vec!["det_003".to_string()],
                tags: vec!["brute-force".to_string(), "external".to_string()],
            },
            Incident {
                id: "inc_003".to_string(),
                name: "Suspicious Data Exfiltration".to_string(),
                description: "Large volume of data transferred to unknown external endpoint"
                    .to_string(),
                severity: "high".to_string(),
                status: "in_progress".to_string(),
                created_timestamp: "2026-03-05T14:30:00Z".to_string(),
                assigned_to: Some("Security Analyst".to_string()),
                detection_ids: vec!["det_004".to_string()],
                tags: vec!["exfiltration".to_string(), "data-loss".to_string()],
            },
        ];

        Ok(incidents)
    }
}
