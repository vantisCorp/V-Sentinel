// Performance Regression Testing Framework for SENTINEL Security System
// Detects performance degradation over time by comparing against baselines

use std::collections::HashMap;
use std::fs;
use std::path::Path;
use std::time::{Duration, Instant};
use serde::{Deserialize, Serialize};

/// Performance baseline data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceBaseline {
    pub component: String,
    pub operation: String,
    pub baseline_mean_ns: f64,
    pub baseline_stddev_ns: f64,
    pub baseline_min_ns: f64,
    pub baseline_max_ns: f64,
    pub baseline_p50_ns: f64,
    pub baseline_p95_ns: f64,
    pub baseline_p99_ns: f64,
    pub sample_count: usize,
    pub timestamp: String,
}

/// Performance measurement for regression testing
#[derive(Debug, Clone)]
pub struct RegressionMeasurement {
    pub component: String,
    pub operation: String,
    pub measurements: Vec<Duration>,
    pub mean_ns: f64,
    pub stddev_ns: f64,
    pub min_ns: f64,
    pub max_ns: f64,
    pub p50_ns: f64,
    pub p95_ns: f64,
    pub p99_ns: f64,
}

/// Regression test result
#[derive(Debug, Clone)]
pub struct RegressionTestResult {
    pub component: String,
    pub operation: String,
    pub baseline: PerformanceBaseline,
    pub current: RegressionMeasurement,
    pub mean_change_percent: f64,
    pub p95_change_percent: f64,
    pub p99_change_percent: f64,
    pub regression_detected: bool,
    pub improvement_detected: bool,
    pub significance: RegressionSignificance,
}

/// Significance level of regression
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum RegressionSignificance {
    None,
    Minor,      // 5-10% degradation
    Moderate,   // 10-25% degradation
    Severe,     // 25-50% degradation
    Critical,   // >50% degradation
}

/// Regression testing configuration
#[derive(Debug, Clone)]
pub struct RegressionTestConfig {
    pub baseline_file: String,
    pub regression_threshold_percent: f64,
    pub improvement_threshold_percent: f64,
    pub sample_count: usize,
    pub warmup_iterations: usize,
}

impl Default for RegressionTestConfig {
    fn default() -> Self {
        Self {
            baseline_file: "baselines/performance_baselines.json".to_string(),
            regression_threshold_percent: 10.0,
            improvement_threshold_percent: 5.0,
            sample_count: 100,
            warmup_iterations: 10,
        }
    }
}

/// Performance regression tester
pub struct PerformanceRegressionTester {
    config: RegressionTestConfig,
    baselines: HashMap<String, PerformanceBaseline>,
}

impl PerformanceRegressionTester {
    pub fn new(config: RegressionTestConfig) -> Self {
        let mut tester = Self {
            config,
            baselines: HashMap::new(),
        };
        tester.load_baselines();
        tester
    }

    pub fn load_baselines(&mut self) {
        let path = Path::new(&self.config.baseline_file);
        if path.exists() {
            if let Ok(content) = fs::read_to_string(path) {
                if let Ok(baseline_list) = serde_json::from_str::<Vec<PerformanceBaseline>>(&content) {
                    for baseline in baseline_list {
                        let key = format!("{}:{}", baseline.component, baseline.operation);
                        self.baselines.insert(key, baseline);
                    }
                }
            }
        }
    }

    pub fn save_baselines(&self, measurements: &[RegressionMeasurement]) {
        let baselines: Vec<PerformanceBaseline> = measurements
            .iter()
            .map(|m| PerformanceBaseline {
                component: m.component.clone(),
                operation: m.operation.clone(),
                baseline_mean_ns: m.mean_ns,
                baseline_stddev_ns: m.stddev_ns,
                baseline_min_ns: m.min_ns,
                baseline_max_ns: m.max_ns,
                baseline_p50_ns: m.p50_ns,
                baseline_p95_ns: m.p95_ns,
                baseline_p99_ns: m.p99_ns,
                sample_count: m.measurements.len(),
                timestamp: chrono::Utc::now().to_rfc3339(),
            })
            .collect();

        if let Ok(json) = serde_json::to_string_pretty(&baselines) {
            let path = Path::new(&self.config.baseline_file);
            if let Some(parent) = path.parent() {
                let _ = fs::create_dir_all(parent);
            }
            let _ = fs::write(path, json);
        }
    }

    pub fn measure_operation<F>(&self, component: &str, operation: &str, mut f: F) -> RegressionMeasurement
    where
        F: FnMut() -> Duration,
    {
        // Warmup iterations
        for _ in 0..self.config.warmup_iterations {
            let _ = f();
        }

        // Actual measurements
        let mut measurements = Vec::with_capacity(self.config.sample_count);
        for _ in 0..self.config.sample_count {
            measurements.push(f());
        }

        // Calculate statistics
        let mut sorted = measurements.clone();
        sorted.sort();

        let mean_ns = measurements.iter().map(|d| d.as_nanos() as f64).sum::<f64>() / measurements.len() as f64;
        let variance = measurements.iter()
            .map(|d| (d.as_nanos() as f64 - mean_ns).powi(2))
            .sum::<f64>() / measurements.len() as f64;
        let stddev_ns = variance.sqrt();

        RegressionMeasurement {
            component: component.to_string(),
            operation: operation.to_string(),
            measurements,
            mean_ns,
            stddev_ns,
            min_ns: sorted.first().map(|d| d.as_nanos() as f64).unwrap_or(0.0),
            max_ns: sorted.last().map(|d| d.as_nanos() as f64).unwrap_or(0.0),
            p50_ns: sorted[sorted.len() / 2].as_nanos() as f64,
            p95_ns: sorted[(sorted.len() as f64 * 0.95) as usize].as_nanos() as f64,
            p99_ns: sorted[(sorted.len() as f64 * 0.99) as usize].as_nanos() as f64,
        }
    }

    pub fn test_regression(&self, measurement: &RegressionMeasurement) -> Option<RegressionTestResult> {
        let key = format!("{}:{}", measurement.component, measurement.operation);
        
        if let Some(baseline) = self.baselines.get(&key) {
            let mean_change = ((measurement.mean_ns - baseline.baseline_mean_ns) / baseline.baseline_mean_ns) * 100.0;
            let p95_change = ((measurement.p95_ns - baseline.baseline_p95_ns) / baseline.baseline_p95_ns) * 100.0;
            let p99_change = ((measurement.p99_ns - baseline.baseline_p99_ns) / baseline.baseline_p99_ns) * 100.0;

            let regression_detected = mean_change > self.config.regression_threshold_percent;
            let improvement_detected = mean_change < -self.config.improvement_threshold_percent;

            let significance = if regression_detected {
                if mean_change > 50.0 {
                    RegressionSignificance::Critical
                } else if mean_change > 25.0 {
                    RegressionSignificance::Severe
                } else if mean_change > 10.0 {
                    RegressionSignificance::Moderate
                } else {
                    RegressionSignificance::Minor
                }
            } else {
                RegressionSignificance::None
            };

            Some(RegressionTestResult {
                component: measurement.component.clone(),
                operation: measurement.operation.clone(),
                baseline: baseline.clone(),
                current: measurement.clone(),
                mean_change_percent: mean_change,
                p95_change_percent: p95_change,
                p99_change_percent: p99_change,
                regression_detected,
                improvement_detected,
                significance,
            })
        } else {
            None
        }
    }

    pub fn run_all_regression_tests(&self) -> Vec<RegressionTestResult> {
        let mut results = Vec::new();

        // Test core system operations
        results.extend(self.test_core_system());
        
        // Test AI engine operations
        results.extend(self.test_ai_engine());
        
        // Test quantum crypto operations
        results.extend(self.test_quantum_crypto());
        
        // Test gaming features
        results.extend(self.test_gaming_features());

        results
    }

    fn test_core_system(&self) -> Vec<RegressionTestResult> {
        let mut results = Vec::new();

        // Hypervisor initialization
        let measurement = self.measure_operation("Core System", "Hypervisor Initialization", || {
            let start = Instant::now();
            self.simulate_hypervisor_init();
            start.elapsed()
        });
        if let Some(result) = self.test_regression(&measurement) {
            results.push(result);
        }

        // VM creation
        let measurement = self.measure_operation("Core System", "VM Creation", || {
            let start = Instant::now();
            self.simulate_vm_creation();
            start.elapsed()
        });
        if let Some(result) = self.test_regression(&measurement) {
            results.push(result);
        }

        // Memory protection
        let measurement = self.measure_operation("Core System", "Memory Protection", || {
            let start = Instant::now();
            self.simulate_memory_protection();
            start.elapsed()
        });
        if let Some(result) = self.test_regression(&measurement) {
            results.push(result);
        }

        results
    }

    fn test_ai_engine(&self) -> Vec<RegressionTestResult> {
        let mut results = Vec::new();

        // Single prediction
        let measurement = self.measure_operation("AI Engine", "Single Prediction", || {
            let start = Instant::now();
            self.simulate_single_prediction();
            start.elapsed()
        });
        if let Some(result) = self.test_regression(&measurement) {
            results.push(result);
        }

        // Feature extraction
        let measurement = self.measure_operation("AI Engine", "Feature Extraction", || {
            let start = Instant::now();
            self.simulate_feature_extraction();
            start.elapsed()
        });
        if let Some(result) = self.test_regression(&measurement) {
            results.push(result);
        }

        results
    }

    fn test_quantum_crypto(&self) -> Vec<RegressionTestResult> {
        let mut results = Vec::new();

        // Kyber keypair generation
        let measurement = self.measure_operation("Quantum Crypto", "Kyber Keypair Generation", || {
            let start = Instant::now();
            self.simulate_keypair_generation("kyber");
            start.elapsed()
        });
        if let Some(result) = self.test_regression(&measurement) {
            results.push(result);
        }

        // Encapsulation
        let measurement = self.measure_operation("Quantum Crypto", "Encapsulation", || {
            let start = Instant::now();
            self.simulate_encapsulation();
            start.elapsed()
        });
        if let Some(result) = self.test_regression(&measurement) {
            results.push(result);
        }

        // Signing
        let measurement = self.measure_operation("Quantum Crypto", "Signing", || {
            let start = Instant::now();
            self.simulate_signing();
            start.elapsed()
        });
        if let Some(result) = self.test_regression(&measurement) {
            results.push(result);
        }

        results
    }

    fn test_gaming_features(&self) -> Vec<RegressionTestResult> {
        let mut results = Vec::new();

        // Game detection
        let measurement = self.measure_operation("Gaming Features", "Game Detection", || {
            let start = Instant::now();
            self.simulate_game_detection();
            start.elapsed()
        });
        if let Some(result) = self.test_regression(&measurement) {
            results.push(result);
        }

        // Trusted handshake
        let measurement = self.measure_operation("Gaming Features", "Trusted Handshake", || {
            let start = Instant::now();
            self.simulate_trusted_handshake();
            start.elapsed()
        });
        if let Some(result) = self.test_regression(&measurement) {
            results.push(result);
        }

        // Attack detection
        let measurement = self.measure_operation("Gaming Features", "Attack Detection", || {
            let start = Instant::now();
            self.simulate_attack_detection();
            start.elapsed()
        });
        if let Some(result) = self.test_regression(&measurement) {
            results.push(result);
        }

        results
    }

    pub fn generate_report(&self, results: &[RegressionTestResult]) -> String {
        let mut report = String::new();
        report.push_str("# SENTINEL Performance Regression Test Report\n\n");
        
        let mut regressions = Vec::new();
        let mut improvements = Vec::new();
        let mut stable = Vec::new();

        for result in results {
            if result.regression_detected {
                regressions.push(result);
            } else if result.improvement_detected {
                improvements.push(result);
            } else {
                stable.push(result);
            }
        }

        // Regressions section
        if !regressions.is_empty() {
            report.push_str("## ⚠️ Performance Regressions Detected\n\n");
            for result in &regressions {
                let significance_str = match result.significance {
                    RegressionSignificance::Minor => "Minor",
                    RegressionSignificance::Moderate => "Moderate",
                    RegressionSignificance::Severe => "Severe",
                    RegressionSignificance::Critical => "CRITICAL",
                    RegressionSignificance::None => "None",
                };
                
                report.push_str(&format!(
                    "### {}. {} - {}\n",
                    result.component, result.operation, significance_str
                ));
                report.push_str(&format!(
                    "- Mean: {:.2}ms → {:.2}ms ({:+.1}%)\n",
                    result.baseline.baseline_mean_ns / 1_000_000.0,
                    result.current.mean_ns / 1_000_000.0,
                    result.mean_change_percent
                ));
                report.push_str(&format!(
                    "- P95: {:.2}ms → {:.2}ms ({:+.1}%)\n",
                    result.baseline.baseline_p95_ns / 1_000_000.0,
                    result.current.p95_ns / 1_000_000.0,
                    result.p95_change_percent
                ));
                report.push_str(&format!(
                    "- P99: {:.2}ms → {:.2}ms ({:+.1}%)\n\n",
                    result.baseline.baseline_p99_ns / 1_000_000.0,
                    result.current.p99_ns / 1_000_000.0,
                    result.p99_change_percent
                ));
            }
        }

        // Improvements section
        if !improvements.is_empty() {
            report.push_str("## ✅ Performance Improvements\n\n");
            for result in &improvements {
                report.push_str(&format!(
                    "### {}. {}\n",
                    result.component, result.operation
                ));
                report.push_str(&format!(
                    "- Mean: {:.2}ms → {:.2}ms ({:+.1}%)\n\n",
                    result.baseline.baseline_mean_ns / 1_000_000.0,
                    result.current.mean_ns / 1_000_000.0,
                    result.mean_change_percent
                ));
            }
        }

        // Stable section
        if !stable.is_empty() {
            report.push_str(&format!("## ✅ Stable Performance ({} operations)\n\n", stable.len()));
            for result in &stable {
                report.push_str(&format!(
                    "- {}. {}: {:.1}% change\n",
                    result.component,
                    result.operation,
                    result.mean_change_percent
                ));
            }
            report.push_str("\n");
        }

        // Summary
        report.push_str("## Summary\n\n");
        report.push_str(&format!("- Total Tests: {}\n", results.len()));
        report.push_str(&format!("- Regressions: {}\n", regressions.len()));
        report.push_str(&format!("- Improvements: {}\n", improvements.len()));
        report.push_str(&format!("- Stable: {}\n", stable.len()));

        report
    }

    // Simulation functions
    fn simulate_hypervisor_init(&self) {
        std::thread::sleep(Duration::from_millis(50));
    }

    fn simulate_vm_creation(&self) {
        std::thread::sleep(Duration::from_millis(25));
    }

    fn simulate_memory_protection(&self) {
        std::thread::sleep(Duration::from_nanos(500));
    }

    fn simulate_single_prediction(&self) {
        std::thread::sleep(Duration::from_millis(5));
    }

    fn simulate_feature_extraction(&self) {
        std::thread::sleep(Duration::from_millis(2));
    }

    fn simulate_keypair_generation(&self, algorithm: &str) {
        let time = match algorithm {
            "kyber" => 50,
            "dilithium" => 25,
            "sphincs" => 100,
            _ => 50,
        };
        std::thread::sleep(Duration::from_millis(time));
    }

    fn simulate_encapsulation(&self) {
        std::thread::sleep(Duration::from_millis(25));
    }

    fn simulate_signing(&self) {
        std::thread::sleep(Duration::from_millis(25));
    }

    fn simulate_game_detection(&self) {
        std::thread::sleep(Duration::from_millis(5));
    }

    fn simulate_trusted_handshake(&self) {
        std::thread::sleep(Duration::from_millis(50));
    }

    fn simulate_attack_detection(&self) {
        std::thread::sleep(Duration::from_millis(5));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_regression_detection() {
        let config = RegressionTestConfig {
            baseline_file: "test_baselines.json".to_string(),
            regression_threshold_percent: 10.0,
            improvement_threshold_percent: 5.0,
            sample_count: 50,
            warmup_iterations: 5,
        };

        let tester = PerformanceRegressionTester::new(config);
        
        // Run regression tests
        let results = tester.run_all_regression_tests();
        
        // Generate report
        let report = tester.generate_report(&results);
        println!("{}", report);
        
        // Verify results
        assert!(results.len() > 0);
    }

    #[test]
    fn test_baseline_creation() {
        let config = RegressionTestConfig {
            baseline_file: "new_baselines.json".to_string(),
            regression_threshold_percent: 10.0,
            improvement_threshold_percent: 5.0,
            sample_count: 50,
            warmup_iterations: 5,
        };

        let tester = PerformanceRegressionTester::new(config);
        
        // Create new baselines
        let measurements = vec![
            tester.measure_operation("Test Component", "Test Operation", || {
                Duration::from_millis(10)
            }),
        ];
        
        tester.save_baselines(&measurements);
        
        // Verify baselines were saved
        let path = Path::new(&tester.config.baseline_file);
        assert!(path.exists());
        
        // Cleanup
        let _ = fs::remove_file(path);
    }
}