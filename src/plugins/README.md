# V-Sentinel Plugin SDK

## Overview

The V-Sentinel Plugin SDK allows developers to create third-party plugins that extend the functionality of the V-Sentinel security platform.

## Quick Start

### Add to Cargo.toml

```toml
[dependencies]
sentinel-plugins = "0.1.0"
```

### Create a Plugin

```rust
use sentinel_plugins::*;
use async_trait::async_trait;

pub struct MyPlugin {
    metadata: PluginMetadata,
    config: Option<PluginConfig>,
}

impl MyPlugin {
    pub fn new() -> Self {
        Self {
            metadata: PluginMetadata {
                id: "my-plugin".to_string(),
                name: "My Security Plugin".to_string(),
                version: "1.0.0".to_string(),
                description: "A custom security plugin".to_string(),
                author: "Your Name".to_string(),
                license: "MIT".to_string(),
                min_sentinel_version: "1.0.0".to_string(),
                category: PluginCategory::Security,
                tags: vec!["custom".to_string()],
                dependencies: vec![],
                permissions: vec![PluginPermission::ReadSecurityEvents],
                ..Default::default()
            },
            config: None,
        }
    }
}

#[async_trait]
impl Plugin for MyPlugin {
    fn metadata(&self) -> &PluginMetadata {
        &self.metadata
    }
    
    async fn initialize(&mut self, config: PluginConfig) -> Result<(), PluginError> {
        self.config = Some(config);
        Ok(())
    }
    
    async fn shutdown(&mut self) -> Result<(), PluginError> {
        Ok(())
    }
    
    async fn handle_event(&self, event: &PluginEvent, context: &PluginContext) -> Result<(), PluginError> {
        // Process security events
        match &event.event_type {
            PluginEventType::ThreatDetected => {
                // Handle threat
            }
            _ => {}
        }
        Ok(())
    }
    
    async fn execute(&self, command: &str, args: &[serde_json::Value], context: &PluginContext) -> Result<serde_json::Value, PluginError> {
        match command {
            "scan" => Ok(serde_json::json!({"status": "completed"})),
            _ => Err(PluginError::ExecutionFailed(format!("Unknown command: {}", command))),
        }
    }
    
    async fn health_check(&self) -> Result<PluginHealth, PluginError> {
        Ok(PluginHealth {
            healthy: true,
            message: "Plugin is healthy".to_string(),
            details: Default::default(),
            last_check: chrono::Utc::now(),
        })
    }
    
    fn get_stats(&self) -> PluginStats {
        PluginStats::default()
    }
}
```

## Plugin Lifecycle

1. **Registration** - Plugin is registered with the plugin manager
2. **Initialization** - Plugin is initialized with configuration
3. **Execution** - Plugin handles events and commands
4. **Shutdown** - Plugin is cleanly stopped

## Plugin Permissions

| Permission | Description |
|------------|-------------|
| `ReadSecurityEvents` | Read security events and alerts |
| `WriteSecurityEvents` | Create or modify security events |
| `AccessNetworkData` | Access network traffic data |
| `AccessUserData` | Access user information |
| `ExecuteCommands` | Execute system commands |
| `AccessConfiguration` | Read system configuration |
| `ModifyConfiguration` | Modify system configuration |
| `AccessCryptography` | Use cryptographic operations |
| `ExternalNetworkAccess` | Make external network requests |
| `FileSystemAccess` | Access the file system |

## Event Types

| Event Type | Description |
|------------|-------------|
| `SecurityEvent` | General security event |
| `ThreatDetected` | Threat detection event |
| `AlertTriggered` | Alert triggered event |
| `ConfigurationChanged` | Configuration change event |
| `UserAction` | User action event |
| `SystemEvent` | System event |
| `Custom(String)` | Custom event type |

## Best Practices

1. **Minimal Permissions** - Request only the permissions your plugin needs
2. **Error Handling** - Handle all errors gracefully
3. **Logging** - Use tracing for plugin logs
4. **Configuration** - Make your plugin configurable
5. **Testing** - Write comprehensive tests
6. **Documentation** - Document all commands and events
7. **Performance** - Keep execution time minimal
8. **Security** - Validate all inputs

## Plugin Configuration

Plugins are configured via TOML:

```toml
[plugins.my-plugin]
enabled = true
priority = 50
timeout_seconds = 60
max_memory_mb = 256

[plugins.my-plugin.settings]
custom_option = "value"
another_option = 42
```

## API Reference

See the [API documentation](https://docs.v-sentinel.io/plugin-api) for complete reference.

## License

MIT OR Apache-2.0