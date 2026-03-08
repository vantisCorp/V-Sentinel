// Performance Dashboard and Monitoring for SENTINEL Security System
// Provides real-time performance monitoring and visualization

use std::collections::HashMap;
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::sync::RwLock;
use serde::{Serialize, Deserialize};
use prometheus::{Counter, Histogram, Gauge, Registry, TextEncoder, Encoder};

/// Performance metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceMetrics {
    pub timestamp: u64,
    pub component: String,
    pub operation: String,
    pub duration_ms: f64,
    pub cpu_usage_percent: f64,
    pub memory_usage_mb: f64,
    pub cache_hit_rate: f64,
    pub error_count: u64,
    pub throughput: f64,
}

/// Performance dashboard
pub struct PerformanceDashboard {
    metrics: Arc<RwLock<Vec<PerformanceMetrics>>>,
    registry: Registry,
    counters: HashMap<String, Counter>,
    histograms: HashMap<String, Histogram>,
    gauges: HashMap<String, Gauge>,
    enabled: bool,
}

impl PerformanceDashboard {
    pub fn new() -> Self {
        let registry = Registry::new();
        
        Self {
            metrics: Arc::new(RwLock::new(Vec::new())),
            registry,
            counters: HashMap::new(),
            histograms: HashMap::new(),
            gauges: HashMap::new(),
            enabled: true,
        }
    }

    pub fn enable(&mut self) {
        self.enabled = true;
    }

    pub fn disable(&mut self) {
        self.enabled = false;
    }

    pub fn is_enabled(&self) -> bool {
        self.enabled
    }

    pub async fn record_metric(&self, metric: PerformanceMetrics) {
        if !self.enabled {
            return;
        }

        let mut metrics = self.metrics.write().await;
        metrics.push(metric);
        
        // Keep only last 10,000 metrics
        if metrics.len() > 10_000 {
            let len = metrics.len();
            metrics.drain(0..len - 10_000);
        }
    }

    pub async fn get_metrics(&self, component: &str, operation: Option<&str>) -> Vec<PerformanceMetrics> {
        let metrics = self.metrics.read().await;
        
        if let Some(op) = operation {
            metrics.iter()
                .filter(|m| m.component == component && m.operation == op)
                .cloned()
                .collect()
        } else {
            metrics.iter()
                .filter(|m| m.component == component)
                .cloned()
                .collect()
        }
    }

    pub async fn get_all_metrics(&self) -> Vec<PerformanceMetrics> {
        let metrics = self.metrics.read().await;
        metrics.clone()
    }

    pub async fn get_summary(&self) -> PerformanceSummary {
        let metrics = self.metrics.read().await;
        
        if metrics.is_empty() {
            return PerformanceSummary::default();
        }

        let total_duration: f64 = metrics.iter().map(|m| m.duration_ms).sum();
        let avg_duration = total_duration / metrics.len() as f64;
        
        let mut durations: Vec<f64> = metrics.iter().map(|m| m.duration_ms).collect();
        durations.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
        
        let p50 = durations[durations.len() / 2];
        let p95 = durations[(durations.len() as f64 * 0.95) as usize];
        let p99 = durations[(durations.len() as f64 * 0.99) as usize];
        
        let avg_cpu = metrics.iter().map(|m| m.cpu_usage_percent).sum::<f64>() / metrics.len() as f64;
        let avg_memory = metrics.iter().map(|m| m.memory_usage_mb).sum::<f64>() / metrics.len() as f64;
        let avg_cache_hit = metrics.iter().map(|m| m.cache_hit_rate).sum::<f64>() / metrics.len() as f64;
        let total_errors = metrics.iter().map(|m| m.error_count).sum::<u64>();
        let avg_throughput = metrics.iter().map(|m| m.throughput).sum::<f64>() / metrics.len() as f64;

        PerformanceSummary {
            total_metrics: metrics.len(),
            avg_duration_ms: avg_duration,
            p50_duration_ms: p50,
            p95_duration_ms: p95,
            p99_duration_ms: p99,
            avg_cpu_usage_percent: avg_cpu,
            avg_memory_usage_mb: avg_memory,
            avg_cache_hit_rate: avg_cache_hit,
            total_errors,
            avg_throughput,
        }
    }

    pub async fn get_component_summary(&self, component: &str) -> ComponentSummary {
        let metrics = self.metrics.read().await;
        let component_metrics: Vec<_> = metrics.iter()
            .filter(|m| m.component == component)
            .collect();
        
        if component_metrics.is_empty() {
            return ComponentSummary::default();
        }

        let total_duration: f64 = component_metrics.iter().map(|m| m.duration_ms).sum();
        let avg_duration = total_duration / component_metrics.len() as f64;
        
        let mut durations: Vec<f64> = component_metrics.iter().map(|m| m.duration_ms).collect();
        durations.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
        
        let p50 = durations[durations.len() / 2];
        let p95 = durations[(durations.len() as f64 * 0.95) as usize];
        let p99 = durations[(durations.len() as f64 * 0.99) as usize];
        
        let avg_cpu = component_metrics.iter().map(|m| m.cpu_usage_percent).sum::<f64>() / component_metrics.len() as f64;
        let avg_memory = component_metrics.iter().map(|m| m.memory_usage_mb).sum::<f64>() / component_metrics.len() as f64;
        let total_errors = component_metrics.iter().map(|m| m.error_count).sum::<u64>();
        let avg_throughput = component_metrics.iter().map(|m| m.throughput).sum::<f64>() / component_metrics.len() as f64;

        ComponentSummary {
            component: component.to_string(),
            total_metrics: component_metrics.len(),
            avg_duration_ms: avg_duration,
            p50_duration_ms: p50,
            p95_duration_ms: p95,
            p99_duration_ms: p99,
            avg_cpu_usage_percent: avg_cpu,
            avg_memory_usage_mb: avg_memory,
            total_errors,
            avg_throughput,
        }
    }

    pub async fn generate_prometheus_metrics(&self) -> String {
        let encoder = TextEncoder::new();
        let metric_families = self.registry.gather();
        let mut buffer = Vec::new();
        
        if let Err(e) = encoder.encode(&metric_families, &mut buffer) {
            return format!("# Error encoding metrics: {}\n", e);
        }
        
        String::from_utf8(buffer).unwrap_or_else(|_| "# Error encoding metrics\n".to_string())
    }

    pub async fn generate_dashboard_html(&self) -> String {
        let summary = self.get_summary().await;
        let component_summaries = self.get_all_component_summaries().await;
        
        format!(
            r#"<!DOCTYPE html>
<html>
<head>
    <title>SENTINEL Performance Dashboard</title>
    <style>
        body {{ font-family: Arial, sans-serif; margin: 20px; background: #f5f5f5; }}
        .container {{ max-width: 1200px; margin: 0 auto; }}
        .header {{ background: linear-gradient(135deg, #667eea 0%, #764ba2 100%); color: white; padding: 20px; border-radius: 10px; margin-bottom: 20px; }}
        .metrics-grid {{ display: grid; grid-template-columns: repeat(auto-fit, minmax(250px, 1fr)); gap: 20px; margin-bottom: 20px; }}
        .metric-card {{ background: white; padding: 20px; border-radius: 10px; box-shadow: 0 2px 4px rgba(0,0,0,0.1); }}
        .metric-value {{ font-size: 2em; font-weight: bold; color: #667eea; }}
        .metric-label {{ color: #666; margin-top: 5px; }}
        .component-section {{ background: white; padding: 20px; border-radius: 10px; margin-bottom: 20px; box-shadow: 0 2px 4px rgba(0,0,0,0.1); }}
        .component-title {{ color: #667eea; margin-bottom: 15px; }}
        table {{ width: 100%; border-collapse: collapse; }}
        th, td {{ padding: 12px; text-align: left; border-bottom: 1px solid #ddd; }}
        th {{ background: #f8f9fa; }}
        .status-good {{ color: #28a745; }}
        .status-warning {{ color: #ffc107; }}
        .status-error {{ color: #dc3545; }}
    </style>
</head>
<body>
    <div class="container">
        <div class="header">
            <h1>🛡️ SENTINEL Performance Dashboard</h1>
            <p>Real-time performance monitoring and metrics</p>
        </div>
        
        <div class="metrics-grid">
            <div class="metric-card">
                <div class="metric-value">{}</div>
                <div class="metric-label">Total Metrics</div>
            </div>
            <div class="metric-card">
                <div class="metric-value">{:.2}ms</div>
                <div class="metric-label">Avg Duration</div>
            </div>
            <div class="metric-card">
                <div class="metric-value">{:.2}ms</div>
                <div class="metric-label">P95 Duration</div>
            </div>
            <div class="metric-card">
                <div class="metric-value">{:.1}%</div>
                <div class="metric-label">Avg CPU Usage</div>
            </div>
            <div class="metric-card">
                <div class="metric-value">{:.1}MB</div>
                <div class="metric-label">Avg Memory Usage</div>
            </div>
            <div class="metric-card">
                <div class="metric-value">{:.1}%</div>
                <div class="metric-label">Cache Hit Rate</div>
            </div>
            <div class="metric-card">
                <div class="metric-value">{}</div>
                <div class="metric-label">Total Errors</div>
            </div>
            <div class="metric-card">
                <div class="metric-value">{:.0}/s</div>
                <div class="metric-label">Avg Throughput</div>
            </div>
        </div>
        
        {}
    </div>
    
    <script>
        // Auto-refresh every 5 seconds
        setTimeout(function() {{ location.reload(); }}, 5000);
    </script>
</body>
</html>"#,
            summary.total_metrics,
            summary.avg_duration_ms,
            summary.p95_duration_ms,
            summary.avg_cpu_usage_percent,
            summary.avg_memory_usage_mb,
            summary.avg_cache_hit_rate,
            summary.total_errors,
            summary.avg_throughput,
            self.generate_component_html(&component_summaries)
        )
    }

    async fn get_all_component_summaries(&self) -> Vec<ComponentSummary> {
        let metrics = self.metrics.read().await;
        let mut components: HashMap<String, Vec<&PerformanceMetrics>> = HashMap::new();
        
        for metric in metrics.iter() {
            components.entry(metric.component.clone())
                .or_insert_with(Vec::new)
                .push(metric);
        }
        
        components.into_iter()
            .map(|(component, component_metrics)| {
                let total_duration: f64 = component_metrics.iter().map(|m| m.duration_ms).sum();
                let avg_duration = total_duration / component_metrics.len() as f64;
                
                let mut durations: Vec<f64> = component_metrics.iter().map(|m| m.duration_ms).collect();
                durations.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
                
                let p50 = durations[durations.len() / 2];
                let p95 = durations[(durations.len() as f64 * 0.95) as usize];
                let p99 = durations[(durations.len() as f64 * 0.99) as usize];
                
                let avg_cpu = component_metrics.iter().map(|m| m.cpu_usage_percent).sum::<f64>() / component_metrics.len() as f64;
                let avg_memory = component_metrics.iter().map(|m| m.memory_usage_mb).sum::<f64>() / component_metrics.len() as f64;
                let total_errors = component_metrics.iter().map(|m| m.error_count).sum::<u64>();
                let avg_throughput = component_metrics.iter().map(|m| m.throughput).sum::<f64>() / component_metrics.len() as f64;

                ComponentSummary {
                    component,
                    total_metrics: component_metrics.len(),
                    avg_duration_ms: avg_duration,
                    p50_duration_ms: p50,
                    p95_duration_ms: p95,
                    p99_duration_ms: p99,
                    avg_cpu_usage_percent: avg_cpu,
                    avg_memory_usage_mb: avg_memory,
                    total_errors,
                    avg_throughput,
                }
            })
            .collect()
    }

    fn generate_component_html(&self, summaries: &[ComponentSummary]) -> String {
        let mut html = String::new();
        
        for summary in summaries {
            html.push_str(&format!(
                r#"<div class="component-section">
                    <h2 class="component-title">📊 {}</h2>
                    <table>
                        <tr>
                            <th>Metric</th>
                            <th>Value</th>
                            <th>Status</th>
                        </tr>
                        <tr>
                            <td>Total Metrics</td>
                            <td>{}</td>
                            <td class="status-good">✓</td>
                        </tr>
                        <tr>
                            <td>Avg Duration</td>
                            <td>{:.2}ms</td>
                            <td class="status-good">✓</td>
                        </tr>
                        <tr>
                            <td>P95 Duration</td>
                            <td>{:.2}ms</td>
                            <td class="status-good">✓</td>
                        </tr>
                        <tr>
                            <td>P99 Duration</td>
                            <td>{:.2}ms</td>
                            <td class="status-good">✓</td>
                        </tr>
                        <tr>
                            <td>Avg CPU Usage</td>
                            <td>{:.1}%</td>
                            <td class="status-good">✓</td>
                        </tr>
                        <tr>
                            <td>Avg Memory Usage</td>
                            <td>{:.1}MB</td>
                            <td class="status-good">✓</td>
                        </tr>
                        <tr>
                            <td>Total Errors</td>
                            <td>{}</td>
                            <td class="{}">{}</td>
                        </tr>
                        <tr>
                            <td>Avg Throughput</td>
                            <td>{:.0}/s</td>
                            <td class="status-good">✓</td>
                        </tr>
                    </table>
                </div>"#,
                summary.component,
                summary.total_metrics,
                summary.avg_duration_ms,
                summary.p95_duration_ms,
                summary.p99_duration_ms,
                summary.avg_cpu_usage_percent,
                summary.avg_memory_usage_mb,
                summary.total_errors,
                if summary.total_errors > 0 { "status-error" } else { "status-good" },
                if summary.total_errors > 0 { "✗" } else { "✓" },
                summary.avg_throughput
            ));
        }
        
        html
    }

    pub async fn clear_metrics(&self) {
        let mut metrics = self.metrics.write().await;
        metrics.clear();
    }
}

/// Performance summary
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceSummary {
    pub total_metrics: usize,
    pub avg_duration_ms: f64,
    pub p50_duration_ms: f64,
    pub p95_duration_ms: f64,
    pub p99_duration_ms: f64,
    pub avg_cpu_usage_percent: f64,
    pub avg_memory_usage_mb: f64,
    pub avg_cache_hit_rate: f64,
    pub total_errors: u64,
    pub avg_throughput: f64,
}

impl Default for PerformanceSummary {
    fn default() -> Self {
        Self {
            total_metrics: 0,
            avg_duration_ms: 0.0,
            p50_duration_ms: 0.0,
            p95_duration_ms: 0.0,
            p99_duration_ms: 0.0,
            avg_cpu_usage_percent: 0.0,
            avg_memory_usage_mb: 0.0,
            avg_cache_hit_rate: 0.0,
            total_errors: 0,
            avg_throughput: 0.0,
        }
    }
}

/// Component summary
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComponentSummary {
    pub component: String,
    pub total_metrics: usize,
    pub avg_duration_ms: f64,
    pub p50_duration_ms: f64,
    pub p95_duration_ms: f64,
    pub p99_duration_ms: f64,
    pub avg_cpu_usage_percent: f64,
    pub avg_memory_usage_mb: f64,
    pub total_errors: u64,
    pub avg_throughput: f64,
}

impl Default for ComponentSummary {
    fn default() -> Self {
        Self {
            component: String::new(),
            total_metrics: 0,
            avg_duration_ms: 0.0,
            p50_duration_ms: 0.0,
            p95_duration_ms: 0.0,
            p99_duration_ms: 0.0,
            avg_cpu_usage_percent: 0.0,
            avg_memory_usage_mb: 0.0,
            total_errors: 0,
            avg_throughput: 0.0,
        }
    }
}

impl Default for PerformanceDashboard {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_dashboard_creation() {
        let dashboard = PerformanceDashboard::new();
        assert!(dashboard.is_enabled());
    }

    #[tokio::test]
    async fn test_metric_recording() {
        let dashboard = PerformanceDashboard::new();
        
        let metric = PerformanceMetrics {
            timestamp: chrono::Utc::now().timestamp() as u64,
            component: "Test".to_string(),
            operation: "Operation".to_string(),
            duration_ms: 10.0,
            cpu_usage_percent: 50.0,
            memory_usage_mb: 100.0,
            cache_hit_rate: 0.95,
            error_count: 0,
            throughput: 1000.0,
        };
        
        dashboard.record_metric(metric).await;
        
        let metrics = dashboard.get_metrics("Test", Some("Operation")).await;
        assert_eq!(metrics.len(), 1);
    }

    #[tokio::test]
    async fn test_summary_generation() {
        let dashboard = PerformanceDashboard::new();
        
        for i in 0..10 {
            let metric = PerformanceMetrics {
                timestamp: chrono::Utc::now().timestamp() as u64,
                component: "Test".to_string(),
                operation: "Operation".to_string(),
                duration_ms: (i * 10) as f64,
                cpu_usage_percent: 50.0,
                memory_usage_mb: 100.0,
                cache_hit_rate: 0.95,
                error_count: 0,
                throughput: 1000.0,
            };
            dashboard.record_metric(metric).await;
        }
        
        let summary = dashboard.get_summary().await;
        assert_eq!(summary.total_metrics, 10);
        assert!(summary.avg_duration_ms > 0.0);
    }

    #[tokio::test]
    async fn test_dashboard_html_generation() {
        let dashboard = PerformanceDashboard::new();
        
        let metric = PerformanceMetrics {
            timestamp: chrono::Utc::now().timestamp() as u64,
            component: "Test".to_string(),
            operation: "Operation".to_string(),
            duration_ms: 10.0,
            cpu_usage_percent: 50.0,
            memory_usage_mb: 100.0,
            cache_hit_rate: 0.95,
            error_count: 0,
            throughput: 1000.0,
        };
        
        dashboard.record_metric(metric).await;
        
        let html = dashboard.generate_dashboard_html().await;
        assert!(html.contains("SENTINEL Performance Dashboard"));
        assert!(html.contains("Test"));
    }
}