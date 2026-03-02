// SENTINEL Hypervisor Example
// This example demonstrates how to use the Ring -1 Hypervisor

use sentinel::core::hypervisor::{Hypervisor, HypervisorConfig, VirtualMachine, VMConfig};
use sentinel::error::{SentinelError, Result};
use tokio::time::{sleep, Duration};

#[tokio::main]
async fn main() -> Result<()> {
    println!("🛡️ SENTINEL Hypervisor Example\n");

    // Step 1: Initialize Hypervisor
    println!("Step 1: Initializing Hypervisor...");
    let mut hypervisor = Hypervisor::new();
    
    let config = HypervisorConfig {
        enable_vmx: true,
        enable_svm: true,
        enable_ept: true,
        enable_vpid: true,
    };
    
    hypervisor.initialize(config).await?;
    println!("✅ Hypervisor initialized successfully\n");

    // Step 2: Create Virtual Machine
    println!("Step 2: Creating Virtual Machine...");
    let vm_config = VMConfig {
        name: "security-vm-001".to_string(),
        vcpus: 2,
        memory_mb: 2048,
        enable_ept: true,
    };
    
    let vm_id = hypervisor.create_vm(vm_config).await?;
    println!("✅ Virtual Machine created with ID: {}\n", vm_id);

    // Step 3: Start Virtual Machine
    println!("Step 3: Starting Virtual Machine...");
    hypervisor.start_vm(vm_id).await?;
    println!("✅ Virtual Machine started\n");

    // Step 4: Monitor VM
    println!("Step 4: Monitoring VM for 5 seconds...");
    for i in 1..=5 {
        sleep(Duration::from_secs(1)).await;
        if let Ok(vm) = hypervisor.get_vm(vm_id).await {
            println!("   VM Status: {}, CPU: {:.1}%, Memory: {} MB", 
                     vm.status, vm.cpu_usage, vm.memory_usage);
        }
    }
    println!();

    // Step 5: Pause VM
    println!("Step 5: Pausing Virtual Machine...");
    hypervisor.pause_vm(vm_id).await?;
    println!("✅ Virtual Machine paused\n");

    // Step 6: Resume VM
    println!("Step 6: Resuming Virtual Machine...");
    hypervisor.resume_vm(vm_id).await?;
    println!("✅ Virtual Machine resumed\n");

    // Step 7: Stop VM
    println!("Step 7: Stopping Virtual Machine...");
    hypervisor.stop_vm(vm_id).await?;
    println!("✅ Virtual Machine stopped\n");

    // Step 8: Cleanup
    println!("Step 8: Cleaning up...");
    hypervisor.cleanup().await?;
    println!("✅ Hypervisor cleaned up\n");

    println!("🎉 Hypervisor example completed successfully!");
    
    Ok(())
}

// Example of VM exit handling
async fn handle_vm_exit(hypervisor: &mut Hypervisor, vm_id: u32) -> Result<()> {
    loop {
        match hypervisor.wait_for_vm_exit(vm_id).await {
            Ok(exit_reason) => {
                println!("VM Exit: {:?}", exit_reason);
                
                match exit_reason {
                    ExitReason::IORequest(port, data) => {
                        // Handle I/O request
                        println!("  I/O Request on port: {}", port);
                        hypervisor.handle_io_request(vm_id, port, data).await?;
                    }
                    ExitReason::MemoryAccess(address, access_type) => {
                        // Handle memory access violation
                        println!("  Memory Access Violation at: 0x{:x}", address);
                        hypervisor.handle_memory_access(vm_id, address, access_type).await?;
                    }
                    ExitReason::Interrupt(vector) => {
                        // Handle interrupt
                        println!("  Interrupt: {}", vector);
                        hypervisor.inject_interrupt(vm_id, vector).await?;
                    }
                    _ => {
                        println!("  Other exit reason");
                    }
                }
            }
            Err(SentinelError::VMStopped) => {
                println!("VM stopped");
                break;
            }
            Err(e) => {
                return Err(e);
            }
        }
    }
    
    Ok(())
}

// Example of memory protection
async fn protect_memory_region(hypervisor: &mut Hypervisor, vm_id: u32) -> Result<()> {
    let start_address = 0x1000;
    let size = 0x1000;
    let flags = MemoryProtectionFlags {
        read: true,
        write: false,
        execute: false,
        guard_page: true,
    };
    
    hypervisor.protect_memory(vm_id, start_address, size, flags).await?;
    println!("Memory region protected: 0x{:x} - 0x{:x}", 
             start_address, start_address + size);
    
    Ok(())
}

// Example: Monitor system activity
async fn monitor_system(hypervisor: &mut Hypervisor) -> Result<()> {
    println!("📊 Monitoring System Activity\n");
    
    loop {
        // Get hypervisor statistics
        let stats = hypervisor.get_statistics().await?;
        
        println!("Hypervisor Statistics:");
        println!("  Active VMs: {}", stats.active_vms);
        println!("  Total VM exits: {}", stats.total_vm_exits);
        println!("  CPU usage: {:.2}%", stats.cpu_usage);
        println!("  Memory usage: {} MB", stats.memory_usage);
        
        // Check for threats
        let threats = hypervisor.detect_threats().await?;
        if !threats.is_empty() {
            println!("\n⚠️  Threats Detected:");
            for threat in threats {
                println!("  - {} (Risk: {:.2})", threat.name, threat.risk_score);
            }
        }
        
        sleep(Duration::from_secs(5)).await;
        
        // Break after some iterations (for example)
        break;
    }
    
    Ok(())
}