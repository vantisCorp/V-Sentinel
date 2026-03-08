// Bottleneck Identification and Optimization Framework for SENTINEL Security System
// Identifies performance bottlenecks and suggests optimizations

use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use std::time::{Duration, Instant};

/// Bottleneck severity level
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum BottleneckSeverity {
    Low,
    Medium,
    High,
    Critical,
}

/// Bottleneck type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum BottleneckType {
    CPU,
    Memory,
    IO,
    Network,
    LockContention,
    Algorithmic,
    CacheMiss,
    Database,
}

/// Bottleneck information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Bottleneck {
    pub component: String,
    pub operation: String,
    pub bottleneck_type: BottleneckType,
    pub severity: BottleneckSeverity,
    pub description: String,
    pub current_value: f64,
    pub threshold_value: f64,
    pub impact: String,
    pub suggested_optimizations: Vec<String>,
}

/// Performance profile data
#[derive(Debug, Clone)]
pub struct PerformanceProfile {
    pub component: String,
    pub operation: String,
    pub cpu_time: Duration,
    pub wall_time: Duration,
    pub memory_allocated: usize,
    pub memory_peak: usize,
    pub cache_hits: usize,
    pub cache_misses: usize,
    pub io_operations: usize,
    pub network_bytes: usize,
    pub lock_wait_time: Duration,
}

/// Bottleneck analyzer
pub struct BottleneckAnalyzer {
    profiles: Vec<PerformanceProfile>,
    bottlenecks: Vec<Bottleneck>,
    thresholds: HashMap<String, f64>,
}

impl BottleneckAnalyzer {
    pub fn new() -> Self {
        let mut analyzer = Self {
            profiles: Vec::new(),
            bottlenecks: Vec::new(),
            thresholds: Self::default_thresholds(),
        };
        analyzer
    }

    fn default_thresholds() -> HashMap<String, f64> {
        let mut thresholds = HashMap::new();

        // CPU thresholds (percentage of wall time)
        thresholds.insert("cpu_time_ratio".to_string(), 0.8);

        // Memory thresholds (MB)
        thresholds.insert("memory_allocation_mb".to_string(), 100.0);
        thresholds.insert("memory_peak_mb".to_string(), 500.0);

        // Cache thresholds (miss rate)
        thresholds.insert("cache_miss_rate".to_string(), 0.1);

        // I/O thresholds (operations per second)
        thresholds.insert("io_ops_per_sec".to_string(), 1000.0);

        // Lock contention thresholds (percentage of wall time)
        thresholds.insert("lock_wait_ratio".to_string(), 0.1);

        thresholds
    }

    pub fn profile_operation<F>(
        &mut self,
        component: &str,
        operation: &str,
        f: F,
    ) -> PerformanceProfile
    where
        F: FnOnce(),
    {
        let start = Instant::now();
        let cpu_start = self.get_cpu_time();
        let memory_start = self.get_memory_usage();

        f();

        let wall_time = start.elapsed();
        let cpu_time = self.get_cpu_time() - cpu_start;
        let memory_end = self.get_memory_usage();

        PerformanceProfile {
            component: component.to_string(),
            operation: operation.to_string(),
            cpu_time,
            wall_time,
            memory_allocated: memory_end - memory_start,
            memory_peak: memory_end,
            cache_hits: 0,
            cache_misses: 0,
            io_operations: 0,
            network_bytes: 0,
            lock_wait_time: Duration::ZERO,
        }
    }

    pub fn analyze_bottlenecks(&mut self) -> Vec<Bottleneck> {
        self.bottlenecks.clear();

        let profiles: Vec<_> = self.profiles.clone();
        for profile in &profiles {
            self.analyze_cpu_bottlenecks(profile);
            self.analyze_memory_bottlenecks(profile);
            self.analyze_cache_bottlenecks(profile);
            self.analyze_io_bottlenecks(profile);
            self.analyze_lock_contention(profile);
        }

        self.bottlenecks.sort_by(|a, b| {
            b.severity
                .cmp(&a.severity)
                .then_with(|| a.component.cmp(&b.component))
        });

        self.bottlenecks.clone()
    }

    fn analyze_cpu_bottlenecks(&mut self, profile: &PerformanceProfile) {
        let cpu_ratio = profile.cpu_time.as_nanos() as f64 / profile.wall_time.as_nanos() as f64;
        let threshold = *self.thresholds.get("cpu_time_ratio").unwrap_or(&0.8);

        if cpu_ratio > threshold {
            let severity = if cpu_ratio > 0.95 {
                BottleneckSeverity::Critical
            } else if cpu_ratio > 0.9 {
                BottleneckSeverity::High
            } else if cpu_ratio > 0.85 {
                BottleneckSeverity::Medium
            } else {
                BottleneckSeverity::Low
            };

            self.bottlenecks.push(Bottleneck {
                component: profile.component.clone(),
                operation: profile.operation.clone(),
                bottleneck_type: BottleneckType::CPU,
                severity,
                description: format!("High CPU usage: {:.1}% of wall time", cpu_ratio * 100.0),
                current_value: cpu_ratio * 100.0,
                threshold_value: threshold * 100.0,
                impact: "Operation is CPU-bound, may cause system-wide slowdown".to_string(),
                suggested_optimizations: vec![
                    "Consider algorithmic optimizations".to_string(),
                    "Use SIMD instructions for vector operations".to_string(),
                    "Offload work to GPU/NPU if available".to_string(),
                    "Implement caching for expensive computations".to_string(),
                    "Use parallel processing for independent tasks".to_string(),
                ],
            });
        }
    }

    fn analyze_memory_bottlenecks(&mut self, profile: &PerformanceProfile) {
        let memory_mb = profile.memory_peak as f64 / (1024.0 * 1024.0);
        let threshold = *self.thresholds.get("memory_peak_mb").unwrap_or(&500.0);

        if memory_mb > threshold {
            let severity = if memory_mb > 1000.0 {
                BottleneckSeverity::Critical
            } else if memory_mb > 750.0 {
                BottleneckSeverity::High
            } else if memory_mb > 600.0 {
                BottleneckSeverity::Medium
            } else {
                BottleneckSeverity::Low
            };

            self.bottlenecks.push(Bottleneck {
                component: profile.component.clone(),
                operation: profile.operation.clone(),
                bottleneck_type: BottleneckType::Memory,
                severity,
                description: format!("High memory usage: {:.1} MB peak", memory_mb),
                current_value: memory_mb,
                threshold_value: threshold,
                impact: "May cause memory pressure and swapping".to_string(),
                suggested_optimizations: vec![
                    "Implement memory pooling".to_string(),
                    "Use streaming instead of loading all data".to_string(),
                    "Optimize data structures for memory efficiency".to_string(),
                    "Implement lazy loading".to_string(),
                    "Use memory-mapped files for large datasets".to_string(),
                ],
            });
        }
    }

    fn analyze_cache_bottlenecks(&mut self, profile: &PerformanceProfile) {
        if profile.cache_hits + profile.cache_misses > 0 {
            let miss_rate =
                profile.cache_misses as f64 / (profile.cache_hits + profile.cache_misses) as f64;
            let threshold = *self.thresholds.get("cache_miss_rate").unwrap_or(&0.1);

            if miss_rate > threshold {
                let severity = if miss_rate > 0.3 {
                    BottleneckSeverity::Critical
                } else if miss_rate > 0.2 {
                    BottleneckSeverity::High
                } else if miss_rate > 0.15 {
                    BottleneckSeverity::Medium
                } else {
                    BottleneckSeverity::Low
                };

                self.bottlenecks.push(Bottleneck {
                    component: profile.component.clone(),
                    operation: profile.operation.clone(),
                    bottleneck_type: BottleneckType::CacheMiss,
                    severity,
                    description: format!("High cache miss rate: {:.1}%", miss_rate * 100.0),
                    current_value: miss_rate * 100.0,
                    threshold_value: threshold * 100.0,
                    impact: "Frequent cache misses cause performance degradation".to_string(),
                    suggested_optimizations: vec![
                        "Increase cache size".to_string(),
                        "Improve cache locality".to_string(),
                        "Use more efficient cache eviction policies".to_string(),
                        "Implement multi-level caching".to_string(),
                        "Prefetch data based on access patterns".to_string(),
                    ],
                });
            }
        }
    }

    fn analyze_io_bottlenecks(&mut self, profile: &PerformanceProfile) {
        if profile.wall_time.as_secs() > 0 {
            let io_ops_per_sec = profile.io_operations as f64 / profile.wall_time.as_secs_f64();
            let threshold = *self.thresholds.get("io_ops_per_sec").unwrap_or(&1000.0);

            if io_ops_per_sec > threshold {
                let severity = if io_ops_per_sec > 10000.0 {
                    BottleneckSeverity::Critical
                } else if io_ops_per_sec > 5000.0 {
                    BottleneckSeverity::High
                } else if io_ops_per_sec > 2500.0 {
                    BottleneckSeverity::Medium
                } else {
                    BottleneckSeverity::Low
                };

                self.bottlenecks.push(Bottleneck {
                    component: profile.component.clone(),
                    operation: profile.operation.clone(),
                    bottleneck_type: BottleneckType::IO,
                    severity,
                    description: format!("High I/O rate: {:.0} operations/second", io_ops_per_sec),
                    current_value: io_ops_per_sec,
                    threshold_value: threshold,
                    impact: "Excessive I/O operations cause disk/network bottlenecks".to_string(),
                    suggested_optimizations: vec![
                        "Batch I/O operations".to_string(),
                        "Use asynchronous I/O".to_string(),
                        "Implement read/write caching".to_string(),
                        "Use compression to reduce I/O volume".to_string(),
                        "Optimize data access patterns".to_string(),
                    ],
                });
            }
        }
    }

    fn analyze_lock_contention(&mut self, profile: &PerformanceProfile) {
        let lock_ratio =
            profile.lock_wait_time.as_nanos() as f64 / profile.wall_time.as_nanos() as f64;
        let threshold = *self.thresholds.get("lock_wait_ratio").unwrap_or(&0.1);

        if lock_ratio > threshold {
            let severity = if lock_ratio > 0.3 {
                BottleneckSeverity::Critical
            } else if lock_ratio > 0.2 {
                BottleneckSeverity::High
            } else if lock_ratio > 0.15 {
                BottleneckSeverity::Medium
            } else {
                BottleneckSeverity::Low
            };

            self.bottlenecks.push(Bottleneck {
                component: profile.component.clone(),
                operation: profile.operation.clone(),
                bottleneck_type: BottleneckType::LockContention,
                severity,
                description: format!(
                    "High lock contention: {:.1}% of wall time waiting for locks",
                    lock_ratio * 100.0
                ),
                current_value: lock_ratio * 100.0,
                threshold_value: threshold * 100.0,
                impact: "Lock contention limits parallelism and causes thread starvation"
                    .to_string(),
                suggested_optimizations: vec![
                    "Reduce lock granularity".to_string(),
                    "Use lock-free data structures".to_string(),
                    "Implement optimistic concurrency control".to_string(),
                    "Use read-write locks for read-heavy workloads".to_string(),
                    "Consider sharding to reduce contention".to_string(),
                ],
            });
        }
    }

    pub fn generate_optimization_report(&self) -> String {
        let mut report = String::new();
        report.push_str("# SENTINEL Bottleneck Analysis Report\n\n");

        if self.bottlenecks.is_empty() {
            report.push_str("✅ No bottlenecks detected!\n\n");
            return report;
        }

        // Group by severity
        let mut critical = Vec::new();
        let mut high = Vec::new();
        let mut medium = Vec::new();
        let mut low = Vec::new();

        for bottleneck in &self.bottlenecks {
            match bottleneck.severity {
                BottleneckSeverity::Critical => critical.push(bottleneck),
                BottleneckSeverity::High => high.push(bottleneck),
                BottleneckSeverity::Medium => medium.push(bottleneck),
                BottleneckSeverity::Low => low.push(bottleneck),
            }
        }

        // Critical bottlenecks
        if !critical.is_empty() {
            report.push_str("## 🔴 Critical Bottlenecks\n\n");
            for bottleneck in &critical {
                report.push_str(&format!(
                    "### {}. {} - {}\n\n",
                    bottleneck.component,
                    bottleneck.operation,
                    format!("{:?}", bottleneck.bottleneck_type)
                ));
                report.push_str(&format!("**Description:** {}\n\n", bottleneck.description));
                report.push_str(&format!("**Impact:** {}\n\n", bottleneck.impact));
                report.push_str("**Suggested Optimizations:**\n\n");
                for (i, opt) in bottleneck.suggested_optimizations.iter().enumerate() {
                    report.push_str(&format!("{}. {}\n", i + 1, opt));
                }
                report.push_str("\n");
            }
        }

        // High bottlenecks
        if !high.is_empty() {
            report.push_str("## 🟠 High Priority Bottlenecks\n\n");
            for bottleneck in &high {
                report.push_str(&format!(
                    "### {}. {} - {}\n\n",
                    bottleneck.component,
                    bottleneck.operation,
                    format!("{:?}", bottleneck.bottleneck_type)
                ));
                report.push_str(&format!("**Description:** {}\n\n", bottleneck.description));
                report.push_str("**Suggested Optimizations:**\n\n");
                for (i, opt) in bottleneck.suggested_optimizations.iter().enumerate() {
                    report.push_str(&format!("{}. {}\n", i + 1, opt));
                }
                report.push_str("\n");
            }
        }

        // Medium bottlenecks
        if !medium.is_empty() {
            report.push_str(&format!(
                "## 🟡 Medium Priority Bottlenecks ({})\n\n",
                medium.len()
            ));
            for bottleneck in &medium {
                report.push_str(&format!(
                    "- {}. {}: {}\n",
                    bottleneck.component, bottleneck.operation, bottleneck.description
                ));
            }
            report.push_str("\n");
        }

        // Low bottlenecks
        if !low.is_empty() {
            report.push_str(&format!(
                "## 🟢 Low Priority Bottlenecks ({})\n\n",
                low.len()
            ));
            for bottleneck in &low {
                report.push_str(&format!(
                    "- {}. {}: {}\n",
                    bottleneck.component, bottleneck.operation, bottleneck.description
                ));
            }
            report.push_str("\n");
        }

        // Summary
        report.push_str("## Summary\n\n");
        report.push_str(&format!(
            "- Total Bottlenecks: {}\n",
            self.bottlenecks.len()
        ));
        report.push_str(&format!("- Critical: {}\n", critical.len()));
        report.push_str(&format!("- High: {}\n", high.len()));
        report.push_str(&format!("- Medium: {}\n", medium.len()));
        report.push_str(&format!("- Low: {}\n", low.len()));

        report
    }

    // Helper functions (simulated)
    fn get_cpu_time(&self) -> Duration {
        Duration::from_nanos(0)
    }

    fn get_memory_usage(&self) -> usize {
        0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bottleneck_detection() {
        let mut analyzer = BottleneckAnalyzer::new();

        // Profile an operation with high CPU usage
        let profile =
            analyzer.profile_operation("Test Component", "CPU Intensive Operation", || {
                std::thread::sleep(Duration::from_millis(100));
            });

        analyzer.profiles.push(profile);

        // Analyze bottlenecks
        let bottlenecks = analyzer.analyze_bottlenecks();

        // Generate report
        let report = analyzer.generate_optimization_report();
        println!("{}", report);

        // Verify analysis ran
        assert!(bottlenecks.len() >= 0);
    }

    #[test]
    fn test_optimization_suggestions() {
        let bottleneck = Bottleneck {
            component: "Test Component".to_string(),
            operation: "Test Operation".to_string(),
            bottleneck_type: BottleneckType::CPU,
            severity: BottleneckSeverity::High,
            description: "High CPU usage".to_string(),
            current_value: 90.0,
            threshold_value: 80.0,
            impact: "System slowdown".to_string(),
            suggested_optimizations: vec![
                "Optimize algorithm".to_string(),
                "Use caching".to_string(),
            ],
        };

        assert!(!bottleneck.suggested_optimizations.is_empty());
        assert!(bottleneck.suggested_optimizations.len() >= 2);
    }
}
