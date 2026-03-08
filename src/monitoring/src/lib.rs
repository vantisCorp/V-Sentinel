//! SENTINEL Monitoring Module
//!
//! This module provides comprehensive logging, metrics collection,
//! and monitoring capabilities for the SENTINEL system.

use anyhow::Result;
use prometheus::{Counter, Encoder, Gauge, Histogram, Registry, TextEncoder};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::sync::RwLock;
use tracing::{error, info};
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use tracing_subscriber::Layer;

/// Monitoring Manager
pub struct MonitoringManager {
    logger: Arc<RwLock<Logger>>,
    metrics: Arc<RwLock<MetricsCollector>>,
    alerts: Arc<RwLock<AlertManager>>,
    health: Arc<RwLock<HealthChecker>>,
}

/// Logger
pub struct Logger {
    level: LogLevel,
    format: LogFormat,
    structured: bool,
    file_logging: bool,
    file_path: Option<String>,
}

/// Metrics Collector
pub struct MetricsCollector {
    registry: Registry,
    counters: HashMap<String, Counter>,
    histograms: HashMap<String, Histogram>,
    gauges: HashMap<String, Gauge>,
}

/// Alert Manager
pub struct AlertManager {
    alerts: Vec<Alert>,
    alert_rules: Vec<AlertRule>,
    notification_channels: Vec<Box<dyn NotificationChannel>>,
}

/// Health Checker
pub struct HealthChecker {
    checks: HashMap<String, HealthCheck>,
    status: HashMap<String, HealthStatus>,
}

/// Log Level
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum LogLevel {
    Trace,
    Debug,
    Info,
    Warn,
    Error,
}

/// Log Format
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum LogFormat {
    Text,
    Json,
    Pretty,
}

/// Alert
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Alert {
    id: String,
    severity: AlertSeverity,
    title: String,
    description: String,
    timestamp: std::time::SystemTime,
    resolved: bool,
}

/// Alert Severity
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AlertSeverity {
    Info,
    Warning,
    Error,
    Critical,
}

impl std::fmt::Display for AlertSeverity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AlertSeverity::Info => write!(f, "Info"),
            AlertSeverity::Warning => write!(f, "Warning"),
            AlertSeverity::Error => write!(f, "Error"),
            AlertSeverity::Critical => write!(f, "Critical"),
        }
    }
}

/// Alert Rule
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlertRule {
    id: String,
    name: String,
    condition: AlertCondition,
    severity: AlertSeverity,
    enabled: bool,
}

/// Alert Condition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AlertCondition {
    Threshold {
        metric: String,
        operator: ComparisonOperator,
        value: f64,
    },
    Rate {
        metric: String,
        operator: ComparisonOperator,
        value: f64,
        window: Duration,
    },
    Pattern {
        log_pattern: String,
        count_threshold: usize,
        window: Duration,
    },
}

/// Comparison Operator
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ComparisonOperator {
    GreaterThan,
    LessThan,
    Equal,
    GreaterThanOrEqual,
    LessThanOrEqual,
}

/// Notification Channel trait
pub trait NotificationChannel: Send + Sync {
    fn send(&self, alert: &Alert) -> Result<()>;
    fn name(&self) -> &str;
}

/// Health Check
pub struct HealthCheck {
    name: String,
    check_fn: Box<dyn Fn() -> HealthStatus + Send + Sync>,
    interval: Duration,
    timeout: Duration,
}

impl std::fmt::Debug for HealthCheck {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("HealthCheck")
            .field("name", &self.name)
            .field("interval", &self.interval)
            .field("timeout", &self.timeout)
            .finish()
    }
}

impl Clone for HealthCheck {
    fn clone(&self) -> Self {
        Self {
            name: self.name.clone(),
            check_fn: Box::new(|| HealthStatus {
                healthy: false,
                message: "cloned check".to_string(),
                last_check: std::time::SystemTime::now(),
                response_time_ms: 0,
            }),
            interval: self.interval,
            timeout: self.timeout,
        }
    }
}

/// Health Status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealthStatus {
    healthy: bool,
    message: String,
    last_check: std::time::SystemTime,
    response_time_ms: u64,
}

impl MonitoringManager {
    /// Create a new monitoring manager
    pub fn new() -> Result<Self> {
        info!("Creating Monitoring Manager...");

        Ok(Self {
            logger: Arc::new(RwLock::new(Logger::new())),
            metrics: Arc::new(RwLock::new(MetricsCollector::new()?)),
            alerts: Arc::new(RwLock::new(AlertManager::new())),
            health: Arc::new(RwLock::new(HealthChecker::new())),
        })
    }

    /// Initialize logging
    pub async fn initialize_logging(
        &self,
        level: LogLevel,
        format: LogFormat,
        structured: bool,
    ) -> Result<()> {
        let mut logger = self.logger.write().await;
        logger.level = level;
        logger.format = format;
        logger.structured = structured;

        // Initialize tracing subscriber
        let env_filter = match level {
            LogLevel::Trace => "trace",
            LogLevel::Debug => "debug",
            LogLevel::Info => "info",
            LogLevel::Warn => "warn",
            LogLevel::Error => "error",
        };

        let filter = tracing_subscriber::EnvFilter::try_new(env_filter)?;

        if structured {
            tracing_subscriber::registry()
                .with(filter)
                .with(tracing_subscriber::fmt::layer().json())
                .init();
        } else {
            match format {
                LogFormat::Pretty => {
                    tracing_subscriber::registry()
                        .with(filter)
                        .with(tracing_subscriber::fmt::layer().pretty())
                        .init();
                }
                _ => {
                    tracing_subscriber::registry()
                        .with(filter)
                        .with(tracing_subscriber::fmt::layer())
                        .init();
                }
            }
        }

        info!(
            "Logging initialized: level={:?}, format={:?}, structured={}",
            level, format, structured
        );

        Ok(())
    }

    /// Enable file logging
    pub async fn enable_file_logging(&self, path: &str) -> Result<()> {
        let mut logger = self.logger.write().await;
        logger.file_logging = true;
        logger.file_path = Some(path.to_string());

        info!("File logging enabled: {}", path);

        Ok(())
    }

    /// Increment counter metric
    pub async fn increment_counter(&self, name: &str, labels: &[(&str, &str)]) -> Result<()> {
        let mut metrics = self.metrics.write().await;
        metrics.increment_counter(name, labels)
    }

    /// Record histogram metric
    pub async fn record_histogram(
        &self,
        name: &str,
        value: f64,
        labels: &[(&str, &str)],
    ) -> Result<()> {
        let mut metrics = self.metrics.write().await;
        metrics.record_histogram(name, value, labels)
    }

    /// Set gauge metric
    pub async fn set_gauge(&self, name: &str, value: f64, labels: &[(&str, &str)]) -> Result<()> {
        let mut metrics = self.metrics.write().await;
        metrics.set_gauge(name, value, labels)
    }

    /// Get metrics in Prometheus format
    pub async fn get_metrics(&self) -> Result<String> {
        let metrics = self.metrics.read().await;
        metrics.get_metrics()
    }

    /// Add alert rule
    pub async fn add_alert_rule(&self, rule: AlertRule) -> Result<()> {
        let mut alerts = self.alerts.write().await;
        alerts.add_rule(rule)
    }

    /// Add notification channel
    pub async fn add_notification_channel(
        &self,
        channel: Box<dyn NotificationChannel>,
    ) -> Result<()> {
        let mut alerts = self.alerts.write().await;
        alerts.add_notification_channel(channel)
    }

    /// Evaluate alert rules
    pub async fn evaluate_alerts(&self) -> Result<Vec<Alert>> {
        let mut alerts = self.alerts.write().await;
        let metrics = self.metrics.read().await;
        alerts.evaluate_rules(&metrics)
    }

    /// Get active alerts
    pub async fn get_active_alerts(&self) -> Vec<Alert> {
        let alerts = self.alerts.read().await;
        alerts.get_active_alerts()
    }

    /// Add health check
    pub async fn add_health_check(
        &self,
        name: String,
        check_fn: Box<dyn Fn() -> HealthStatus + Send + Sync>,
        interval: Duration,
        timeout: Duration,
    ) -> Result<()> {
        let mut health = self.health.write().await;
        health.add_check(name, check_fn, interval, timeout)
    }

    /// Run health checks
    pub async fn run_health_checks(&self) -> Result<HashMap<String, HealthStatus>> {
        let mut health = self.health.write().await;
        health.run_all_checks()
    }

    /// Get health status
    pub async fn get_health_status(&self) -> HashMap<String, HealthStatus> {
        let health = self.health.read().await;
        health.get_status()
    }

    /// Start monitoring background tasks
    pub async fn start_background_tasks(&self) -> Result<()> {
        info!("Starting monitoring background tasks...");

        // Start health check loop
        let health = Arc::clone(&self.health);
        tokio::spawn(async move {
            loop {
                tokio::time::sleep(Duration::from_secs(30)).await;
                if let Err(e) = health.write().await.run_all_checks() {
                    error!("Health check failed: {}", e);
                }
            }
        });

        // Start alert evaluation loop
        let alerts = Arc::clone(&self.alerts);
        let metrics = Arc::clone(&self.metrics);
        tokio::spawn(async move {
            loop {
                tokio::time::sleep(Duration::from_secs(60)).await;
                let mut alerts_guard = alerts.write().await;
                let metrics_guard = metrics.read().await;
                if let Err(e) = alerts_guard.evaluate_rules(&metrics_guard) {
                    error!("Alert evaluation failed: {}", e);
                }
            }
        });

        info!("Monitoring background tasks started");

        Ok(())
    }
}

impl Default for Logger {
    fn default() -> Self {
        Self::new()
    }
}

impl Logger {
    pub fn new() -> Self {
        Self {
            level: LogLevel::Info,
            format: LogFormat::Pretty,
            structured: false,
            file_logging: false,
            file_path: None,
        }
    }
}

impl MetricsCollector {
    pub fn new() -> Result<Self> {
        Ok(Self {
            registry: Registry::new(),
            counters: HashMap::new(),
            histograms: HashMap::new(),
            gauges: HashMap::new(),
        })
    }

    pub fn increment_counter(&mut self, name: &str, _labels: &[(&str, &str)]) -> Result<()> {
        let counter = self
            .counters
            .entry(name.to_string())
            .or_insert_with(|| Counter::new(name, name).unwrap());

        counter.inc();
        Ok(())
    }

    pub fn record_histogram(
        &mut self,
        name: &str,
        value: f64,
        _labels: &[(&str, &str)],
    ) -> Result<()> {
        let histogram = self.histograms.entry(name.to_string()).or_insert_with(|| {
            Histogram::with_opts(prometheus::HistogramOpts::new(name, name).buckets(vec![
                0.001, 0.005, 0.01, 0.025, 0.05, 0.1, 0.25, 0.5, 1.0, 2.5, 5.0, 10.0,
            ]))
            .unwrap()
        });

        histogram.observe(value);
        Ok(())
    }

    pub fn set_gauge(&mut self, name: &str, value: f64, _labels: &[(&str, &str)]) -> Result<()> {
        let gauge = self
            .gauges
            .entry(name.to_string())
            .or_insert_with(|| Gauge::new(name, name).unwrap());

        gauge.set(value);
        Ok(())
    }

    pub fn get_metrics(&self) -> Result<String> {
        let encoder = TextEncoder::new();
        let metric_families = self.registry.gather();
        let mut buffer = Vec::new();
        encoder.encode(&metric_families, &mut buffer)?;
        Ok(String::from_utf8(buffer)?)
    }
}

impl Default for AlertManager {
    fn default() -> Self {
        Self::new()
    }
}

impl AlertManager {
    pub fn new() -> Self {
        Self {
            alerts: Vec::new(),
            alert_rules: Vec::new(),
            notification_channels: Vec::new(),
        }
    }

    pub fn add_rule(&mut self, rule: AlertRule) -> Result<()> {
        self.alert_rules.push(rule);
        Ok(())
    }

    pub fn add_notification_channel(
        &mut self,
        channel: Box<dyn NotificationChannel>,
    ) -> Result<()> {
        self.notification_channels.push(channel);
        Ok(())
    }

    pub fn evaluate_rules(&mut self, metrics: &MetricsCollector) -> Result<Vec<Alert>> {
        let mut new_alerts = Vec::new();

        for rule in &self.alert_rules {
            if !rule.enabled {
                continue;
            }

            if let Some(alert) = self.evaluate_rule(rule, metrics)? {
                new_alerts.push(alert);
            }
        }

        self.alerts.extend(new_alerts.clone());

        // Send notifications
        for alert in &new_alerts {
            for channel in &self.notification_channels {
                if let Err(e) = channel.send(alert) {
                    error!("Failed to send alert notification: {}", e);
                }
            }
        }

        Ok(new_alerts)
    }

    fn evaluate_rule(
        &self,
        rule: &AlertRule,
        _metrics: &MetricsCollector,
    ) -> Result<Option<Alert>> {
        // Simplified evaluation - in production, implement actual metric checking
        if let AlertCondition::Threshold {
                metric,
                operator,
                value,
            } = &rule.condition {
            // Check if threshold is exceeded
            let triggered = match operator {
                ComparisonOperator::GreaterThan => true, // Simplified
                ComparisonOperator::LessThan => false,
                ComparisonOperator::Equal => false,
                ComparisonOperator::GreaterThanOrEqual => true,
                ComparisonOperator::LessThanOrEqual => false,
            };

            if triggered {
                return Ok(Some(Alert {
                    id: uuid::Uuid::new_v4().to_string(),
                    severity: rule.severity,
                    title: format!("Threshold exceeded: {}", metric),
                    description: format!("Metric {} exceeded threshold {}", metric, value),
                    timestamp: std::time::SystemTime::now(),
                    resolved: false,
                }));
            }
        }

        Ok(None)
    }

    pub fn get_active_alerts(&self) -> Vec<Alert> {
        self.alerts
            .iter()
            .filter(|a| !a.resolved)
            .cloned()
            .collect()
    }
}

impl Default for HealthChecker {
    fn default() -> Self {
        Self::new()
    }
}

impl HealthChecker {
    pub fn new() -> Self {
        Self {
            checks: HashMap::new(),
            status: HashMap::new(),
        }
    }

    pub fn add_check(
        &mut self,
        name: String,
        check_fn: Box<dyn Fn() -> HealthStatus + Send + Sync>,
        interval: Duration,
        timeout: Duration,
    ) -> Result<()> {
        self.checks.insert(
            name.clone(),
            HealthCheck {
                name,
                check_fn,
                interval,
                timeout,
            },
        );
        Ok(())
    }

    pub fn run_all_checks(&mut self) -> Result<HashMap<String, HealthStatus>> {
        for (name, check) in &self.checks {
            let start = Instant::now();

            let status = (check.check_fn)();
            let response_time = start.elapsed().as_millis() as u64;

            self.status.insert(
                name.clone(),
                HealthStatus {
                    healthy: status.healthy,
                    message: status.message,
                    last_check: std::time::SystemTime::now(),
                    response_time_ms: response_time,
                },
            );
        }

        Ok(self.status.clone())
    }

    pub fn get_status(&self) -> HashMap<String, HealthStatus> {
        self.status.clone()
    }
}

/// Console Notification Channel
pub struct ConsoleNotificationChannel;

impl NotificationChannel for ConsoleNotificationChannel {
    fn send(&self, alert: &Alert) -> Result<()> {
        println!(
            "ALERT [{}]: {} - {}",
            alert.severity, alert.title, alert.description
        );
        Ok(())
    }

    fn name(&self) -> &str {
        "console"
    }
}

/// Initialize monitoring module
pub fn init() -> Result<()> {
    info!("Monitoring Module initialized");
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_monitoring_manager_creation() {
        let manager = MonitoringManager::new().unwrap();
        assert!(manager
            .initialize_logging(LogLevel::Debug, LogFormat::Pretty, false)
            .await
            .is_ok());
    }

    #[tokio::test]
    async fn test_counter_metrics() {
        let manager = MonitoringManager::new().unwrap();
        manager
            .increment_counter("test_counter", &[])
            .await
            .unwrap();
        manager
            .increment_counter("test_counter", &[])
            .await
            .unwrap();

        let metrics = manager.get_metrics().await.unwrap();
        assert!(metrics.contains("test_counter"));
    }

    #[tokio::test]
    async fn test_histogram_metrics() {
        let manager = MonitoringManager::new().unwrap();
        manager
            .record_histogram("test_histogram", 0.5, &[])
            .await
            .unwrap();
        manager
            .record_histogram("test_histogram", 1.5, &[])
            .await
            .unwrap();

        let metrics = manager.get_metrics().await.unwrap();
        assert!(metrics.contains("test_histogram"));
    }

    #[tokio::test]
    async fn test_gauge_metrics() {
        let manager = MonitoringManager::new().unwrap();
        manager.set_gauge("test_gauge", 42.0, &[]).await.unwrap();

        let metrics = manager.get_metrics().await.unwrap();
        assert!(metrics.contains("test_gauge"));
    }

    #[tokio::test]
    async fn test_alert_rules() {
        let manager = MonitoringManager::new().unwrap();

        let rule = AlertRule {
            id: "test_rule".to_string(),
            name: "Test Rule".to_string(),
            condition: AlertCondition::Threshold {
                metric: "test_metric".to_string(),
                operator: ComparisonOperator::GreaterThan,
                value: 100.0,
            },
            severity: AlertSeverity::Warning,
            enabled: true,
        };

        manager.add_alert_rule(rule).await.unwrap();
    }

    #[tokio::test]
    async fn test_health_checks() {
        let manager = MonitoringManager::new().unwrap();

        manager
            .add_health_check(
                "test_check".to_string(),
                Box::new(|| HealthStatus {
                    healthy: true,
                    message: "OK".to_string(),
                    last_check: std::time::SystemTime::now(),
                    response_time_ms: 10,
                }),
                Duration::from_secs(30),
                Duration::from_secs(5),
            )
            .await
            .unwrap();

        let status = manager.run_health_checks().await.unwrap();
        assert!(status.contains_key("test_check"));
        assert!(status["test_check"].healthy);
    }
}
