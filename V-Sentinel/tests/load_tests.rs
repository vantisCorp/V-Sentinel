// Load Testing Framework for SENTINEL Security System
// Tests system performance under realistic load conditions

use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::sync::Semaphore;
use tokio::task::JoinSet;
use std::collections::HashMap;

/// Load test configuration
#[derive(Debug, Clone)]
pub struct LoadTestConfig {
    pub concurrent_users: usize,
    pub requests_per_user: usize,
    pub ramp_up_duration: Duration,
    pub test_duration: Duration,
    pub think_time: Duration,
}

impl Default for LoadTestConfig {
    fn default() -> Self {
        Self {
            concurrent_users: 100,
            requests_per_user: 10,
            ramp_up_duration: Duration::from_secs(30),
            test_duration: Duration::from_secs(300),
            think_time: Duration::from_millis(100),
        }
    }
}

/// Load test scenario types
#[derive(Debug, Clone, Copy)]
pub enum LoadTestScenario {
    /// Normal daily usage
    Normal,
    /// Peak usage (e.g., during a security event)
    Peak,
    /// Stress test (extreme load)
    Stress,
    /// Sustained load (long duration)
    Sustained,
}

impl LoadTestScenario {
    pub fn config(&self) -> LoadTestConfig {
        match self {
            LoadTestScenario::Normal => LoadTestConfig {
                concurrent_users: 100,
                requests_per_user: 10,
                ramp_up_duration: Duration::from_secs(30),
                test_duration: Duration::from_secs(300),
                think_time: Duration::from_millis(100),
            },
            LoadTestScenario::Peak => LoadTestConfig {
                concurrent_users: 1000,
                requests_per_user: 20,
                ramp_up_duration: Duration::from_secs(60),
                test_duration: Duration::from_secs(600),
                think_time: Duration::from_millis(50),
            },
            LoadTestScenario::Stress => LoadTestConfig {
                concurrent_users: 10000,
                requests_per_user: 100,
                ramp_up_duration: Duration::from_secs(120),
                test_duration: Duration::from_secs(1800),
                think_time: Duration::from_millis(10),
            },
            LoadTestScenario::Sustained => LoadTestConfig {
                concurrent_users: 500,
                requests_per_user: 50,
                ramp_up_duration: Duration::from_secs(60),
                test_duration: Duration::from_secs(3600),
                think_time: Duration::from_millis(200),
            },
        }
    }
}

/// Load test result
#[derive(Debug, Clone)]
pub struct LoadTestResult {
    pub scenario: LoadTestScenario,
    pub total_requests: usize,
    pub successful_requests: usize,
    pub failed_requests: usize,
    pub total_duration: Duration,
    pub avg_response_time: Duration,
    pub min_response_time: Duration,
    pub max_response_time: Duration,
    pub p50_response_time: Duration,
    pub p95_response_time: Duration,
    pub p99_response_time: Duration,
    pub requests_per_second: f64,
    pub errors: HashMap<String, usize>,
}

/// Load test executor
pub struct LoadTestExecutor {
    config: LoadTestConfig,
    scenario: LoadTestScenario,
}

impl LoadTestExecutor {
    pub fn new(scenario: LoadTestScenario) -> Self {
        Self {
            config: scenario.config(),
            scenario,
        }
    }

    pub async fn run_threat_detection_load_test(&self) -> LoadTestResult {
        let start = Instant::now();
        let mut response_times = Vec::new();
        let mut successful = 0;
        let mut failed = 0;
        let mut errors = HashMap::new();

        let semaphore = Arc::new(Semaphore::new(self.config.concurrent_users));
        let mut join_set = JoinSet::new();

        for user_id in 0..self.config.concurrent_users {
            let semaphore = semaphore.clone();
            let requests = self.config.requests_per_user;
            let think_time = self.config.think_time;

            join_set.spawn(async move {
                let _permit = semaphore.acquire().await.unwrap();
                let mut user_response_times = Vec::new();
                let mut user_successful = 0;
                let mut user_failed = 0;

                for req_id in 0..requests {
                    let req_start = Instant::now();
                    
                    // Simulate threat detection request
                    match simulate_threat_detection_request(user_id, req_id).await {
                        Ok(_) => {
                            user_successful += 1;
                            user_response_times.push(req_start.elapsed());
                        }
                        Err(e) => {
                            user_failed += 1;
                        }
                    }

                    // Think time between requests
                    tokio::time::sleep(think_time).await;
                }

                (user_response_times, user_successful, user_failed)
            });
        }

        while let Some(result) = join_set.join_next().await {
            match result {
                Ok((times, succ, fail)) => {
                    response_times.extend(times);
                    successful += succ;
                    failed += fail;
                }
                Err(_) => {
                    failed += self.config.requests_per_user;
                }
            }
        }

        let total_duration = start.elapsed();
        let total_requests = successful + failed;

        // Calculate percentiles
        response_times.sort();
        let p50 = response_times[response_times.len() / 2];
        let p95 = response_times[(response_times.len() as f64 * 0.95) as usize];
        let p99 = response_times[(response_times.len() as f64 * 0.99) as usize];

        LoadTestResult {
            scenario: self.scenario,
            total_requests,
            successful_requests: successful,
            failed_requests: failed,
            total_duration,
            avg_response_time: Duration::from_nanos(
                (response_times.iter().map(|d| d.as_nanos()).sum::<u128>() as f64 / response_times.len() as f64) as u64
            ),
            min_response_time: *response_times.first().unwrap_or(&Duration::ZERO),
            max_response_time: *response_times.last().unwrap_or(&Duration::ZERO),
            p50_response_time: p50,
            p95_response_time: p95,
            p99_response_time: p99,
            requests_per_second: total_requests as f64 / total_duration.as_secs_f64(),
            errors,
        }
    }

    pub async fn run_ai_prediction_load_test(&self) -> LoadTestResult {
        let start = Instant::now();
        let mut response_times = Vec::new();
        let mut successful = 0;
        let mut failed = 0;
        let mut errors = HashMap::new();

        let semaphore = Arc::new(Semaphore::new(self.config.concurrent_users));
        let mut join_set = JoinSet::new();

        for user_id in 0..self.config.concurrent_users {
            let semaphore = semaphore.clone();
            let requests = self.config.requests_per_user;
            let think_time = self.config.think_time;

            join_set.spawn(async move {
                let _permit = semaphore.acquire().await.unwrap();
                let mut user_response_times = Vec::new();
                let mut user_successful = 0;
                let mut user_failed = 0;

                for req_id in 0..requests {
                    let req_start = Instant::now();
                    
                    // Simulate AI prediction request
                    match simulate_ai_prediction_request(user_id, req_id).await {
                        Ok(_) => {
                            user_successful += 1;
                            user_response_times.push(req_start.elapsed());
                        }
                        Err(e) => {
                            user_failed += 1;
                        }
                    }

                    tokio::time::sleep(think_time).await;
                }

                (user_response_times, user_successful, user_failed)
            });
        }

        while let Some(result) = join_set.join_next().await {
            match result {
                Ok((times, succ, fail)) => {
                    response_times.extend(times);
                    successful += succ;
                    failed += fail;
                }
                Err(_) => {
                    failed += self.config.requests_per_user;
                }
            }
        }

        let total_duration = start.elapsed();
        let total_requests = successful + failed;

        response_times.sort();
        let p50 = response_times[response_times.len() / 2];
        let p95 = response_times[(response_times.len() as f64 * 0.95) as usize];
        let p99 = response_times[(response_times.len() as f64 * 0.99) as usize];

        LoadTestResult {
            scenario: self.scenario,
            total_requests,
            successful_requests: successful,
            failed_requests: failed,
            total_duration,
            avg_response_time: Duration::from_nanos(
                (response_times.iter().map(|d| d.as_nanos()).sum::<u128>() as f64 / response_times.len() as f64) as u64
            ),
            min_response_time: *response_times.first().unwrap_or(&Duration::ZERO),
            max_response_time: *response_times.last().unwrap_or(&Duration::ZERO),
            p50_response_time: p50,
            p95_response_time: p95,
            p99_response_time: p99,
            requests_per_second: total_requests as f64 / total_duration.as_secs_f64(),
            errors,
        }
    }

    pub async fn run_quantum_crypto_load_test(&self) -> LoadTestResult {
        let start = Instant::now();
        let mut response_times = Vec::new();
        let mut successful = 0;
        let mut failed = 0;
        let mut errors = HashMap::new();

        let semaphore = Arc::new(Semaphore::new(self.config.concurrent_users));
        let mut join_set = JoinSet::new();

        for user_id in 0..self.config.concurrent_users {
            let semaphore = semaphore.clone();
            let requests = self.config.requests_per_user;
            let think_time = self.config.think_time;

            join_set.spawn(async move {
                let _permit = semaphore.acquire().await.unwrap();
                let mut user_response_times = Vec::new();
                let mut user_successful = 0;
                let mut user_failed = 0;

                for req_id in 0..requests {
                    let req_start = Instant::now();
                    
                    // Simulate quantum crypto request
                    match simulate_quantum_crypto_request(user_id, req_id).await {
                        Ok(_) => {
                            user_successful += 1;
                            user_response_times.push(req_start.elapsed());
                        }
                        Err(e) => {
                            user_failed += 1;
                        }
                    }

                    tokio::time::sleep(think_time).await;
                }

                (user_response_times, user_successful, user_failed)
            });
        }

        while let Some(result) = join_set.join_next().await {
            match result {
                Ok((times, succ, fail)) => {
                    response_times.extend(times);
                    successful += succ;
                    failed += fail;
                }
                Err(_) => {
                    failed += self.config.requests_per_user;
                }
            }
        }

        let total_duration = start.elapsed();
        let total_requests = successful + failed;

        response_times.sort();
        let p50 = response_times[response_times.len() / 2];
        let p95 = response_times[(response_times.len() as f64 * 0.95) as usize];
        let p99 = response_times[(response_times.len() as f64 * 0.99) as usize];

        LoadTestResult {
            scenario: self.scenario,
            total_requests,
            successful_requests: successful,
            failed_requests: failed,
            total_duration,
            avg_response_time: Duration::from_nanos(
                (response_times.iter().map(|d| d.as_nanos()).sum::<u128>() as f64 / response_times.len() as f64) as u64
            ),
            min_response_time: *response_times.first().unwrap_or(&Duration::ZERO),
            max_response_time: *response_times.last().unwrap_or(&Duration::ZERO),
            p50_response_time: p50,
            p95_response_time: p95,
            p99_response_time: p99,
            requests_per_second: total_requests as f64 / total_duration.as_secs_f64(),
            errors,
        }
    }

    pub async fn run_gaming_load_test(&self) -> LoadTestResult {
        let start = Instant::now();
        let mut response_times = Vec::new();
        let mut successful = 0;
        let mut failed = 0;
        let mut errors = HashMap::new();

        let semaphore = Arc::new(Semaphore::new(self.config.concurrent_users));
        let mut join_set = JoinSet::new();

        for user_id in 0..self.config.concurrent_users {
            let semaphore = semaphore.clone();
            let requests = self.config.requests_per_user;
            let think_time = self.config.think_time;

            join_set.spawn(async move {
                let _permit = semaphore.acquire().await.unwrap();
                let mut user_response_times = Vec::new();
                let mut user_successful = 0;
                let mut user_failed = 0;

                for req_id in 0..requests {
                    let req_start = Instant::now();
                    
                    // Simulate gaming request
                    match simulate_gaming_request(user_id, req_id).await {
                        Ok(_) => {
                            user_successful += 1;
                            user_response_times.push(req_start.elapsed());
                        }
                        Err(e) => {
                            user_failed += 1;
                        }
                    }

                    tokio::time::sleep(think_time).await;
                }

                (user_response_times, user_successful, user_failed)
            });
        }

        while let Some(result) = join_set.join_next().await {
            match result {
                Ok((times, succ, fail)) => {
                    response_times.extend(times);
                    successful += succ;
                    failed += fail;
                }
                Err(_) => {
                    failed += self.config.requests_per_user;
                }
            }
        }

        let total_duration = start.elapsed();
        let total_requests = successful + failed;

        response_times.sort();
        let p50 = response_times[response_times.len() / 2];
        let p95 = response_times[(response_times.len() as f64 * 0.95) as usize];
        let p99 = response_times[(response_times.len() as f64 * 0.99) as usize];

        LoadTestResult {
            scenario: self.scenario,
            total_requests,
            successful_requests: successful,
            failed_requests: failed,
            total_duration,
            avg_response_time: Duration::from_nanos(
                (response_times.iter().map(|d| d.as_nanos()).sum::<u128>() as f64 / response_times.len() as f64) as u64
            ),
            min_response_time: *response_times.first().unwrap_or(&Duration::ZERO),
            max_response_time: *response_times.last().unwrap_or(&Duration::ZERO),
            p50_response_time: p50,
            p95_response_time: p95,
            p99_response_time: p99,
            requests_per_second: total_requests as f64 / total_duration.as_secs_f64(),
            errors,
        }
    }
}

// Simulation functions

async fn simulate_threat_detection_request(user_id: usize, req_id: usize) -> Result<(), String> {
    // Simulate threat detection processing
    let processing_time = Duration::from_micros(100 + (req_id % 10) * 10);
    tokio::time::sleep(processing_time).await;
    Ok(())
}

async fn simulate_ai_prediction_request(user_id: usize, req_id: usize) -> Result<(), String> {
    // Simulate AI prediction processing
    let processing_time = Duration::from_micros(50 + (req_id % 5) * 10);
    tokio::time::sleep(processing_time).await;
    Ok(())
}

async fn simulate_quantum_crypto_request(user_id: usize, req_id: usize) -> Result<(), String> {
    // Simulate quantum crypto processing
    let processing_time = Duration::from_micros(200 + (req_id % 20) * 10);
    tokio::time::sleep(processing_time).await;
    Ok(())
}

async fn simulate_gaming_request(user_id: usize, req_id: usize) -> Result<(), String> {
    // Simulate gaming processing
    let processing_time = Duration::from_micros(30 + (req_id % 3) * 10);
    tokio::time::sleep(processing_time).await;
    Ok(())
}

#[cfg(test)]
mod load_tests {
    use super::*;
    use tokio::test;

    #[tokio::test]
    async fn test_normal_load_threat_detection() {
        let executor = LoadTestExecutor::new(LoadTestScenario::Normal);
        let result = executor.run_threat_detection_load_test().await;
        
        assert!(result.successful_requests > 0);
        assert!(result.requests_per_second > 0.0);
        assert!(result.avg_response_time < Duration::from_millis(100));
    }

    #[tokio::test]
    async fn test_peak_load_ai_prediction() {
        let executor = LoadTestExecutor::new(LoadTestScenario::Peak);
        let result = executor.run_ai_prediction_load_test().await;
        
        assert!(result.successful_requests > 0);
        assert!(result.requests_per_second > 0.0);
        assert!(result.avg_response_time < Duration::from_millis(50));
    }

    #[tokio::test]
    async fn test_sustained_load_quantum_crypto() {
        let executor = LoadTestExecutor::new(LoadTestScenario::Sustained);
        let result = executor.run_quantum_crypto_load_test().await;
        
        assert!(result.successful_requests > 0);
        assert!(result.requests_per_second > 0.0);
        assert!(result.avg_response_time < Duration::from_millis(200));
    }

    #[tokio::test]
    async fn test_gaming_load() {
        let executor = LoadTestExecutor::new(LoadTestScenario::Normal);
        let result = executor.run_gaming_load_test().await;
        
        assert!(result.successful_requests > 0);
        assert!(result.requests_per_second > 0.0);
        assert!(result.avg_response_time < Duration::from_millis(30));
    }
}