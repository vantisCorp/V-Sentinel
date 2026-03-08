use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use crate::deepfake::{
    MediaType, DeepfakeError, DeepfakeAlert, AlertSeverity,
};

/// Configuration for alerting
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlertConfig {
    /// Enable email alerts
    pub enable_email: bool,
    /// Enable Slack alerts
    pub enable_slack: bool,
    /// Enable webhook alerts
    pub enable_webhook: bool,
    /// Webhook URL
    pub webhook_url: Option<String>,
    /// Slack webhook URL
    pub slack_webhook: Option<String>,
    /// Email recipients
    pub email_recipients: Vec<String>,
    /// Minimum severity for alerts
    pub min_severity: AlertSeverity,
}

impl Default for AlertConfig {
    fn default() -> Self {
        Self {
            enable_email: true,
            enable_slack: false,
            enable_webhook: false,
            webhook_url: None,
            slack_webhook: None,
            email_recipients: vec!["security@example.com".to_string()],
            min_severity: AlertSeverity::Medium,
        }
    }
}

/// Threat Intelligence Integrator
pub struct ThreatIntegrator {
    config: AlertConfig,
    /// Incident database
    incidents: Arc<RwLock<HashMap<String, DeepfakeIncident>>>,
    /// Alert history
    alert_history: Arc<RwLock<Vec<DeepfakeAlert>>>,
    /// Threat indicators
    threat_indicators: Arc<RwLock<HashMap<String, ThreatIndicator>>>,
}

impl ThreatIntegrator {
    /// Create new threat integrator
    pub fn new(config: AlertConfig) -> Self {
        Self {
            config,
            incidents: Arc::new(RwLock::new(HashMap::new())),
            alert_history: Arc::new(RwLock::new(Vec::new())),
            threat_indicators: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Create an incident for detected deepfake
    pub async fn create_incident(&self, deepfake_info: &DeepfakeDetectionInfo) -> Result<DeepfakeIncident, DeepfakeError> {
        let incident_id = uuid::Uuid::new_v4().to_string();
        
        // Determine severity based on confidence
        let severity = match deepfake_info.confidence {
            c if c >= 0.9 => IncidentSeverity::Critical,
            c if c >= 0.75 => IncidentSeverity::High,
            c if c >= 0.5 => IncidentSeverity::Medium,
            _ => IncidentSeverity::Low,
        };

        let incident = DeepfakeIncident {
            incident_id: incident_id.clone(),
            timestamp: Utc::now(),
            media_type: deepfake_info.media_type,
            source_file: deepfake_info.source_file.clone(),
            confidence: deepfake_info.confidence,
            severity,
            affected_entities: deepfake_info.affected_entities.clone(),
            threat_intel: None,
            response_status: ResponseStatus::Detected,
        };

        // Store incident
        self.incidents.write().await.insert(incident_id, incident.clone());

        // Create alert
        self.create_alert_for_incident(&incident).await?;

        Ok(incident)
    }

    /// Create alert for incident
    async fn create_alert_for_incident(&self, incident: &DeepfakeIncident) -> Result<DeepfakeAlert, DeepfakeError> {
        let alert = DeepfakeAlert {
            alert_id: uuid::Uuid::new_v4().to_string(),
            timestamp: Utc::now(),
            severity: self.map_incident_severity(&incident.severity),
            media_type: incident.media_type,
            file_path: Some(incident.source_file.clone()),
            confidence: incident.confidence,
            message: format!(
                "Deepfake detected: {} confidence {:.2}% in {}",
                incident.media_type,
                incident.confidence * 100.0,
                incident.source_file
            ),
        };

        // Send alert through configured channels
        if self.should_send_alert(&alert.severity) {
            self.send_alert(&alert).await?;
        }

        // Store alert
        self.alert_history.write().await.push(alert.clone());

        Ok(alert)
    }

    /// Send alert through configured channels
    async fn send_alert(&self, alert: &DeepfakeAlert) -> Result<(), DeepfakeError> {
        if self.config.enable_email {
            self.send_email_alert(alert).await?;
        }

        if self.config.enable_slack {
            if let Some(webhook) = &self.config.slack_webhook {
                self.send_slack_alert(alert, webhook).await?;
            }
        }

        if self.config.enable_webhook {
            if let Some(url) = &self.config.webhook_url {
                self.send_webhook_alert(alert, url).await?;
            }
        }

        Ok(())
    }

    /// Send email alert
    async fn send_email_alert(&self, alert: &DeepfakeAlert) -> Result<(), DeepfakeError> {
        // In a real implementation, this would send an actual email
        log::info!(
            "Email alert sent to {:?}: [{}] {}",
            self.config.email_recipients,
            alert.severity,
            alert.message
        );
        Ok(())
    }

    /// Send Slack alert
    async fn send_slack_alert(&self, alert: &DeepfakeAlert, webhook: &str) -> Result<(), DeepfakeError> {
        // In a real implementation, this would send to Slack webhook
        log::info!(
            "Slack alert sent to {}: [{}] {}",
            webhook,
            alert.severity,
            alert.message
        );
        Ok(())
    }

    /// Send webhook alert
    async fn send_webhook_alert(&self, alert: &DeepfakeAlert, url: &str) -> Result<(), DeepfakeError> {
        // In a real implementation, this would send HTTP POST to webhook
        log::info!(
            "Webhook alert sent to {}: [{}] {}",
            url,
            alert.severity,
            alert.message
        );
        Ok(())
    }

    /// Check if alert should be sent based on severity
    fn should_send_alert(&self, severity: &AlertSeverity) -> bool {
        match (&self.config.min_severity, severity) {
            (AlertSeverity::Critical, AlertSeverity::Critical) => true,
            (AlertSeverity::High, AlertSeverity::Critical | AlertSeverity::High) => true,
            (AlertSeverity::Medium, AlertSeverity::Critical | AlertSeverity::High | AlertSeverity::Medium) => true,
            (AlertSeverity::Low, _) => true,
            _ => false,
        }
    }

    /// Map incident severity to alert severity
    fn map_incident_severity(&self, severity: &IncidentSeverity) -> AlertSeverity {
        match severity {
            IncidentSeverity::Critical => AlertSeverity::Critical,
            IncidentSeverity::High => AlertSeverity::High,
            IncidentSeverity::Medium => AlertSeverity::Medium,
            IncidentSeverity::Low => AlertSeverity::Low,
        }
    }

    /// Update incident status
    pub async fn update_incident_status(&self, incident_id: &str, status: ResponseStatus) -> Result<DeepfakeIncident, DeepfakeError> {
        let mut incidents = self.incidents.write().await;
        let incident = incidents.get_mut(incident_id)
            .ok_or_else(|| DeepfakeError::IntegrationError(
                format!("Incident {} not found", incident_id)
            ))?;

        incident.response_status = status;
        Ok(incident.clone())
    }

    /// Add threat indicator
    pub async fn add_threat_indicator(&self, indicator: ThreatIndicator) -> Result<(), DeepfakeError> {
        let indicator_id = indicator.indicator_id.clone();
        self.threat_indicators.write().await.insert(indicator_id, indicator);
        Ok(())
    }

    /// Check if content matches known threats
    pub async fn check_threat_match(&self, content_hash: &str) -> Option<ThreatIndicator> {
        let indicators = self.threat_indicators.read().await;
        indicators.values()
            .find(|i| i.content_hash.as_deref() == Some(content_hash))
            .cloned()
    }

    /// Get incident by ID
    pub async fn get_incident(&self, incident_id: &str) -> Option<DeepfakeIncident> {
        self.incidents.read().await.get(incident_id).cloned()
    }

    /// Get all incidents
    pub async fn get_all_incidents(&self) -> Vec<DeepfakeIncident> {
        self.incidents.read().await.values().cloned().collect()
    }

    /// Get alerts
    pub async fn get_alerts(&self) -> Vec<DeepfakeAlert> {
        self.alert_history.read().await.clone()
    }
}

/// Deepfake incident
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeepfakeIncident {
    /// Incident ID
    pub incident_id: String,
    /// Incident timestamp
    pub timestamp: DateTime<Utc>,
    /// Media type
    pub media_type: MediaType,
    /// Source file
    pub source_file: String,
    /// Detection confidence
    pub confidence: f64,
    /// Incident severity
    pub severity: IncidentSeverity,
    /// Affected entities
    pub affected_entities: Vec<String>,
    /// Threat intelligence
    pub threat_intel: Option<ThreatIntelligence>,
    /// Response status
    pub response_status: ResponseStatus,
}

/// Incident severity
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum IncidentSeverity {
    Low,
    Medium,
    High,
    Critical,
}

/// Threat intelligence
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThreatIntelligence {
    /// Campaign ID
    pub campaign_id: Option<String>,
    /// Threat actor
    pub threat_actor: Option<String>,
    /// Attack technique
    pub attack_technique: Option<String>,
    /// Indicators of compromise
    pub iocs: Vec<String>,
    /// Related incidents
    pub related_incidents: Vec<String>,
}

/// Response status
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ResponseStatus {
    Detected,
    Investigating,
    Contained,
    Remediated,
    FalsePositive,
    Closed,
}

/// Threat indicator
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThreatIndicator {
    /// Indicator ID
    pub indicator_id: String,
    /// Content hash (if applicable)
    pub content_hash: Option<String>,
    /// Threat type
    pub threat_type: ThreatType,
    /// Source
    pub source: String,
    /// Confidence
    pub confidence: f64,
    /// Timestamp
    pub timestamp: DateTime<Utc>,
    /// Description
    pub description: String,
}

/// Threat type
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ThreatType {
    DeepfakeCampaign,
    Disinformation,
    IdentityFraud,
    FinancialFraud,
    ReputationAttack,
    SocialEngineering,
    Other(String),
}

/// Deepfake detection info for incident creation
#[derive(Debug, Clone)]
pub struct DeepfakeDetectionInfo {
    pub media_type: MediaType,
    pub source_file: String,
    pub confidence: f64,
    pub affected_entities: Vec<String>,
}

/// Incident Response Handler
pub struct IncidentResponse {
    /// Response workflows
    workflows: Arc<RwLock<HashMap<String, ResponseWorkflow>>>,
}

impl IncidentResponse {
    /// Create new incident response handler
    pub fn new() -> Self {
        Self {
            workflows: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Create response workflow for incident
    pub async fn create_workflow(&self, incident_id: &str, incident_severity: &IncidentSeverity) -> Result<ResponseWorkflow, DeepfakeError> {
        let workflow_id = uuid::Uuid::new_v4().to_string();

        // Create steps based on severity
        let steps = self.create_response_steps(incident_severity);

        let workflow = ResponseWorkflow {
            workflow_id: workflow_id.clone(),
            incident_id: incident_id.to_string(),
            severity: incident_severity.clone(),
            status: WorkflowStatus::Pending,
            steps,
            current_step: 0,
            created_at: Utc::now(),
            completed_at: None,
        };

        // Store workflow
        self.workflows.write().await.insert(workflow_id, workflow.clone());

        Ok(workflow)
    }

    /// Create response steps based on severity
    fn create_response_steps(&self, severity: &IncidentSeverity) -> Vec<ResponseStep> {
        let mut steps = Vec::new();

        // Common steps
        steps.push(ResponseStep {
            step_id: uuid::Uuid::new_v4().to_string(),
            name: "Initial Assessment".to_string(),
            description: "Assess the detected deepfake and verify detection".to_string(),
            status: StepStatus::Pending,
            order: 0,
        });

        steps.push(ResponseStep {
            step_id: uuid::Uuid::new_v4().to_string(),
            name: "Evidence Collection".to_string(),
            description: "Collect and preserve evidence".to_string(),
            status: StepStatus::Pending,
            order: 1,
        });

        // Severity-specific steps
        match severity {
            IncidentSeverity::Critical => {
                steps.push(ResponseStep {
                    step_id: uuid::Uuid::new_v4().to_string(),
                    name: "Executive Notification".to_string(),
                    description: "Notify executive leadership".to_string(),
                    status: StepStatus::Pending,
                    order: 2,
                });
                steps.push(ResponseStep {
                    step_id: uuid::Uuid::new_v4().to_string(),
                    name: "Public Statement".to_string(),
                    description: "Prepare public statement if needed".to_string(),
                    status: StepStatus::Pending,
                    order: 3,
                });
            }
            IncidentSeverity::High => {
                steps.push(ResponseStep {
                    step_id: uuid::Uuid::new_v4().to_string(),
                    name: "Management Notification".to_string(),
                    description: "Notify management team".to_string(),
                    status: StepStatus::Pending,
                    order: 2,
                });
            }
            _ => {}
        }

        steps.push(ResponseStep {
            step_id: uuid::Uuid::new_v4().to_string(),
            name: "Documentation".to_string(),
            description: "Document incident and response actions".to_string(),
            status: StepStatus::Pending,
            order: 99,
        });

        steps
    }

    /// Execute workflow step
    pub async fn execute_step(&self, workflow_id: &str, step_id: &str) -> Result<ResponseWorkflow, DeepfakeError> {
        let mut workflows = self.workflows.write().await;
        let workflow = workflows.get_mut(workflow_id)
            .ok_or_else(|| DeepfakeError::IntegrationError(
                format!("Workflow {} not found", workflow_id)
            ))?;

        let step = workflow.steps.iter_mut()
            .find(|s| s.step_id == step_id)
            .ok_or_else(|| DeepfakeError::IntegrationError(
                format!("Step {} not found", step_id)
            ))?;

        step.status = StepStatus::Completed;
        workflow.current_step += 1;

        // Check if all steps completed
        if workflow.steps.iter().all(|s| s.status == StepStatus::Completed) {
            workflow.status = WorkflowStatus::Completed;
            workflow.completed_at = Some(Utc::now());
        }

        Ok(workflow.clone())
    }

    /// Get workflow
    pub async fn get_workflow(&self, workflow_id: &str) -> Option<ResponseWorkflow> {
        self.workflows.read().await.get(workflow_id).cloned()
    }
}

/// Response workflow
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResponseWorkflow {
    /// Workflow ID
    pub workflow_id: String,
    /// Associated incident ID
    pub incident_id: String,
    /// Severity
    pub severity: IncidentSeverity,
    /// Workflow status
    pub status: WorkflowStatus,
    /// Response steps
    pub steps: Vec<ResponseStep>,
    /// Current step index
    pub current_step: usize,
    /// Created at
    pub created_at: DateTime<Utc>,
    /// Completed at
    pub completed_at: Option<DateTime<Utc>>,
}

/// Workflow status
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum WorkflowStatus {
    Pending,
    InProgress,
    Completed,
    Failed,
    Cancelled,
}

/// Response step
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResponseStep {
    /// Step ID
    pub step_id: String,
    /// Step name
    pub name: String,
    /// Step description
    pub description: String,
    /// Step status
    pub status: StepStatus,
    /// Step order
    pub order: u32,
}

/// Step status
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum StepStatus {
    Pending,
    InProgress,
    Completed,
    Failed,
    Skipped,
}

/// Deepfake Monitoring Service
pub struct DeepfakeMonitoring {
    /// Monitoring targets
    targets: Arc<RwLock<Vec<MonitoringTarget>>>,
    /// Monitoring results
    results: Arc<RwLock<HashMap<String, MonitoringResult>>>,
}

impl DeepfakeMonitoring {
    /// Create new monitoring service
    pub fn new() -> Self {
        Self {
            targets: Arc::new(RwLock::new(Vec::new())),
            results: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Add monitoring target
    pub async fn add_target(&self, target: MonitoringTarget) {
        self.targets.write().await.push(target);
    }

    /// Remove monitoring target
    pub async fn remove_target(&self, target_id: &str) {
        self.targets.write().await.retain(|t| t.target_id != target_id);
    }

    /// Get monitoring targets
    pub async fn get_targets(&self) -> Vec<MonitoringTarget> {
        self.targets.read().await.clone()
    }

    /// Run monitoring scan
    pub async fn run_scan(&self) -> Result<Vec<MonitoringResult>, DeepfakeError> {
        let targets = self.targets.read().await.clone();
        let mut results = Vec::new();

        for target in targets {
            // In a real implementation, this would scan the target
            let result = MonitoringResult {
                result_id: uuid::Uuid::new_v4().to_string(),
                target_id: target.target_id.clone(),
                scanned_at: Utc::now(),
                deepfakes_found: 0,
                status: MonitoringStatus::Clean,
                details: HashMap::new(),
            };

            self.results.write().await.insert(target.target_id.clone(), result.clone());
            results.push(result);
        }

        Ok(results)
    }

    /// Get monitoring results
    pub async fn get_results(&self) -> HashMap<String, MonitoringResult> {
        self.results.read().await.clone()
    }
}

/// Monitoring target
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MonitoringTarget {
    /// Target ID
    pub target_id: String,
    /// Target name
    pub name: String,
    /// Target type
    pub target_type: MonitoringTargetType,
    /// Target URL or path
    pub target_url: String,
    /// Enabled
    pub enabled: bool,
    /// Scan interval in seconds
    pub scan_interval_secs: u64,
}

/// Monitoring target type
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum MonitoringTargetType {
    SocialMedia,
    News,
    DarkWeb,
    FileRepository,
    BrandWebsite,
    Custom,
}

/// Monitoring result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MonitoringResult {
    /// Result ID
    pub result_id: String,
    /// Target ID
    pub target_id: String,
    /// Scanned at
    pub scanned_at: DateTime<Utc>,
    /// Deepfakes found
    pub deepfakes_found: u32,
    /// Monitoring status
    pub status: MonitoringStatus,
    /// Additional details
    pub details: HashMap<String, String>,
}

/// Monitoring status
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum MonitoringStatus {
    Clean,
    Suspicious,
    DeepfakeDetected,
    Error,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_alert_config() {
        let config = AlertConfig::default();
        assert!(config.enable_email);
        assert!(!config.enable_slack);
        assert_eq!(config.min_severity, AlertSeverity::Medium);
    }

    #[test]
    fn test_should_send_alert() {
        let config = AlertConfig::default();
        let integrator = ThreatIntegrator::new(config);

        assert!(integrator.should_send_alert(&AlertSeverity::Critical));
        assert!(integrator.should_send_alert(&AlertSeverity::High));
        assert!(integrator.should_send_alert(&AlertSeverity::Medium));
        assert!(!integrator.should_send_alert(&AlertSeverity::Low));
    }
}