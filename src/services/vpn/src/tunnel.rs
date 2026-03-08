//! PQC Tunnel Module

use anyhow::Result;
use chrono::{DateTime, Utc};
use serde::{Serialize, Deserialize};

/// PQC Tunnel
#[derive(Debug, Clone)]
pub struct PqcTunnel {
    pub tunnel_id: String,
    pub client_id: String,
    pub local_address: String,
    pub remote_address: String,
    pub state: TunnelState,
    pub created_at: DateTime<Utc>,
    pub last_rekeyed: DateTime<Utc>,
    pub bytes_transmitted: u64,
    pub bytes_received: u64,
}

/// Tunnel State
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TunnelState {
    Initializing,
    Established,
    Rekeying,
    Closed,
    Error,
}

impl PqcTunnel {
    /// Create a new tunnel
    pub fn new(tunnel_id: String, client_id: String) -> Self {
        Self {
            tunnel_id,
            client_id,
            local_address: String::new(),
            remote_address: String::new(),
            state: TunnelState::Initializing,
            created_at: Utc::now(),
            last_rekeyed: Utc::now(),
            bytes_transmitted: 0,
            bytes_received: 0,
        }
    }
    
    /// Establish the tunnel
    pub fn establish(&mut self) -> Result<()> {
        self.state = TunnelState::Established;
        Ok(())
    }
    
    /// Close the tunnel
    pub fn close(&mut self) {
        self.state = TunnelState::Closed;
    }
    
    /// Check if tunnel needs rekeying
    pub fn needs_rekey(&self, interval_secs: i64) -> bool {
        let elapsed = Utc::now() - self.last_rekeyed;
        elapsed.num_seconds() >= interval_secs
    }
    
    /// Update traffic statistics
    pub fn update_stats(&mut self, tx_bytes: u64, rx_bytes: u64) {
        self.bytes_transmitted += tx_bytes;
        self.bytes_received += rx_bytes;
    }
}