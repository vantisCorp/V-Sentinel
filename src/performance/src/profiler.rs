// Performance Profiling Tools for SENTINEL Security System
// Provides detailed performance profiling and analysis capabilities

use std::collections::{HashMap, BTreeMap};
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};
use std::thread;
use serde::{Deserialize, Serialize};

/// Profiling event type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ProfileEventType {
    FunctionCall,
    MemoryAllocation,
    I/OOperation,
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
        // Get current thread ID (simplified)
        thread::current().id().as_u64().get()
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
        metadata.insert("allocated_bytes".to_string(), (memory_after - memory_before).to_string());
        
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
            event_type: ProfileEventType::I/OOperation,
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
        let function_events: Vec<_> = events.iter()
            .filter(|e| e.event_type == ProfileEventType::FunctionCall)
            .collect();
        
        let mut call_map: HashMap<(String, String), Vec<u64>> = HashMap::new();
        
        for event in &function_events {
            let key = (event.component.clone(), event.operation.clone());
            call_map.entry(key).or_insert_with(Vec::new).push(event.duration_ns);
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
        let memory_events: Vec<_> = events.iter()
            .filter(|e| e.event_type == ProfileEventType::MemoryAllocation)
            .collect();
        
        let mut allocation_map: HashMap<(String, String), Vec<u64>> = HashMap::new();
        
        for event in &memory_events {
            let key = (event.component.clone(), event.operation.clone());
            if let Some(allocated_str) = event.metadata.get("allocated_bytes") {
                if let Ok(allocated) = allocated_str.parse::<u64>() {
                    allocation_map.entry(key).or_insert_with(Vec::new).push(allocated);
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
        let mut call_stack: Vec<(String, String, u64)> = Vec::new();
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
        report.push_str("| Component | Operation | Calls | Total (ms) | Avg (ms) | P95 (ms) | P99 (ms) |\n");
        report.push_str("|-----------|-----------|-------|------------|----------|----------|----------|\n");
        
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
        report.push_str("| Component | Operation | Allocations | Total (MB) | Peak (MB) | Avg (KB) |\n");
        report.push_str("|-----------|-----------|-------------|------------|-----------|----------|\n");
        
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