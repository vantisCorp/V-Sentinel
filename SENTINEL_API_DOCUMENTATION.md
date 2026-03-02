# SENTINEL - API Documentation

## Overview

The SENTINEL API provides RESTful endpoints for interacting with the SENTINEL Security System. All API requests require authentication and authorization.

**Base URL:** `https://api.sentinel.ai/v1`

**API Version:** 1.1.0

**Authentication:** Bearer Token (JWT) or API Key

**Content-Type:** `application/json`

---

## Authentication

### API Key Authentication

Include your API key in the `Authorization` header:

```
Authorization: Bearer YOUR_API_KEY
```

### JWT Token Authentication

Include your JWT token in the `Authorization` header:

```
Authorization: Bearer YOUR_JWT_TOKEN
```

### OAuth 2.0

SENTINEL supports OAuth 2.0 for third-party integrations.

**Authorization Endpoint:** `https://auth.sentinel.ai/oauth/authorize`

**Token Endpoint:** `https://auth.sentinel.ai/oauth/token`

---

## Response Format

All API responses follow this format:

### Success Response
```json
{
  "success": true,
  "data": { ... },
  "message": "Operation successful",
  "timestamp": "2024-01-01T00:00:00Z"
}
```

### Error Response
```json
{
  "success": false,
  "error": {
    "code": "ERROR_CODE",
    "message": "Error message",
    "details": { ... }
  },
  "timestamp": "2024-01-01T00:00:00Z"
}
```

---

## Health Check

### Get System Health

Check the health status of the SENTINEL system.

**Endpoint:** `GET /health`

**Authentication:** None required

**Response:**
```json
{
  "status": "healthy",
  "version": "1.1.0",
  "timestamp": "2024-01-01T00:00:00Z",
  "services": {
    "api": "healthy",
    "database": "healthy",
    "cache": "healthy",
    "ai_engine": "healthy",
    "hypervisor": "healthy"
  }
}
```

---

## Core APIs

### System Status

#### Get System Status

Retrieve the current status of the SENTINEL system.

**Endpoint:** `GET /system/status`

**Authentication:** Required

**Response:**
```json
{
  "system_id": "sentinel-prod-001",
  "status": "running",
  "uptime": 86400,
  "version": "1.1.0",
  "load": {
    "cpu_usage": 1.5,
    "memory_usage": 180,
    "disk_usage": 45.2
  },
  "active_threats": 0,
  "protected_devices": 15234
}
```

#### Get System Statistics

Retrieve system statistics and metrics.

**Endpoint:** `GET /system/stats`

**Authentication:** Required

**Query Parameters:**
- `period` (optional): Time period (1h, 24h, 7d, 30d). Default: 24h

**Response:**
```json
{
  "period": "24h",
  "threats_detected": 1523,
  "threats_blocked": 1520,
  "false_positives": 3,
  "detection_rate": 99.8,
  "avg_response_time": 45,
  "protected_devices": 15234,
  "active_users": 8234
}
```

---

## Hypervisor APIs

### Hypervisor Management

#### Initialize Hypervisor

Initialize the Ring -1 hypervisor.

**Endpoint:** `POST /hypervisor/initialize`

**Authentication:** Required

**Request Body:**
```json
{
  "enable_vmx": true,
  "enable_svm": true,
  "enable_ept": true,
  "enable_vpid": true
}
```

**Response:**
```json
{
  "success": true,
  "data": {
    "hypervisor_id": "hyp-001",
    "status": "initialized",
    "vmx_enabled": true,
    "svm_enabled": true,
    "ept_enabled": true,
    "vpid_enabled": true
  }
}
```

#### Create Virtual Machine

Create a new virtual machine.

**Endpoint:** `POST /hypervisor/vms`

**Authentication:** Required

**Request Body:**
```json
{
  "name": "security-vm-001",
  "vcpus": 2,
  "memory_mb": 2048,
  "enable_ept": true
}
```

**Response:**
```json
{
  "success": true,
  "data": {
    "vm_id": "vm-001",
    "name": "security-vm-001",
    "status": "created",
    "vcpus": 2,
    "memory_mb": 2048
  }
}
```

#### Start Virtual Machine

Start a virtual machine.

**Endpoint:** `POST /hypervisor/vms/{vm_id}/start`

**Authentication:** Required

**Path Parameters:**
- `vm_id`: Virtual machine ID

**Response:**
```json
{
  "success": true,
  "data": {
    "vm_id": "vm-001",
    "status": "running",
    "started_at": "2024-01-01T00:00:00Z"
  }
}
```

#### Stop Virtual Machine

Stop a virtual machine.

**Endpoint:** `POST /hypervisor/vms/{vm_id}/stop`

**Authentication:** Required

**Path Parameters:**
- `vm_id`: Virtual machine ID

**Response:**
```json
{
  "success": true,
  "data": {
    "vm_id": "vm-001",
    "status": "stopped",
    "stopped_at": "2024-01-01T00:00:00Z"
  }
}
```

#### List Virtual Machines

List all virtual machines.

**Endpoint:** `GET /hypervisor/vms`

**Authentication:** Required

**Query Parameters:**
- `status` (optional): Filter by status (created, running, paused, stopped)

**Response:**
```json
{
  "success": true,
  "data": {
    "vms": [
      {
        "vm_id": "vm-001",
        "name": "security-vm-001",
        "status": "running",
        "vcpus": 2,
        "memory_mb": 2048
      }
    ],
    "total": 1
  }
}
```

---

## AI Prediction APIs

### Threat Prediction

#### Predict Threat

Predict if a process or file is malicious.

**Endpoint:** `POST /ai/predict`

**Authentication:** Required

**Request Body:**
```json
{
  "features": {
    "process_behavior": [0.1, 0.2, 0.3, 0.4, 0.5],
    "file_operations": [0.6, 0.7, 0.8, 0.9, 1.0],
    "network_activity": [0.1, 0.2, 0.3, 0.4, 0.5],
    "system_calls": [0.6, 0.7, 0.8, 0.9, 1.0],
    "registry_changes": [0.1, 0.2, 0.3, 0.4, 0.5]
  },
  "context": {
    "process_name": "suspicious.exe",
    "file_path": "C:\\Temp\\suspicious.exe",
    "pid": 1234
  }
}
```

**Response:**
```json
{
  "success": true,
  "data": {
    "threat_type": "Malware",
    "confidence": 0.95,
    "risk_score": 0.9,
    "recommended_action": "Quarantine",
    "prediction_id": "pred-001",
    "model_version": "1.2.0",
    "inference_time_ms": 45
  }
}
```

#### Batch Predict

Predict threats for multiple items.

**Endpoint:** `POST /ai/predict/batch`

**Authentication:** Required

**Request Body:**
```json
{
  "predictions": [
    {
      "features": { ... },
      "context": { ... }
    },
    {
      "features": { ... },
      "context": { ... }
    }
  ]
}
```

**Response:**
```json
{
  "success": true,
  "data": {
    "predictions": [
      {
        "threat_type": "Malware",
        "confidence": 0.95,
        "risk_score": 0.9,
        "recommended_action": "Quarantine"
      }
    ],
    "total": 1,
    "processing_time_ms": 120
  }
}
```

#### Get Model Information

Get information about the AI model.

**Endpoint:** `GET /ai/model`

**Authentication:** Required

**Response:**
```json
{
  "success": true,
  "data": {
    "model_name": "SENTINEL-Threat-Predictor",
    "model_version": "1.2.0",
    "model_type": "Deep Neural Network",
    "training_data_size": 10000000,
    "last_updated": "2024-01-01T00:00:00Z",
    "accuracy": 0.998,
    "false_positive_rate": 0.0003
  }
}
```

---

## Gaming APIs

### Game Detection

#### Detect Game

Detect if a game is running.

**Endpoint:** `POST /gaming/detect`

**Authentication:** Required

**Request Body:**
```json
{
  "process_name": "valorant.exe",
  "window_title": "VALORANT"
}
```

**Response:**
```json
{
  "success": true,
  "data": {
    "game_detected": true,
    "game_id": "valorant",
    "game_name": "VALORANT",
    "anti_cheat_system": "Vanguard",
    "process_id": 1234
  }
}
```

### Trusted Handshake

#### Initiate Handshake

Initiate trusted handshake with anti-cheat system.

**Endpoint:** `POST /gaming/handshake`

**Authentication:** Required

**Request Body:**
```json
{
  "game_id": "valorant",
  "anti_cheat": "Vanguard"
}
```

**Response:**
```json
{
  "success": true,
  "data": {
    "session_id": "session-001",
    "status": "in_progress",
    "game_id": "valorant",
    "anti_cheat_system": "Vanguard",
    "started_at": "2024-01-01T00:00:00Z"
  }
}
```

#### Complete Handshake

Complete the trusted handshake.

**Endpoint:** `POST /gaming/handshake/{session_id}/complete`

**Authentication:** Required

**Path Parameters:**
- `session_id`: Handshake session ID

**Response:**
```json
{
  "success": true,
  "data": {
    "session_id": "session-001",
    "status": "completed",
    "trust_level": "high",
    "zero_scan_mode": true,
    "completed_at": "2024-01-01T00:00:05Z"
  }
}
```

#### Activate Zero-Scan Mode

Activate zero-scan mode for gaming.

**Endpoint:** `POST /gaming/zero-scan/activate`

**Authentication:** Required

**Request Body:**
```json
{
  "session_id": "session-001"
}
```

**Response:**
```json
{
  "success": true,
  "data": {
    "zero_scan_mode": true,
    "activated_at": "2024-01-01T00:00:00Z",
    "performance_improvement": {
      "fps_increase": 21,
      "latency_reduction": 77
    }
  }
}
```

#### Deactivate Zero-Scan Mode

Deactivate zero-scan mode.

**Endpoint:** `POST /gaming/zero-scan/deactivate`

**Authentication:** Required

**Response:**
```json
{
  "success": true,
  "data": {
    "zero_scan_mode": false,
    "deactivated_at": "2024-01-01T00:00:00Z"
  }
}
```

### Anti-DDoS Shield

#### Monitor Traffic

Monitor network traffic for DDoS attacks.

**Endpoint:** `GET /gaming/traffic`

**Authentication:** Required

**Response:**
```json
{
  "success": true,
  "data": {
    "packets_per_second": 1000,
    "bytes_per_second": 1000000,
    "unique_sources": 50,
    "average_packet_size": 1000,
    "attack_detected": false
  }
}
```

#### Detect Attack

Detect if a DDoS attack is in progress.

**Endpoint:** `POST /gaming/ddos/detect`

**Authentication:** Required

**Request Body:**
```json
{
  "traffic_stats": {
    "packets_per_second": 100000,
    "bytes_per_second": 100000000,
    "unique_sources": 10000
  }
}
```

**Response:**
```json
{
  "success": true,
  "data": {
    "attack_detected": true,
    "attack_type": "UDPFlood",
    "severity": "high",
    "source_ips": ["192.168.1.1", "192.168.1.2"],
    "target_port": 27015,
    "started_at": "2024-01-01T00:00:00Z"
  }
}
```

#### Mitigate Attack

Mitigate a DDoS attack.

**Endpoint:** `POST /gaming/ddos/mitigate`

**Authentication:** Required

**Request Body:**
```json
{
  "attack_id": "attack-001",
  "mitigation_strategy": "rate_limiting"
}
```

**Response:**
```json
{
  "success": true,
  "data": {
    "attack_id": "attack-001",
    "mitigation_applied": true,
    "mitigation_strategy": "rate_limiting",
    "packets_blocked": 50000,
    "mitigated_at": "2024-01-01T00:00:01Z"
  }
}
```

---

## Quantum Cryptography APIs

### Key Management

#### Generate Key Pair

Generate a post-quantum key pair.

**Endpoint:** `POST /quantum/keypair`

**Authentication:** Required

**Request Body:**
```json
{
  "algorithm": "CrystalsKyber768"
}
```

**Response:**
```json
{
  "success": true,
  "data": {
    "keypair_id": "kp-001",
    "algorithm": "CrystalsKyber768",
    "public_key": "base64_encoded_public_key",
    "secret_key": "base64_encoded_secret_key",
    "created_at": "2024-01-01T00:00:00Z"
  }
}
```

#### Encapsulate

Encapsulate a shared secret using a public key.

**Endpoint:** `POST /quantum/encapsulate`

**Authentication:** Required

**Request Body:**
```json
{
  "public_key": "base64_encoded_public_key",
  "algorithm": "CrystalsKyber768"
}
```

**Response:**
```json
{
  "success": true,
  "data": {
    "ciphertext": "base64_encoded_ciphertext",
    "shared_secret": "base64_encoded_shared_secret"
  }
}
```

#### Decapsulate

Decapsulate a shared secret using a secret key.

**Endpoint:** `POST /quantum/decapsulate`

**Authentication:** Required

**Request Body:**
```json
{
  "ciphertext": "base64_encoded_ciphertext",
  "secret_key": "base64_encoded_secret_key",
  "algorithm": "CrystalsKyber768"
}
```

**Response:**
```json
{
  "success": true,
  "data": {
    "shared_secret": "base64_encoded_shared_secret"
  }
}
```

### Digital Signatures

#### Sign

Sign a message using a post-quantum signature algorithm.

**Endpoint:** `POST /quantum/sign`

**Authentication:** Required

**Request Body:**
```json
{
  "message": "base64_encoded_message",
  "secret_key": "base64_encoded_secret_key",
  "algorithm": "CrystalsDilithium3"
}
```

**Response:**
```json
{
  "success": true,
  "data": {
    "signature": "base64_encoded_signature",
    "algorithm": "CrystalsDilithium3"
  }
}
```

#### Verify

Verify a signature.

**Endpoint:** `POST /quantum/verify`

**Authentication:** Required

**Request Body:**
```json
{
  "message": "base64_encoded_message",
  "signature": "base64_encoded_signature",
  "public_key": "base64_encoded_public_key",
  "algorithm": "CrystalsDilithium3"
}
```

**Response:**
```json
{
  "success": true,
  "data": {
    "valid": true,
    "algorithm": "CrystalsDilithium3"
  }
}
```

### Hybrid Encryption

#### Encrypt

Encrypt data using hybrid encryption (classical + post-quantum).

**Endpoint:** `POST /quantum/encrypt`

**Authentication:** Required

**Request Body:**
```json
{
  "plaintext": "base64_encoded_plaintext",
  "public_key": "base64_encoded_public_key"
}
```

**Response:**
```json
{
  "success": true,
  "data": {
    "ciphertext": "base64_encoded_ciphertext",
    "encryption_method": "hybrid",
    "classical_algorithm": "AES-256-GCM",
    "post_quantum_algorithm": "CrystalsKyber768"
  }
}
```

#### Decrypt

Decrypt data using hybrid encryption.

**Endpoint:** `POST /quantum/decrypt`

**Authentication:** Required

**Request Body:**
```json
{
  "ciphertext": "base64_encoded_ciphertext",
  "secret_key": "base64_encoded_secret_key"
}
```

**Response:**
```json
{
  "success": true,
  "data": {
    "plaintext": "base64_encoded_plaintext"
  }
}
```

---

## Memory Management APIs

### Memory Protection

#### Protect Memory Region

Protect a memory region.

**Endpoint:** `POST /memory/protect`

**Authentication:** Required

**Request Body:**
```json
{
  "start_address": 140737488355328,
  "size": 4096,
  "protection_flags": {
    "read": true,
    "write": false,
    "execute": false,
    "guard_page": true
  }
}
```

**Response:**
```json
{
  "success": true,
  "data": {
    "region_id": "region-001",
    "start_address": 140737488355328,
    "end_address": 140737488359424,
    "protection_flags": {
      "read": true,
      "write": false,
      "execute": false,
      "guard_page": true
    }
  }
}
```

#### Unprotect Memory Region

Unprotect a memory region.

**Endpoint:** `POST /memory/unprotect`

**Authentication:** Required

**Request Body:**
```json
{
  "region_id": "region-001"
}
```

**Response:**
```json
{
  "success": true,
  "data": {
    "region_id": "region-001",
    "unprotected": true
  }
}
```

#### Get Memory Statistics

Get memory statistics.

**Endpoint:** `GET /memory/stats`

**Authentication:** Required

**Response:**
```json
{
  "success": true,
  "data": {
    "total_memory": 17179869184,
    "used_memory": 8589934592,
    "free_memory": 8589934592,
    "protected_regions": 152,
    "monitoring_enabled": true
  }
}
```

---

## Process Management APIs

### Process Monitoring

#### Monitor Process

Start monitoring a process.

**Endpoint:** `POST /process/monitor`

**Authentication:** Required

**Request Body:**
```json
{
  "pid": 1234,
  "process_name": "suspicious.exe"
}
```

**Response:**
```json
{
  "success": true,
  "data": {
    "process_id": "proc-001",
    "pid": 1234,
    "process_name": "suspicious.exe",
    "status": "monitoring",
    "started_at": "2024-01-01T00:00:00Z"
  }
}
```

#### Unmonitor Process

Stop monitoring a process.

**Endpoint:** `POST /process/unmonitor`

**Authentication:** Required

**Request Body:**
```json
{
  "process_id": "proc-001"
}
```

**Response:**
```json
{
  "success": true,
  "data": {
    "process_id": "proc-001",
    "unmonitored": true
  }
}
```

#### Suspend Process

Suspend a process.

**Endpoint:** `POST /process/suspend`

**Authentication:** Required

**Request Body:**
```json
{
  "pid": 1234
}
```

**Response:**
```json
{
  "success": true,
  "data": {
    "pid": 1234,
    "status": "suspended",
    "suspended_at": "2024-01-01T00:00:00Z"
  }
}
```

#### Resume Process

Resume a suspended process.

**Endpoint:** `POST /process/resume`

**Authentication:** Required

**Request Body:**
```json
{
  "pid": 1234
}
```

**Response:**
```json
{
  "success": true,
  "data": {
    "pid": 1234,
    "status": "running",
    "resumed_at": "2024-01-01T00:00:00Z"
  }
}
```

#### Terminate Process

Terminate a process.

**Endpoint:** `POST /process/terminate`

**Authentication:** Required

**Request Body:**
```json
{
  "pid": 1234
}
```

**Response:**
```json
{
  "success": true,
  "data": {
    "pid": 1234,
    "status": "terminated",
    "terminated_at": "2024-01-01T00:00:00Z"
  }
}
```

#### Get Process Info

Get information about a process.

**Endpoint:** `GET /process/{pid}`

**Authentication:** Required

**Path Parameters:**
- `pid`: Process ID

**Response:**
```json
{
  "success": true,
  "data": {
    "pid": 1234,
    "name": "suspicious.exe",
    "state": "Running",
    "cpu_usage": 5.2,
    "memory_usage": 104857600,
    "start_time": "2024-01-01T00:00:00Z"
  }
}
```

---

## Error Codes

### Common Error Codes

| Code | Description | HTTP Status |
|------|-------------|-------------|
| `INVALID_REQUEST` | Invalid request format | 400 |
| `UNAUTHORIZED` | Authentication required | 401 |
| `FORBIDDEN` | Insufficient permissions | 403 |
| `NOT_FOUND` | Resource not found | 404 |
| `RATE_LIMIT_EXCEEDED` | Rate limit exceeded | 429 |
| `INTERNAL_ERROR` | Internal server error | 500 |
| `SERVICE_UNAVAILABLE` | Service temporarily unavailable | 503 |

### Hypervisor Error Codes

| Code | Description |
|------|-------------|
| `HYPERVISOR_NOT_INITIALIZED` | Hypervisor not initialized |
| `VM_NOT_FOUND` | Virtual machine not found |
| `VM_ALREADY_RUNNING` | Virtual machine already running |
| `VM_CREATION_FAILED` | Failed to create virtual machine |
| `VM_START_FAILED` | Failed to start virtual machine |
| `VM_STOP_FAILED` | Failed to stop virtual machine |

### AI Error Codes

| Code | Description |
|------|-------------|
| `MODEL_NOT_LOADED` | AI model not loaded |
| `INVALID_FEATURES` | Invalid feature vector |
| `PREDICTION_FAILED` | Prediction failed |
| `MODEL_LOAD_FAILED` | Failed to load model |
| `TRAINING_FAILED` | Training failed |

### Gaming Error Codes

| Code | Description |
|------|-------------|
| `GAME_NOT_DETECTED` | Game not detected |
| `ANTI_CHEAT_NOT_SUPPORTED` | Anti-cheat system not supported |
| `HANDSHAKE_FAILED` | Handshake failed |
| `ZERO_SCAN_MODE_FAILED` | Failed to activate zero-scan mode |
| `DDOS_DETECTION_FAILED` | DDoS detection failed |
| `MITIGATION_FAILED` | Mitigation failed |

### Quantum Error Codes

| Code | Description |
|------|-------------|
| `ALGORITHM_NOT_SUPPORTED` | Algorithm not supported |
| `KEY_GENERATION_FAILED` | Key generation failed |
| `ENCAPSULATION_FAILED` | Encapsulation failed |
| `DECAPSULATION_FAILED` | Decapsulation failed |
| `SIGNING_FAILED` | Signing failed |
| `VERIFICATION_FAILED` | Verification failed |
| `ENCRYPTION_FAILED` | Encryption failed |
| `DECRYPTION_FAILED` | Decryption failed |

---

## Rate Limiting

API requests are rate limited to prevent abuse.

**Default Limits:**
- 1000 requests per hour per API key
- 100 requests per minute per API key

**Rate Limit Headers:**
```
X-RateLimit-Limit: 1000
X-RateLimit-Remaining: 999
X-RateLimit-Reset: 1609459200
```

---

## SDKs

Official SDKs are available for:

- **Python**: `pip install sentinel-sdk`
- **JavaScript/TypeScript**: `npm install @sentinel/sdk`
- **Go**: `go get github.com/sentinel/sdk-go`
- **Rust**: `cargo add sentinel-sdk`
- **Java**: `implementation 'ai.sentinel:sentinel-sdk:1.1.0'`
- **C#**: `Install-Package SentinelSDK`

---

## Webhooks

SENTINEL supports webhooks for real-time notifications.

### Register Webhook

**Endpoint:** `POST /webhooks`

**Request Body:**
```json
{
  "url": "https://your-server.com/webhook",
  "events": ["threat.detected", "threat.blocked", "system.alert"]
}
```

**Response:**
```json
{
  "success": true,
  "data": {
    "webhook_id": "webhook-001",
    "url": "https://your-server.com/webhook",
    "events": ["threat.detected", "threat.blocked", "system.alert"],
    "secret": "webhook_secret_key"
  }
}
```

### Webhook Events

| Event | Description |
|-------|-------------|
| `threat.detected` | Threat detected |
| `threat.blocked` | Threat blocked |
| `system.alert` | System alert |
| `system.offline` | System offline |
| `system.online` | System online |

---

## Support

For API support, contact:

- **Email**: api-support@sentinel.ai
- **Documentation**: https://docs.sentinel.ai
- **Status Page**: https://status.sentinel.ai
- **GitHub**: https://github.com/vantisCorp/V-Sentinel

---

**Document Version:** 1.1.0
**Last Updated:** 2024-01-01
**API Version:** 1.1.0