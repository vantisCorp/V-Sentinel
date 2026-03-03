# SENTINEL SDK Documentation

## Overview

The SENTINEL SDK provides developers with a comprehensive set of tools and APIs to integrate SENTINEL security capabilities into their applications. This SDK supports multiple programming languages and platforms.

---

## Table of Contents

1. [Getting Started](#getting-started)
2. [Installation](#installation)
3. [Authentication](#authentication)
4. [Core APIs](#core-apis)
5. [Language-Specific SDKs](#language-specific-sdks)
6. [Examples](#examples)
7. [Best Practices](#best-practices)
8. [Error Handling](#error-handling)
9. [Rate Limiting](#rate-limiting)
10. [Support](#support)

---

## Getting Started

### Prerequisites

- SENTINEL API key (obtain from https://sentinel.ai/developers)
- Supported programming language (Python, JavaScript, Go, Rust, Java, C#)
- Internet connection for API calls

### Quick Start

```python
from sentinel_sdk import SentinelClient

# Initialize client
client = SentinelClient(api_key="your-api-key")

# Check system health
health = client.health_check()
print(f"Status: {health['status']}")

# Predict threat
result = client.predict_threat(
    features={
        "process_name": "malicious.exe",
        "file_hash": "abc123...",
        "network_connections": ["192.168.1.1:443"]
    }
)

print(f"Threat level: {result['threat_level']}")
print(f"Confidence: {result['confidence']}")
```

---

## Installation

### Python

```bash
pip install sentinel-sdk
```

### JavaScript/Node.js

```bash
npm install @sentinel/sdk
```

### Go

```bash
go get github.com/vantisCorp/sentinel-go
```

### Rust

```toml
[dependencies]
sentinel-rust = "1.0.0"
```

### Java

```xml
<dependency>
    <groupId>ai.sentinel</groupId>
    <artifactId>sentinel-java</artifactId>
    <version>1.0.0</version>
</dependency>
```

### C#

```bash
dotnet add package Sentinel.SDK
```

---

## Authentication

### API Key Authentication

All API requests require an API key for authentication.

```python
from sentinel_sdk import SentinelClient

client = SentinelClient(api_key="your-api-key")
```

### JWT Authentication

For enterprise customers, JWT authentication is also supported.

```python
from sentinel_sdk import SentinelClient

client = SentinelClient(
    jwt_token="your-jwt-token",
    auth_type="jwt"
)
```

### API Key Management

1. Generate API key from https://sentinel.ai/developers
2. Store API key securely (environment variables, secret management)
3. Rotate API keys regularly
4. Use different keys for different environments

---

## Core APIs

### 1. Health Check

Check the status of the SENTINEL API.

**Python**
```python
health = client.health_check()
# Returns: {"status": "healthy", "version": "1.0.0"}
```

**JavaScript**
```javascript
const health = await client.healthCheck();
// Returns: { status: "healthy", version: "1.0.0" }
```

**Go**
```go
health, err := client.HealthCheck()
// Returns: {Status: "healthy", Version: "1.0.0"}
```

---

### 2. Threat Prediction

Predict whether a file, process, or network connection is malicious.

**Python**
```python
result = client.predict_threat(
    features={
        "process_name": "suspicious.exe",
        "file_hash": "sha256:abc123...",
        "file_size": 1024000,
        "network_connections": ["192.168.1.1:443", "10.0.0.1:80"],
        "registry_keys": ["HKLM\\Software\\Suspicious"]
    }
)

# Returns:
{
    "threat_level": "high",  # low, medium, high, critical
    "confidence": 0.95,
    "threat_type": "malware",
    "recommendations": ["isolate", "quarantine"]
}
```

**JavaScript**
```javascript
const result = await client.predictThreat({
    processName: "suspicious.exe",
    fileHash: "sha256:abc123...",
    fileSize: 1024000,
    networkConnections: ["192.168.1.1:443", "10.0.0.1:80"],
    registryKeys: ["HKLM\\Software\\Suspicious"]
});
```

---

### 3. Batch Threat Prediction

Predict threats for multiple items in a single request.

**Python**
```python
results = client.batch_predict_threat([
    {
        "process_name": "file1.exe",
        "file_hash": "sha256:abc123..."
    },
    {
        "process_name": "file2.exe",
        "file_hash": "sha256:def456..."
    }
])

# Returns array of prediction results
```

---

### 4. File Scanning

Scan files for malware and threats.

**Python**
```python
result = client.scan_file(
    file_path="/path/to/file.exe",
    scan_options={
        "deep_scan": True,
        "heuristics": True,
        "cloud_lookup": True
    }
)

# Returns:
{
    "scan_id": "scan_12345",
    "status": "completed",
    "threats_found": 2,
    "threats": [
        {
            "type": "trojan",
            "name": "Trojan.GenericKD.123",
            "severity": "high"
        }
    ]
}
```

---

### 5. Process Monitoring

Monitor processes in real-time.

**Python**
```python
# Start monitoring
client.start_process_monitoring(
    filters={
        "process_names": ["chrome.exe", "firefox.exe"],
        "monitor_children": True
    },
    callback=lambda event: print(f"Event: {event}")
)

# Stop monitoring
client.stop_process_monitoring()
```

---

### 6. Network Protection

Protect network connections from threats.

**Python**
```python
# Enable network protection
client.enable_network_protection(
    rules={
        "block_malicious_ips": True,
        "block_suspicious_domains": True,
        "ddos_protection": True
    }
)

# Get network events
events = client.get_network_events(limit=100)
```

---

### 7. Quantum Cryptography

Use quantum-resistant encryption.

**Python**
```python
# Generate quantum-resistant key pair
key_pair = client.generate_quantum_key_pair(
    algorithm="crystals-kyber"  # or "crystals-dilithium"
)

# Encrypt data
encrypted = client.quantum_encrypt(
    data="sensitive information",
    public_key=key_pair["public_key"],
    algorithm="crystals-kyber"
)

# Decrypt data
decrypted = client.quantum_decrypt(
    encrypted_data=encrypted,
    private_key=key_pair["private_key"]
)
```

---

### 8. Gaming Mode

Enable gaming mode for optimal performance.

**Python**
```python
# Enable gaming mode
client.enable_gaming_mode(
    game_process="valorant.exe",
    anti_cheat="vanguard",
    options={
        "zero_scan_mode": True,
        "ram_defolding": True,
        "ddos_protection": True
    }
)

# Check gaming performance
performance = client.get_gaming_performance()
# Returns: {"fps": 145, "latency": 35, "ram_usage": "10GB"}
```

---

### 9. Threat Intelligence

Access global threat intelligence.

**Python**
```python
# Get threat feed
threats = client.get_threat_feed(
    threat_types=["malware", "ransomware"],
    severity=["high", "critical"],
    limit=50
)

# Report threat
client.report_threat(
    threat_type="malware",
    description="New variant of ransomware",
    evidence={"file_hash": "sha256:abc123..."}
)
```

---

### 10. SIEM Integration

Send security events to SIEM systems.

**Python**
```python
# Configure SIEM integration
client.configure_siem(
    platform="splunk",  # splunk, qradar, sentinel, etc.
    endpoint="https://splunk.example.com:8088",
    token="your-hec-token"
)

# Send event to SIEM
client.send_to_siem(
    event={
        "timestamp": "2025-03-02T12:00:00Z",
        "event_type": "threat_detected",
        "severity": "high",
        "details": {
            "threat_type": "malware",
            "confidence": 0.95
        }
    }
)
```

---

## Language-Specific SDKs

### Python SDK

**Installation**
```bash
pip install sentinel-sdk
```

**Complete Example**
```python
from sentinel_sdk import SentinelClient

# Initialize
client = SentinelClient(api_key="your-api-key")

# Health check
health = client.health_check()
print(f"Status: {health['status']}")

# Predict threat
result = client.predict_threat(features={
    "process_name": "test.exe",
    "file_hash": "sha256:abc123..."
})
print(f"Threat: {result['threat_level']}")

# Scan file
scan = client.scan_file("/path/to/file.exe")
print(f"Threats found: {scan['threats_found']}")

# Cleanup
client.close()
```

---

### JavaScript/Node.js SDK

**Installation**
```bash
npm install @sentinel/sdk
```

**Complete Example**
```javascript
const { SentinelClient } = require('@sentinel/sdk');

// Initialize
const client = new SentinelClient({
    apiKey: 'your-api-key'
});

// Health check
const health = await client.healthCheck();
console.log(`Status: ${health.status}`);

// Predict threat
const result = await client.predictThreat({
    processName: 'test.exe',
    fileHash: 'sha256:abc123...'
});
console.log(`Threat: ${result.threatLevel}`);

// Scan file
const scan = await client.scanFile('/path/to/file.exe');
console.log(`Threats found: ${scan.threatsFound}`);

// Cleanup
await client.close();
```

---

### Go SDK

**Installation**
```bash
go get github.com/vantisCorp/sentinel-go
```

**Complete Example**
```go
package main

import (
    "fmt"
    "github.com/vantisCorp/sentinel-go"
)

func main() {
    // Initialize
    client := sentinel.NewClient("your-api-key")
    
    // Health check
    health, err := client.HealthCheck()
    if err != nil {
        panic(err)
    }
    fmt.Printf("Status: %s\n", health.Status)
    
    // Predict threat
    result, err := client.PredictThreat(sentinel.Features{
        ProcessName: "test.exe",
        FileHash:    "sha256:abc123...",
    })
    if err != nil {
        panic(err)
    }
    fmt.Printf("Threat: %s\n", result.ThreatLevel)
    
    // Cleanup
    client.Close()
}
```

---

### Rust SDK

**Installation**
```toml
[dependencies]
sentinel-rust = "1.0.0"
tokio = { version = "1.0", features = ["full"] }
```

**Complete Example**
```rust
use sentinel_rust::SentinelClient;
use tokio;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize
    let client = SentinelClient::new("your-api-key")?;
    
    // Health check
    let health = client.health_check().await?;
    println!("Status: {}", health.status);
    
    // Predict threat
    let result = client.predict_threat(
        "test.exe",
        "sha256:abc123..."
    ).await?;
    println!("Threat: {}", result.threat_level);
    
    Ok(())
}
```

---

### Java SDK

**Installation**
```xml
<dependency>
    <groupId>ai.sentinel</groupId>
    <artifactId>sentinel-java</artifactId>
    <version>1.0.0</version>
</dependency>
```

**Complete Example**
```java
import ai.sentinel.SentinelClient;
import ai.sentinel.models.*;

public class Example {
    public static void main(String[] args) {
        // Initialize
        SentinelClient client = new SentinelClient("your-api-key");
        
        // Health check
        Health health = client.healthCheck();
        System.out.println("Status: " + health.getStatus());
        
        // Predict threat
        Features features = new Features();
        features.setProcessName("test.exe");
        features.setFileHash("sha256:abc123...");
        
        PredictionResult result = client.predictThreat(features);
        System.out.println("Threat: " + result.getThreatLevel());
        
        // Cleanup
        client.close();
    }
}
```

---

### C# SDK

**Installation**
```bash
dotnet add package Sentinel.SDK
```

**Complete Example**
```csharp
using Sentinel.SDK;

class Program
{
    static void Main(string[] args)
    {
        // Initialize
        var client = new SentinelClient("your-api-key");
        
        // Health check
        var health = client.HealthCheck();
        Console.WriteLine($"Status: {health.Status}");
        
        // Predict threat
        var result = client.PredictThreat(new Features {
            ProcessName = "test.exe",
            FileHash = "sha256:abc123..."
        });
        Console.WriteLine($"Threat: {result.ThreatLevel}");
        
        // Cleanup
        client.Close();
    }
}
```

---

## Examples

### Example 1: Real-time File Scanner

```python
from sentinel_sdk import SentinelClient
import os
import time

client = SentinelClient(api_key="your-api-key")

def scan_directory(directory_path):
    """Scan all files in a directory"""
    for root, dirs, files in os.walk(directory_path):
        for file in files:
            file_path = os.path.join(root, file)
            print(f"Scanning: {file_path}")
            
            result = client.scan_file(file_path)
            if result['threats_found'] > 0:
                print(f"  THREAT FOUND: {result['threats']}")
            else:
                print(f"  Clean")

scan_directory("/path/to/directory")
client.close()
```

---

### Example 2: Process Monitor

```python
from sentinel_sdk import SentinelClient
import psutil

client = SentinelClient(api_key="your-api-key")

def monitor_processes():
    """Monitor all running processes"""
    for proc in psutil.process_iter(['pid', 'name', 'exe']):
        try:
            features = {
                "process_name": proc.info['name'],
                "file_path": proc.info['exe']
            }
            
            result = client.predict_threat(features)
            if result['threat_level'] in ['high', 'critical']:
                print(f"THREAT: {proc.info['name']} - {result['threat_type']}")
        except (psutil.NoSuchProcess, psutil.AccessDenied):
            continue

monitor_processes()
client.close()
```

---

### Example 3: Network Traffic Analyzer

```python
from sentinel_sdk import SentinelClient
import scapy.all as scapy

client = SentinelClient(api_key="your-api-key")

def analyze_packet(packet):
    """Analyze network packets"""
    if packet.haslayer(scapy.IP):
        src_ip = packet[scapy.IP].src
        dst_ip = packet[scapy.IP].dst
        dst_port = packet[scapy.IP].dport
        
        # Check if IP is malicious
        threat = client.check_ip_reputation(src_ip)
        if threat['is_malicious']:
            print(f"MALICIOUS IP: {src_ip} - {threat['threat_type']}")

# Start packet capture
scapy.sniff(prn=analyze_packet, store=0)
client.close()
```

---

### Example 4: Gaming Mode Integration

```python
from sentinel_sdk import SentinelClient
import psutil

client = SentinelClient(api_key="your-api-key")

def enable_gaming_mode_for_game(game_name):
    """Enable gaming mode for specific game"""
    # Find game process
    for proc in psutil.process_iter(['name']):
        if proc.info['name'].lower() == game_name.lower():
            print(f"Found game: {game_name}")
            
            # Enable gaming mode
            client.enable_gaming_mode(
                game_process=game_name,
                anti_cheat="vanguard",  # or "eac", "battleye"
                options={
                    "zero_scan_mode": True,
                    "ram_defolding": True,
                    "ddos_protection": True,
                    "ai_overclocking": True
                }
            )
            
            print("Gaming mode enabled!")
            return
    
    print(f"Game not found: {game_name}")

enable_gaming_mode_for_game("valorant.exe")
client.close()
```

---

## Best Practices

### 1. Error Handling

Always implement proper error handling:

```python
from sentinel_sdk import SentinelClient, SentinelError

client = SentinelClient(api_key="your-api-key")

try:
    result = client.predict_threat(features=...)
except SentinelError as e:
    print(f"API Error: {e}")
except Exception as e:
    print(f"Unexpected error: {e}")
```

---

### 2. Rate Limiting

Respect rate limits and implement exponential backoff:

```python
import time
from sentinel_sdk import SentinelClient, RateLimitError

client = SentinelClient(api_key="your-api-key")

def predict_with_retry(features, max_retries=3):
    for attempt in range(max_retries):
        try:
            return client.predict_threat(features)
        except RateLimitError:
            wait_time = 2 ** attempt
            print(f"Rate limited. Waiting {wait_time} seconds...")
            time.sleep(wait_time)
    
    raise Exception("Max retries exceeded")
```

---

### 3. Caching

Cache responses to reduce API calls:

```python
from functools import lru_cache

@lru_cache(maxsize=1000)
def get_cached_threat_info(file_hash):
    """Cache threat predictions"""
    return client.predict_threat({"file_hash": file_hash})
```

---

### 4. Async Operations

Use async operations for better performance:

```python
import asyncio
from sentinel_sdk import AsyncSentinelClient

async def scan_multiple_files(file_paths):
    client = AsyncSentinelClient(api_key="your-api-key")
    
    tasks = [client.scan_file(path) for path in file_paths]
    results = await asyncio.gather(*tasks)
    
    await client.close()
    return results

# Usage
results = asyncio.run(scan_multiple_files(["file1.exe", "file2.exe"]))
```

---

### 5. Logging

Implement comprehensive logging:

```python
import logging
from sentinel_sdk import SentinelClient

logging.basicConfig(level=logging.INFO)
logger = logging.getLogger(__name__)

client = SentinelClient(api_key="your-api-key")

logger.info("Starting threat prediction")
result = client.predict_threat(features=...)
logger.info(f"Prediction result: {result}")
```

---

### 6. Configuration Management

Use environment variables for configuration:

```python
import os
from sentinel_sdk import SentinelClient

api_key = os.getenv("SENTINEL_API_KEY")
client = SentinelClient(api_key=api_key)
```

---

## Error Handling

### Error Types

1. **SentinelError** - Base error class
2. **AuthenticationError** - Invalid API key
3. **RateLimitError** - Too many requests
4. **InvalidRequestError** - Invalid request parameters
5. **ServerError** - Server-side error
6. **NetworkError** - Network connectivity issue

### Error Handling Example

```python
from sentinel_sdk import (
    SentinelClient,
    AuthenticationError,
    RateLimitError,
    InvalidRequestError,
    ServerError,
    NetworkError
)

client = SentinelClient(api_key="your-api-key")

try:
    result = client.predict_threat(features=...)
except AuthenticationError:
    print("Invalid API key")
except RateLimitError:
    print("Rate limit exceeded")
except InvalidRequestError as e:
    print(f"Invalid request: {e}")
except ServerError:
    print("Server error")
except NetworkError:
    print("Network error")
except SentinelError as e:
    print(f"Unexpected error: {e}")
```

---

## Rate Limiting

### Rate Limits

- **Free Tier**: 100 requests/minute
- **Standard Tier**: 1,000 requests/minute
- **Enterprise Tier**: Unlimited

### Handling Rate Limits

```python
import time
from sentinel_sdk import SentinelClient, RateLimitError

client = SentinelClient(api_key="your-api-key")

def make_request_with_backoff(request_func, max_retries=5):
    for attempt in range(max_retries):
        try:
            return request_func()
        except RateLimitError as e:
            retry_after = e.retry_after or 2 ** attempt
            print(f"Rate limited. Waiting {retry_after} seconds...")
            time.sleep(retry_after)
    
    raise Exception("Max retries exceeded")

# Usage
result = make_request_with_backoff(
    lambda: client.predict_threat(features=...)
)
```

---

## Support

### Documentation
- API Reference: https://sentinel.ai/docs/api
- SDK Documentation: https://sentinel.ai/docs/sdk
- GitHub: https://github.com/vantisCorp/V-Sentinel

### Community
- Discord: https://discord.gg/sentinel
- Forums: https://forum.sentinel.ai
- Stack Overflow: https://stackoverflow.com/questions/tagged/sentinel-ai

### Support
- Email: support@sentinel.ai
- Status Page: https://status.sentinel.ai
- Twitter: https://twitter.com/sentinelsecurity

---

## Changelog

### Version 1.0.0 (March 2025)
- Initial release
- Support for Python, JavaScript, Go, Rust, Java, C#
- Core APIs: threat prediction, file scanning, process monitoring
- Quantum cryptography support
- Gaming mode integration
- SIEM integration

---

## License

MIT License - See LICENSE file for details.

---

*© 2025 SENTINEL. All rights reserved. | vantisCorp*

*Last Updated: March 2025*