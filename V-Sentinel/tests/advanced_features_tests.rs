//! Advanced Features Integration Tests
//!
//! These tests verify that the advanced features work correctly
//! and integrate well with the core components.

use sentinel_behavioral::{BehavioralAnalysisEngine, BehaviorEvent, BehaviorEventType};
use sentinel_threat_intel::{ThreatIntelNetwork, ThreatIntel, ThreatType, ThreatSeverity};
use sentinel_siem::{SiemIntegrationManager, SiemPlatform, SiemConfig, SecurityEvent, SecurityEventType, EventSeverity};
use sentinel_mobile::{MobileSecurityManager, MobilePlatform};
use sentinel_iot::{IotSecurityManager, IotDevice, IotDeviceType};
use sentinel_cloud::{CloudSecurityManager, CloudProvider, CloudProviderConfig, ContainerImage};

#[tokio::test]
async fn test_behavioral_analysis_integration() {
    let engine = BehavioralAnalysisEngine::new().unwrap();
    engine.initialize().await.unwrap();
    engine.start_monitoring().await.unwrap();
    
    // Record multiple behaviors
    for i in 0..15 {
        let behavior = BehaviorEvent {
            event_type: if i % 2 == 0 {
                BehaviorEventType::FileCreated
            } else {
                BehaviorEventType::FileModified
            },
            timestamp: std::time::Instant::now(),
            details: format!("Event {}", i),
        };
        engine.record_behavior(1234, behavior).await.unwrap();
    }
    
    let result = engine.analyze_process(1234).await.unwrap();
    assert!(result.risk_score >= 0.0);
    
    let stats = engine.get_stats().await;
    assert!(stats.analysis_count > 0);
}

#[tokio::test]
async fn test_threat_intelligence_integration() {
    let network = ThreatIntelNetwork::new().unwrap();
    network.initialize().await.unwrap();
    network.start().await.unwrap();
    
    // Share threat
    let threat = ThreatIntel {
        threat_id: "threat_001".to_string(),
        threat_type: ThreatType::Malware,
        severity: ThreatSeverity::High,
        hashes: vec!["hash123".to_string()],
        domains: vec!["malicious.com".to_string()],
        ips: vec!["192.168.1.1".to_string()],
        description: "Test threat".to_string(),
        first_seen: std::time::Instant::now(),
        last_seen: std::time::Instant::now(),
        confidence: 0.9,
        source: "test".to_string(),
    };
    
    assert!(network.share_threat(threat).await.is_ok());
    
    let stats = network.get_stats().await;
    assert!(stats.threats_shared > 0);
}

#[tokio::test]
async fn test_siem_integration() {
    let manager = SiemIntegrationManager::new().unwrap();
    manager.initialize().await.unwrap();
    manager.start().await.unwrap();
    
    let config = SiemConfig {
        endpoint: "https://splunk.example.com:8088".to_string(),
        api_key: Some("test_key".to_string()),
        username: None,
        password: None,
        index: Some("sentinel".to_string()),
        source_type: Some("sentinel:security".to_string()),
        enabled: true,
    };
    
    manager.add_integration(SiemPlatform::Splunk, config).await.unwrap();
    
    let event = SecurityEvent {
        event_id: "event_001".to_string(),
        event_type: SecurityEventType::ThreatDetected,
        severity: EventSeverity::High,
        timestamp: std::time::Instant::now(),
        source: "sentinel".to_string(),
        details: serde_json::json!({"test": "data"}),
    };
    
    assert!(manager.send_event(event).await.is_ok());
    
    let stats = manager.get_stats().await;
    assert!(stats.events_sent > 0);
}

#[tokio::test]
async fn test_mobile_security_integration() {
    let manager = MobileSecurityManager::new(MobilePlatform::Android).unwrap();
    manager.initialize().await.unwrap();
    manager.start().await.unwrap();
    
    // Scan apps
    let results = manager.scan_apps().await.unwrap();
    assert!(!results.is_empty());
    
    // Optimize battery
    let result = manager.optimize_battery().await.unwrap();
    assert!(result.battery_saved_percent > 0.0);
    
    // Monitor network
    let analysis = manager.monitor_network().await.unwrap();
    assert!(analysis.total_connections >= 0);
}

#[tokio::test]
async fn test_iot_security_integration() {
    let manager = IotSecurityManager::new().unwrap();
    manager.initialize().await.unwrap();
    manager.start().await.unwrap();
    
    let device = IotDevice {
        device_id: "device_001".to_string(),
        device_type: IotDeviceType::SmartCamera,
        ip_address: "192.168.1.100".to_string(),
        mac_address: "00:11:22:33:44:55".to_string(),
        firmware_version: "1.0.0".to_string(),
        is_active: true,
        is_secure: true,
        agent_active: true,
        last_scan: None,
        threat_count: 0,
    };
    
    manager.register_device(device).await.unwrap();
    
    let result = manager.scan_device("device_001".to_string()).await.unwrap();
    assert_eq!(result.device_id, "device_001");
}

#[tokio::test]
async fn test_cloud_security_integration() {
    let manager = CloudSecurityManager::new().unwrap();
    manager.initialize().await.unwrap();
    manager.start().await.unwrap();
    
    let config = CloudProviderConfig {
        region: "us-east-1".to_string(),
        access_key: Some("test_key".to_string()),
        secret_key: Some("test_secret".to_string()),
        subscription_id: None,
        project_id: None,
        enabled: true,
    };
    
    manager.add_provider(CloudProvider::AWS, config).await.unwrap();
    
    let image = ContainerImage {
        image_name: "nginx".to_string(),
        image_tag: "latest".to_string(),
        registry: "docker.io".to_string(),
    };
    
    let result = manager.scan_container(image).await.unwrap();
    assert_eq!(result.image_name, "nginx");
}

#[tokio::test]
async fn test_full_advanced_features_integration() {
    // Test all advanced features working together
    
    // Behavioral Analysis
    let behavioral = BehavioralAnalysisEngine::new().unwrap();
    behavioral.initialize().await.unwrap();
    behavioral.start_monitoring().await.unwrap();
    
    // Threat Intelligence
    let threat_intel = ThreatIntelNetwork::new().unwrap();
    threat_intel.initialize().await.unwrap();
    threat_intel.start().await.unwrap();
    
    // SIEM Integration
    let siem = SiemIntegrationManager::new().unwrap();
    siem.initialize().await.unwrap();
    siem.start().await.unwrap();
    
    // Mobile Security
    let mobile = MobileSecurityManager::new(MobilePlatform::IOS).unwrap();
    mobile.initialize().await.unwrap();
    mobile.start().await.unwrap();
    
    // IoT Security
    let iot = IotSecurityManager::new().unwrap();
    iot.initialize().await.unwrap();
    iot.start().await.unwrap();
    
    // Cloud Security
    let cloud = CloudSecurityManager::new().unwrap();
    cloud.initialize().await.unwrap();
    cloud.start().await.unwrap();
    
    // Verify all are active
    assert!(behavioral.get_stats().await.monitoring_active);
    assert!(threat_intel.get_stats().await.network_active);
    assert!(siem.get_stats().await.manager_active);
    assert!(mobile.get_stats().await.manager_active);
    assert!(iot.get_stats().await.manager_active);
    assert!(cloud.get_stats().await.manager_active);
}

#[tokio::test]
async fn test_advanced_features_performance() {
    // Test performance of advanced features
    
    let start = std::time::Instant::now();
    
    // Initialize all features
    let behavioral = BehavioralAnalysisEngine::new().unwrap();
    behavioral.initialize().await.unwrap();
    
    let threat_intel = ThreatIntelNetwork::new().unwrap();
    threat_intel.initialize().await.unwrap();
    
    let siem = SiemIntegrationManager::new().unwrap();
    siem.initialize().await.unwrap();
    
    let mobile = MobileSecurityManager::new(MobilePlatform::Android).unwrap();
    mobile.initialize().await.unwrap();
    
    let iot = IotSecurityManager::new().unwrap();
    iot.initialize().await.unwrap();
    
    let cloud = CloudSecurityManager::new().unwrap();
    cloud.initialize().await.unwrap();
    
    let init_time = start.elapsed();
    
    // All should initialize quickly
    assert!(init_time < std::time::Duration::from_millis(500));
}

#[tokio::test]
async fn test_advanced_features_error_handling() {
    // Test error handling across advanced features
    
    let behavioral = BehavioralAnalysisEngine::new().unwrap();
    
    // Should fail to record behavior before starting monitoring
    let behavior = BehaviorEvent {
        event_type: BehaviorEventType::FileCreated,
        timestamp: std::time::Instant::now(),
        details: "Test".to_string(),
    };
    assert!(behavioral.record_behavior(1234, behavior).await.is_err());
    
    // Should fail to analyze non-existent process
    behavioral.initialize().await.unwrap();
    behavioral.start_monitoring().await.unwrap();
    assert!(behavioral.analyze_process(999).await.is_err());
}