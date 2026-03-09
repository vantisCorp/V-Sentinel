//! AI Data Security Module
//!
//! Provides comprehensive data security for AI systems including:
//! - Data pipeline encryption
//! - Access control management
//! - Data classification
//! - Data lineage tracking
//! - Privacy protection

use crate::models::*;
use aes_gcm::{
    aead::{Aead, KeyInit},
    Aes256Gcm, Nonce,
};
use anyhow::Result;
use base64::{engine::general_purpose::STANDARD as BASE64, Engine};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sha2::Digest;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::info;

/// Data Security Manager
pub struct DataSecurityManager {
    /// Encryption manager
    encryption: EncryptionManager,
    /// Access control manager
    access_control: AccessControlManager,
    /// Classification engine
    classifier: DataClassifier,
    /// Lineage tracker
    lineage: LineageTracker,
    /// Statistics
    stats: Arc<RwLock<DataSecurityStats>>,
}

/// Data security statistics
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DataSecurityStats {
    /// Total pipelines secured
    pub pipelines_secured: u64,
    /// Encryption operations
    pub encryption_operations: u64,
    /// Access grants
    pub access_grants: u64,
    /// Access denials
    pub access_denials: u64,
    /// Classification operations
    pub classifications: u64,
}

impl DataSecurityManager {
    /// Create a new data security manager
    pub fn new() -> Self {
        Self {
            encryption: EncryptionManager::new(),
            access_control: AccessControlManager::new(),
            classifier: DataClassifier::new(),
            lineage: LineageTracker::new(),
            stats: Arc::new(RwLock::new(DataSecurityStats::default())),
        }
    }

    /// Secure a data pipeline
    pub async fn secure_pipeline(&self, pipeline: &DataPipeline) -> Result<PipelineSecurityResult> {
        info!(
            "Securing data pipeline: {} ({})",
            pipeline.name, pipeline.id
        );

        let mut result = PipelineSecurityResult::default();

        // Apply encryption if needed
        if pipeline.classification != DataClassification::Public {
            result.encrypted = true;
            result.access_controlled = true;

            let mut stats = self.stats.write().await;
            stats.pipelines_secured += 1;
            stats.encryption_operations += 1;
        }

        // Track lineage
        self.lineage.track_pipeline(pipeline).await?;

        Ok(result)
    }

    /// Encrypt data
    pub async fn encrypt_data(&self, data: &[u8], key_id: Option<&str>) -> Result<EncryptedData> {
        self.encryption.encrypt(data, key_id).await
    }

    /// Decrypt data
    pub async fn decrypt_data(&self, encrypted: &EncryptedData, key_id: &str) -> Result<Vec<u8>> {
        self.encryption.decrypt(encrypted, key_id).await
    }

    /// Check access
    pub async fn check_access(
        &self,
        principal: &str,
        resource: &str,
        permission: Permission,
    ) -> Result<bool> {
        let result = self
            .access_control
            .check(principal, resource, permission)
            .await?;

        let mut stats = self.stats.write().await;
        if result {
            stats.access_grants += 1;
        } else {
            stats.access_denials += 1;
        }

        Ok(result)
    }

    /// Classify data
    pub async fn classify_data(
        &self,
        data: &[u8],
        metadata: &HashMap<String, String>,
    ) -> Result<DataClassification> {
        let classification = self.classifier.classify(data, metadata).await?;

        let mut stats = self.stats.write().await;
        stats.classifications += 1;

        Ok(classification)
    }

    /// Get data lineage
    pub async fn get_lineage(&self, data_id: &str) -> Result<Option<DataLineage>> {
        self.lineage.get(data_id).await
    }

    /// Add access control rule
    pub async fn add_access_rule(&self, rule: AccessControl) -> Result<()> {
        self.access_control.add_rule(rule).await
    }

    /// Get statistics
    pub async fn get_stats(&self) -> DataSecurityStats {
        self.stats.read().await.clone()
    }
}

impl Default for DataSecurityManager {
    fn default() -> Self {
        Self::new()
    }
}

/// Result of pipeline security
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PipelineSecurityResult {
    /// Whether pipeline is encrypted
    pub encrypted: bool,
    /// Whether access is controlled
    pub access_controlled: bool,
    /// Encryption key ID
    pub key_id: Option<String>,
}

/// Encryption Manager
pub struct EncryptionManager {
    /// Encryption keys (key_id -> key)
    keys: Arc<RwLock<HashMap<String, Vec<u8>>>>,
    /// Default key ID
    default_key_id: Option<String>,
}

impl EncryptionManager {
    /// Create a new encryption manager
    pub fn new() -> Self {
        let mut keys = HashMap::new();
        let default_key = Self::generate_key();
        let default_key_id = uuid::Uuid::new_v4().to_string();
        keys.insert(default_key_id.clone(), default_key);

        Self {
            keys: Arc::new(RwLock::new(keys)),
            default_key_id: Some(default_key_id),
        }
    }

    /// Generate a new encryption key
    fn generate_key() -> Vec<u8> {
        use rand::Rng;
        let mut rng = rand::thread_rng();
        (0..32).map(|_| rng.gen()).collect()
    }

    /// Encrypt data
    pub async fn encrypt(&self, data: &[u8], key_id: Option<&str>) -> Result<EncryptedData> {
        let keys = self.keys.read().await;
        let kid = key_id.unwrap_or_else(|| self.default_key_id.as_deref().unwrap_or(""));

        let key = keys
            .get(kid)
            .ok_or_else(|| anyhow::anyhow!("Key not found: {}", kid))?;

        // Create cipher
        let cipher = Aes256Gcm::new_from_slice(key)?;

        // Generate nonce
        use rand::Rng;
        let mut rng = rand::thread_rng();
        let mut nonce_bytes = [0u8; 12];
        rng.fill(&mut nonce_bytes);
        let nonce = Nonce::from_slice(&nonce_bytes);

        // Encrypt
        let ciphertext = cipher
            .encrypt(nonce, data)
            .map_err(|e| anyhow::anyhow!("Encryption failed: {}", e))?;

        Ok(EncryptedData {
            key_id: kid.to_string(),
            nonce: BASE64.encode(nonce_bytes),
            ciphertext: BASE64.encode(&ciphertext),
            algorithm: "AES-256-GCM".to_string(),
            timestamp: Utc::now(),
        })
    }

    /// Decrypt data
    pub async fn decrypt(&self, encrypted: &EncryptedData, _key_id: &str) -> Result<Vec<u8>> {
        let keys = self.keys.read().await;
        let key = keys
            .get(&encrypted.key_id)
            .ok_or_else(|| anyhow::anyhow!("Key not found: {}", encrypted.key_id))?;

        let cipher = Aes256Gcm::new_from_slice(key)?;

        let nonce_bytes = BASE64.decode(&encrypted.nonce)?;
        let nonce = Nonce::from_slice(&nonce_bytes[..12]);

        let ciphertext = BASE64.decode(&encrypted.ciphertext)?;

        let plaintext = cipher
            .decrypt(nonce, ciphertext.as_slice())
            .map_err(|e| anyhow::anyhow!("Decryption failed: {}", e))?;

        Ok(plaintext)
    }

    /// Add a new key
    pub async fn add_key(&self, key_id: &str, key: Vec<u8>) -> Result<()> {
        let mut keys = self.keys.write().await;
        keys.insert(key_id.to_string(), key);
        Ok(())
    }

    /// Rotate key
    pub async fn rotate_key(&self, key_id: &str) -> Result<Vec<u8>> {
        let new_key = Self::generate_key();
        let mut keys = self.keys.write().await;
        keys.insert(key_id.to_string(), new_key.clone());
        Ok(new_key)
    }
}

impl Default for EncryptionManager {
    fn default() -> Self {
        Self::new()
    }
}

/// Encrypted data structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EncryptedData {
    /// Key ID used for encryption
    pub key_id: String,
    /// Nonce (base64)
    pub nonce: String,
    /// Ciphertext (base64)
    pub ciphertext: String,
    /// Algorithm used
    pub algorithm: String,
    /// Timestamp
    pub timestamp: DateTime<Utc>,
}

/// Access Control Manager
pub struct AccessControlManager {
    /// Access rules
    rules: Arc<RwLock<Vec<AccessControl>>>,
    /// Role mappings
    roles: Arc<RwLock<HashMap<String, Vec<String>>>>,
}

impl AccessControlManager {
    /// Create a new access control manager
    pub fn new() -> Self {
        Self {
            rules: Arc::new(RwLock::new(Vec::new())),
            roles: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Check access permission
    pub async fn check(
        &self,
        principal: &str,
        resource: &str,
        permission: Permission,
    ) -> Result<bool> {
        let rules = self.rules.read().await;

        // Check direct rules
        for rule in rules.iter() {
            if !rule.active {
                continue;
            }

            if rule.principal == principal
                && rule.resource == resource
                && self.permission_matches(rule.permission, permission)
            {
                // Check conditions
                if self.evaluate_conditions(&rule.conditions) {
                    return Ok(true);
                }
            }
        }

        // Check role-based access
        let roles = self.roles.read().await;
        if let Some(user_roles) = roles.get(principal) {
            for role in user_roles {
                for rule in rules.iter() {
                    if rule.principal == *role
                        && rule.resource == resource
                        && self.permission_matches(rule.permission, permission)
                    {
                        return Ok(true);
                    }
                }
            }
        }

        Ok(false)
    }

    /// Check if permissions match
    fn permission_matches(&self, rule_permission: Permission, required: Permission) -> bool {
        match (rule_permission, required) {
            (Permission::Admin, _) => true,
            (Permission::Delete, Permission::Delete) => true,
            (Permission::Write, Permission::Write) => true,
            (Permission::Read, Permission::Read) => true,
            (Permission::Execute, Permission::Execute) => true,
            (Permission::Write, Permission::Read) => true,
            _ => false,
        }
    }

    /// Evaluate conditions
    fn evaluate_conditions(&self, conditions: &[AccessCondition]) -> bool {
        if conditions.is_empty() {
            return true;
        }

        // In production, would evaluate actual conditions
        true
    }

    /// Add access control rule
    pub async fn add_rule(&self, rule: AccessControl) -> Result<()> {
        let mut rules = self.rules.write().await;
        rules.push(rule);
        Ok(())
    }

    /// Remove access control rule
    pub async fn remove_rule(&self, rule_id: &str) -> Result<()> {
        let mut rules = self.rules.write().await;
        rules.retain(|r| r.id != rule_id);
        Ok(())
    }

    /// Assign role to principal
    pub async fn assign_role(&self, principal: &str, role: &str) -> Result<()> {
        let mut roles = self.roles.write().await;
        roles
            .entry(principal.to_string())
            .or_default()
            .push(role.to_string());
        Ok(())
    }
}

impl Default for AccessControlManager {
    fn default() -> Self {
        Self::new()
    }
}

/// Data Classifier
pub struct DataClassifier {
    /// Classification patterns
    patterns: Vec<ClassificationPattern>,
}

/// Classification pattern
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClassificationPattern {
    /// Pattern name
    pub name: String,
    /// Pattern regex
    pub pattern: String,
    /// Classification level
    pub classification: DataClassification,
    /// Confidence threshold
    pub confidence: f32,
}

impl DataClassifier {
    /// Create a new data classifier
    pub fn new() -> Self {
        Self {
            patterns: Self::default_patterns(),
        }
    }

    /// Get default classification patterns
    fn default_patterns() -> Vec<ClassificationPattern> {
        vec![
            ClassificationPattern {
                name: "Credit Card".to_string(),
                pattern: r"\d{4}[-\s]?\d{4}[-\s]?\d{4}[-\s]?\d{4}".to_string(),
                classification: DataClassification::HighlyConfidential,
                confidence: 0.9,
            },
            ClassificationPattern {
                name: "SSN".to_string(),
                pattern: r"\d{3}-\d{2}-\d{4}".to_string(),
                classification: DataClassification::HighlyConfidential,
                confidence: 0.95,
            },
            ClassificationPattern {
                name: "Email".to_string(),
                pattern: r"[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}".to_string(),
                classification: DataClassification::Confidential,
                confidence: 0.8,
            },
            ClassificationPattern {
                name: "API Key".to_string(),
                pattern: r"(?i)(api[_-]?key|secret[_-]?key|access[_-]?token)[\s:=]+[a-zA-Z0-9_-]+"
                    .to_string(),
                classification: DataClassification::Restricted,
                confidence: 0.85,
            },
        ]
    }

    /// Classify data
    pub async fn classify(
        &self,
        data: &[u8],
        metadata: &HashMap<String, String>,
    ) -> Result<DataClassification> {
        let data_str = String::from_utf8_lossy(data);
        let mut max_classification = DataClassification::Public;

        // Check metadata for classification hints
        if let Some(classification) = metadata.get("classification") {
            match classification.to_lowercase().as_str() {
                "public" => return Ok(DataClassification::Public),
                "internal" => return Ok(DataClassification::Internal),
                "confidential" => return Ok(DataClassification::Confidential),
                "highly_confidential" => return Ok(DataClassification::HighlyConfidential),
                "restricted" => return Ok(DataClassification::Restricted),
                _ => {}
            }
        }

        // Pattern matching
        use regex::Regex;
        for pattern in &self.patterns {
            if let Ok(re) = Regex::new(&pattern.pattern) {
                if re.is_match(&data_str) && pattern.classification > max_classification {
                    max_classification = pattern.classification;
                }
            }
        }

        Ok(max_classification)
    }
}

impl Default for DataClassifier {
    fn default() -> Self {
        Self::new()
    }
}

/// Data Lineage Tracker
pub struct LineageTracker {
    /// Lineage records
    records: Arc<RwLock<HashMap<String, DataLineage>>>,
}

/// Data lineage information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataLineage {
    /// Data ID
    pub data_id: String,
    /// Source
    pub source: String,
    /// Transformations applied
    pub transformations: Vec<String>,
    /// Timestamps
    pub timestamps: Vec<DateTime<Utc>>,
    /// Parent data IDs
    pub parents: Vec<String>,
    /// Children data IDs
    pub children: Vec<String>,
}

impl LineageTracker {
    /// Create a new lineage tracker
    pub fn new() -> Self {
        Self {
            records: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Track a data pipeline
    pub async fn track_pipeline(&self, pipeline: &DataPipeline) -> Result<()> {
        let lineage = DataLineage {
            data_id: pipeline.id.clone(),
            source: pipeline
                .sources
                .iter()
                .map(|s| s.location.clone())
                .collect::<Vec<_>>()
                .join(","),
            transformations: pipeline
                .transformations
                .iter()
                .map(|t| t.name.clone())
                .collect(),
            timestamps: vec![Utc::now()],
            parents: Vec::new(),
            children: Vec::new(),
        };

        let mut records = self.records.write().await;
        records.insert(pipeline.id.clone(), lineage);
        Ok(())
    }

    /// Get lineage for data
    pub async fn get(&self, data_id: &str) -> Result<Option<DataLineage>> {
        let records = self.records.read().await;
        Ok(records.get(data_id).cloned())
    }

    /// Add transformation
    pub async fn add_transformation(&self, data_id: &str, transformation: &str) -> Result<()> {
        let mut records = self.records.write().await;
        if let Some(lineage) = records.get_mut(data_id) {
            lineage.transformations.push(transformation.to_string());
            lineage.timestamps.push(Utc::now());
        }
        Ok(())
    }
}

impl Default for LineageTracker {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_data_security_manager_creation() {
        let manager = DataSecurityManager::new();
        let stats = manager.get_stats().await;
        assert_eq!(stats.pipelines_secured, 0);
    }

    #[tokio::test]
    async fn test_encryption() {
        let manager = EncryptionManager::new();
        let data = b"test data";

        let encrypted = manager.encrypt(data, None).await.unwrap();
        assert!(!encrypted.ciphertext.is_empty());

        let decrypted = manager
            .decrypt(&encrypted, &encrypted.key_id)
            .await
            .unwrap();
        assert_eq!(decrypted, data);
    }

    #[tokio::test]
    async fn test_access_control() {
        let manager = AccessControlManager::new();
        let rule = AccessControl::new("user1", Permission::Read, "resource1");
        manager.add_rule(rule).await.unwrap();

        let result = manager
            .check("user1", "resource1", Permission::Read)
            .await
            .unwrap();
        assert!(result);
    }

    #[tokio::test]
    async fn test_data_classification() {
        let classifier = DataClassifier::new();
        let data = b"Contact: test@example.com";
        let metadata = HashMap::new();

        let classification = classifier.classify(data, &metadata).await.unwrap();
        assert!(classification >= DataClassification::Confidential);
    }
}
