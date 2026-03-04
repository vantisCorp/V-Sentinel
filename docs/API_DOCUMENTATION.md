# V-Sentinel API Documentation

## Table of Contents
1. [Overview](#overview)
2. [Authentication](#authentication)
3. [Core APIs](#core-apis)
4. [Security Modules](#security-modules)
5. [AI & Machine Learning](#ai--machine-learning)
6. [Monitoring & Logging](#monitoring--logging)
7. [Configuration](#configuration)
8. [Error Handling](#error-handling)
9. [Examples](#examples)
10. [Rate Limiting](#rate-limiting)

---

## Overview

V-Sentinel provides a comprehensive REST API for managing security operations, threat detection, and autonomous defense systems. The API is designed for high performance, security, and ease of integration.

### Base URL
```
https://api.v-sentinel.io/v1
```

### Versioning
API versions are specified in the URL path. The current version is `v1`.

### Response Format
All responses use JSON format:
```json
{
  "success": true,
  "data": {},
  "meta": {
    "timestamp": "2024-03-04T10:00:00Z",
    "request_id": "req_abc123"
  }
}
```

---

## Authentication

### API Key Authentication
All API requests require authentication using an API key.

```http
GET /api/v1/health
Authorization: Bearer YOUR_API_KEY
Content-Type: application/json
```

### Token Generation
Generate an API token for programmatic access:

```http
POST /api/v1/auth/token
Content-Type: application/json

{
  "client_id": "your_client_id",
  "client_secret": "your_client_secret",
  "scope": ["read", "write"]
}
```

**Response:**
```json
{
  "success": true,
  "data": {
    "access_token": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9...",
    "token_type": "Bearer",
    "expires_in": 3600,
    "refresh_token": "refresh_abc123"
  }
}
```

---

## Core APIs

### Health Check
Check system health status:

```http
GET /api/v1/health
```

**Response:**
```json
{
  "success": true,
  "data": {
    "status": "healthy",
    "version": "1.0.0",
    "uptime": 86400,
    "components": {
      "database": "ok",
      "redis": "ok",
      "quantum": "ok",
      "ai_engine": "ok"
    }
  }
}
```

### System Status
Get detailed system status:

```http
GET /api/v1/status
```

**Response:**
```json
{
  "success": true,
  "data": {
    "cpu_usage": 45.2,
    "memory_usage": 62.8,
    "disk_usage": 34.1,
    "active_connections": 128,
    "requests_per_second": 523
  }
}
```

---

## Security Modules

### Privacy Module

#### Zero-Knowledge Proofs
Generate and verify zero-knowledge proofs:

```http
POST /api/v1/privacy/zkp/generate
Content-Type: application/json

{
  "statement": "user_age >= 18",
  "witness": {"age": 25},
  "circuit_type": "bulletproofs"
}
```

**Response:**
```json
{
  "success": true,
  "data": {
    "proof": "0x1a2b3c4d...",
    "public_inputs": ["age_commitment"],
    "verification_key": "0x5e6f7a8b..."
  }
}
```

#### Verify ZKP
```http
POST /api/v1/privacy/zkp/verify
Content-Type: application/json

{
  "proof": "0x1a2b3c4d...",
  "public_inputs": ["age_commitment"],
  "verification_key": "0x5e6f7a8b..."
}
```

#### Differential Privacy
Apply differential privacy to sensitive data:

```http
POST /api/v1/privacy/dp/apply
Content-Type: application/json

{
  "data": [100, 200, 300, 400, 500],
  "epsilon": 1.0,
  "mechanism": "laplace"
}
```

#### Homomorphic Encryption
Encrypt data for homomorphic operations:

```http
POST /api/v1/privacy/he/encrypt
Content-Type: application/json

{
  "plaintext": "sensitive_data",
  "scheme": "bfv",
  "public_key": "0x..."
}
```

---

### Quantum Module

#### Post-Quantum Encryption
Encrypt data using post-quantum algorithms:

```http
POST /api/v1/quantum/encrypt
Content-Type: application/json

{
  "plaintext": "secret_message",
  "algorithm": "crystals-kyber",
  "key_size": 1024
}
```

**Response:**
```json
{
  "success": true,
  "data": {
    "ciphertext": "0x9a8b7c6d...",
    "nonce": "0x12345678",
    "algorithm": "crystals-kyber"
  }
}
```

#### Post-Quantum Decrypt
```http
POST /api/v1/quantum/decrypt
Content-Type: application/json

{
  "ciphertext": "0x9a8b7c6d...",
  "nonce": "0x12345678",
  "private_key": "0x...",
  "algorithm": "crystals-kyber"
}
```

#### Quantum Key Distribution
Initiate QKD session:

```http
POST /api/v1/quantum/qkd/initiate
Content-Type: application/json

{
  "peer_id": "peer_abc123",
  "protocol": "BB84"
}
```

---

### Biometrics Module

#### Multi-Modal Authentication
Authenticate using multiple biometric factors:

```http
POST /api/v1/biometrics/authenticate
Content-Type: application/json

{
  "user_id": "user_123",
  "fingerprint": "0x...",
  "facial_features": "0x...",
  "voice_sample": "base64_encoded_audio"
}
```

**Response:**
```json
{
  "success": true,
  "data": {
    "authenticated": true,
    "confidence": 0.987,
    "factors_verified": ["fingerprint", "facial", "voice"]
  }
}
```

#### Enroll Biometric Data
```http
POST /api/v1/biometrics/enroll
Content-Type: application/json

{
  "user_id": "user_123",
  "biometric_type": "fingerprint",
  "template": "0x...",
  "quality_score": 0.95
}
```

---

### Neural Module

#### Threat Detection
Detect threats using neural networks:

```http
POST /api/v1/neural/detect
Content-Type: application/json

{
  "input_data": "network_traffic_logs",
  "model_type": "transformer",
  "threshold": 0.85
}
```

**Response:**
```json
{
  "success": true,
  "data": {
    "threat_detected": true,
    "threat_type": "ddos",
    "confidence": 0.92,
    "features": {
      "traffic_pattern": "anomalous",
      "source_ip": "192.168.1.100",
      "destination_port": 80
    }
  }
}
```

#### Federated Learning
Contribute to federated learning:

```http
POST /api/v1/neural/federated/contribute
Content-Type: application/json

{
  "model_id": "threat_detector_v2",
  "local_updates": "0x...",
  "num_examples": 1000
}
```

---

### Autonomous Module

#### Deploy Autonomous Agent
```http
POST /api/v1/autonomous/agent/deploy
Content-Type: application/json

{
  "agent_type": "threat_responder",
  "config": {
    "autonomous_level": "high",
    "response_actions": ["block", "isolate", "alert"]
  }
}
```

#### Agent Status
```http
GET /api/v1/autonomous/agent/{agent_id}/status
```

**Response:**
```json
{
  "success": true,
  "data": {
    "agent_id": "agent_123",
    "status": "active",
    "tasks_completed": 1523,
    "last_action": "blocked_ip_192.168.1.100",
    "uptime": 86400
  }
}
```

---

### Metaverse Module

#### VR Security
Secure VR environments:

```http
POST /api/v1/metaverse/vr/secure
Content-Type: application/json

{
  "vr_session_id": "session_abc123",
  "security_level": "high",
  "features": ["avatar_protection", "spatial_security", "asset_protection"]
}
```

#### AR Security
```http
POST /api/v1/metaverse/ar/secure
Content-Type: application/json

{
  "ar_session_id": "session_def456",
  "environment": "public",
  "permissions": ["camera", "location", "microphone"]
}
```

---

### Blockchain Module

#### Threat Reputation
Query blockchain-based threat reputation:

```http
GET /api/v1/blockchain/reputation/{ip_address}
```

**Response:**
```json
{
  "success": true,
  "data": {
    "ip_address": "192.168.1.100",
    "reputation_score": 0.25,
    "threat_level": "high",
    "reports": 47,
    "last_reported": "2024-03-04T09:00:00Z"
  }
}
```

#### Report Threat
```http
POST /api/v1/blockchain/report
Content-Type: application/json

{
  "threat_type": "malware",
  "source_ip": "192.168.1.100",
  "evidence": "0x...",
  "timestamp": "2024-03-04T10:00:00Z"
}
```

---

## AI & Machine Learning

### Model Training
Train a custom ML model:

```http
POST /api/v1/ai/train
Content-Type: application/json

{
  "model_type": "anomaly_detector",
  "dataset_id": "dataset_123",
  "hyperparameters": {
    "learning_rate": 0.001,
    "epochs": 100,
    "batch_size": 32
  }
}
```

### Model Inference
Run model inference:

```http
POST /api/v1/ai/infer
Content-Type: application/json

{
  "model_id": "model_123",
  "input_data": {...}
}
```

### Feature Extraction
Extract features from data:

```http
POST /api/v1/ai/features/extract
Content-Type: application/json

{
  "data": "network_traffic_logs",
  "feature_types": ["statistical", "temporal", "behavioral"]
}
```

---

## Monitoring & Logging

### Real-Time Metrics
```http
GET /api/v1/monitoring/metrics
```

**Response:**
```json
{
  "success": true,
  "data": {
    "requests_total": 152342,
    "requests_per_second": 523,
    "error_rate": 0.02,
    "latency_p50": 45,
    "latency_p95": 120,
    "latency_p99": 200
  }
}
```

### Query Logs
```http
GET /api/v1/monitoring/logs?level=error&limit=100
```

### Create Alert
```http
POST /api/v1/monitoring/alerts
Content-Type: application/json

{
  "name": "High Threat Level",
  "condition": "threat_level > 0.9",
  "actions": ["email", "slack", "webhook"]
}
```

---

## Configuration

### Get Configuration
```http
GET /api/v1/config
```

### Update Configuration
```http
PATCH /api/v1/config
Content-Type: application/json

{
  "security": {
    "min_password_length": 12,
    "mfa_required": true
  },
  "performance": {
    "max_connections": 1000,
    "timeout": 30
  }
}
```

---

## Error Handling

### Error Response Format
```json
{
  "success": false,
  "error": {
    "code": "AUTH_001",
    "message": "Invalid API key",
    "details": "The provided API key is expired or invalid"
  },
  "meta": {
    "timestamp": "2024-03-04T10:00:00Z",
    "request_id": "req_abc123"
  }
}
```

### Common Error Codes
| Code | Description |
|------|-------------|
| AUTH_001 | Invalid API key |
| AUTH_002 | Token expired |
| AUTH_003 | Insufficient permissions |
| REQ_001 | Invalid request format |
| REQ_002 | Missing required parameter |
| REQ_003 | Parameter validation failed |
| SRV_001 | Internal server error |
| SRV_002 | Service unavailable |
| SEC_001 | Security violation detected |

---

## Examples

### Python Example
```python
import requests

# Configure API
API_KEY = "your_api_key"
BASE_URL = "https://api.v-sentinel.io/v1"
headers = {
    "Authorization": f"Bearer {API_KEY}",
    "Content-Type": "application/json"
}

# Health check
response = requests.get(f"{BASE_URL}/health", headers=headers)
print(response.json())

# Generate zero-knowledge proof
data = {
    "statement": "user_age >= 18",
    "witness": {"age": 25},
    "circuit_type": "bulletproofs"
}
response = requests.post(
    f"{BASE_URL}/privacy/zkp/generate",
    json=data,
    headers=headers
)
print(response.json())
```

### JavaScript Example
```javascript
// Configure API
const API_KEY = "your_api_key";
const BASE_URL = "https://api.v-sentinel.io/v1";

async function apiRequest(endpoint, method = "GET", data = null) {
  const headers = {
    "Authorization": `Bearer ${API_KEY}`,
    "Content-Type": "application/json"
  };

  const options = {
    method,
    headers,
    body: data ? JSON.stringify(data) : undefined
  };

  const response = await fetch(`${BASE_URL}${endpoint}`, options);
  return response.json();
}

// Health check
apiRequest("/health").then(console.log);

// Detect threats
apiRequest("/neural/detect", "POST", {
  input_data: "network_traffic_logs",
  model_type: "transformer",
  threshold: 0.85
}).then(console.log);
```

### Rust Example
```rust
use reqwest::Client;
use serde_json::json;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new();
    let api_key = "your_api_key";
    let base_url = "https://api.v-sentinel.io/v1";

    // Health check
    let response = client
        .get(&format!("{}/health", base_url))
        .header("Authorization", format!("Bearer {}", api_key))
        .send()
        .await?
        .json::<serde_json::Value>()
        .await?;
    println!("{}", response);

    // Generate ZKP
    let data = json!({
        "statement": "user_age >= 18",
        "witness": {"age": 25},
        "circuit_type": "bulletproofs"
    });
    let response = client
        .post(&format!("{}/privacy/zkp/generate", base_url))
        .header("Authorization", format!("Bearer {}", api_key))
        .json(&data)
        .send()
        .await?
        .json::<serde_json::Value>()
        .await?;
    println!("{}", response);

    Ok(())
}
```

---

## Rate Limiting

### Rate Limit Headers
All API responses include rate limit information:

```http
X-RateLimit-Limit: 1000
X-RateLimit-Remaining: 950
X-RateLimit-Reset: 1617560400
```

### Rate Limits by Tier
| Tier | Requests per Hour | Requests per Minute |
|------|------------------|---------------------|
| Free | 1,000 | 10 |
| Pro | 10,000 | 100 |
| Enterprise | Unlimited | Unlimited |

### Handling Rate Limits
```python
import requests
import time

def api_request_with_retry(url, headers, max_retries=3):
    for i in range(max_retries):
        response = requests.get(url, headers=headers)
        
        if response.status_code == 429:
            # Rate limited - wait and retry
            retry_after = int(response.headers.get("Retry-After", 60))
            time.sleep(retry_after)
            continue
        
        return response
    
    raise Exception("Max retries exceeded")
```

---

## Support & Resources

- **Documentation**: https://docs.v-sentinel.io
- **API Reference**: https://api.v-sentinel.io/docs
- **GitHub**: https://github.com/vantisCorp/V-Sentinel
- **Support**: support@v-sentinel.io
- **Status Page**: https://status.v-sentinel.io

---

*Last Updated: March 4, 2024*
*Version: 1.0.0*