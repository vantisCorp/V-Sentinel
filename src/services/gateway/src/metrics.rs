//! Gateway Metrics Module

use std::sync::atomic::{AtomicU64, Ordering};

/// Gateway Metrics
#[derive(Debug)]
pub struct GatewayMetrics {
    pub total_requests: AtomicU64,
    pub pqc_handshakes: AtomicU64,
    pub classical_fallbacks: AtomicU64,
    pub active_sessions: AtomicU64,
    pub failed_handshakes: AtomicU64,
}

impl Default for GatewayMetrics {
    fn default() -> Self {
        Self::new()
    }
}

impl GatewayMetrics {
    pub fn new() -> Self {
        Self {
            total_requests: AtomicU64::new(0),
            pqc_handshakes: AtomicU64::new(0),
            classical_fallbacks: AtomicU64::new(0),
            active_sessions: AtomicU64::new(0),
            failed_handshakes: AtomicU64::new(0),
        }
    }
    
    pub fn increment_requests(&self) {
        self.total_requests.fetch_add(1, Ordering::Relaxed);
    }
    
    pub fn increment_pqc_handshakes(&self) {
        self.pqc_handshakes.fetch_add(1, Ordering::Relaxed);
    }
    
    pub fn increment_classical_fallbacks(&self) {
        self.classical_fallbacks.fetch_add(1, Ordering::Relaxed);
    }
    
    pub fn set_active_sessions(&self, count: u64) {
        self.active_sessions.store(count, Ordering::Relaxed);
    }
    
    pub fn increment_failed_handshakes(&self) {
        self.failed_handshakes.fetch_add(1, Ordering::Relaxed);
    }
    
    pub fn get_all(&self) -> MetricsSnapshot {
        MetricsSnapshot {
            total_requests: self.total_requests.load(Ordering::Relaxed),
            pqc_handshakes: self.pqc_handshakes.load(Ordering::Relaxed),
            classical_fallbacks: self.classical_fallbacks.load(Ordering::Relaxed),
            active_sessions: self.active_sessions.load(Ordering::Relaxed),
            failed_handshakes: self.failed_handshakes.load(Ordering::Relaxed),
        }
    }
}

/// Metrics snapshot
#[derive(Debug, Clone, serde::Serialize)]
pub struct MetricsSnapshot {
    pub total_requests: u64,
    pub pqc_handshakes: u64,
    pub classical_fallbacks: u64,
    pub active_sessions: u64,
    pub failed_handshakes: u64,
}