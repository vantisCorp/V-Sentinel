//! MLOps Security Engine
//! 
//! Provides security for MLOps pipelines including infrastructure protection,
// model deployment security, monitoring, and drift detection.

use crate::ai_security::models::*;
use crate::ai_security::AISecurityError;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

/// MLOps Security Configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MLOpsSecurityConfig {
    /// Enable drift detection
    pub drift_detection_enabled: bool,
    /// Drift threshold
    pub drift_threshold: f64,
    /// Enable performance monitoring
    pub performance_monitoring: bool,
    /// Performance degradation threshold
    pub performance_threshold: f64,
    /// Enable infrastructure monitoring
    pub infrastructure_monitoring: bool,
    /// Resource usage thresholds
    pub resource_thresholds: ResourceThresholds,
}

impl Default for MLOpsSecurityConfig {
    fn default() -> Self {
        Self {
            drift_detection_enabled: true,
            drift_threshold: 0.7,
            performance_monitoring: true,
            performance_threshold: 0.85,
            infrastructure_monitoring: true,
            resource_thresholds: ResourceThresholds::default(),
        }
    }
}

/// Resource usage thresholds
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceThresholds {
    pub cpu_threshold: f64,
    pub memory_threshold: f64,
    pub gpu_threshold: Option<f64>,
}

impl Default for ResourceThresholds {
    fn default() -> Self {
        Self {
            cpu_threshold: 80.0,
            memory_threshold: 85.0,
            gpu_threshold: Some(90.0),
        }
    }
}

/// MLOps Security Engine
pub struct MLOpsSecurityEngine {
    config: MLOpsSecurityConfig,
    drift_detector: DriftDetector,
    performance_monitor: PerformanceMonitor,
    infrastructure_monitor: InfrastructureMonitor,
    baseline_store: Arc<RwLock<HashMap<String, BaselineMetrics>>>,
}

impl MLOpsSecurityEngine {
    /// Create a new MLOps Security Engine
    pub async fn new(config: MLOpsSecurityConfig) -> Result<Self, AISecurityError> {
        let drift_detector = DriftDetector::new(config.drift_threshold);
        let performance_monitor = PerformanceMonitor::new(config.performance_threshold);
        let infrastructure_monitor = InfrastructureMonitor::new(config.resource_thresholds);
        let baseline_store = Arc::new(RwLock::new(HashMap::new()));

        Ok(Self {
            config,
            drift_detector,
            performance_monitor,
            infrastructure_monitor,
            baseline_store,
        })
    }

    /// Monitor MLOps pipeline
    pub async fn monitor_pipeline(&self, metrics: &MLOpsMetrics) -> Result<MLOpsReport, AISecurityError> {
        let mut issues = Vec::new();
        let mut status = MLOpsSecurityStatus::Secure;
        let mut recommendations = Vec::new();

        // Step 1: Check for drift
        if self.config.drift_detection_enabled {
            if let Some(ref drift_metrics) = metrics.drift {
                let drift_result = self.detect_drift(&metrics.pipeline_id, drift_metrics).await?;
                
                if drift_result.drift_detected {
                    issues.push(MLOpsIssue {
                        id: format!("drift_{}", metrics.pipeline_id),
                        issue_type: MLOpsIssueType::ModelDrift,
                        description: format!("Model drift detected (score: {})", drift_metrics.model_drift),
                        severity: if drift_metrics.model_drift > 0.9 {
                            Severity::High
                        } else {
                            Severity::Medium
                        },
                        component: metrics.pipeline_id.clone(),
                        detected_at: Utc::now(),
                    });
                    status = MLOpsSecurityStatus::AtRisk;
                    recommendations.push("Consider retraining the model".to_string());
                }
            }
        }

        // Step 2: Check performance
        if self.config.performance_monitoring {
            let perf_result = self.check_performance(&metrics.pipeline_id, &metrics.performance).await?;
            
            if !perf_result.is_acceptable {
                issues.push(MLOpsIssue {
                    id: format!("perf_{}", metrics.pipeline_id),
                    issue_type: MLOpsIssueType::PerformanceDegradation,
                    description: format!("Performance degradation detected (latency: {}ms)", metrics.performance.latency_ms),
                    severity: if metrics.performance.latency_ms > 1000.0 {
                        Severity::High
                    } else {
                        Severity::Medium
                    },
                    component: metrics.pipeline_id.clone(),
                    detected_at: Utc::now(),
                });
                status = MLOpsSecurityStatus::Warning;
                recommendations.push("Optimize model or increase resources".to_string());
            }
        }

        // Step 3: Check infrastructure
        if self.config.infrastructure_monitoring {
            let infra_result = self.check_infrastructure(&metrics.pipeline_id, &metrics.resources).await?;
            
            if !infra_result.is_healthy {
                issues.push(MLOpsIssue {
                    id: format!("infra_{}", metrics.pipeline_id),
                    issue_type: MLOpsIssueType::InfrastructureCompromise,
                    description: format!("Resource usage high (CPU: {}%, Memory: {}%)", 
                        metrics.resources.cpu_usage, metrics.resources.memory_usage),
                    severity: Severity::Medium,
                    component: metrics.pipeline_id.clone(),
                    detected_at: Utc::now(),
                });
                if status == MLOpsSecurityStatus::Secure {
                    status = MLOpsSecurityStatus::Warning;
                }
                recommendations.push("Scale infrastructure or optimize resource usage".to_string());
            }
        }

        // Update baseline if no issues
        if issues.is_empty() {
            self.update_baseline(&metrics.pipeline_id, metrics).await?;
        }

        Ok(MLOpsReport {
            pipeline_id: metrics.pipeline_id.clone(),
            status,
            issues,
            recommendations,
            generated_at: Utc::now(),
        })
    }

    /// Detect drift
    async fn detect_drift(&self, pipeline_id: &str, drift_metrics: &DriftMetrics) -> Result<DriftResult, AISecurityError> {
        let baseline = self.baseline_store.read().await;
        let drift_threshold = self.config.drift_threshold;

        let drift_detected = drift_metrics.data_drift > drift_threshold
            || drift_metrics.model_drift > drift_threshold
            || drift_metrics.concept_drift > drift_threshold;

        Ok(DriftResult {
            drift_detected,
            max_drift_score: drift_metrics.data_drift
                .max(drift_metrics.model_drift)
                .max(drift_metrics.concept_drift),
            drift_type: if drift_metrics.data_drift > drift_threshold {
                Some("data_drift".to_string())
            } else if drift_metrics.model_drift > drift_threshold {
                Some("model_drift".to_string())
            } else if drift_metrics.concept_drift > drift_threshold {
                Some("concept_drift".to_string())
            } else {
                None
            },
        })
    }

    /// Check performance
    async fn check_performance(&self, pipeline_id: &str, perf: &PerformanceMetrics) -> Result<PerformanceCheckResult, AISecurityError> {
        let threshold = self.config.performance_threshold;
        let latency_threshold = 1000.0; // 1 second

        let is_acceptable = perf.accuracy.unwrap_or(1.0) >= threshold
            && perf.latency_ms <= latency_threshold;

        Ok(PerformanceCheckResult {
            is_acceptable,
            metrics_score: perf.accuracy.unwrap_or(1.0),
            latency_ok: perf.latency_ms <= latency_threshold,
        })
    }

    /// Check infrastructure
    async fn check_infrastructure(&self, _pipeline_id: &str, resources: &ResourceMetrics) -> Result<InfrastructureCheckResult, AISecurityError> {
        let thresholds = &self.config.resource_thresholds;

        let is_healthy = resources.cpu_usage <= thresholds.cpu_threshold
            && resources.memory_usage <= thresholds.memory_threshold
            && resources.gpu_usage.map_or(true, |gpu| gpu <= thresholds.gpu_threshold.unwrap_or(100.0));

        Ok(InfrastructureCheckResult { is_healthy })
    }

    /// Update baseline metrics
    async fn update_baseline(&self, pipeline_id: &str, metrics: &MLOpsMetrics) -> Result<(), AISecurityError> {
        let mut baseline = self.baseline_store.write().await;
        baseline.insert(pipeline_id.to_string(), BaselineMetrics::from(metrics.clone()));
        Ok(())
    }

    /// Get component status
    pub async fn get_status(&self) -> Result<ComponentStatus, AISecurityError> {
        let baseline = self.baseline_store.read().await;
        
        Ok(ComponentStatus {
            healthy: true,
            active_protections: 3, // drift, performance, infrastructure
            threats_detected: 0,
            last_check: Utc::now(),
        })
    }
}

/// Drift Detector
pub struct DriftDetector {
    threshold: f64,
}

impl DriftDetector {
    pub fn new(threshold: f64) -> Self {
        Self { threshold }
    }

    pub fn calculate_drift(&self, current: &[f64], baseline: &[f64]) -> f64 {
        if current.len() != baseline.len() {
            return 1.0;
        }

        // Calculate KL divergence (simplified)
        let mut divergence = 0.0;
        for (c, b) in current.iter().zip(baseline.iter()) {
            if *b > 0.0 && *c > 0.0 {
                divergence += (c / b).ln() * c;
            }
        }

        divergence.min(1.0).max(0.0)
    }
}

/// Drift Detection Result
struct DriftResult {
    drift_detected: bool,
    max_drift_score: f64,
    drift_type: Option<String>,
}

/// Performance Monitor
pub struct PerformanceMonitor {
    threshold: f64,
}

impl PerformanceMonitor {
    pub fn new(threshold: f64) -> Self {
        Self { threshold }
    }

    pub fn analyze_performance(&self, metrics: &PerformanceMetrics) -> PerformanceAnalysis {
        let accuracy = metrics.accuracy.unwrap_or(1.0);
        let latency = metrics.latency_ms;

        let is_degraded = accuracy < self.threshold || latency > 1000.0;

        PerformanceAnalysis {
            is_degraded,
            accuracy_score: accuracy,
            latency_score: (1000.0 - latency) / 1000.0,
        }
    }
}

/// Performance Analysis
struct PerformanceAnalysis {
    is_degraded: bool,
    accuracy_score: f64,
    latency_score: f64,
}

/// Performance Check Result
struct PerformanceCheckResult {
    is_acceptable: bool,
    metrics_score: f64,
    latency_ok: bool,
}

/// Infrastructure Monitor
pub struct InfrastructureMonitor {
    thresholds: ResourceThresholds,
}

impl InfrastructureMonitor {
    pub fn new(thresholds: ResourceThresholds) -> Self {
        Self { thresholds }
    }

    pub fn check_resources(&self, resources: &ResourceMetrics) -> InfrastructureHealth {
        let cpu_healthy = resources.cpu_usage <= self.thresholds.cpu_threshold;
        let memory_healthy = resources.memory_usage <= self.thresholds.memory_threshold;
        let gpu_healthy = resources.gpu_usage.map_or(true, |gpu| gpu <= self.thresholds.gpu_threshold.unwrap_or(100.0));

        let is_healthy = cpu_healthy && memory_healthy && gpu_healthy;

        InfrastructureHealth {
            is_healthy,
            cpu_healthy,
            memory_healthy,
            gpu_healthy,
        }
    }
}

/// Infrastructure Health
struct InfrastructureHealth {
    is_healthy: bool,
    cpu_healthy: bool,
    memory_healthy: bool,
    gpu_healthy: bool,
}

/// Infrastructure Check Result
struct InfrastructureCheckResult {
    is_healthy: bool,
}

/// Baseline Metrics
#[derive(Debug, Clone)]
struct BaselineMetrics {
    pipeline_id: String,
    performance: PerformanceMetrics,
    timestamp: DateTime<Utc>,
}

impl From<MLOpsMetrics> for BaselineMetrics {
    fn from(metrics: MLOpsMetrics) -> Self {
        Self {
            pipeline_id: metrics.pipeline_id,
            performance: metrics.performance,
            timestamp: metrics.timestamp,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_mlops_security_engine_creation() {
        let config = MLOpsSecurityConfig::default();
        let engine = MLOpsSecurityEngine::new(config).await;
        assert!(engine.is_ok());
    }

    #[tokio::test]
    async fn test_monitor_pipeline() {
        let config = MLOpsSecurityConfig::default();
        let engine = MLOpsSecurityEngine::new(config).await.unwrap();
        
        let metrics = MLOpsMetrics {
            pipeline_id: "pipeline-1".to_string(),
            model_version: "v1.0".to_string(),
            performance: PerformanceMetrics {
                accuracy: Some(0.95),
                precision: None,
                recall: None,
                f1_score: None,
                latency_ms: 100.0,
                throughput: 1000.0,
            },
            drift: Some(DriftMetrics {
                data_drift: 0.1,
                model_drift: 0.1,
                concept_drift: 0.1,
                drift_detected: false,
            }),
            resources: ResourceMetrics {
                cpu_usage: 50.0,
                memory_usage: 60.0,
                gpu_usage: None,
                disk_io: 100.0,
                network_io: 50.0,
            },
            timestamp: Utc::now(),
        };
        
        let result = engine.monitor_pipeline(&metrics).await;
        assert!(result.is_ok());
    }
}