//! Integration Tests for SENTINEL Core Module
//!
//! These tests verify that the core components work together correctly.

use sentinel_core::hypervisor::{Hypervisor, VMState, VMExitReason};
use sentinel_core::memory::MemoryManager;
use sentinel_ai::PredictionEngine;
use sentinel_gaming::{TrustedHandshake, AntiDdosShield, GameInfo, AntiCheatSystem};
use sentinel_quantum::QuantumCryptoManager;

#[tokio::test]
async fn test_hypervisor_memory_integration() {
    // Test that hypervisor and memory manager work together
    let hypervisor = Hypervisor::new().unwrap();
    hypervisor.initialize().await.unwrap();
    
    let memory_manager = MemoryManager::new().unwrap();
    memory_manager.initialize().await.unwrap();
    
    // Create VM and protect memory region
    let vm_id = hypervisor.create_vm("test_vm".to_string()).await.unwrap().id();
    hypervisor.start_vm(vm_id).await.unwrap();
    
    memory_manager.protect_region(0x1000, 0x1000, sentinel_core::memory::MemoryProtection::ReadWrite).await.unwrap();
    
    // Verify both components are working
    assert_eq!(hypervisor.get_vm_state(vm_id).await.unwrap(), VMState::Running);
    assert_eq!(memory_manager.protected_region_count().await, 1);
}

#[tokio::test]
async fn test_hypervisor_ai_integration() {
    // Test that hypervisor can provide data to AI engine
    let hypervisor = Hypervisor::new().unwrap();
    hypervisor.initialize().await.unwrap();
    
    let ai_engine = PredictionEngine::new().unwrap();
    ai_engine.initialize().await.unwrap();
    ai_engine.load_model("model.pt").await.unwrap();
    
    // Create VM and simulate threat detection
    let vm_id = hypervisor.create_vm("test_vm".to_string()).await.unwrap().id();
    hypervisor.start_vm(vm_id).await.unwrap();
    
    // Simulate VM exit (potential threat)
    hypervisor.handle_vm_exit(vm_id, VMExitReason::EptViolation).await.unwrap();
    
    // Use AI to analyze threat
    let features = sentinel_ai::ThreatFeatures {
        suspicious_api_calls: 15,
        file_modifications: 5,
        registry_changes: 3,
        network_connections: 2,
        execution_time_ms: 1000,
        memory_usage: 1024 * 1024,
        cpu_usage: 50.0,
        child_processes: 2,
        is_signed: false,
        has_good_reputation: false,
    };
    
    let prediction = ai_engine.predict(&features).await.unwrap();
    assert!(prediction.is_malicious);
}

#[tokio::test]
async fn test_gaming_components_integration() {
    // Test that gaming components work together
    let handshake = TrustedHandshake::new().unwrap();
    handshake.initialize().await.unwrap();
    
    let ddos_shield = AntiDdosShield::new().unwrap();
    ddos_shield.initialize().await.unwrap();
    ddos_shield.start_protection().await.unwrap();
    
    // Detect game and initiate handshake
    let game_info = GameInfo {
        name: "Test Game".to_string(),
        process_id: 1234,
        executable_path: "/path/to/game.exe".to_string(),
        anti_cheat_system: AntiCheatSystem::Vanguard,
        is_detected: true,
    };
    
    let session_id = handshake.initiate_handshake(&game_info).await.unwrap();
    handshake.complete_handshake(&session_id).await.unwrap();
    handshake.activate_zero_scan_mode(&session_id).await.unwrap();
    
    // Monitor network traffic
    let packet = sentinel_gaming::NetworkPacket {
        source_ip: "192.168.1.1".to_string(),
        destination_ip: "192.168.1.2".to_string(),
        source_port: 12345,
        destination_port: 80,
        protocol: "TCP".to_string(),
        size: 1024,
        payload: vec![0u8; 1024],
    };
    
    let analysis = ddos_shield.monitor_traffic(&packet).await.unwrap();
    assert!(!analysis.source_ip.is_empty());
}

#[tokio::test]
async fn test_quantum_crypto_integration() {
    // Test that quantum crypto works with other components
    let quantum_manager = QuantumCryptoManager::new().unwrap();
    quantum_manager.initialize().await.unwrap();
    
    // Generate key pair
    let keypair = quantum_manager.generate_kem_keypair(
        sentinel_quantum::KemAlgorithm::CrystalsKyber768
    ).await.unwrap();
    
    // Encapsulate key
    let encapsulated = quantum_manager.encapsulate(
        &keypair.public_key,
        sentinel_quantum::KemAlgorithm::CrystalsKyber768
    ).await.unwrap();
    
    // Decapsulate key
    let decapsulated = quantum_manager.decapsulate(
        &encapsulated.ciphertext,
        &keypair.private_key,
        sentinel_quantum::KemAlgorithm::CrystalsKyber768
    ).await.unwrap();
    
    // Verify shared secrets match
    assert_eq!(encapsulated.shared_secret, decapsulated);
}

#[tokio::test]
async fn test_full_system_integration() {
    // Test all components working together
    let hypervisor = Hypervisor::new().unwrap();
    hypervisor.initialize().await.unwrap();
    
    let memory_manager = MemoryManager::new().unwrap();
    memory_manager.initialize().await.unwrap();
    
    let ai_engine = PredictionEngine::new().unwrap();
    ai_engine.initialize().await.unwrap();
    ai_engine.load_model("model.pt").await.unwrap();
    
    let quantum_manager = QuantumCryptoManager::new().unwrap();
    quantum_manager.initialize().await.unwrap();
    
    // Create VM and protect memory
    let vm_id = hypervisor.create_vm("secure_vm".to_string()).await.unwrap().id();
    hypervisor.start_vm(vm_id).await.unwrap();
    
    memory_manager.protect_region(0x1000, 0x2000, sentinel_core::memory::MemoryProtection::ReadWrite).await.unwrap();
    
    // Generate quantum keys for secure communication
    let keypair = quantum_manager.generate_kem_keypair(
        sentinel_quantum::KemAlgorithm::CrystalsKyber768
    ).await.unwrap();
    
    // Simulate threat detection and AI analysis
    let features = sentinel_ai::ThreatFeatures {
        suspicious_api_calls: 20,
        file_modifications: 10,
        registry_changes: 5,
        network_connections: 5,
        execution_time_ms: 2000,
        memory_usage: 2048 * 1024,
        cpu_usage: 80.0,
        child_processes: 5,
        is_signed: false,
        has_good_reputation: false,
    };
    
    let prediction = ai_engine.predict(&features).await.unwrap();
    
    // Verify all components are working
    assert_eq!(hypervisor.get_vm_state(vm_id).await.unwrap(), VMState::Running);
    assert_eq!(memory_manager.protected_region_count().await, 1);
    assert!(prediction.is_malicious);
    assert!(!keypair.public_key.is_empty());
}

#[tokio::test]
async fn test_performance_integration() {
    // Test that components meet performance targets
    let hypervisor = Hypervisor::new().unwrap();
    let start = std::time::Instant::now();
    hypervisor.initialize().await.unwrap();
    let init_time = start.elapsed();
    
    // VM creation should be fast
    let start = std::time::Instant::now();
    let vm_id = hypervisor.create_vm("perf_test".to_string()).await.unwrap().id();
    let creation_time = start.elapsed();
    
    // VM start should be fast
    let start = std::time::Instant::now();
    hypervisor.start_vm(vm_id).await.unwrap();
    let start_time = start.elapsed();
    
    // Verify performance targets
    assert!(init_time < std::time::Duration::from_millis(100));
    assert!(creation_time < std::time::Duration::from_millis(10));
    assert!(start_time < std::time::Duration::from_millis(10));
}

#[tokio::test]
async fn test_error_handling_integration() {
    // Test error handling across components
    let hypervisor = Hypervisor::new().unwrap();
    
    // Should fail to create VM before initialization
    assert!(hypervisor.create_vm("test".to_string()).await.is_err());
    
    hypervisor.initialize().await.unwrap();
    
    // Should fail to operate on non-existent VM
    assert!(hypervisor.start_vm(999).await.is_err());
    assert!(hypervisor.get_vm_state(999).await.is_err());
    
    // Should fail invalid state transitions
    let vm_id = hypervisor.create_vm("test".to_string()).await.unwrap().id();
    assert!(hypervisor.start_vm(vm_id).await.is_ok());
    assert!(hypervisor.start_vm(vm_id).await.is_err()); // Already running
}