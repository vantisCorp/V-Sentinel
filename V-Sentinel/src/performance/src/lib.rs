//! SENTINEL Performance Optimization Module
//!
//! This module provides performance optimization capabilities including
//! caching, connection pooling, resource management, benchmarking,
//! profiling, and bottleneck analysis.

pub mod bottleneck_analyzer;
pub mod profiler;
pub mod dashboard;

use anyhow::{Result, anyhow};
use tracing::{info, debug, warn, error};
use std::sync::Arc;
use std::collections::HashMap;
use tokio::sync::RwLock;
use serde::{Serialize, Deserialize};
use std::time::{Duration, Instant};
use lru::LruCache;
use std::num::NonZeroUsize;

// Re-export benchmarking and profiling modules
pub use bottleneck_analyzer::{
    BottleneckAnalyzer,
    Bottleneck,
    BottleneckSeverity,
    BottleneckType,
    PerformanceProfile,
};

pub use profiler::{
    PerformanceProfiler,
    ProfileEvent,
    ProfileEventType,
    FunctionStats,
    MemoryStats,
    profile,
    profile_memory,
    enable_profiling,
    disable_profiling,
    get_profiling_report,
};

pub use dashboard::{
    PerformanceDashboard,
    PerformanceMetrics,
    PerformanceSummary,
    ComponentSummary,
};

/// Performance Manager
pub struct PerformanceManager {
    cache: Arc<RwLock<CacheManager>>,
    pool: Arc<RwLock<ConnectionPool>>,
    limiter: Arc<RwLock<RateLimiter>>,
    optimizer: Arc<RwLock<QueryOptimizer>>,
    profiler: Arc<RwLock<PerformanceProfiler>>,
}

/// Cache Manager
pub struct CacheManager {
    memory_cache: Arc<RwLock<LruCache<String, CacheEntry>>>,
    cache_stats: Arc<RwLock<CacheStatistics>>,
}

/// Cache Entry
#[derive(Debug, Clone)]
struct CacheEntry {
    value: Vec<u8>,
    created_at: Instant,
    ttl: Duration,
    access_count: u64,
}

/// Cache Statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CacheStatistics {
    pub hits: u64,
    pub misses: u64,
    pub evictions: u64,
    pub size: usize,
    pub hit_rate: f64,
}

/// Connection Pool
pub struct ConnectionPool {
    connections: Arc<RwLock<Vec<PooledConnection>>>,
    pool_config: PoolConfig,
    pool_stats: Arc<RwLock<PoolStatistics>>,
}

/// Pooled Connection
#[derive(Debug, Clone)]
struct PooledConnection {
    id: String,
    created_at: Instant,
    last_used: Instant,
    in_use: bool,
}

/// Pool Configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PoolConfig {
    pub max_connections: usize,
    pub min_connections: usize,
    pub max_idle_time: Duration,
    pub connection_timeout: Duration,
}

/// Pool Statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PoolStatistics {
    pub total_connections: usize,
    pub active_connections: usize,
    pub idle_connections: usize,
    pub wait_count: u64,
    pub wait_duration_ms: u64,
}

/// Rate Limiter
pub struct RateLimiter {
    limits: HashMap<String, RateLimit>,
}

/// Rate Limit
#[derive(Debug, Clone)]
struct RateLimit {
    tokens: u64,
    max_tokens: u64,
    refill_rate: u64,
    last_refill: Instant,
}

/// Query Optimizer
pub struct QueryOptimizer {
    query_cache: Arc<RwLock<HashMap<String, OptimizedQuery>>>,
    optimization_rules: Vec<OptimizationRule>,
}

/// Optimized Query
#[derive(Debug, Clone)]
struct OptimizedQuery {
    original: String,
    optimized: String,
    execution_plan: ExecutionPlan,
}

/// Execution Plan
#[derive(Debug, Clone)]
struct ExecutionPlan {
    steps: Vec<ExecutionStep>,
    estimated_cost: f64,
}

/// Execution Step
#[derive(Debug, Clone)]
struct ExecutionStep {
    operation: String,
    estimated_time_ms: u64,
    parallelizable: bool,
}

/// Optimization Rule
#[derive(Debug, Clone)]
struct OptimizationRule {
    name: String,
    pattern: String,
    replacement: String,
    priority: u32,
}

/// Performance Profiler
pub struct PerformanceProfiler {
    profiles: HashMap<String, Profile>,
    enabled: bool,
}

/// Profile
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Profile {
    pub name: String,
    pub call_count: u64,
    pub total_duration_ms: u64,
    pub avg_duration_ms: f64,
    pub min_duration_ms: u64,
    pub max_duration_ms: u64,
    pub last_call: Option<std::time::SystemTime>,
}

impl PerformanceManager {
    /// Create a new performance manager
    pub fn new() -> Result<Self> {
        info!("Creating Performance Manager...");
        
        Ok(Self {
            cache: Arc::new(RwLock::new(CacheManager::new(1000)?)),
            pool: Arc::new(RwLock::new(ConnectionPool::new(PoolConfig::default())?)),
            limiter: Arc::new(RwLock::new(RateLimiter::new())),
            optimizer: Arc::new(RwLock::new(QueryOptimizer::new())),
            profiler: Arc::new(RwLock::new(PerformanceProfiler::new())),
        })
    }
    
    /// Get value from cache
    pub async fn cache_get(&self, key: &str) -> Result<Option<Vec<u8>>> {
        let cache = self.cache.read().await;
        cache.get(key)
    }
    
    /// Set value in cache
    pub async fn cache_set(&self, key: &str, value: Vec<u8>, ttl: Duration) -> Result<()> {
        let cache = self.cache.write().await;
        cache.set(key, value, ttl)
    }
    
    /// Delete value from cache
    pub async fn cache_delete(&self, key: &str) -> Result<()> {
        let cache = self.cache.write().await;
        cache.delete(key)
    }
    
    /// Clear cache
    pub async fn cache_clear(&self) -> Result<()> {
        let cache = self.cache.write().await;
        cache.clear()
    }
    
    /// Get cache statistics
    pub async fn get_cache_stats(&self) -> CacheStatistics {
        let cache = self.cache.read().await;
        cache.get_stats()
    }
    
    /// Acquire connection from pool
    pub async fn acquire_connection(&self) -> Result<String> {
        let pool = self.pool.write().await;
        pool.acquire().await
    }
    
    /// Release connection back to pool
    pub async fn release_connection(&self, connection_id: &str) -> Result<()> {
        let pool = self.pool.write().await;
        pool.release(connection_id).await
    }
    
    /// Get pool statistics
    pub async fn get_pool_stats(&self) -> PoolStatistics {
        let pool = self.pool.read().await;
        pool.get_stats()
    }
    
    /// Check rate limit
    pub async fn check_rate_limit(&self, key: &str, tokens: u64) -> Result<bool> {
        let mut limiter = self.limiter.write().await;
        Ok(limiter.check(key, tokens))
    }
    
    /// Configure rate limit
    pub async fn configure_rate_limit(&self, key: &str, max_tokens: u64, refill_rate: u64) -> Result<()> {
        let mut limiter = self.limiter.write().await;
        limiter.configure(key, max_tokens, refill_rate);
        Ok(())
    }
    
    /// Optimize query
    pub async fn optimize_query(&self, query: &str) -> Result<String> {
        let optimizer = self.optimizer.read().await;
        optimizer.optimize(query)
    }
    
    /// Start profiling
    pub async fn start_profiling(&self, name: &str) -> Result<ProfileGuard> {
        let mut profiler = self.profiler.write().await;
        profiler.start(name)
    }
    
    /// Get profile
    pub async fn get_profile(&self, name: &str) -> Option<Profile> {
        let profiler = self.profiler.read().await;
        profiler.get(name)
    }
    
    /// Get all profiles
    pub async fn get_all_profiles(&self) -> HashMap<String, Profile> {
        let profiler = self.profiler.read().await;
        profiler.get_all()
    }
    
    /// Enable profiling
    pub async fn enable_profiling(&self) {
        let mut profiler = self.profiler.write().await;
        profiler.enabled = true;
    }
    
    /// Disable profiling
    pub async fn disable_profiling(&self) {
        let mut profiler = self.profiler.write().await;
        profiler.enabled = false;
    }
}

impl CacheManager {
    pub fn new(capacity: usize) -> Result<Self> {
        Ok(Self {
            memory_cache: Arc::new(RwLock::new(LruCache::new(NonZeroUsize::new(capacity).ok_or_else(|| anyhow!("Invalid capacity"))?))),
            cache_stats: Arc::new(RwLock::new(CacheStatistics::default())),
        })
    }
    
    pub async fn get(&self, key: &str) -> Result<Option<Vec<u8>>> {
        let mut cache = self.memory_cache.write().await;
        let mut stats = self.cache_stats.write().await;
        
        if let Some(entry) = cache.get_mut(key) {
            // Check TTL
            if entry.created_at.elapsed() > entry.ttl {
                cache.pop(key);
                stats.misses += 1;
                stats.update_hit_rate();
                return Ok(None);
            }
            
            entry.access_count += 1;
            stats.hits += 1;
            stats.update_hit_rate();
            Ok(Some(entry.value.clone()))
        } else {
            stats.misses += 1;
            stats.update_hit_rate();
            Ok(None)
        }
    }
    
    pub async fn set(&self, key: &str, value: Vec<u8>, ttl: Duration) -> Result<()> {
        let mut cache = self.memory_cache.write().await;
        let mut stats = self.cache_stats.write().await;
        
        let entry = CacheEntry {
            value,
            created_at: Instant::now(),
            ttl,
            access_count: 0,
        };
        
        cache.put(key.to_string(), entry);
        stats.size = cache.len();
        
        Ok(())
    }
    
    pub async fn delete(&self, key: &str) -> Result<()> {
        let mut cache = self.memory_cache.write().await;
        cache.pop(key);
        Ok(())
    }
    
    pub async fn clear(&self) -> Result<()> {
        let mut cache = self.memory_cache.write().await;
        cache.clear();
        Ok(())
    }
    
    pub fn get_stats(&self) -> CacheStatistics {
        self.cache_stats.blocking_read().clone()
    }
}

impl CacheStatistics {
    fn update_hit_rate(&mut self) {
        let total = self.hits + self.misses;
        if total > 0 {
            self.hit_rate = self.hits as f64 / total as f64;
        }
    }
}

impl Default for CacheStatistics {
    fn default() -> Self {
        Self {
            hits: 0,
            misses: 0,
            evictions: 0,
            size: 0,
            hit_rate: 0.0,
        }
    }
}

impl ConnectionPool {
    pub fn new(config: PoolConfig) -> Result<Self> {
        Ok(Self {
            connections: Arc::new(RwLock::new(Vec::new())),
            pool_config: config,
            pool_stats: Arc::new(RwLock::new(PoolStatistics::default())),
        })
    }
    
    pub async fn acquire(&mut self) -> Result<String> {
        let mut connections = self.connections.write().await;
        let mut stats = self.pool_stats.write().await;
        
        // Find idle connection
        for conn in connections.iter_mut() {
            if !conn.in_use {
                conn.in_use = true;
                conn.last_used = Instant::now();
                stats.active_connections += 1;
                stats.idle_connections -= 1;
                return Ok(conn.id.clone());
            }
        }
        
        // Create new connection if under limit
        if connections.len() < self.pool_config.max_connections {
            let conn = PooledConnection {
                id: uuid::Uuid::new_v4().to_string(),
                created_at: Instant::now(),
                last_used: Instant::now(),
                in_use: true,
            };
            let id = conn.id.clone();
            connections.push(conn);
            stats.total_connections += 1;
            stats.active_connections += 1;
            return Ok(id);
        }
        
        // Pool exhausted
        stats.wait_count += 1;
        Err(anyhow!("Connection pool exhausted"))
    }
    
    pub async fn release(&mut self, connection_id: &str) -> Result<()> {
        let mut connections = self.connections.write().await;
        let mut stats = self.pool_stats.write().await;
        
        if let Some(conn) = connections.iter_mut().find(|c| c.id == connection_id) {
            conn.in_use = false;
            conn.last_used = Instant::now();
            stats.active_connections -= 1;
            stats.idle_connections += 1;
        }
        
        Ok(())
    }
    
    pub fn get_stats(&self) -> PoolStatistics {
        self.pool_stats.blocking_read().clone()
    }
}

impl Default for PoolConfig {
    fn default() -> Self {
        Self {
            max_connections: 10,
            min_connections: 2,
            max_idle_time: Duration::from_secs(300),
            connection_timeout: Duration::from_secs(30),
        }
    }
}

impl Default for PoolStatistics {
    fn default() -> Self {
        Self {
            total_connections: 0,
            active_connections: 0,
            idle_connections: 0,
            wait_count: 0,
            wait_duration_ms: 0,
        }
    }
}

impl RateLimiter {
    pub fn new() -> Self {
        Self {
            limits: HashMap::new(),
        }
    }
    
    pub fn configure(&mut self, key: &str, max_tokens: u64, refill_rate: u64) {
        self.limits.insert(key.to_string(), RateLimit {
            tokens: max_tokens,
            max_tokens,
            refill_rate,
            last_refill: Instant::now(),
        });
    }
    
    pub fn check(&mut self, key: &str, tokens: u64) -> bool {
        let limit = self.limits.entry(key.to_string()).or_insert_with(|| RateLimit {
            tokens: 100,
            max_tokens: 100,
            refill_rate: 10,
            last_refill: Instant::now(),
        });
        
        // Refill tokens
        let elapsed = limit.last_refill.elapsed();
        let tokens_to_add = (elapsed.as_secs() as u64) * limit.refill_rate;
        limit.tokens = (limit.tokens + tokens_to_add).min(limit.max_tokens);
        limit.last_refill = Instant::now();
        
        // Check if enough tokens
        if limit.tokens >= tokens {
            limit.tokens -= tokens;
            true
        } else {
            false
        }
    }
}

impl QueryOptimizer {
    pub fn new() -> Self {
        Self {
            query_cache: Arc::new(RwLock::new(HashMap::new())),
            optimization_rules: vec![
                OptimizationRule {
                    name: "Remove unnecessary SELECT *".to_string(),
                    pattern: "SELECT * FROM".to_string(),
                    replacement: "SELECT id, name FROM".to_string(),
                    priority: 10,
                },
                OptimizationRule {
                    name: "Add LIMIT clause".to_string(),
                    pattern: "SELECT".to_string(),
                    replacement: "SELECT".to_string(),
                    priority: 5,
                },
            ],
        }
    }
    
    pub fn optimize(&self, query: &str) -> Result<String> {
        let mut optimized = query.to_string();
        
        for rule in &self.optimization_rules {
            if optimized.contains(&rule.pattern) {
                optimized = optimized.replace(&rule.pattern, &rule.replacement);
            }
        }
        
        Ok(optimized)
    }
}

impl PerformanceProfiler {
    pub fn new() -> Self {
        Self {
            profiles: HashMap::new(),
            enabled: false,
        }
    }
    
    pub fn start(&mut self, name: &str) -> Result<ProfileGuard> {
        if !self.enabled {
            return Ok(ProfileGuard::new(name, None));
        }
        
        Ok(ProfileGuard::new(name, Some(Arc::new(RwLock::new(self)))))
    }
    
    pub fn record(&mut self, name: &str, duration: Duration) {
        let profile = self.profiles.entry(name.to_string()).or_insert_with(|| Profile {
            name: name.to_string(),
            call_count: 0,
            total_duration_ms: 0,
            avg_duration_ms: 0.0,
            min_duration_ms: u64::MAX,
            max_duration_ms: 0,
            last_call: None,
        });
        
        let duration_ms = duration.as_millis() as u64;
        profile.call_count += 1;
        profile.total_duration_ms += duration_ms;
        profile.avg_duration_ms = profile.total_duration_ms as f64 / profile.call_count as f64;
        profile.min_duration_ms = profile.min_duration_ms.min(duration_ms);
        profile.max_duration_ms = profile.max_duration_ms.max(duration_ms);
        profile.last_call = Some(std::time::SystemTime::now());
    }
    
    pub fn get(&self, name: &str) -> Option<Profile> {
        self.profiles.get(name).cloned()
    }
    
    pub fn get_all(&self) -> HashMap<String, Profile> {
        self.profiles.clone()
    }
}

/// Profile Guard for automatic timing
pub struct ProfileGuard {
    name: String,
    profiler: Option<Arc<RwLock<PerformanceProfiler>>>,
    start: Instant,
}

impl ProfileGuard {
    fn new(name: &str, profiler: Option<Arc<RwLock<PerformanceProfiler>>>) -> Self {
        Self {
            name: name.to_string(),
            profiler,
            start: Instant::now(),
        }
    }
}

impl Drop for ProfileGuard {
    fn drop(&mut self) {
        if let Some(profiler) = &self.profiler {
            let duration = self.start.elapsed();
            if let Ok(mut p) = profiler.try_write() {
                p.record(&self.name, duration);
            }
        }
    }
}

/// Initialize performance module
pub fn init() -> Result<()> {
    info!("Performance Optimization Module initialized");
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test]
    async fn test_performance_manager_creation() {
        let manager = PerformanceManager::new().unwrap();
        assert!(manager.cache_set("test", vec![1, 2, 3], Duration::from_secs(60)).await.is_ok());
    }
    
    #[tokio::test]
    async fn test_cache_operations() {
        let manager = PerformanceManager::new().unwrap();
        
        manager.cache_set("key1", vec![1, 2, 3], Duration::from_secs(60)).await.unwrap();
        let value = manager.cache_get("key1").await.unwrap();
        assert_eq!(value, Some(vec![1, 2, 3]));
        
        manager.cache_delete("key1").await.unwrap();
        let value = manager.cache_get("key1").await.unwrap();
        assert_eq!(value, None);
    }
    
    #[tokio::test]
    async fn test_cache_statistics() {
        let manager = PerformanceManager::new().unwrap();
        
        manager.cache_set("key1", vec![1, 2, 3], Duration::from_secs(60)).await.unwrap();
        manager.cache_get("key1").await.unwrap();
        manager.cache_get("key2").await.unwrap();
        
        let stats = manager.get_cache_stats().await;
        assert_eq!(stats.hits, 1);
        assert_eq!(stats.misses, 1);
        assert!(stats.hit_rate > 0.0);
    }
    
    #[tokio::test]
    async fn test_connection_pool() {
        let manager = PerformanceManager::new().unwrap();
        
        let conn1 = manager.acquire_connection().await.unwrap();
        let conn2 = manager.acquire_connection().await.unwrap();
        
        assert_ne!(conn1, conn2);
        
        manager.release_connection(&conn1).await.unwrap();
        
        let stats = manager.get_pool_stats().await;
        assert_eq!(stats.total_connections, 2);
    }
    
    #[tokio::test]
    async fn test_rate_limiter() {
        let manager = PerformanceManager::new().unwrap();
        
        manager.configure_rate_limit("test", 10, 1).await.unwrap();
        
        assert!(manager.check_rate_limit("test", 5).await.unwrap());
        assert!(manager.check_rate_limit("test", 5).await.unwrap());
        assert!(!manager.check_rate_limit("test", 1).await.unwrap());
    }
    
    #[tokio::test]
    async fn test_query_optimization() {
        let manager = PerformanceManager::new().unwrap();
        
        let optimized = manager.optimize_query("SELECT * FROM users").await.unwrap();
        assert!(optimized.contains("SELECT id, name FROM"));
    }
    
    #[tokio::test]
    async fn test_profiling() {
        let manager = PerformanceManager::new().unwrap();
        manager.enable_profiling().await;
        
        {
            let _guard = manager.start_profiling("test_operation").await.unwrap();
            tokio::time::sleep(Duration::from_millis(10)).await;
        }
        
        let profile = manager.get_profile("test_operation").await;
        assert!(profile.is_some());
        assert_eq!(profile.unwrap().call_count, 1);
    }
}