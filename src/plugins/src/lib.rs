//! V-Sentinel Plugin System
//!
//! A flexible plugin architecture for third-party integrations.

use std::collections::HashMap;
use std::sync::Arc;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use thiserror::Error;

// ============================================================================
// Error Types
// ============================================================================

#[derive(Error, Debug)]
pub enum PluginError {
    #[error("Plugin not found: {0}")]
    NotFound(String),
    
    #[error("Plugin initialization failed: {0}")]
    InitializationFailed(String),
    
    #[error("Plugin execution failed: {0}")]
    ExecutionFailed(String),
    
    #[error("Invalid plugin configuration: {0}")]
    InvalidConfiguration(String),
    
    #[error("Plugin dependency not met: {0}")]
    DependencyNotMet(String),
    
    #[error("Plugin version mismatch: {0}")]
    VersionMismatch(String),
    
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
}

// ============================================================================
// Plugin Metadata
// ============================================================================

/// Plugin metadata information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PluginMetadata {
    /// Unique plugin identifier
    pub id: String,
    /// Plugin name
    pub name: String,
    /// Plugin version
    pub version: String,
    /// Plugin description
    pub description: String,
    /// Plugin author
    pub author: String,
    /// Plugin homepage URL
    pub homepage: Option<String>,
    /// Plugin repository URL
    pub repository: Option<String>,
    /// Plugin license
    pub license: String,
    /// Required V-Sentinel version
    pub min_sentinel_version: String,
    /// Plugin category
    pub category: PluginCategory,
    /// Plugin tags
    pub tags: Vec<String>,
    /// Plugin dependencies
    pub dependencies: Vec<PluginDependency>,
    /// Plugin permissions required
    pub permissions: Vec<PluginPermission>,
    /// Configuration schema
    pub config_schema: Option<serde_json::Value>,
}

/// Plugin category
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum PluginCategory {
    /// Security module plugin
    Security,
    /// Authentication plugin
    Authentication,
    /// Threat detection plugin
    ThreatDetection,
    /// Reporting plugin
    Reporting,
    /// Integration plugin
    Integration,
    /// Custom plugin
    Custom,
}

/// Plugin dependency
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PluginDependency {
    /// Dependency plugin ID
    pub plugin_id: String,
    /// Minimum version required
    pub min_version: String,
    /// Maximum version supported
    pub max_version: Option<String>,
    /// Whether dependency is optional
    pub optional: bool,
}

/// Plugin permission
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum PluginPermission {
    /// Read security events
    ReadSecurityEvents,
    /// Write security events
    WriteSecurityEvents,
    /// Access network data
    AccessNetworkData,
    /// Access user data
    AccessUserData,
    /// Execute system commands
    ExecuteCommands,
    /// Access configuration
    AccessConfiguration,
    /// Modify configuration
    ModifyConfiguration,
    /// Access cryptographic operations
    AccessCryptography,
    /// External network access
    ExternalNetworkAccess,
    /// File system access
    FileSystemAccess,
}

// ============================================================================
// Plugin Configuration
// ============================================================================

/// Plugin configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PluginConfig {
    /// Plugin ID
    pub plugin_id: String,
    /// Whether plugin is enabled
    pub enabled: bool,
    /// Plugin priority (higher = more important)
    pub priority: i32,
    /// Custom configuration values
    pub settings: HashMap<String, serde_json::Value>,
    /// Execution schedule (cron format)
    pub schedule: Option<String>,
    /// Timeout in seconds
    pub timeout_seconds: u64,
    /// Maximum memory in MB
    pub max_memory_mb: u64,
    /// Maximum CPU percentage
    pub max_cpu_percent: u64,
}

impl Default for PluginConfig {
    fn default() -> Self {
        Self {
            plugin_id: String::new(),
            enabled: true,
            priority: 50,
            settings: HashMap::new(),
            schedule: None,
            timeout_seconds: 60,
            max_memory_mb: 256,
            max_cpu_percent: 50,
        }
    }
}

// ============================================================================
// Plugin Context
// ============================================================================

/// Plugin execution context
#[derive(Debug, Clone)]
pub struct PluginContext {
    /// Request ID for tracing
    pub request_id: String,
    /// User ID if authenticated
    pub user_id: Option<String>,
    /// Session ID if available
    pub session_id: Option<String>,
    /// Additional metadata
    pub metadata: HashMap<String, String>,
    /// Configuration reference
    pub config: Arc<PluginConfig>,
}

impl PluginContext {
    /// Create a new plugin context
    pub fn new(request_id: String) -> Self {
        Self {
            request_id,
            user_id: None,
            session_id: None,
            metadata: HashMap::new(),
            config: Arc::new(PluginConfig::default()),
        }
    }

    /// Add metadata to context
    pub fn with_metadata(mut self, key: String, value: String) -> Self {
        self.metadata.insert(key, value);
        self
    }
}

// ============================================================================
// Plugin Event
// ============================================================================

/// Plugin event types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PluginEvent {
    /// Event ID
    pub id: String,
    /// Event type
    pub event_type: PluginEventType,
    /// Event timestamp
    pub timestamp: chrono::DateTime<chrono::Utc>,
    /// Event source
    pub source: String,
    /// Event data
    pub data: serde_json::Value,
    /// Event severity
    pub severity: EventSeverity,
}

/// Plugin event types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PluginEventType {
    /// Security event
    SecurityEvent,
    /// Threat detected
    ThreatDetected,
    /// Alert triggered
    AlertTriggered,
    /// Configuration changed
    ConfigurationChanged,
    /// User action
    UserAction,
    /// System event
    SystemEvent,
    /// Custom event
    Custom(String),
}

/// Event severity levels
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, PartialOrd)]
pub enum EventSeverity {
    Debug,
    Info,
    Warning,
    Error,
    Critical,
}

// ============================================================================
// Plugin Trait
// ============================================================================

/// Main plugin trait
#[async_trait]
pub trait Plugin: Send + Sync {
    /// Get plugin metadata
    fn metadata(&self) -> &PluginMetadata;
    
    /// Initialize the plugin
    async fn initialize(&mut self, config: PluginConfig) -> Result<(), PluginError>;
    
    /// Shutdown the plugin
    async fn shutdown(&mut self) -> Result<(), PluginError>;
    
    /// Handle an event
    async fn handle_event(&self, event: &PluginEvent, context: &PluginContext) -> Result<(), PluginError>;
    
    /// Execute a command
    async fn execute(&self, command: &str, args: &[serde_json::Value], context: &PluginContext) -> Result<serde_json::Value, PluginError>;
    
    /// Health check
    async fn health_check(&self) -> Result<PluginHealth, PluginError>;
    
    /// Get plugin statistics
    fn get_stats(&self) -> PluginStats;
}

/// Plugin health status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PluginHealth {
    /// Whether plugin is healthy
    pub healthy: bool,
    /// Health message
    pub message: String,
    /// Additional health details
    pub details: HashMap<String, String>,
    /// Last check timestamp
    pub last_check: chrono::DateTime<chrono::Utc>,
}

/// Plugin execution statistics
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PluginStats {
    /// Total events processed
    pub events_processed: u64,
    /// Total commands executed
    pub commands_executed: u64,
    /// Total errors
    pub errors: u64,
    /// Average execution time in ms
    pub avg_execution_time_ms: f64,
    /// Peak memory usage in MB
    pub peak_memory_mb: u64,
    /// Last execution timestamp
    pub last_execution: Option<chrono::DateTime<chrono::Utc>>,
}

// ============================================================================
// Plugin Registry
// ============================================================================

/// Plugin registry for managing plugins
pub struct PluginRegistry {
    plugins: HashMap<String, Box<dyn Plugin>>,
    configs: HashMap<String, PluginConfig>,
}

impl PluginRegistry {
    /// Create a new plugin registry
    pub fn new() -> Self {
        Self {
            plugins: HashMap::new(),
            configs: HashMap::new(),
        }
    }
    
    /// Register a plugin
    pub fn register<P: Plugin + 'static>(&mut self, plugin: P, config: PluginConfig) -> Result<(), PluginError> {
        let metadata = plugin.metadata();
        let plugin_id = metadata.id.clone();
        
        // Validate dependencies
        self.validate_dependencies(&metadata.dependencies)?;
        
        // Store config
        self.configs.insert(plugin_id.clone(), config);
        
        // Store plugin
        self.plugins.insert(plugin_id, Box::new(plugin));
        
        Ok(())
    }
    
    /// Unregister a plugin
    pub fn unregister(&mut self, plugin_id: &str) -> Result<(), PluginError> {
        if self.plugins.remove(plugin_id).is_none() {
            return Err(PluginError::NotFound(plugin_id.to_string()));
        }
        self.configs.remove(plugin_id);
        Ok(())
    }
    
    /// Get a plugin by ID
    pub fn get(&self, plugin_id: &str) -> Option<&dyn Plugin> {
        self.plugins.get(plugin_id).map(|p| p.as_ref())
    }
    
    /// List all registered plugins
    pub fn list(&self) -> Vec<&PluginMetadata> {
        self.plugins.values().map(|p| p.metadata()).collect()
    }
    
    /// Initialize all plugins
    pub async fn initialize_all(&mut self) -> Result<(), PluginError> {
        for (plugin_id, plugin) in &mut self.plugins {
            let config = self.configs.get(plugin_id).cloned().unwrap_or_default();
            plugin.initialize(config).await?;
        }
        Ok(())
    }
    
    /// Shutdown all plugins
    pub async fn shutdown_all(&mut self) -> Result<(), PluginError> {
        for plugin in self.plugins.values_mut() {
            plugin.shutdown().await?;
        }
        Ok(())
    }
    
    /// Validate plugin dependencies
    fn validate_dependencies(&self, dependencies: &[PluginDependency]) -> Result<(), PluginError> {
        for dep in dependencies {
            if !dep.optional && !self.plugins.contains_key(&dep.plugin_id) {
                return Err(PluginError::DependencyNotMet(format!(
                    "Required dependency '{}' not found",
                    dep.plugin_id
                )));
            }
        }
        Ok(())
    }
}

impl Default for PluginRegistry {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// Plugin Manager
// ============================================================================

/// Plugin manager for orchestrating plugin lifecycle
pub struct PluginManager {
    registry: Arc<tokio::sync::RwLock<PluginRegistry>>,
    event_tx: tokio::sync::broadcast::Sender<PluginEvent>,
}

impl PluginManager {
    /// Create a new plugin manager
    pub fn new() -> Self {
        let (event_tx, _) = tokio::sync::broadcast::channel(1000);
        Self {
            registry: Arc::new(tokio::sync::RwLock::new(PluginRegistry::new())),
            event_tx,
        }
    }
    
    /// Register a plugin
    pub async fn register<P: Plugin + 'static>(&self, plugin: P, config: PluginConfig) -> Result<(), PluginError> {
        let mut registry = self.registry.write().await;
        registry.register(plugin, config)
    }
    
    /// Dispatch an event to all plugins
    pub async fn dispatch_event(&self, event: &PluginEvent, context: &PluginContext) -> Result<(), PluginError> {
        // Broadcast event
        let _ = self.event_tx.send(event.clone());
        
        // Process with plugins
        let registry = self.registry.read().await;
        for plugin in registry.plugins.values() {
            if let Err(e) = plugin.handle_event(event, context).await {
                tracing::warn!("Plugin event handling failed: {}", e);
            }
        }
        
        Ok(())
    }
    
    /// Execute a command on a specific plugin
    pub async fn execute_command(&self, plugin_id: &str, command: &str, args: &[serde_json::Value], context: &PluginContext) -> Result<serde_json::Value, PluginError> {
        let registry = self.registry.read().await;
        let plugin = registry.plugins.get(plugin_id).ok_or_else(|| PluginError::NotFound(plugin_id.to_string()))?;
        plugin.execute(command, args, context).await
    }
    
    /// Get plugin health
    pub async fn get_health(&self, plugin_id: &str) -> Result<PluginHealth, PluginError> {
        let registry = self.registry.read().await;
        let plugin = registry.plugins.get(plugin_id).ok_or_else(|| PluginError::NotFound(plugin_id.to_string()))?;
        plugin.health_check().await
    }
    
    /// Get all plugin health statuses
    pub async fn get_all_health(&self) -> HashMap<String, PluginHealth> {
        let registry = self.registry.read().await;
        let mut health = HashMap::new();
        for (id, plugin) in &registry.plugins {
            if let Ok(h) = plugin.health_check().await {
                health.insert(id.clone(), h);
            }
        }
        health
    }
    
    /// Subscribe to events
    pub fn subscribe(&self) -> tokio::sync::broadcast::Receiver<PluginEvent> {
        self.event_tx.subscribe()
    }
}

impl Default for PluginManager {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// Example Plugin Implementation
// ============================================================================

/// Example plugin for demonstration
pub struct ExamplePlugin {
    metadata: PluginMetadata,
    config: Option<PluginConfig>,
    stats: PluginStats,
}

impl ExamplePlugin {
    /// Create a new example plugin
    pub fn new() -> Self {
        Self {
            metadata: PluginMetadata {
                id: "example-plugin".to_string(),
                name: "Example Plugin".to_string(),
                version: "1.0.0".to_string(),
                description: "An example plugin demonstrating the plugin system".to_string(),
                author: "V-Sentinel Team".to_string(),
                homepage: None,
                repository: None,
                license: "MIT".to_string(),
                min_sentinel_version: "1.0.0".to_string(),
                category: PluginCategory::Custom,
                tags: vec!["example".to_string(), "demo".to_string()],
                dependencies: vec![],
                permissions: vec![PluginPermission::ReadSecurityEvents],
                config_schema: None,
            },
            config: None,
            stats: PluginStats::default(),
        }
    }
}

#[async_trait]
impl Plugin for ExamplePlugin {
    fn metadata(&self) -> &PluginMetadata {
        &self.metadata
    }
    
    async fn initialize(&mut self, config: PluginConfig) -> Result<(), PluginError> {
        self.config = Some(config);
        tracing::info!("Example plugin initialized");
        Ok(())
    }
    
    async fn shutdown(&mut self) -> Result<(), PluginError> {
        tracing::info!("Example plugin shutting down");
        Ok(())
    }
    
    async fn handle_event(&self, event: &PluginEvent, _context: &PluginContext) -> Result<(), PluginError> {
        tracing::debug!("Example plugin handling event: {:?}", event.event_type);
        Ok(())
    }
    
    async fn execute(&self, command: &str, args: &[serde_json::Value], _context: &PluginContext) -> Result<serde_json::Value, PluginError> {
        match command {
            "ping" => Ok(serde_json::json!({"pong": true})),
            "echo" => Ok(serde_json::json!({"echo": args})),
            _ => Err(PluginError::ExecutionFailed(format!("Unknown command: {}", command))),
        }
    }
    
    async fn health_check(&self) -> Result<PluginHealth, PluginError> {
        Ok(PluginHealth {
            healthy: true,
            message: "Plugin is healthy".to_string(),
            details: HashMap::new(),
            last_check: chrono::Utc::now(),
        })
    }
    
    fn get_stats(&self) -> PluginStats {
        self.stats.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_plugin_registry() {
        let mut registry = PluginRegistry::new();
        let plugin = ExamplePlugin::new();
        let config = PluginConfig::default();
        
        assert!(registry.register(plugin, config).is_ok());
        assert_eq!(registry.list().len(), 1);
    }

    #[tokio::test]
    async fn test_plugin_manager() {
        let manager = PluginManager::new();
        let plugin = ExamplePlugin::new();
        let config = PluginConfig {
            plugin_id: "example-plugin".to_string(),
            ..Default::default()
        };
        
        assert!(manager.register(plugin, config).await.is_ok());
        
        let health = manager.get_health("example-plugin").await;
        assert!(health.is_ok());
        assert!(health.unwrap().healthy);
    }
}