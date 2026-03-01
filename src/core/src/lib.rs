//! SENTINEL Core Security Module
//! 
//! This module provides the core security functionality including:
//! - Ring -1 hypervisor for maximum isolation
//! - Memory protection and monitoring
//! - Process isolation and control
//! - Hardware-level security features

pub mod hypervisor;
pub mod memory;
pub mod process;
pub mod hardware;

pub use hypervisor::Hypervisor;
pub use memory::MemoryManager;
pub use process::ProcessManager;
pub use hardware::HardwareSecurity;

use anyhow::Result;
use tracing::{info, error};

/// SENTINEL Core - Main entry point for core security functionality
pub struct SentinelCore {
    hypervisor: Hypervisor,
    memory_manager: MemoryManager,
    process_manager: ProcessManager,
    hardware_security: HardwareSecurity,
}

impl SentinelCore {
    /// Create a new SENTINEL Core instance
    pub fn new() -> Result<Self> {
        info!("Initializing SENTINEL Core...");
        
        let hypervisor = Hypervisor::new()?;
        let memory_manager = MemoryManager::new()?;
        let process_manager = ProcessManager::new()?;
        let hardware_security = HardwareSecurity::new()?;
        
        info!("SENTINEL Core initialized successfully");
        
        Ok(Self {
            hypervisor,
            memory_manager,
            process_manager,
            hardware_security,
        })
    }
    
    /// Start the core security system
    pub async fn start(&mut self) -> Result<()> {
        info!("Starting SENTINEL Core...");
        
        // Initialize hypervisor
        self.hypervisor.initialize().await?;
        
        // Start memory monitoring
        self.memory_manager.start_monitoring().await?;
        
        // Start process monitoring
        self.process_manager.start_monitoring().await?;
        
        // Initialize hardware security
        self.hardware_security.initialize().await?;
        
        info!("SENTINEL Core started successfully");
        
        Ok(())
    }
    
    /// Stop the core security system
    pub async fn stop(&mut self) -> Result<()> {
        info!("Stopping SENTINEL Core...");
        
        self.hypervisor.shutdown().await?;
        self.memory_manager.stop_monitoring().await?;
        self.process_manager.stop_monitoring().await?;
        self.hardware_security.shutdown().await?;
        
        info!("SENTINEL Core stopped successfully");
        
        Ok(())
    }
    
    /// Get hypervisor reference
    pub fn hypervisor(&self) -> &Hypervisor {
        &self.hypervisor
    }
    
    /// Get memory manager reference
    pub fn memory_manager(&self) -> &MemoryManager {
        &self.memory_manager
    }
    
    /// Get process manager reference
    pub fn process_manager(&self) -> &ProcessManager {
        &self.process_manager
    }
    
    /// Get hardware security reference
    pub fn hardware_security(&self) -> &HardwareSecurity {
        &self.hardware_security
    }
}

impl Default for SentinelCore {
    fn default() -> Self {
        Self::new().expect("Failed to create SentinelCore")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test]
    async fn test_core_initialization() {
        let core = SentinelCore::new().unwrap();
        assert!(core.start().await.is_ok());
        assert!(core.stop().await.is_ok());
    }
    
    #[tokio::test]
    async fn test_core_components() {
        let core = SentinelCore::new().unwrap();
        
        // Verify all components are initialized
        assert!(core.hypervisor().is_initialized());
        assert!(core.memory_manager().is_initialized());
        assert!(core.process_manager().is_initialized());
        assert!(core.hardware_security().is_initialized());
    }
}
