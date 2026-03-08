//! SENTINEL Error Handling Module
//!
//! This module provides comprehensive error handling and recovery capabilities
//! including retry logic, circuit breakers, and recovery mechanisms.

use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::sync::RwLock;
use tracing::{info, warn};

/// Error Handler
pub struct ErrorHandler {
    retry_manager: Arc<RwLock<RetryManager>>,
    circuit_breaker: Arc<RwLock<CircuitBreakerManager>>,
    recovery_manager: Arc<RwLock<RecoveryManager>>,
    error_log: Arc<RwLock<ErrorLog>>,
}

/// Retry Manager
pub struct RetryManager {
    retry_policies: HashMap<String, RetryPolicy>,
    retry_stats: Arc<RwLock<RetryStatistics>>,
}

/// Retry Policy
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RetryPolicy {
    pub max_attempts: u32,
    pub initial_delay: Duration,
    pub max_delay: Duration,
    pub backoff_multiplier: f64,
    pub retryable_errors: Vec<String>,
}

/// Retry Statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RetryStatistics {
    pub total_retries: u64,
    pub successful_retries: u64,
    pub failed_retries: u64,
    pub average_retry_count: f64,
}

/// Circuit Breaker Manager
pub struct CircuitBreakerManager {
    breakers: HashMap<String, CircuitBreaker>,
}

/// Circuit Breaker
#[derive(Debug, Clone)]
pub struct CircuitBreaker {
    name: String,
    state: CircuitState,
    failure_count: u32,
    success_count: u32,
    failure_threshold: u32,
    success_threshold: u32,
    timeout: Duration,
    last_failure_time: Option<Instant>,
    last_state_change: Instant,
}

/// Circuit State
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CircuitState {
    Closed,
    Open,
    HalfOpen,
}

/// Recovery Manager
pub struct RecoveryManager {
    recovery_strategies: HashMap<String, RecoveryStrategy>,
    recovery_stats: Arc<RwLock<RecoveryStatistics>>,
}

/// Recovery Strategy
pub struct RecoveryStrategy {
    name: String,
    recovery_fn: Box<dyn Fn() -> RecoveryAction + Send + Sync>,
}

impl std::fmt::Debug for RecoveryStrategy {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("RecoveryStrategy")
            .field("name", &self.name)
            .finish()
    }
}

impl Clone for RecoveryStrategy {
    fn clone(&self) -> Self {
        Self {
            name: self.name.clone(),
            recovery_fn: Box::new(|| RecoveryAction::Ignore),
        }
    }
}

/// Recovery Action
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RecoveryAction {
    Retry,
    Fallback,
    CircuitBreak,
    Ignore,
    Custom(String),
}

/// Recovery Statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecoveryStatistics {
    pub total_recoveries: u64,
    pub successful_recoveries: u64,
    pub failed_recoveries: u64,
    pub recovery_rate: f64,
}

/// Error Log
pub struct ErrorLog {
    entries: Vec<ErrorEntry>,
}

/// Error Entry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ErrorEntry {
    pub entry_id: String,
    pub timestamp: std::time::SystemTime,
    pub error_type: String,
    pub error_message: String,
    pub severity: ErrorSeverity,
    pub context: HashMap<String, String>,
    pub recovered: bool,
}

/// Error Severity
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ErrorSeverity {
    Info,
    Warning,
    Error,
    Critical,
}

impl ErrorHandler {
    /// Create a new error handler
    pub fn new() -> Result<Self> {
        info!("Creating Error Handler...");

        Ok(Self {
            retry_manager: Arc::new(RwLock::new(RetryManager::new())),
            circuit_breaker: Arc::new(RwLock::new(CircuitBreakerManager::new())),
            recovery_manager: Arc::new(RwLock::new(RecoveryManager::new())),
            error_log: Arc::new(RwLock::new(ErrorLog::new())),
        })
    }

    /// Execute with retry
    pub async fn execute_with_retry<F, T, E>(&self, operation_name: &str, operation: F) -> Result<T>
    where
        F: Fn() -> Result<T, E> + Send + Sync,
        E: std::error::Error + Send + Sync + 'static,
    {
        let retry_manager = self.retry_manager.read().await;
        let policy = retry_manager.get_policy(operation_name);

        let mut last_error = None;
        let mut attempt = 0;
        let mut delay = policy.initial_delay;

        while attempt < policy.max_attempts {
            attempt += 1;

            match operation() {
                Ok(result) => {
                    // Log successful retry
                    if attempt > 1 {
                        let mut stats = retry_manager.retry_stats.write().await;
                        stats.successful_retries += 1;
                        stats.update_average_retry_count(attempt);
                    }
                    return Ok(result);
                }
                Err(e) => {
                    last_error = Some(e);

                    if attempt < policy.max_attempts {
                        warn!(
                            "Operation {} failed (attempt {}/{}), retrying in {:?}: {}",
                            operation_name,
                            attempt,
                            policy.max_attempts,
                            delay,
                            last_error.as_ref().unwrap()
                        );

                        tokio::time::sleep(delay).await;
                        delay = Duration::from_millis(
                            (delay.as_millis() as f64 * policy.backoff_multiplier)
                                .min(policy.max_delay.as_millis() as f64)
                                as u64,
                        );
                    }
                }
            }
        }

        // All retries failed
        let mut stats = retry_manager.retry_stats.write().await;
        stats.failed_retries += 1;
        stats.total_retries += 1;

        Err(anyhow!(
            "Operation {} failed after {} attempts: {}",
            operation_name,
            attempt,
            last_error.unwrap()
        ))
    }

    /// Execute with circuit breaker
    pub async fn execute_with_circuit_breaker<F, T, E>(
        &self,
        circuit_name: &str,
        operation: F,
    ) -> Result<T>
    where
        F: Fn() -> Result<T, E> + Send + Sync,
        E: std::error::Error + Send + Sync + 'static,
    {
        let mut circuit_manager = self.circuit_breaker.write().await;

        // Check circuit state
        if !circuit_manager.can_execute(circuit_name) {
            return Err(anyhow!("Circuit breaker {} is open", circuit_name));
        }

        // Execute operation
        match operation() {
            Ok(result) => {
                circuit_manager.record_success(circuit_name);
                Ok(result)
            }
            Err(e) => {
                circuit_manager.record_failure(circuit_name);
                Err(anyhow!("Operation failed: {}", e))
            }
        }
    }

    /// Handle error with recovery
    pub async fn handle_error(
        &self,
        error: &str,
        context: HashMap<String, String>,
    ) -> Result<RecoveryAction> {
        let recovery_manager = self.recovery_manager.read().await;

        // Log error
        let mut log = self.error_log.write().await;
        log.add_entry(ErrorEntry {
            entry_id: uuid::Uuid::new_v4().to_string(),
            timestamp: std::time::SystemTime::now(),
            error_type: "GenericError".to_string(),
            error_message: error.to_string(),
            severity: ErrorSeverity::Error,
            context: context.clone(),
            recovered: false,
        });

        // Determine recovery action
        let action = recovery_manager.determine_recovery(error, context)?;

        // Update recovery statistics
        let mut stats = recovery_manager.recovery_stats.write().await;
        stats.total_recoveries += 1;

        Ok(action)
    }

    /// Add retry policy
    pub async fn add_retry_policy(&self, name: String, policy: RetryPolicy) -> Result<()> {
        let mut retry_manager = self.retry_manager.write().await;
        retry_manager.add_policy(name, policy);
        Ok(())
    }

    /// Add circuit breaker
    pub async fn add_circuit_breaker(
        &self,
        name: String,
        failure_threshold: u32,
        timeout: Duration,
    ) -> Result<()> {
        let mut circuit_manager = self.circuit_breaker.write().await;
        circuit_manager.add_breaker(name, failure_threshold, timeout);
        Ok(())
    }

    /// Get error log entries
    pub async fn get_error_log(&self, limit: Option<usize>) -> Vec<ErrorEntry> {
        let log = self.error_log.read().await;
        log.get_entries(limit)
    }

    /// Get retry statistics
    pub async fn get_retry_stats(&self) -> RetryStatistics {
        let retry_manager = self.retry_manager.read().await;
        retry_manager.get_stats()
    }

    /// Get circuit breaker states
    pub async fn get_circuit_states(&self) -> HashMap<String, CircuitState> {
        let circuit_manager = self.circuit_breaker.read().await;
        circuit_manager.get_states()
    }

    /// Get recovery statistics
    pub async fn get_recovery_stats(&self) -> RecoveryStatistics {
        let recovery_manager = self.recovery_manager.read().await;
        recovery_manager.get_stats()
    }
}

impl Default for RetryManager {
    fn default() -> Self {
        Self::new()
    }
}

impl RetryManager {
    pub fn new() -> Self {
        Self {
            retry_policies: HashMap::new(),
            retry_stats: Arc::new(RwLock::new(RetryStatistics::default())),
        }
    }

    pub fn add_policy(&mut self, name: String, policy: RetryPolicy) {
        self.retry_policies.insert(name, policy);
    }

    pub fn get_policy(&self, name: &str) -> RetryPolicy {
        self.retry_policies
            .get(name)
            .cloned()
            .unwrap_or_else(RetryPolicy::default)
    }

    pub fn get_stats(&self) -> RetryStatistics {
        self.retry_stats.blocking_read().clone()
    }
}

impl Default for RetryPolicy {
    fn default() -> Self {
        Self {
            max_attempts: 3,
            initial_delay: Duration::from_millis(100),
            max_delay: Duration::from_secs(10),
            backoff_multiplier: 2.0,
            retryable_errors: vec!["timeout".to_string(), "connection".to_string()],
        }
    }
}

impl RetryStatistics {
    fn update_average_retry_count(&mut self, count: u32) {
        let total = self.successful_retries + self.failed_retries;
        if total > 0 {
            self.average_retry_count =
                (self.average_retry_count * (total - 1) as f64 + count as f64) / total as f64;
        }
    }
}

impl Default for RetryStatistics {
    fn default() -> Self {
        Self {
            total_retries: 0,
            successful_retries: 0,
            failed_retries: 0,
            average_retry_count: 0.0,
        }
    }
}

impl Default for CircuitBreakerManager {
    fn default() -> Self {
        Self::new()
    }
}

impl CircuitBreakerManager {
    pub fn new() -> Self {
        Self {
            breakers: HashMap::new(),
        }
    }

    pub fn add_breaker(&mut self, name: String, failure_threshold: u32, timeout: Duration) {
        self.breakers.insert(
            name.clone(),
            CircuitBreaker::new(name, failure_threshold, timeout),
        );
    }

    pub fn can_execute(&mut self, name: &str) -> bool {
        let breaker = self
            .breakers
            .entry(name.to_string())
            .or_insert_with(|| CircuitBreaker::new(name.to_string(), 5, Duration::from_secs(60)));

        breaker.can_execute()
    }

    pub fn record_success(&mut self, name: &str) {
        if let Some(breaker) = self.breakers.get_mut(name) {
            breaker.record_success();
        }
    }

    pub fn record_failure(&mut self, name: &str) {
        if let Some(breaker) = self.breakers.get_mut(name) {
            breaker.record_failure();
        }
    }

    pub fn get_states(&self) -> HashMap<String, CircuitState> {
        self.breakers
            .iter()
            .map(|(name, breaker)| (name.clone(), breaker.state))
            .collect()
    }
}

impl CircuitBreaker {
    pub fn new(name: String, failure_threshold: u32, timeout: Duration) -> Self {
        Self {
            name,
            state: CircuitState::Closed,
            failure_count: 0,
            success_count: 0,
            failure_threshold,
            success_threshold: 3,
            timeout,
            last_failure_time: None,
            last_state_change: Instant::now(),
        }
    }

    pub fn can_execute(&mut self) -> bool {
        match self.state {
            CircuitState::Closed => true,
            CircuitState::Open => {
                // Check if timeout has elapsed
                if let Some(last_failure) = self.last_failure_time {
                    if last_failure.elapsed() > self.timeout {
                        self.transition_to_half_open();
                        return true;
                    }
                }
                false
            }
            CircuitState::HalfOpen => true,
        }
    }

    pub fn record_success(&mut self) {
        match self.state {
            CircuitState::Closed => {
                self.failure_count = 0;
            }
            CircuitState::HalfOpen => {
                self.success_count += 1;
                if self.success_count >= self.success_threshold {
                    self.transition_to_closed();
                }
            }
            CircuitState::Open => {}
        }
    }

    pub fn record_failure(&mut self) {
        self.failure_count += 1;
        self.last_failure_time = Some(Instant::now());

        match self.state {
            CircuitState::Closed => {
                if self.failure_count >= self.failure_threshold {
                    self.transition_to_open();
                }
            }
            CircuitState::HalfOpen => {
                self.transition_to_open();
            }
            CircuitState::Open => {}
        }
    }

    fn transition_to_open(&mut self) {
        self.state = CircuitState::Open;
        self.last_state_change = Instant::now();
        warn!("Circuit breaker {} transitioned to OPEN", self.name);
    }

    fn transition_to_closed(&mut self) {
        self.state = CircuitState::Closed;
        self.failure_count = 0;
        self.success_count = 0;
        self.last_state_change = Instant::now();
        info!("Circuit breaker {} transitioned to CLOSED", self.name);
    }

    fn transition_to_half_open(&mut self) {
        self.state = CircuitState::HalfOpen;
        self.success_count = 0;
        self.last_state_change = Instant::now();
        info!("Circuit breaker {} transitioned to HALF_OPEN", self.name);
    }
}

impl Default for RecoveryManager {
    fn default() -> Self {
        Self::new()
    }
}

impl RecoveryManager {
    pub fn new() -> Self {
        Self {
            recovery_strategies: HashMap::new(),
            recovery_stats: Arc::new(RwLock::new(RecoveryStatistics::default())),
        }
    }

    pub fn determine_recovery(
        &self,
        error: &str,
        _context: HashMap<String, String>,
    ) -> Result<RecoveryAction> {
        // Simple recovery logic based on error type
        if error.contains("timeout") || error.contains("connection") {
            Ok(RecoveryAction::Retry)
        } else if error.contains("circuit") {
            Ok(RecoveryAction::CircuitBreak)
        } else if error.contains("critical") {
            Ok(RecoveryAction::Fallback)
        } else {
            Ok(RecoveryAction::Ignore)
        }
    }

    pub fn get_stats(&self) -> RecoveryStatistics {
        self.recovery_stats.blocking_read().clone()
    }
}

impl Default for RecoveryStatistics {
    fn default() -> Self {
        Self {
            total_recoveries: 0,
            successful_recoveries: 0,
            failed_recoveries: 0,
            recovery_rate: 0.0,
        }
    }
}

impl Default for ErrorLog {
    fn default() -> Self {
        Self::new()
    }
}

impl ErrorLog {
    pub fn new() -> Self {
        Self {
            entries: Vec::new(),
        }
    }

    pub fn add_entry(&mut self, entry: ErrorEntry) {
        self.entries.push(entry);
    }

    pub fn get_entries(&self, limit: Option<usize>) -> Vec<ErrorEntry> {
        let entries = self.entries.clone();
        match limit {
            Some(l) => entries.into_iter().rev().take(l).collect(),
            None => entries.into_iter().rev().collect(),
        }
    }
}

/// Initialize error handling module
pub fn init() -> Result<()> {
    info!("Error Handling Module initialized");
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_error_handler_creation() {
        let handler = ErrorHandler::new().unwrap();
        assert!(handler
            .handle_error("test error", HashMap::new())
            .await
            .is_ok());
    }

    #[tokio::test]
    async fn test_retry_logic() {
        use std::sync::atomic::{AtomicUsize, Ordering};
        let handler = ErrorHandler::new().unwrap();
        let attempt_count = std::sync::Arc::new(AtomicUsize::new(0));
        let attempt_count_clone = attempt_count.clone();

        let result = handler
            .execute_with_retry("test_operation", move || {
                let count = attempt_count_clone.fetch_add(1, Ordering::SeqCst) + 1;
                if count < 3 {
                    Err(std::io::Error::new(std::io::ErrorKind::Other, "Temporary error"))
                } else {
                    Ok("success")
                }
            })
            .await;

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "success");
        assert_eq!(attempt_count.load(Ordering::SeqCst), 3);
    }

    #[tokio::test]
    async fn test_circuit_breaker() {
        let handler = ErrorHandler::new().unwrap();
        handler
            .add_circuit_breaker("test_circuit".to_string(), 3, Duration::from_secs(5))
            .await
            .unwrap();

        // Record failures
        for _ in 0..3 {
            let result = handler
                .execute_with_circuit_breaker("test_circuit", || {
                    Err::<(), _>(std::io::Error::new(std::io::ErrorKind::Other, "Error"))
                })
                .await;
            assert!(result.is_err());
        }

        // Circuit should be open now
        let result = handler
            .execute_with_circuit_breaker("test_circuit", || Ok::<(), std::io::Error>(()))
            .await;
        assert!(result.is_err());

        let states = handler.get_circuit_states().await;
        assert_eq!(states.get("test_circuit"), Some(&CircuitState::Open));
    }

    #[tokio::test]
    async fn test_error_logging() {
        let handler = ErrorHandler::new().unwrap();
        handler
            .handle_error("test error", HashMap::new())
            .await
            .unwrap();

        let entries = handler.get_error_log(Some(10)).await;
        assert!(!entries.is_empty());
        assert_eq!(entries[0].error_message, "test error");
    }

    #[tokio::test]
    async fn test_retry_statistics() {
        use std::sync::atomic::{AtomicUsize, Ordering};
        let handler = ErrorHandler::new().unwrap();
        let attempt_count = std::sync::Arc::new(AtomicUsize::new(0));
        let attempt_count_clone = attempt_count.clone();

        handler
            .execute_with_retry("test_operation", move || {
                let count = attempt_count_clone.fetch_add(1, Ordering::SeqCst) + 1;
                if count < 2 {
                    Err(std::io::Error::new(std::io::ErrorKind::Other, "Temporary error"))
                } else {
                    Ok("success")
                }
            })
            .await
            .unwrap();

        let stats = handler.get_retry_stats().await;
        assert_eq!(stats.successful_retries, 1);
    }
}
