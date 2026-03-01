//! Advanced Caching Strategies
//! 
//! This module provides comprehensive caching capabilities:
//! - Multi-level caching (L1, L2, L3)
//! - Cache eviction policies (LRU, LFU, TTL)
//! - Distributed caching support
//! - Cache warming and preloading
//! - Cache statistics and monitoring
//! - Cache invalidation strategies

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::sync::RwLock;
use tracing::{debug, error, info, warn};

/// Cache level
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum CacheLevel {
    L1, // In-memory, fastest
    L2, // In-memory, larger
    L3, // Distributed, largest
}

/// Cache eviction policy
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EvictionPolicy {
    LRU,  // Least Recently Used
    LFU,  // Least Frequently Used
    FIFO, // First In First Out
    TTL,  // Time To Live
}

/// Cache entry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CacheEntry<T> {
    pub key: String,
    pub value: T,
    pub created_at: Instant,
    pub last_accessed: Instant,
    pub access_count: u64,
    pub ttl: Option<Duration>,
    pub size_bytes: u64,
}

/// Cache statistics
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CacheStatistics {
    pub hits: u64,
    pub misses: u64,
    pub evictions: u64,
    pub size_bytes: u64,
    pub entry_count: usize,
    pub hit_rate: f64,
    pub avg_access_time_ns: u64,
}

/// Cache configuration
#[derive(Debug, Clone)]
pub struct CacheConfig {
    pub max_size_bytes: u64,
    pub max_entries: usize,
    pub default_ttl: Option<Duration>,
    pub eviction_policy: EvictionPolicy,
    pub enable_stats: bool,
    pub enable_warming: bool,
}

impl Default for CacheConfig {
    fn default() -> Self {
        Self {
            max_size_bytes: 100 * 1024 * 1024, // 100 MB
            max_entries: 10_000,
            default_ttl: Some(Duration::from_secs(3600)), // 1 hour
            eviction_policy: EvictionPolicy::LRU,
            enable_stats: true,
            enable_warming: false,
        }
    }
}

/// Advanced Cache
pub struct AdvancedCache<T: Clone + Serialize + for<'de> Deserialize<'de>> {
    level: CacheLevel,
    config: CacheConfig,
    entries: Arc<RwLock<HashMap<String, CacheEntry<T>>>>,
    statistics: Arc<RwLock<CacheStatistics>>,
    access_times: Arc<RwLock<Vec<Duration>>>,
}

impl<T: Clone + Serialize + for<'de> Deserialize<'de>> AdvancedCache<T> {
    /// Create a new advanced cache
    pub fn new(level: CacheLevel, config: CacheConfig) -> Self {
        Self {
            level,
            config,
            entries: Arc::new(RwLock::new(HashMap::new())),
            statistics: Arc::new(RwLock::new(CacheStatistics::default())),
            access_times: Arc::new(RwLock::new(Vec::new())),
        }
    }

    /// Initialize the cache
    pub async fn initialize(&self) -> Result<()> {
        info!("Initializing Advanced Cache (Level: {:?})", self.level);
        
        // Start background tasks
        self.start_ttl_cleanup().await?;
        self.start_stats_update().await?;
        
        info!("Advanced Cache initialized successfully");
        Ok(())
    }

    /// Get a value from the cache
    pub async fn get(&self, key: &str) -> Result<Option<T>> {
        let start = Instant::now();
        
        let mut entries = self.entries.write().await;
        
        if let Some(entry) = entries.get_mut(key) {
            // Check TTL
            if let Some(ttl) = entry.ttl {
                if entry.created_at.elapsed()? > ttl {
                    entries.remove(key);
                    
                    let mut stats = self.statistics.write().await;
                    stats.misses += 1;
                    stats.entry_count = entries.len();
                    
                    return Ok(None);
                }
            }
            
            // Update access statistics
            entry.last_accessed = Instant::now();
            entry.access_count += 1;
            
            let value = entry.value.clone();
            
            // Update statistics
            if self.config.enable_stats {
                let mut stats = self.statistics.write().await;
                stats.hits += 1;
                stats.hit_rate = stats.hits as f64 / (stats.hits + stats.misses) as f64;
                
                let access_time = start.elapsed();
                let total_time = stats.avg_access_time_ns * (stats.hits + stats.misses - 1) + access_time.as_nanos() as u64;
                stats.avg_access_time_ns = total_time / (stats.hits + stats.misses);
            }
            
            debug!("Cache hit for key: {}", key);
            return Ok(Some(value));
        }
        
        // Update statistics
        if self.config.enable_stats {
            let mut stats = self.statistics.write().await;
            stats.misses += 1;
            stats.hit_rate = stats.hits as f64 / (stats.hits + stats.misses) as f64;
        }
        
        debug!("Cache miss for key: {}", key);
        Ok(None)
    }

    /// Set a value in the cache
    pub async fn set(&self, key: String, value: T, ttl: Option<Duration>) -> Result<()> {
        let size_bytes = self.calculate_size(&value)?;
        
        // Check if we need to evict entries
        self.check_capacity().await?;
        
        let entry = CacheEntry {
            key: key.clone(),
            value,
            created_at: Instant::now(),
            last_accessed: Instant::now(),
            access_count: 0,
            ttl: ttl.or(self.config.default_ttl),
            size_bytes,
        };
        
        let mut entries = self.entries.write().await;
        entries.insert(key, entry);
        
        // Update statistics
        if self.config.enable_stats {
            let mut stats = self.statistics.write().await;
            stats.entry_count = entries.len();
            stats.size_bytes = entries.values().map(|e| e.size_bytes).sum();
        }
        
        debug!("Cache set for key: {}", key);
        Ok(())
    }

    /// Delete a value from the cache
    pub async fn delete(&self, key: &str) -> Result<bool> {
        let mut entries = self.entries.write().await;
        let removed = entries.remove(key).is_some();
        
        if removed {
            debug!("Cache delete for key: {}", key);
            
            // Update statistics
            if self.config.enable_stats {
                let mut stats = self.statistics.write().await;
                stats.entry_count = entries.len();
                stats.size_bytes = entries.values().map(|e| e.size_bytes).sum();
            }
        }
        
        Ok(removed)
    }

    /// Clear all entries from the cache
    pub async fn clear(&self) -> Result<()> {
        let mut entries = self.entries.write().await;
        entries.clear();
        
        info!("Cache cleared");
        
        // Update statistics
        if self.config.enable_stats {
            let mut stats = self.statistics.write().await;
            stats.entry_count = 0;
            stats.size_bytes = 0;
        }
        
        Ok(())
    }

    /// Get cache statistics
    pub async fn get_statistics(&self) -> CacheStatistics {
        self.statistics.read().await.clone()
    }

    /// Warm the cache with preloaded data
    pub async fn warm_cache(&self, data: HashMap<String, T>) -> Result<()> {
        info!("Warming cache with {} entries", data.len());
        
        for (key, value) in data {
            if let Err(e) = self.set(key, value, None).await {
                warn!("Failed to warm cache entry: {}", e);
            }
        }
        
        info!("Cache warming complete");
        Ok(())
    }

    /// Invalidate cache entries matching a pattern
    pub async fn invalidate_pattern(&self, pattern: &str) -> Result<usize> {
        let mut entries = self.entries.write().await;
        let initial_count = entries.len();
        
        entries.retain(|key, _| !key.contains(pattern));
        
        let invalidated = initial_count - entries.len();
        
        info!("Invalidated {} cache entries matching pattern: {}", invalidated, pattern);
        
        // Update statistics
        if self.config.enable_stats {
            let mut stats = self.statistics.write().await;
            stats.entry_count = entries.len();
            stats.size_bytes = entries.values().map(|e| e.size_bytes).sum();
        }
        
        Ok(invalidated)
    }

    /// Calculate size of a value
    fn calculate_size(&self, value: &T) -> Result<u64> {
        let serialized = serde_json::to_vec(value)?;
        Ok(serialized.len() as u64)
    }

    /// Check capacity and evict if necessary
    async fn check_capacity(&self) -> Result<()> {
        let entries = self.entries.read().await;
        let current_size: u64 = entries.values().map(|e| e.size_bytes).sum();
        let current_count = entries.len();
        drop(entries);
        
        // Check size limit
        if current_size > self.config.max_size_bytes {
            self.evict_by_size().await?;
        }
        
        // Check entry limit
        if current_count > self.config.max_entries {
            self.evict_by_count().await?;
        }
        
        Ok(())
    }

    /// Evict entries by size
    async fn evict_by_size(&self) -> Result<()> {
        let mut entries = self.entries.write().await;
        let mut evicted = 0;
        
        while entries.values().map(|e| e.size_bytes).sum() > self.config.max_size_bytes * 9 / 10 {
            if let Some(key_to_evict) = self.find_eviction_candidate(&entries) {
                entries.remove(&key_to_evict);
                evicted += 1;
            } else {
                break;
            }
        }
        
        if evicted > 0 {
            info!("Evicted {} entries due to size limit", evicted);
            
            // Update statistics
            if self.config.enable_stats {
                let mut stats = self.statistics.write().await;
                stats.evictions += evicted;
                stats.entry_count = entries.len();
                stats.size_bytes = entries.values().map(|e| e.size_bytes).sum();
            }
        }
        
        Ok(())
    }

    /// Evict entries by count
    async fn evict_by_count(&self) -> Result<()> {
        let mut entries = self.entries.write().await;
        let target_count = self.config.max_entries * 9 / 10;
        let mut evicted = 0;
        
        while entries.len() > target_count {
            if let Some(key_to_evict) = self.find_eviction_candidate(&entries) {
                entries.remove(&key_to_evict);
                evicted += 1;
            } else {
                break;
            }
        }
        
        if evicted > 0 {
            info!("Evicted {} entries due to count limit", evicted);
            
            // Update statistics
            if self.config.enable_stats {
                let mut stats = self.statistics.write().await;
                stats.evictions += evicted;
                stats.entry_count = entries.len();
                stats.size_bytes = entries.values().map(|e| e.size_bytes).sum();
            }
        }
        
        Ok(())
    }

    /// Find eviction candidate based on policy
    fn find_eviction_candidate(&self, entries: &HashMap<String, CacheEntry<T>>) -> Option<String> {
        match self.config.eviction_policy {
            EvictionPolicy::LRU => {
                // Find least recently used
                entries.iter()
                    .min_by_key(|(_, e)| e.last_accessed)
                    .map(|(k, _)| k.clone())
            }
            EvictionPolicy::LFU => {
                // Find least frequently used
                entries.iter()
                    .min_by_key(|(_, e)| e.access_count)
                    .map(|(k, _)| k.clone())
            }
            EvictionPolicy::FIFO => {
                // Find oldest entry
                entries.iter()
                    .min_by_key(|(_, e)| e.created_at)
                    .map(|(k, _)| k.clone())
            }
            EvictionPolicy::TTL => {
                // Find entry with shortest TTL
                entries.iter()
                    .filter(|(_, e)| e.ttl.is_some())
                    .min_by_key(|(_, e)| e.ttl.unwrap())
                    .map(|(k, _)| k.clone())
            }
        }
    }

    /// Start TTL cleanup task
    async fn start_ttl_cleanup(&self) -> Result<()> {
        let entries = Arc::clone(&self.entries);
        let statistics = Arc::clone(&self.statistics);
        
        tokio::spawn(async move {
            let mut timer = interval(Duration::from_secs(60)); // Every minute
            
            loop {
                timer.tick().await;
                
                let mut entries_lock = entries.write().await;
                let now = Instant::now();
                let mut expired_keys = Vec::new();
                
                for (key, entry) in entries_lock.iter() {
                    if let Some(ttl) = entry.ttl {
                        if let Ok(elapsed) = now.duration_since(entry.created_at) {
                            if elapsed > ttl {
                                expired_keys.push(key.clone());
                            }
                        }
                    }
                }
                
                for key in expired_keys {
                    entries_lock.remove(&key);
                }
                
                if !expired_keys.is_empty() {
                    debug!("Cleaned up {} expired cache entries", expired_keys.len());
                    
                    // Update statistics
                    let mut stats = statistics.write().await;
                    stats.entry_count = entries_lock.len();
                    stats.size_bytes = entries_lock.values().map(|e| e.size_bytes).sum();
                }
            }
        });
        
        Ok(())
    }

    /// Start statistics update task
    async fn start_stats_update(&self) -> Result<()> {
        if !self.config.enable_stats {
            return Ok(());
        }
        
        let statistics = Arc::clone(&self.statistics);
        
        tokio::spawn(async move {
            let mut timer = interval(Duration::from_secs(10)); // Every 10 seconds
            
            loop {
                timer.tick().await;
                
                let mut stats = statistics.write().await;
                // Update hit rate
                if stats.hits + stats.misses > 0 {
                    stats.hit_rate = stats.hits as f64 / (stats.hits + stats.misses) as f64;
                }
            }
        });
        
        Ok(())
    }
}

/// Multi-level cache manager
pub struct MultiLevelCacheManager<T: Clone + Serialize + for<'de> Deserialize<'de>> {
    l1_cache: Arc<AdvancedCache<T>>,
    l2_cache: Arc<AdvancedCache<T>>,
    l3_cache: Option<Arc<AdvancedCache<T>>>,
}

impl<T: Clone + Serialize + for<'de> Deserialize<'de>> MultiLevelCacheManager<T> {
    /// Create a new multi-level cache manager
    pub fn new(l1_config: CacheConfig, l2_config: CacheConfig, l3_config: Option<CacheConfig>) -> Self {
        let l1_cache = Arc::new(AdvancedCache::new(CacheLevel::L1, l1_config));
        let l2_cache = Arc::new(AdvancedCache::new(CacheLevel::L2, l2_config));
        let l3_cache = l3_config.map(|config| Arc::new(AdvancedCache::new(CacheLevel::L3, config)));
        
        Self {
            l1_cache,
            l2_cache,
            l3_cache,
        }
    }

    /// Initialize the cache manager
    pub async fn initialize(&self) -> Result<()> {
        info!("Initializing Multi-Level Cache Manager");
        
        self.l1_cache.initialize().await?;
        self.l2_cache.initialize().await?;
        
        if let Some(ref l3) = self.l3_cache {
            l3.initialize().await?;
        }
        
        info!("Multi-Level Cache Manager initialized successfully");
        Ok(())
    }

    /// Get a value from the cache (checks L1, then L2, then L3)
    pub async fn get(&self, key: &str) -> Result<Option<T>> {
        // Try L1 first
        if let Some(value) = self.l1_cache.get(key).await? {
            return Ok(Some(value));
        }
        
        // Try L2
        if let Some(value) = self.l2_cache.get(key).await? {
            // Promote to L1
            let _ = self.l1_cache.set(key.to_string(), value.clone(), None).await;
            return Ok(Some(value));
        }
        
        // Try L3
        if let Some(ref l3) = self.l3_cache {
            if let Some(value) = l3.get(key).await? {
                // Promote to L2 and L1
                let _ = self.l2_cache.set(key.to_string(), value.clone(), None).await;
                let _ = self.l1_cache.set(key.to_string(), value.clone(), None).await;
                return Ok(Some(value));
            }
        }
        
        Ok(None)
    }

    /// Set a value in the cache (stores in all levels)
    pub async fn set(&self, key: String, value: T, ttl: Option<Duration>) -> Result<()> {
        self.l1_cache.set(key.clone(), value.clone(), ttl).await?;
        self.l2_cache.set(key.clone(), value.clone(), ttl).await?;
        
        if let Some(ref l3) = self.l3_cache {
            l3.set(key, value, ttl).await?;
        }
        
        Ok(())
    }

    /// Delete a value from all cache levels
    pub async fn delete(&self, key: &str) -> Result<()> {
        self.l1_cache.delete(key).await?;
        self.l2_cache.delete(key).await?;
        
        if let Some(ref l3) = self.l3_cache {
            l3.delete(key).await?;
        }
        
        Ok(())
    }

    /// Clear all cache levels
    pub async fn clear(&self) -> Result<()> {
        self.l1_cache.clear().await?;
        self.l2_cache.clear().await?;
        
        if let Some(ref l3) = self.l3_cache {
            l3.clear().await?;
        }
        
        Ok(())
    }

    /// Get combined statistics
    pub async fn get_statistics(&self) -> MultiLevelCacheStatistics {
        let l1_stats = self.l1_cache.get_statistics().await;
        let l2_stats = self.l2_cache.get_statistics().await;
        let l3_stats = if let Some(ref l3) = self.l3_cache {
            Some(l3.get_statistics().await)
        } else {
            None
        };
        
        MultiLevelCacheStatistics {
            l1: l1_stats,
            l2: l2_stats,
            l3: l3_stats,
        }
    }
}

/// Multi-level cache statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MultiLevelCacheStatistics {
    pub l1: CacheStatistics,
    pub l2: CacheStatistics,
    pub l3: Option<CacheStatistics>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_cache_initialization() {
        let config = CacheConfig::default();
        let cache = AdvancedCache::<String>::new(CacheLevel::L1, config);
        
        assert!(cache.initialize().await.is_ok());
    }

    #[tokio::test]
    async fn test_cache_set_get() {
        let config = CacheConfig::default();
        let cache = AdvancedCache::<String>::new(CacheLevel::L1, config);
        cache.initialize().await.unwrap();
        
        cache.set("key1".to_string(), "value1".to_string(), None).await.unwrap();
        
        let value = cache.get("key1").await.unwrap();
        assert_eq!(value, Some("value1".to_string()));
    }

    #[tokio::test]
    async fn test_cache_miss() {
        let config = CacheConfig::default();
        let cache = AdvancedCache::<String>::new(CacheLevel::L1, config);
        cache.initialize().await.unwrap();
        
        let value = cache.get("nonexistent".to_string()).await.unwrap();
        assert_eq!(value, None);
    }

    #[tokio::test]
    async fn test_cache_delete() {
        let config = CacheConfig::default();
        let cache = AdvancedCache::<String>::new(CacheLevel::L1, config);
        cache.initialize().await.unwrap();
        
        cache.set("key1".to_string(), "value1".to_string(), None).await.unwrap();
        let deleted = cache.delete("key1").await.unwrap();
        assert!(deleted);
        
        let value = cache.get("key1").await.unwrap();
        assert_eq!(value, None);
    }

    #[tokio::test]
    async fn test_cache_statistics() {
        let config = CacheConfig::default();
        let cache = AdvancedCache::<String>::new(CacheLevel::L1, config);
        cache.initialize().await.unwrap();
        
        cache.set("key1".to_string(), "value1".to_string(), None).await.unwrap();
        cache.get("key1").await.unwrap();
        cache.get("key2").await.unwrap();
        
        let stats = cache.get_statistics().await;
        assert_eq!(stats.hits, 1);
        assert_eq!(stats.misses, 1);
        assert_eq!(stats.hit_rate, 0.5);
    }

    #[tokio::test]
    async fn test_multi_level_cache() {
        let l1_config = CacheConfig {
            max_entries: 10,
            ..Default::default()
        };
        let l2_config = CacheConfig {
            max_entries: 100,
            ..Default::default()
        };
        
        let manager = MultiLevelCacheManager::<String>::new(l1_config, l2_config, None);
        manager.initialize().await.unwrap();
        
        manager.set("key1".to_string(), "value1".to_string(), None).await.unwrap();
        
        let value = manager.get("key1").await.unwrap();
        assert_eq!(value, Some("value1".to_string()));
    }
}