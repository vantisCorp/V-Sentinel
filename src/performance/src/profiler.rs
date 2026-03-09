// Performance Profiling Tools for SENTINEL Security System
// Provides detailed performance profiling and analysis capabilities

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

/// Profiling event type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ProfileEventType {
    FunctionCall,
    MemoryAllocation,
    IOOperation,
    LockAcquisition,
    CacheAccess,
    NetworkRequest,
}

/// Profiling event
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProfileEvent {
    pub event_type: ProfileEventType,
    pub component: String,
    pub operation: String,
    pub start_time: u64,
    pub end_time: u64,
    pub duration_ns: u64,
    pub thread_id: u64,
    pub metadata: HashMap<String, String>,
}

/// Function call statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FunctionStats {
    pub component: String,
    pub operation: String,
    pub call_count: usize,
    pub total_duration_ns: u64,
    pub min_duration_ns: u64,
    pub max_duration_ns: u64,
    pub avg_duration_ns: u64,
    pub p50_duration_ns: u64,
    pub p95_duration_ns: u64,
    pub p99_duration_ns: u64,
}

/// Memory allocation statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryStats {
    pub component: String,
    pub operation: String,
    pub allocation_count: usize,
    pub total_allocated_bytes: u64,
    pub peak_allocated_bytes: u64,
    pub avg_allocation_bytes: u64,
}

/// Performance profiler
pub struct PerformanceProfiler {
    events: Arc<Mutex<Vec<ProfileEvent>>>,
    enabled: Arc<Mutex<bool>>,
    thread_id: u64,
}

impl PerformanceProfiler {
    pub fn new() -> Self {
        Self {
            events: Arc::new(Mutex::new(Vec::new())),
            enabled: Arc::new(Mutex::new(false)),
            thread_id: Self::get_thread_id(),
        }
    }

    fn get_thread_id() -> u64 {
        use std::thread;
        // Get current thread ID (simplified - use hash of thread ID)
        format!("{:?}", thread::current().id())
            .chars()
            .map(|c| c as u64)
            .sum()
    }

    pub fn enable(&self) {
        let mut enabled = self.enabled.lock().unwrap();
        *enabled = true;
    }

    pub fn disable(&self) {
        let mut enabled = self.enabled.lock().unwrap();
        *enabled = false;
    }

    pub fn is_enabled(&self) -> bool {
        let enabled = self.enabled.lock().unwrap();
        *enabled
    }

    pub fn profile_function<F, R>(&self, component: &str, operation: &str, f: F) -> R
    where
        F: FnOnce() -> R,
    {
        if !self.is_enabled() {
            return f();
        }

        let start = Instant::now();
        let start_time = Self::get_timestamp_ns();

        let result = f();

        let end = start.elapsed();
        let end_time = Self::get_timestamp_ns();

        let event = ProfileEvent {
            event_type: ProfileEventType::FunctionCall,
            component: component.to_string(),
            operation: operation.to_string(),
            start_time,
            end_time,
            duration_ns: end.as_nanos() as u64,
            thread_id: self.thread_id,
            metadata: HashMap::new(),
        };

        let mut events = self.events.lock().unwrap();
        events.push(event);

        result
    }

    pub fn profile_memory<F, R>(&self, component: &str, operation: &str, f: F) -> R
    where
        F: FnOnce() -> R,
    {
        if !self.is_enabled() {
            return f();
        }

        let start = Instant::now();
        let start_time = Self::get_timestamp_ns();
        let memory_before = Self::get_memory_usage();

        let result = f();

        let end = start.elapsed();
        let end_time = Self::get_timestamp_ns();
        let memory_after = Self::get_memory_usage();

        let mut metadata = HashMap::new();
        metadata.insert(
            "allocated_bytes".to_string(),
            (memory_after - memory_before).to_string(),
        );

        let event = ProfileEvent {
            event_type: ProfileEventType::MemoryAllocation,
            component: component.to_string(),
            operation: operation.to_string(),
            start_time,
            end_time,
            duration_ns: end.as_nanos() as u64,
            thread_id: self.thread_id,
            metadata,
        };

        let mut events = self.events.lock().unwrap();
        events.push(event);

        result
    }

    pub fn profile_io<F, R>(&self, component: &str, operation: &str, f: F) -> R
    where
        F: FnOnce() -> R,
    {
        if !self.is_enabled() {
            return f();
        }

        let start = Instant::now();
        let start_time = Self::get_timestamp_ns();

        let result = f();

        let end = start.elapsed();
        let end_time = Self::get_timestamp_ns();

        let event = ProfileEvent {
            event_type: ProfileEventType::IOOperation,
            component: component.to_string(),
            operation: operation.to_string(),
            start_time,
            end_time,
            duration_ns: end.as_nanos() as u64,
            thread_id: self.thread_id,
            metadata: HashMap::new(),
        };

        let mut events = self.events.lock().unwrap();
        events.push(event);

        result
    }

    pub fn get_events(&self) -> Vec<ProfileEvent> {
        let events = self.events.lock().unwrap();
        events.clone()
    }

    pub fn clear_events(&self) {
        let mut events = self.events.lock().unwrap();
        events.clear();
    }

    pub fn analyze_function_calls(&self) -> Vec<FunctionStats> {
        let events = self.get_events();
        let function_events: Vec<_> = events
            .iter()
            .filter(|e| e.event_type == ProfileEventType::FunctionCall)
            .collect();

        let mut call_map: HashMap<(String, String), Vec<u64>> = HashMap::new();

        for event in &function_events {
            let key = (event.component.clone(), event.operation.clone());
            call_map.entry(key).or_default().push(event.duration_ns);
        }

        let mut stats = Vec::new();

        for ((component, operation), durations) in call_map {
            let mut sorted = durations.clone();
            sorted.sort();

            let total: u64 = durations.iter().sum();
            let count = durations.len();

            stats.push(FunctionStats {
                component: component.clone(),
                operation: operation.clone(),
                call_count: count,
                total_duration_ns: total,
                min_duration_ns: *sorted.first().unwrap_or(&0),
                max_duration_ns: *sorted.last().unwrap_or(&0),
                avg_duration_ns: total / count as u64,
                p50_duration_ns: sorted[sorted.len() / 2],
                p95_duration_ns: sorted[(sorted.len() as f64 * 0.95) as usize],
                p99_duration_ns: sorted[(sorted.len() as f64 * 0.99) as usize],
            });
        }

        stats.sort_by(|a, b| b.total_duration_ns.cmp(&a.total_duration_ns));
        stats
    }

    pub fn analyze_memory_allocations(&self) -> Vec<MemoryStats> {
        let events = self.get_events();
        let memory_events: Vec<_> = events
            .iter()
            .filter(|e| e.event_type == ProfileEventType::MemoryAllocation)
            .collect();

        let mut allocation_map: HashMap<(String, String), Vec<u64>> = HashMap::new();

        for event in &memory_events {
            let key = (event.component.clone(), event.operation.clone());
            if let Some(allocated_str) = event.metadata.get("allocated_bytes") {
                if let Ok(allocated) = allocated_str.parse::<u64>() {
                    allocation_map.entry(key).or_default().push(allocated);
                }
            }
        }

        let mut stats = Vec::new();

        for ((component, operation), allocations) in allocation_map {
            let total: u64 = allocations.iter().sum();
            let count = allocations.len();
            let peak = *allocations.iter().max().unwrap_or(&0);

            stats.push(MemoryStats {
                component: component.clone(),
                operation: operation.clone(),
                allocation_count: count,
                total_allocated_bytes: total,
                peak_allocated_bytes: peak,
                avg_allocation_bytes: total / count as u64,
            });
        }

        stats.sort_by(|a, b| b.total_allocated_bytes.cmp(&a.total_allocated_bytes));
        stats
    }

    pub fn generate_flamegraph_data(&self) -> String {
        let events = self.get_events();
        let mut flamegraph = String::new();

        // Group events by component and operation
        let _call_stack: Vec<(String, String, u64)> = Vec::new();
        let mut sorted_events = events.clone();
        sorted_events.sort_by_key(|e| e.start_time);

        for event in &sorted_events {
            if event.event_type == ProfileEventType::FunctionCall {
                let name = format!("{};{}", event.component, event.operation);
                flamegraph.push_str(&format!("{} {}\n", name, event.duration_ns));
            }
        }

        flamegraph
    }

    pub fn generate_report(&self) -> String {
        let mut report = String::new();
        report.push_str("# SENTINEL Performance Profiling Report\n\n");

        let function_stats = self.analyze_function_calls();
        let memory_stats = self.analyze_memory_allocations();

        // Function call statistics
        report.push_str("## Function Call Statistics\n\n");
        report.push_str(
            "| Component | Operation | Calls | Total (ms) | Avg (ms) | P95 (ms) | P99 (ms) |\n",
        );
        report.push_str(
            "|-----------|-----------|-------|------------|----------|----------|----------|\n",
        );

        for stat in &function_stats {
            report.push_str(&format!(
                "| {} | {} | {} | {:.2} | {:.2} | {:.2} | {:.2} |\n",
                stat.component,
                stat.operation,
                stat.call_count,
                stat.total_duration_ns as f64 / 1_000_000.0,
                stat.avg_duration_ns as f64 / 1_000_000.0,
                stat.p95_duration_ns as f64 / 1_000_000.0,
                stat.p99_duration_ns as f64 / 1_000_000.0
            ));
        }

        // Memory allocation statistics
        report.push_str("\n## Memory Allocation Statistics\n\n");
        report.push_str(
            "| Component | Operation | Allocations | Total (MB) | Peak (MB) | Avg (KB) |\n",
        );
        report.push_str(
            "|-----------|-----------|-------------|------------|-----------|----------|\n",
        );

        for stat in &memory_stats {
            report.push_str(&format!(
                "| {} | {} | {} | {:.2} | {:.2} | {:.2} |\n",
                stat.component,
                stat.operation,
                stat.allocation_count,
                stat.total_allocated_bytes as f64 / (1024.0 * 1024.0),
                stat.peak_allocated_bytes as f64 / (1024.0 * 1024.0),
                stat.avg_allocation_bytes as f64 / 1024.0
            ));
        }

        // Summary
        report.push_str("\n## Summary\n\n");
        report.push_str(&format!("- Total Events: {}\n", self.get_events().len()));
        report.push_str(&format!("- Unique Functions: {}\n", function_stats.len()));
        report.push_str(&format!("- Memory Operations: {}\n", memory_stats.len()));

        report
    }

    fn get_timestamp_ns() -> u64 {
        use std::time::{SystemTime, UNIX_EPOCH};
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos() as u64
    }

    fn get_memory_usage() -> usize {
        // Simplified - in real implementation would use platform-specific APIs
        0
    }
}

impl Default for PerformanceProfiler {
    fn default() -> Self {
        Self::new()
    }
}

// Additional methods for named profiling
impl PerformanceProfiler {
    /// Start a named profile
    pub fn start(&mut self, name: &str) -> ProfileGuard {
        if !self.is_enabled() {
            return ProfileGuard::new(name, None);
        }
        ProfileGuard::new(name, Some(Arc::new(Mutex::new(self.clone()))))
    }

    /// Record a profile duration
    pub fn record_profile(&mut self, name: &str, duration: Duration) {
        // Add event for this profile
        let event = ProfileEvent {
            event_type: ProfileEventType::FunctionCall,
            component: "profile".to_string(),
            operation: name.to_string(),
            start_time: 0,
            end_time: 0,
            duration_ns: duration.as_nanos() as u64,
            thread_id: self.thread_id,
            metadata: HashMap::new(),
        };

        let mut events = self.events.lock().unwrap();
        events.push(event);
    }

    /// Get profile by name (aggregates events with matching operation)
    pub fn get(&self, name: &str) -> Option<Profile> {
        let events = self.events.lock().unwrap();
        let matching: Vec<_> = events.iter().filter(|e| e.operation == name).collect();

        if matching.is_empty() {
            return None;
        }

        let call_count = matching.len() as u64;
        let total_duration_ns: u64 = matching.iter().map(|e| e.duration_ns).sum();
        let durations: Vec<u64> = matching.iter().map(|e| e.duration_ns).collect();

        Some(Profile {
            name: name.to_string(),
            call_count,
            total_duration_ms: total_duration_ns / 1_000_000,
            avg_duration_ms: (total_duration_ns as f64 / call_count as f64) / 1_000_000.0,
            min_duration_ms: durations.iter().min().copied().unwrap_or(0) / 1_000_000,
            max_duration_ms: durations.iter().max().copied().unwrap_or(0) / 1_000_000,
            last_call: matching.last().map(|e| e.end_time),
        })
    }

    /// Get all profiles
    pub fn get_all(&self) -> HashMap<String, Profile> {
        let events = self.events.lock().unwrap();
        let mut profiles: HashMap<String, Profile> = HashMap::new();

        for event in events.iter() {
            let entry = profiles.entry(event.operation.clone()).or_insert(Profile {
                name: event.operation.clone(),
                call_count: 0,
                total_duration_ms: 0,
                avg_duration_ms: 0.0,
                min_duration_ms: u64::MAX,
                max_duration_ms: 0,
                last_call: None,
            });

            let duration_ms = event.duration_ns / 1_000_000;
            entry.call_count += 1;
            entry.total_duration_ms += duration_ms;
            entry.min_duration_ms = entry.min_duration_ms.min(duration_ms);
            entry.max_duration_ms = entry.max_duration_ms.max(duration_ms);
            entry.last_call = Some(event.end_time);
        }

        // Calculate averages
        for profile in profiles.values_mut() {
            if profile.call_count > 0 {
                profile.avg_duration_ms =
                    profile.total_duration_ms as f64 / profile.call_count as f64;
            }
        }

        profiles
    }
}

impl Clone for PerformanceProfiler {
    fn clone(&self) -> Self {
        Self {
            events: Arc::clone(&self.events),
            enabled: Arc::clone(&self.enabled),
            thread_id: self.thread_id,
        }
    }
}

/// Global profiler instance
lazy_static::lazy_static! {
    static ref GLOBAL_PROFILER: PerformanceProfiler = PerformanceProfiler::new();
}

/// Convenience function to profile a function call
pub fn profile<F, R>(component: &str, operation: &str, f: F) -> R
where
    F: FnOnce() -> R,
{
    GLOBAL_PROFILER.profile_function(component, operation, f)
}

/// Convenience function to profile memory allocation
pub fn profile_memory<F, R>(component: &str, operation: &str, f: F) -> R
where
    F: FnOnce() -> R,
{
    GLOBAL_PROFILER.profile_memory(component, operation, f)
}

/// Enable global profiling
pub fn enable_profiling() {
    GLOBAL_PROFILER.enable();
}

/// Disable global profiling
pub fn disable_profiling() {
    GLOBAL_PROFILER.disable();
}

/// Get profiling report
pub fn get_profiling_report() -> String {
    GLOBAL_PROFILER.generate_report()
}

/// Profile data for named profiling
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Profile {
    pub name: String,
    pub call_count: u64,
    pub total_duration_ms: u64,
    pub avg_duration_ms: f64,
    pub min_duration_ms: u64,
    pub max_duration_ms: u64,
    pub last_call: Option<u64>,
}

/// Profile Guard for automatic timing
pub struct ProfileGuard {
    name: String,
    profiler: Option<Arc<Mutex<PerformanceProfiler>>>,
    start: Instant,
}

impl ProfileGuard {
    pub fn new(name: &str, profiler: Option<Arc<Mutex<PerformanceProfiler>>>) -> Self {
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
            if let Ok(mut p) = profiler.try_lock() {
                p.record_profile(&self.name, duration);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_profiler_function_call() {
        let profiler = PerformanceProfiler::new();
        profiler.enable();

        let result = profiler.profile_function("Test", "Operation", || {
            std::thread::sleep(Duration::from_millis(10));
            42
        });

        assert_eq!(result, 42);

        let events = profiler.get_events();
        assert!(events.len() > 0);

        let function_stats = profiler.analyze_function_calls();
        assert!(function_stats.len() > 0);
    }

    #[test]
    fn test_profiler_memory() {
        let profiler = PerformanceProfiler::new();
        profiler.enable();

        profiler.profile_memory("Test", "Allocation", || {
            let _data = vec![0u8; 1024 * 1024]; // 1MB
        });

        let memory_stats = profiler.analyze_memory_allocations();
        assert!(memory_stats.len() > 0);
    }

    #[test]
    fn test_profiler_report() {
        let profiler = PerformanceProfiler::new();
        profiler.enable();

        profiler.profile_function("Test", "Operation1", || {
            std::thread::sleep(Duration::from_millis(5));
        });

        profiler.profile_function("Test", "Operation2", || {
            std::thread::sleep(Duration::from_millis(10));
        });

        let report = profiler.generate_report();
        assert!(report.contains("Function Call Statistics"));
        assert!(report.contains("Test"));
    }

    #[test]
    fn test_global_profiler() {
        enable_profiling();

        let result = profile("Global", "Test", || {
            std::thread::sleep(Duration::from_millis(5));
            123
        });

        assert_eq!(result, 123);

        let report = get_profiling_report();
        assert!(report.contains("Global"));

        disable_profiling();
    }
}
