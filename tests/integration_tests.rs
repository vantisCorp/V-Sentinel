//! V-Sentinel Integration Tests
//! 
//! Comprehensive integration tests for all modules

use std::time::Duration;
use tokio::time::sleep;

// ============================================================================
// Quantum Module Integration Tests
// ============================================================================

mod quantum_tests {
    use super::*;
    
    /// Test quantum key generation and exchange
    #[tokio::test]
    async fn test_quantum_key_exchange() {
        // Initialize quantum manager for Alice
        let alice_config = sentinel_quantum::QuantumConfig {
            default_algorithm: sentinel_quantum::KeyType::Kyber1024,
            enable_hybrid_mode: true,
            key_rotation_days: 90,
        };
        let alice_manager = sentinel_quantum::QuantumManager::new(alice_config)
            .expect("Failed to create Alice's quantum manager");
        
        // Initialize quantum manager for Bob
        let bob_config = sentinel_quantum::QuantumConfig {
            default_algorithm: sentinel_quantum::KeyType::Kyber1024,
            enable_hybrid_mode: true,
            key_rotation_days: 90,
        };
        let bob_manager = sentinel_quantum::QuantumManager::new(bob_config)
            .expect("Failed to create Bob's quantum manager");
        
        // Alice generates keypair
        let alice_keypair = alice_manager
            .generate_keypair(sentinel_quantum::KeyType::Kyber1024)
            .await
            .expect("Failed to generate Alice's keypair");
        
        // Bob generates keypair
        let bob_keypair = bob_manager
            .generate_keypair(sentinel_quantum::KeyType::Kyber1024)
            .await
            .expect("Failed to generate Bob's keypair");
        
        // Alice encrypts message for Bob
        let message = b"Hello, Quantum World!";
        let ciphertext = alice_manager
            .encrypt(message, &bob_keypair.public_key)
            .await
            .expect("Failed to encrypt message");
        
        // Bob decrypts message
        let plaintext = bob_manager
            .decrypt(&ciphertext, &bob_keypair.private_key)
            .await
            .expect("Failed to decrypt message");
        
        assert_eq!(message.to_vec(), plaintext, "Decrypted message should match original");
    }
    
    /// Test quantum signatures
    #[tokio::test]
    async fn test_dilithium_signatures() {
        let config = sentinel_quantum::QuantumConfig::default();
        let manager = sentinel_quantum::QuantumManager::new(config)
            .expect("Failed to create quantum manager");
        
        let keypair = manager
            .generate_keypair(sentinel_quantum::KeyType::Dilithium5)
            .await
            .expect("Failed to generate Dilithium keypair");
        
        let message = b"Important security message";
        
        // Sign
        let signature = manager
            .sign(message, &keypair.private_key)
            .await
            .expect("Failed to sign message");
        
        // Verify
        let valid = manager
            .verify(message, &signature, &keypair.public_key)
            .await
            .expect("Failed to verify signature");
        
        assert!(valid, "Signature should be valid");
        
        // Test with modified message
        let modified_message = b"Modified security message";
        let invalid = manager
            .verify(modified_message, &signature, &keypair.public_key)
            .await
            .expect("Failed to verify signature");
        
        assert!(!invalid, "Modified message should fail verification");
    }
    
    /// Test hybrid encryption mode
    #[tokio::test]
    async fn test_hybrid_encryption() {
        let config = sentinel_quantum::QuantumConfig {
            enable_hybrid_mode: true,
            ..Default::default()
        };
        let manager = sentinel_quantum::QuantumManager::new(config)
            .expect("Failed to create quantum manager");
        
        let keypair = manager
            .generate_keypair(sentinel_quantum::KeyType::Kyber768)
            .await
            .expect("Failed to generate hybrid keypair");
        
        let data = vec![0u8; 1024]; // 1KB of data
        
        let ciphertext = manager
            .encrypt(&data, &keypair.public_key)
            .await
            .expect("Failed to encrypt with hybrid mode");
        
        let plaintext = manager
            .decrypt(&ciphertext, &keypair.private_key)
            .await
            .expect("Failed to decrypt with hybrid mode");
        
        assert_eq!(data, plaintext, "Hybrid decryption should match original");
    }
}

// ============================================================================
// Privacy Module Integration Tests
// ============================================================================

mod privacy_tests {
    use super::*;
    
    /// Test zero-knowledge proof generation and verification
    #[tokio::test]
    async fn test_zero_knowledge_proofs() {
        let manager = sentinel_privacy::PrivacyManager::new(Default::default())
            .expect("Failed to create privacy manager");
        
        let secret = b"secret_value";
        let public_inputs = vec![1, 2, 3, 4];
        
        // Generate proof
        let proof = manager
            .generate_zk_proof(
                sentinel_privacy::ZkProofType::Bulletproof,
                secret,
                &public_inputs
            )
            .await
            .expect("Failed to generate ZK proof");
        
        // Verify proof
        let valid = manager
            .verify_zk_proof(&proof)
            .await
            .expect("Failed to verify ZK proof");
        
        assert!(valid, "ZK proof should be valid");
    }
    
    /// Test differential privacy
    #[tokio::test]
    async fn test_differential_privacy() {
        let manager = sentinel_privacy::PrivacyManager::new(Default::default())
            .expect("Failed to create privacy manager");
        
        let raw_data = vec![100.0, 200.0, 150.0, 180.0, 220.0];
        
        let private_data = manager
            .apply_differential_privacy(&raw_data, 1.0, 1e-5)
            .await
            .expect("Failed to apply differential privacy");
        
        // Private data should be different but statistically similar
        assert_ne!(raw_data, private_data.data, "Data should be perturbed");
        
        // Check epsilon is set correctly
        assert_eq!(private_data.epsilon, 1.0, "Epsilon should match");
    }
    
    /// Test homomorphic encryption
    #[tokio::test]
    async fn test_homomorphic_encryption() {
        let manager = sentinel_privacy::PrivacyManager::new(Default::default())
            .expect("Failed to create privacy manager");
        
        let a = 10;
        let b = 20;
        
        // Encrypt both values
        let encrypted_a = manager
            .homomorphic_encrypt(&a.to_le_bytes())
            .await
            .expect("Failed to encrypt a");
        
        let encrypted_b = manager
            .homomorphic_encrypt(&b.to_le_bytes())
            .await
            .expect("Failed to encrypt b");
        
        // Perform addition on encrypted data
        let encrypted_sum = manager
            .homomorphic_add(&encrypted_a, &encrypted_b)
            .await
            .expect("Failed to add encrypted values");
        
        // Decrypt result
        let sum_bytes = manager
            .homomorphic_decrypt(&encrypted_sum)
            .await
            .expect("Failed to decrypt sum");
        
        let sum = i32::from_le_bytes(
            sum_bytes[..4].try_into().expect("Invalid bytes")
        );
        
        assert_eq!(a + b, sum, "Homomorphic addition should work correctly");
    }
}

// ============================================================================
// Cross-Module Integration Tests
// ============================================================================

mod cross_module_tests {
    use super::*;
    
    /// Test quantum + privacy integration
    #[tokio::test]
    async fn test_quantum_privacy_integration() {
        // Create quantum manager
        let quantum_manager = sentinel_quantum::QuantumManager::new(Default::default())
            .expect("Failed to create quantum manager");
        
        // Create privacy manager
        let privacy_manager = sentinel_privacy::PrivacyManager::new(Default::default())
            .expect("Failed to create privacy manager");
        
        // Generate quantum keys
        let keypair = quantum_manager
            .generate_keypair(sentinel_quantum::KeyType::Kyber1024)
            .await
            .expect("Failed to generate keypair");
        
        // Generate ZK proof of key possession without revealing key
        let proof = privacy_manager
            .generate_zk_proof(
                sentinel_privacy::ZkProofType::Bulletproof,
                &keypair.public_key,
                &vec![]
            )
            .await
            .expect("Failed to generate ZK proof");
        
        // Verify proof
        let valid = privacy_manager
            .verify_zk_proof(&proof)
            .await
            .expect("Failed to verify proof");
        
        assert!(valid, "ZK proof of quantum key should be valid");
    }
}

// ============================================================================
// Performance Tests
// ============================================================================

mod performance_tests {
    use super::*;
    use std::time::Instant;
    
    /// Test quantum encryption performance
    #[tokio::test]
    async fn test_quantum_encryption_performance() {
        let manager = sentinel_quantum::QuantumManager::new(Default::default())
            .expect("Failed to create quantum manager");
        
        let keypair = manager
            .generate_keypair(sentinel_quantum::KeyType::Kyber1024)
            .await
            .expect("Failed to generate keypair");
        
        let data = vec![0u8; 1024]; // 1KB
        
        let start = Instant::now();
        for _ in 0..100 {
            let _ = manager.encrypt(&data, &keypair.public_key).await;
        }
        let duration = start.elapsed();
        
        println!("100 encryptions took {:?}", duration);
        println!("Average: {:?}", duration / 100);
        
        // Should complete 100 encryptions in reasonable time
        assert!(duration.as_secs() < 10, "Encryption should be fast");
    }
}

// ============================================================================
// Stress Tests
// ============================================================================

mod stress_tests {
    use super::*;
    
    /// Test concurrent sessions
    #[tokio::test(flavor = "multi_thread", worker_threads = 4)]
    async fn test_concurrent_sessions() {
        let manager = std::sync::Arc::new(
            tokio::sync::Mutex::new(
                sentinel_metaverse::MetaverseManager::new(Default::default())
                    .expect("Failed to create manager")
            )
        );
        
        let mut handles = vec![];
        
        for i in 0..50 {
            let manager_clone = manager.clone();
            let handle = tokio::spawn(async move {
                let mut mgr = manager_clone.lock().await;
                let user_id = uuid::Uuid::new_v4();
                let avatar_id = uuid::Uuid::new_v4();
                
                let session = mgr
                    .start_vr_session(
                        user_id,
                        avatar_id,
                        format!("world_{}", i),
                        sentinel_metaverse::VrSecurityLevel::Standard
                    )
                    .await
                    .expect("Failed to start session");
                
                // Simulate some work
                tokio::time::sleep(tokio::time::Duration::from_millis(10)).await;
                
                mgr.end_vr_session(session)
                    .await
                    .expect("Failed to end session");
            });
            handles.push(handle);
        }
        
        // Wait for all sessions to complete
        for handle in handles {
            handle.await.expect("Session failed");
        }
        
        let mgr = manager.lock().await;
        let stats = mgr.get_statistics();
        assert_eq!(stats.active_vr_sessions, 0, "All sessions should be closed");
    }
}