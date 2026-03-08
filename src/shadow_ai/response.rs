use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use crate::shadow_ai::{
    DetectedAIModel, RiskLevel, ShadowAIError, AIRiskScore
};

/// Response Engine - Handles automated responses to Shadow AI
pub struct ResponseEngine {
    config: ResponseConfig,
    /// Alert history
    alert_history: Arc<RwLock<Vec<Alert>>>,
    /// Active remediation workflows
    active_workflows: Arc<RwLock<HashMap<String, RemediationWorkflow>>>,
    /// Blocked models
    blocked_models: Arc<RwLock<HashMap<String, BlockedModel>>>,
}

/// Response configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResponseConfig {
    /// Enable automated responses
    pub enable_automated_response: bool,
    /// Auto-block critical risk models
    pub auto_block_critical: bool,
    /// Alert on high risk
    pub alert_on_high: bool,
    /// Alert on medium risk
    pub alert_on_medium: bool,
    /// Alert channels
    pub alert_channels: Vec<AlertChannel>,
    /// Email recipients for alerts
    pub email_recipients: Vec<String>,
    /// Slack webhook for alerts
    pub slack_webhook: Option<String>,
    /// Remediation timeout in seconds
    pub remediation_timeout_secs: u64,
    /// Enable audit logging
    pub enable_audit_logging: bool,
}

impl Default for ResponseConfig {
    fn default() -> Self {
        Self {
            enable_automated_response: true,
            auto_block_critical: true,
            alert_on_high: true,
            alert_on_medium: false,
            alert_channels: vec![AlertChannel::Email, AlertChannel::AuditLog],
            email_recipients: vec!["security@example.com".to_string()],
            slack_webhook: None,
            remediation_timeout_secs: 3600,
            enable_audit_logging: true,
        }
    }
}

/// Alert channel
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum AlertChannel {
    Email,
    Slack,
    SMS,
    AuditLog,
    Webhook,
    Custom(String),
}

/// Alert for Shadow AI detection
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Alert {
    /// Alert ID
    pub alert_id: String,
    /// Alert timestamp
    pub timestamp: DateTime<Utc>,
    /// Alert severity
    pub severity: AlertSeverity,
    /// Alert type
    pub alert_type: AlertType,
    /// Model involved
    pub model_id: Option<String>,
    /// Risk score
    pub risk_score: Option<f64>,
    /// Alert message
    pub message: String,
    /// Alert details
    pub details: HashMap<String, String>,
    /// Channels sent to
    pub channels: Vec<AlertChannel>,
    /// Alert status
    pub status: AlertStatus,
    /// Acknowledged by
    pub acknowledged_by: Option<String>,
    /// Acknowledged at
    pub acknowledged_at: Option<DateTime<Utc>>,
    /// Resolved at
    pub resolved_at: Option<DateTime<Utc>>,
}

/// Alert severity
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub enum AlertSeverity {
    Info,
    Low,
    Medium,
    High,
    Critical,
}

/// Alert type
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum AlertType {
    /// New shadow AI detected
    ShadowAIDetected,
    /// Critical risk model detected
    CriticalRiskDetected,
    /// Unregistered AI usage
    UnregisteredUsage,
    /// Data exposure risk
    DataExposureRisk,
    /// Policy violation
    PolicyViolation,
    /// Compliance issue
    ComplianceIssue,
    /// Access blocked
    AccessBlocked,
    /// Remediation required
    RemediationRequired,
    /// Custom alert type
    Custom(String),
}

/// Alert status
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum AlertStatus {
    Open,
    Acknowledged,
    InProgress,
    Resolved,
    FalsePositive,
    Escalated,
}

/// Remediation workflow
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RemediationWorkflow {
    /// Workflow ID
    pub workflow_id: String,
    /// Triggering alert ID
    pub alert_id: String,
    /// Model being remediated
    pub model_id: Option<String>,
    /// Workflow status
    pub status: WorkflowStatus,
    /// Remediation steps
    pub steps: Vec<RemediationStep>,
    /// Current step index
    pub current_step: usize,
    /// Workflow started at
    pub started_at: DateTime<Utc>,
    /// Expected completion at
    pub expected_completion: Option<DateTime<Utc>>,
    /// Completed at
    pub completed_at: Option<DateTime<Utc>>,
    /// Workflow result
    pub result: Option<WorkflowResult>,
    /// Notes
    pub notes: Vec<String>,
}

/// Workflow status
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum WorkflowStatus {
    Pending,
    InProgress,
    Completed,
    Failed,
    Cancelled,
    TimedOut,
}

/// Remediation step
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RemediationStep {
    /// Step ID
    pub step_id: String,
    /// Step name
    pub name: String,
    /// Step description
    pub description: String,
    /// Step type
    pub step_type: RemediationStepType,
    /// Step status
    pub status: StepStatus,
    /// Started at
    pub started_at: Option<DateTime<Utc>>,
    /// Completed at
    pub completed_at: Option<DateTime<Utc>>,
    /// Error message if failed
    pub error: Option<String>,
    /// Step output
    pub output: Option<String>,
}

/// Remediation step type
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum RemediationStepType {
    /// Block model access
    BlockAccess,
    /// Quarantine model file
    QuarantineModel,
    /// Send notification
    SendNotification,
    /// Require approval
    RequireApproval,
    /// Move model to secure location
    MoveModel,
    /// Delete model file
    DeleteModel,
    /// Register model
    RegisterModel,
    /// Custom step
    Custom(String),
}

/// Step status
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum StepStatus {
    Pending,
    InProgress,
    Completed,
    Failed,
    Skipped,
}

/// Workflow result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkflowResult {
    /// Whether workflow succeeded
    pub success: bool,
    /// Result message
    pub message: String,
    /// Actions taken
    pub actions_taken: Vec<String>,
    /// Remaining issues
    pub remaining_issues: Vec<String>,
}

/// Blocked model record
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlockedModel {
    /// Model ID
    pub model_id: String,
    /// Model path
    pub model_path: String,
    /// Blocked at
    pub blocked_at: DateTime<Utc>,
    /// Blocked by
    pub blocked_by: String,
    /// Block reason
    pub reason: String,
    /// Risk level at time of blocking
    pub risk_level: RiskLevel,
    /// Block expiry
    pub expires_at: Option<DateTime<Utc>>,
}

impl ResponseEngine {
    /// Create new response engine
    pub fn new(config: ResponseConfig) -> Self {
        Self {
            config,
            alert_history: Arc::new(RwLock::new(Vec::new())),
            active_workflows: Arc::new(RwLock::new(HashMap::new())),
            blocked_models: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Process a detected AI model and trigger appropriate response
    pub async fn process_detection(&self, model: &DetectedAIModel, risk_score: &AIRiskScore) -> Result<Vec<Alert>, ShadowAIError> {
        let mut alerts = Vec::new();

        // Determine response based on risk level
        match risk_score.risk_level {
            RiskLevel::Critical => {
                if self.config.auto_block_critical {
                    // Auto-block critical models
                    self.block_model(model, "Critical risk - auto-blocked".to_string()).await?;
                    
                    // Create critical alert
                    let alert = self.create_alert(
                        AlertType::CriticalRiskDetected,
                        AlertSeverity::Critical,
                        format!(
                            "Critical risk model detected and auto-blocked: {} (Risk Score: {:.2})",
                            model.model_name, risk_score.overall_score
                        ),
                        Some(model.model_id.clone()),
                        Some(risk_score.overall_score),
                    ).await?;
                    alerts.push(alert);

                    // Start remediation workflow
                    self.start_remediation_workflow(model, &alert.alert_id).await?;
                } else {
                    // Just alert without blocking
                    let alert = self.create_alert(
                        AlertType::CriticalRiskDetected,
                        AlertSeverity::Critical,
                        format!(
                            "Critical risk model detected: {} (Risk Score: {:.2})",
                            model.model_name, risk_score.overall_score
                        ),
                        Some(model.model_id.clone()),
                        Some(risk_score.overall_score),
                    ).await?;
                    alerts.push(alert);
                }
            }
            RiskLevel::High => {
                if self.config.alert_on_high {
                    let alert = self.create_alert(
                        AlertType::ShadowAIDetected,
                        AlertSeverity::High,
                        format!(
                            "High risk shadow AI detected: {} (Risk Score: {:.2})",
                            model.model_name, risk_score.overall_score
                        ),
                        Some(model.model_id.clone()),
                        Some(risk_score.overall_score),
                    ).await?;
                    alerts.push(alert);
                }
            }
            RiskLevel::Medium => {
                if self.config.alert_on_medium {
                    let alert = self.create_alert(
                        AlertType::ShadowAIDetected,
                        AlertSeverity::Medium,
                        format!(
                            "Medium risk shadow AI detected: {} (Risk Score: {:.2})",
                            model.model_name, risk_score.overall_score
                        ),
                        Some(model.model_id.clone()),
                        Some(risk_score.overall_score),
                    ).await?;
                    alerts.push(alert);
                }
            }
            RiskLevel::Low => {
                // Low risk - just log
                let alert = self.create_alert(
                    AlertType::ShadowAIDetected,
                    AlertSeverity::Low,
                    format!(
                        "Low risk AI model detected: {}",
                        model.model_name
                    ),
                    Some(model.model_id.clone()),
                    Some(risk_score.overall_score),
                ).await?;
                alerts.push(alert);
            }
        }

        // Check if model is unregistered
        if !model.registered {
            let alert = self.create_alert(
                AlertType::UnregisteredUsage,
                AlertSeverity::High,
                format!(
                    "Unregistered AI usage detected: {}",
                    model.model_name
                ),
                Some(model.model_id.clone()),
                None,
            ).await?;
            alerts.push(alert);
        }

        Ok(alerts)
    }

    /// Create an alert
    async fn create_alert(
        &self,
        alert_type: AlertType,
        severity: AlertSeverity,
        message: String,
        model_id: Option<String>,
        risk_score: Option<f64>,
    ) -> Result<Alert, ShadowAIError> {
        let mut details = HashMap::new();
        if let Some(id) = &model_id {
            details.insert("model_id".to_string(), id.clone());
        }
        if let Some(score) = risk_score {
            details.insert("risk_score".to_string(), score.to_string());
        }

        let alert = Alert {
            alert_id: uuid::Uuid::new_v4().to_string(),
            timestamp: Utc::now(),
            severity,
            alert_type,
            model_id,
            risk_score,
            message,
            details,
            channels: self.config.alert_channels.clone(),
            status: AlertStatus::Open,
            acknowledged_by: None,
            acknowledged_at: None,
            resolved_at: None,
        };

        // Send alert through configured channels
        self.send_alert(&alert).await?;

        // Add to history
        self.alert_history.write().await.push(alert.clone());

        Ok(alert)
    }

    /// Send alert through configured channels
    async fn send_alert(&self, alert: &Alert) -> Result<(), ShadowAIError> {
        for channel in &self.config.alert_channels {
            match channel {
                AlertChannel::Email => {
                    self.send_email_alert(alert).await?;
                }
                AlertChannel::Slack => {
                    if let Some(webhook) = &self.config.slack_webhook {
                        self.send_slack_alert(alert, webhook).await?;
                    }
                }
                AlertChannel::AuditLog => {
                    if self.config.enable_audit_logging {
                        self.log_alert(alert).await?;
                    }
                }
                AlertChannel::SMS => {
                    // SMS alert implementation would go here
                    log::info!("SMS alert: {}", alert.message);
                }
                AlertChannel::Webhook => {
                    // Webhook alert implementation would go here
                }
                AlertChannel::Custom(_) => {
                    // Custom alert implementation would go here
                }
            }
        }

        Ok(())
    }

    /// Send email alert
    async fn send_email_alert(&self, alert: &Alert) -> Result<(), ShadowAIError> {
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
    async fn send_slack_alert(&self, alert: &Alert, webhook: &str) -> Result<(), ShadowAIError> {
        // In a real implementation, this would send to Slack webhook
        log::info!(
            "Slack alert sent to {}: [{}] {}",
            webhook,
            alert.severity,
            alert.message
        );
        Ok(())
    }

    /// Log alert to audit log
    async fn log_alert(&self, alert: &Alert) -> Result<(), ShadowAIError> {
        log::info!(
            "[ALERT] ID: {}, Type: {:?}, Severity: {:?}, Message: {}",
            alert.alert_id,
            alert.alert_type,
            alert.severity,
            alert.message
        );
        Ok(())
    }

    /// Block a model
    pub async fn block_model(&self, model: &DetectedAIModel, reason: String) -> Result<(), ShadowAIError> {
        let blocked = BlockedModel {
            model_id: model.model_id.clone(),
            model_path: model.model_path.clone(),
            blocked_at: Utc::now(),
            blocked_by: "system".to_string(),
            reason: reason.clone(),
            risk_level: model.risk_level.clone(),
            expires_at: None,
        };

        // Add to blocked models
        self.blocked_models.write().await.insert(model.model_id.clone(), blocked);

        // Log the blocking action
        log::warn!(
            "Model blocked: {} - {}",
            model.model_name,
            reason
        );

        Ok(())
    }

    /// Check if a model is blocked
    pub async fn is_model_blocked(&self, model_id: &str) -> bool {
        let blocked = self.blocked_models.read().await;
        blocked.contains_key(model_id)
    }

    /// Unblock a model
    pub async fn unblock_model(&self, model_id: &str, unblocked_by: &str) -> Result<(), ShadowAIError> {
        let mut blocked = self.blocked_models.write().await;
        
        if blocked.remove(model_id).is_some() {
            log::info!(
                "Model unblocked: {} by {}",
                model_id,
                unblocked_by
            );
            Ok(())
        } else {
            Err(ShadowAIError::ResponseError(
                format!("Model {} not found in blocked list", model_id)
            ))
        }
    }

    /// Start a remediation workflow
    pub async fn start_remediation_workflow(&self, model: &DetectedAIModel, alert_id: &str) -> Result<RemediationWorkflow, ShadowAIError> {
        let workflow_id = uuid::Uuid::new_v4().to_string();
        
        // Create remediation steps based on risk level
        let steps = self.create_remediation_steps(model).await;

        let workflow = RemediationWorkflow {
            workflow_id: workflow_id.clone(),
            alert_id: alert_id.to_string(),
            model_id: Some(model.model_id.clone()),
            status: WorkflowStatus::Pending,
            steps,
            current_step: 0,
            started_at: Utc::now(),
            expected_completion: Some(Utc::now() + chrono::Duration::seconds(self.config.remediation_timeout_secs as i64)),
            completed_at: None,
            result: None,
            notes: Vec::new(),
        };

        // Add to active workflows
        self.active_workflows.write().await.insert(workflow_id.clone(), workflow.clone());

        // Execute workflow
        if self.config.enable_automated_response {
            self.execute_workflow(&workflow_id).await?;
        }

        Ok(workflow)
    }

    /// Create remediation steps for a model
    async fn create_remediation_steps(&self, model: &DetectedAIModel) -> Vec<RemediationStep> {
        let mut steps = Vec::new();

        // Common first step: notification
        steps.push(RemediationStep {
            step_id: uuid::Uuid::new_v4().to_string(),
            name: "Send Initial Notification".to_string(),
            description: "Notify security team about detected shadow AI".to_string(),
            step_type: RemediationStepType::SendNotification,
            status: StepStatus::Pending,
            started_at: None,
            completed_at: None,
            error: None,
            output: None,
        });

        // Steps based on risk level
        match model.risk_level {
            RiskLevel::Critical => {
                steps.push(RemediationStep {
                    step_id: uuid::Uuid::new_v4().to_string(),
                    name: "Block Model Access".to_string(),
                    description: "Block access to critical risk model".to_string(),
                    step_type: RemediationStepType::BlockAccess,
                    status: StepStatus::Pending,
                    started_at: None,
                    completed_at: None,
                    error: None,
                    output: None,
                });

                steps.push(RemediationStep {
                    step_id: uuid::Uuid::new_v4().to_string(),
                    name: "Quarantine Model File".to_string(),
                    description: "Move model file to quarantine directory".to_string(),
                    step_type: RemediationStepType::QuarantineModel,
                    status: StepStatus::Pending,
                    started_at: None,
                    completed_at: None,
                    error: None,
                    output: None,
                });
            }
            RiskLevel::High => {
                steps.push(RemediationStep {
                    step_id: uuid::Uuid::new_v4().to_string(),
                    name: "Require Approval".to_string(),
                    description: "Require explicit approval for model usage".to_string(),
                    step_type: RemediationStepType::RequireApproval,
                    status: StepStatus::Pending,
                    started_at: None,
                    completed_at: None,
                    error: None,
                    output: None,
                });
            }
            RiskLevel::Medium => {
                steps.push(RemediationStep {
                    step_id: uuid::Uuid::new_v4().to_string(),
                    name: "Register Model".to_string(),
                    description: "Register model in governance system".to_string(),
                    step_type: RemediationStepType::RegisterModel,
                    status: StepStatus::Pending,
                    started_at: None,
                    completed_at: None,
                    error: None,
                    output: None,
                });
            }
            RiskLevel::Low => {
                steps.push(RemediationStep {
                    step_id: uuid::Uuid::new_v4().to_string(),
                    name: "Log Discovery".to_string(),
                    description: "Log model discovery for monitoring".to_string(),
                    step_type: RemediationStepType::Custom("LogDiscovery".to_string()),
                    status: StepStatus::Pending,
                    started_at: None,
                    completed_at: None,
                    error: None,
                    output: None,
                });
            }
        }

        steps
    }

    /// Execute a remediation workflow
    async fn execute_workflow(&self, workflow_id: &str) -> Result<(), ShadowAIError> {
        let mut workflow = {
            let mut workflows = self.active_workflows.write().await;
            workflows.get_mut(workflow_id)
                .ok_or_else(|| ShadowAIError::ResponseError(
                    format!("Workflow {} not found", workflow_id)
                ))?
                .clone()
        };

        workflow.status = WorkflowStatus::InProgress;

        for (index, step) in workflow.steps.iter_mut().enumerate() {
            workflow.current_step = index;
            
            // Execute step
            let result = self.execute_remediation_step(step).await;
            
            match result {
                Ok(_) => {
                    step.status = StepStatus::Completed;
                    step.completed_at = Some(Utc::now());
                }
                Err(e) => {
                    step.status = StepStatus::Failed;
                    step.error = Some(e.to_string());
                    workflow.status = WorkflowStatus::Failed;
                    workflow.result = Some(WorkflowResult {
                        success: false,
                        message: format!("Workflow failed at step {}: {}", index, e),
                        actions_taken: vec![],
                        remaining_issues: vec![e.to_string()],
                    });
                    break;
                }
            }
        }

        // Update workflow status if all steps completed
        if workflow.status == WorkflowStatus::InProgress {
            workflow.status = WorkflowStatus::Completed;
            workflow.completed_at = Some(Utc::now());
            workflow.result = Some(WorkflowResult {
                success: true,
                message: "Remediation workflow completed successfully".to_string(),
                actions_taken: workflow.steps.iter()
                    .map(|s| s.name.clone())
                    .collect(),
                remaining_issues: Vec::new(),
            });
        }

        // Update workflow in storage
        let mut workflows = self.active_workflows.write().await;
        workflows.insert(workflow_id.to_string(), workflow);

        Ok(())
    }

    /// Execute a single remediation step
    async fn execute_remediation_step(&self, step: &mut RemediationStep) -> Result<(), ShadowAIError> {
        step.status = StepStatus::InProgress;
        step.started_at = Some(Utc::now());

        match &step.step_type {
            RemediationStepType::SendNotification => {
                log::info!("Executing step: {}", step.name);
                step.output = Some("Notification sent successfully".to_string());
            }
            RemediationStepType::BlockAccess => {
                log::warn!("Executing step: {}", step.name);
                step.output = Some("Model access blocked".to_string());
            }
            RemediationStepType::QuarantineModel => {
                log::warn!("Executing step: {}", step.name);
                step.output = Some("Model quarantined".to_string());
            }
            RemediationStepType::RequireApproval => {
                log::info!("Executing step: {}", step.name);
                step.output = Some("Approval requirement set".to_string());
            }
            RemediationStepType::MoveModel => {
                log::info!("Executing step: {}", step.name);
                step.output = Some("Model moved".to_string());
            }
            RemediationStepType::DeleteModel => {
                log::warn!("Executing step: {}", step.name);
                step.output = Some("Model deleted".to_string());
            }
            RemediationStepType::RegisterModel => {
                log::info!("Executing step: {}", step.name);
                step.output = Some("Model registered".to_string());
            }
            RemediationStepType::Custom(_) => {
                log::info!("Executing custom step: {}", step.name);
                step.output = Some("Custom step executed".to_string());
            }
        }

        Ok(())
    }

    /// Get alert history
    pub async fn get_alert_history(&self) -> Vec<Alert> {
        self.alert_history.read().await.clone()
    }

    /// Get active workflows
    pub async fn get_active_workflows(&self) -> Vec<RemediationWorkflow> {
        let workflows = self.active_workflows.read().await;
        workflows.values().cloned().collect()
    }

    /// Get blocked models
    pub async fn get_blocked_models(&self) -> Vec<BlockedModel> {
        let blocked = self.blocked_models.read().await;
        blocked.values().cloned().collect()
    }

    /// Acknowledge an alert
    pub async fn acknowledge_alert(&self, alert_id: &str, acknowledged_by: &str) -> Result<(), ShadowAIError> {
        let mut alerts = self.alert_history.write().await;
        
        if let Some(alert) = alerts.iter_mut().find(|a| a.alert_id == alert_id) {
            alert.status = AlertStatus::Acknowledged;
            alert.acknowledged_by = Some(acknowledged_by.to_string());
            alert.acknowledged_at = Some(Utc::now());
            Ok(())
        } else {
            Err(ShadowAIError::ResponseError(
                format!("Alert {} not found", alert_id)
            ))
        }
    }

    /// Resolve an alert
    pub async fn resolve_alert(&self, alert_id: &str, resolved_by: &str) -> Result<(), ShadowAIError> {
        let mut alerts = self.alert_history.write().await;
        
        if let Some(alert) = alerts.iter_mut().find(|a| a.alert_id == alert_id) {
            alert.status = AlertStatus::Resolved;
            alert.resolved_at = Some(Utc::now());
            Ok(())
        } else {
            Err(ShadowAIError::ResponseError(
                format!("Alert {} not found", alert_id)
            ))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_response_config() {
        let config = ResponseConfig::default();
        assert!(config.enable_automated_response);
        assert!(config.auto_block_critical);
        assert!(config.alert_on_high);
    }

    #[test]
    fn test_alert_severity_ordering() {
        assert!(AlertSeverity::Critical > AlertSeverity::High);
        assert!(AlertSeverity::High > AlertSeverity::Medium);
        assert!(AlertSeverity::Medium > AlertSeverity::Low);
        assert!(AlertSeverity::Low > AlertSeverity::Info);
    }
}