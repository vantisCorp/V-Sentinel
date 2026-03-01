# SENTINEL Security System - API Documentation

## Table of Contents
1. [Overview](#overview)
2. [Authentication & Authorization](#authentication--authorization)
3. [Core APIs](#core-apis)
4. [AI Prediction APIs](#ai-prediction-apis)
5. [Gaming APIs](#gaming-apis)
6. [Quantum Cryptography APIs](#quantum-cryptography-apis)
7. [Behavioral Analysis APIs](#behavioral-analysis-apis)
8. [Threat Intelligence APIs](#threat-intelligence-apis)
9. [SIEM Integration APIs](#siem-integration-apis)
10. [Mobile Security APIs](#mobile-security-apis)
11. [IoT Security APIs](#iot-security-apis)
12. [Cloud Security APIs](#cloud-security-apis)
13. [Monitoring APIs](#monitoring-apis)
14. [Configuration APIs](#configuration-apis)
15. [Audit APIs](#audit-apis)
16. [Performance APIs](#performance-apis)
17. [Security APIs](#security-apis)
18. [Error Codes](#error-codes)
19. [Rate Limiting](#rate-limiting)
20. [API Versioning](#api-versioning)

---

## Overview

The SENTINEL Security System provides a comprehensive REST API for managing security operations, threat detection, and system administration. All APIs use JSON for request/response bodies and follow RESTful conventions.

### Base URL
```
Production: https://api.sentinel.security/v1
Staging: https://api-staging.sentinel.security/v1
Development: http://localhost:8080/v1
```

### API Response Format
All API responses follow this standard format:

```json
{
  "success": true,
  "data": { ... },
  "error": null,
  "metadata": {
    "timestamp": "2026-01-15T10:30:00Z",
    "request_id": "req_abc123",
    "version": "1.0.0"
  }
}
```

### Error Response Format
```json
{
  "success": false,
  "data": null,
  "error": {
    "code": "INVALID_REQUEST",
    "message": "Invalid request parameters",
    "details": { ... }
  },
  "metadata": {
    "timestamp": "2026-01-15T10:30:00Z",
    "request_id": "req_abc123",
    "version": "1.0.0"
  }
}
```

---

## Authentication & Authorization

### Authentication Methods

SENTINEL supports multiple authentication methods:

1. **API Key Authentication** - For service-to-service communication
2. **JWT Bearer Token** - For user authentication
3. **OAuth 2.0** - For third-party integrations
4. **Mutual TLS** - For high-security environments

### API Key Authentication

Include your API key in the request header:

```http
Authorization: Bearer YOUR_API_KEY
X-API-Key: YOUR_API_KEY
```

### JWT Bearer Token

```http
Authorization: Bearer YOUR_JWT_TOKEN
```

### OAuth 2.0 Flow

#### 1. Authorization Code Flow
```
GET /oauth/authorize?response_type=code&client_id=CLIENT_ID&redirect_uri=REDIRECT_URI&scope=read write
```

#### 2. Token Exchange
```
POST /oauth/token
Content-Type: application/x-www-form-urlencoded

grant_type=authorization_code&code=AUTH_CODE&redirect_uri=REDIRECT_URI&client_id=CLIENT_ID&client_secret=CLIENT_SECRET
```

### Authorization Levels

| Role | Permissions |
|------|-------------|
| `admin` | Full access to all resources |
| `operator` | Read/write access to operational resources |
| `analyst` | Read access to all resources, write access to analysis resources |
| `user` | Read access to user-specific resources |
| `readonly` | Read-only access to public resources |

---

## Core APIs

### Health Check

Check the health status of the SENTINEL system.

**Endpoint:** `GET /health`

**Response:**
```json
{
  "success": true,
  "data": {
    "status": "healthy",
    "version": "1.0.0",
    "uptime": 86400,
    "components": {
      "hypervisor": "healthy",
      "ai_engine": "healthy",
      "quantum_crypto": "healthy",
      "database": "healthy",
      "cache": "healthy"
    }
  }
}
```

### System Status

Get detailed system status information.

**Endpoint:** `GET /status`

**Response:**
```json
{
  "success": true,
  "data": {
    "system": {
      "status": "operational",
      "cpu_usage": 45.2,
      "memory_usage": 62.8,
      "disk_usage": 34.5,
      "network_io": {
        "in": 1024000,
        "out": 512000
      }
    },
    "services": {
      "hypervisor": {
        "status": "running",
        "vms": 5,
        "uptime": 86400
      },
      "ai_engine": {
        "status": "running",
        "predictions_today": 15234,
        "accuracy": 99.8
      },
      "quantum_crypto": {
        "status": "running",
        "operations_today": 8934,
        "algorithms": ["kyber", "dilithium"]
      }
    }
  }
}
```

### Hypervisor Management

#### Create Virtual Machine
**Endpoint:** `POST /hypervisor/vms`

**Request:**
```json
{
  "name": "vm-001",
  "memory_mb": 4096,
  "vcpus": 2,
  "os_type": "linux",
  "network_config": {
    "bridge": "br0",
    "ip_address": "192.168.1.100"
  }
}
```

**Response:**
```json
{
  "success": true,
  "data": {
    "vm_id": "vm-001",
    "status": "created",
    "config": { ... }
  }
}
```

#### Start Virtual Machine
**Endpoint:** `POST /hypervisor/vms/{vm_id}/start`

**Response:**
```json
{
  "success": true,
  "data": {
    "vm_id": "vm-001",
    "status": "running",
    "pid": 12345
  }
}
```

#### Stop Virtual Machine
**Endpoint:** `POST /hypervisor/vms/{vm_id}/stop`

**Response:**
```json
{
  "success": true,
  "data": {
    "vm_id": "vm-001",
    "status": "stopped"
  }
}
```

#### Get VM Status
**Endpoint:** `GET /hypervisor/vms/{vm_id}`

**Response:**
```json
{
  "success": true,
  "data": {
    "vm_id": "vm-001",
    "status": "running",
    "memory_mb": 4096,
    "vcpus": 2,
    "uptime": 3600,
    "cpu_usage": 25.3,
    "memory_usage": 45.2
  }
}
```

#### List All VMs
**Endpoint:** `GET /hypervisor/vms`

**Query Parameters:**
- `status` (optional): Filter by status (running, stopped, paused)
- `limit` (optional): Maximum number of results (default: 50)
- `offset` (optional): Offset for pagination (default: 0)

**Response:**
```json
{
  "success": true,
  "data": {
    "vms": [
      {
        "vm_id": "vm-001",
        "status": "running",
        "memory_mb": 4096,
        "vcpus": 2
      },
      {
        "vm_id": "vm-002",
        "status": "stopped",
        "memory_mb": 2048,
        "vcpus": 1
      }
    ],
    "total": 2,
    "limit": 50,
    "offset": 0
  }
}
```

### Memory Protection

#### Protect Memory Region
**Endpoint:** `POST /memory/protect`

**Request:**
```json
{
  "start_address": "0x1000",
  "size": 4096,
  "flags": ["read", "write"],
  "description": "Protected heap region"
}
```

**Response:**
```json
{
  "success": true,
  "data": {
    "region_id": "region-001",
    "start_address": "0x1000",
    "size": 4096,
    "flags": ["read", "write"],
    "status": "protected"
  }
}
```

#### Unprotect Memory Region
**Endpoint:** `DELETE /memory/protect/{region_id}`

**Response:**
```json
{
  "success": true,
  "data": {
    "region_id": "region-001",
    "status": "unprotected"
  }
}
```

#### List Protected Regions
**Endpoint:** `GET /memory/protect`

**Response:**
```json
{
  "success": true,
  "data": {
    "regions": [
      {
        "region_id": "region-001",
        "start_address": "0x1000",
        "size": 4096,
        "flags": ["read", "write"],
        "description": "Protected heap region"
      }
    ],
    "total": 1
  }
}
```

### Process Management

#### Monitor Process
**Endpoint:** `POST /process/monitor`

**Request:**
```json
{
  "pid": 1234,
  "name": "chrome",
  "monitor_level": "high"
}
```

**Response:**
```json
{
  "success": true,
  "data": {
    "process_id": "proc-001",
    "pid": 1234,
    "name": "chrome",
    "status": "monitoring",
    "monitor_level": "high"
  }
}
```

#### Stop Monitoring Process
**Endpoint:** `DELETE /process/monitor/{process_id}`

**Response:**
```json
{
  "success": true,
  "data": {
    "process_id": "proc-001",
    "status": "stopped"
  }
}
```

#### Get Process Status
**Endpoint:** `GET /process/monitor/{process_id}`

**Response:**
```json
{
  "success": true,
  "data": {
    "process_id": "proc-001",
    "pid": 1234,
    "name": "chrome",
    "status": "running",
    "cpu_usage": 15.2,
    "memory_usage": 256.5,
    "monitor_level": "high",
    "alerts": []
  }
}
```

---

## AI Prediction APIs

### Predict Threat

Predict whether a file, process, or network connection is malicious.

**Endpoint:** `POST /ai/predict`

**Request:**
```json
{
  "type": "file",
  "data": {
    "file_path": "/path/to/file.exe",
    "file_hash": "sha256:abc123...",
    "file_size": 1024000,
    "file_type": "executable"
  }
}
```

**Response:**
```json
{
  "success": true,
  "data": {
    "prediction": {
      "threat_type": "malware",
      "confidence": 0.95,
      "risk_level": "high",
      "features": {
        "file_entropy": 7.8,
        "api_calls": ["CreateFile", "WriteFile", "CreateProcess"],
        "suspicious_strings": ["cmd.exe", "powershell"]
      }
    },
    "recommendation": "quarantine",
    "model_version": "1.2.0"
  }
}
```

### Batch Predict

Predict multiple items in a single request.

**Endpoint:** `POST /ai/predict/batch`

**Request:**
```json
{
  "items": [
    {
      "type": "file",
      "data": {
        "file_path": "/path/to/file1.exe",
        "file_hash": "sha256:abc123...",
        "file_size": 1024000
      }
    },
    {
      "type": "process",
      "data": {
        "pid": 1234,
        "name": "unknown.exe",
        "command_line": "unknown.exe /hidden"
      }
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
        "item_id": 0,
        "threat_type": "malware",
        "confidence": 0.95,
        "risk_level": "high"
      },
      {
        "item_id": 1,
        "threat_type": "benign",
        "confidence": 0.98,
        "risk_level": "low"
      }
    ],
    "model_version": "1.2.0"
  }
}
```

### Get Model Information

Get information about the AI prediction models.

**Endpoint:** `GET /ai/models`

**Response:**
```json
{
  "success": true,
  "data": {
    "models": [
      {
        "name": "malware_detection",
        "version": "1.2.0",
        "type": "neural_network",
        "accuracy": 99.8,
        "last_trained": "2026-01-10T00:00:00Z",
        "status": "active"
      },
      {
        "name": "ransomware_detection",
        "version": "1.1.0",
        "type": "random_forest",
        "accuracy": 99.5,
        "last_trained": "2026-01-08T00:00:00Z",
        "status": "active"
      }
    ]
  }
}
```

### Extract Features

Extract features from a file or process for analysis.

**Endpoint:** `POST /ai/features/extract`

**Request:**
```json
{
  "type": "file",
  "data": {
    "file_path": "/path/to/file.exe",
    "file_hash": "sha256:abc123..."
  }
}
```

**Response:**
```json
{
  "success": true,
  "data": {
    "features": {
      "file_entropy": 7.8,
      "file_size": 1024000,
      "section_count": 5,
      "import_count": 45,
      "export_count": 12,
      "api_calls": ["CreateFile", "WriteFile", "CreateProcess"],
      "suspicious_strings": ["cmd.exe", "powershell"],
      "digital_signature": {
        "signed": false,
        "signer": null
      }
    }
  }
}
```

---

## Gaming APIs

### Detect Game

Detect if a game is running and identify it.

**Endpoint:** `POST /gaming/detect`

**Request:**
```json
{
  "process_name": "valorant.exe",
  "pid": 1234
}
```

**Response:**
```json
{
  "success": true,
  "data": {
    "game": {
      "name": "Valorant",
      "publisher": "Riot Games",
      "anti_cheat": "Vanguard",
      "process_name": "valorant.exe",
      "pid": 1234,
      "detected_at": "2026-01-15T10:30:00Z"
    }
  }
}
```

### Initiate Trusted Handshake

Initiate the trusted handshake protocol with a game.

**Endpoint:** `POST /gaming/handshake/initiate`

**Request:**
```json
{
  "game_id": "valorant",
  "pid": 1234
}
```

**Response:**
```json
{
  "success": true,
  "data": {
    "session_id": "session-001",
    "status": "initiated",
    "challenge": "abc123...",
    "expires_at": "2026-01-15T10:35:00Z"
  }
}
```

### Complete Handshake

Complete the trusted handshake protocol.

**Endpoint:** `POST /gaming/handshake/complete`

**Request:**
```json
{
  "session_id": "session-001",
  "response": "xyz789..."
}
```

**Response:**
```json
{
  "success": true,
  "data": {
    "session_id": "session-001",
    "status": "completed",
    "trust_level": "high",
    "anti_cheat_compatible": true
  }
}
```

### Activate Zero-Scan Mode

Activate zero-scan mode for a game.

**Endpoint:** `POST /gaming/zero-scan/activate`

**Request:**
```json
{
  "game_id": "valorant",
  "pid": 1234
}
```

**Response:**
```json
{
  "success": true,
  "data": {
    "status": "activated",
    "game_id": "valorant",
    "pid": 1234,
    "activated_at": "2026-01-15T10:30:00Z"
  }
}
```

### Deactivate Zero-Scan Mode

Deactivate zero-scan mode.

**Endpoint:** `POST /gaming/zero-scan/deactivate`

**Request:**
```json
{
  "game_id": "valorant",
  "pid": 1234
}
```

**Response:**
```json
{
  "success": true,
  "data": {
    "status": "deactivated",
    "game_id": "valorant",
    "pid": 1234,
    "deactivated_at": "2026-01-15T10:35:00Z"
  }
}
```

### Monitor Traffic

Monitor network traffic for DDoS attacks.

**Endpoint:** `POST /gaming/traffic/monitor`

**Request:**
```json
{
  "interface": "eth0",
  "threshold": {
    "packets_per_second": 10000,
    "bytes_per_second": 10000000
  }
}
```

**Response:**
```json
{
  "success": true,
  "data": {
    "monitor_id": "monitor-001",
    "status": "monitoring",
    "interface": "eth0",
    "threshold": {
      "packets_per_second": 10000,
      "bytes_per_second": 10000000
    }
  }
}
```

### Detect Attack

Detect if a DDoS attack is occurring.

**Endpoint:** `GET /gaming/traffic/attack/detect`

**Response:**
```json
{
  "success": true,
  "data": {
    "attack_detected": true,
    "attack_type": "udp_flood",
    "severity": "high",
    "source_ips": ["192.168.1.100", "192.168.1.101"],
    "target_port": 27015,
    "packets_per_second": 50000,
    "bytes_per_second": 50000000,
    "detected_at": "2026-01-15T10:30:00Z"
  }
}
```

### Mitigate Attack

Mitigate a detected DDoS attack.

**Endpoint:** `POST /gaming/traffic/attack/mitigate`

**Request:**
```json
{
  "attack_id": "attack-001",
  "strategy": "block_source_ips"
}
```

**Response:**
```json
{
  "success": true,
  "data": {
    "attack_id": "attack-001",
    "status": "mitigating",
    "strategy": "block_source_ips",
    "blocked_ips": ["192.168.1.100", "192.168.1.101"],
    "mitigated_at": "2026-01-15T10:30:00Z"
  }
}
```

---

## Quantum Cryptography APIs

### Generate Key Pair

Generate a post-quantum key pair.

**Endpoint:** `POST /quantum/keypair/generate`

**Request:**
```json
{
  "algorithm": "kyber",
  "key_size": 1024
}
```

**Response:**
```json
{
  "success": true,
  "data": {
    "keypair_id": "keypair-001",
    "algorithm": "kyber",
    "key_size": 1024,
    "public_key": "base64_encoded_public_key",
    "private_key": "base64_encoded_private_key",
    "created_at": "2026-01-15T10:30:00Z"
  }
}
```

### Encapsulate

Encapsulate a shared secret using a public key.

**Endpoint:** `POST /quantum/encapsulate`

**Request:**
```json
{
  "public_key": "base64_encoded_public_key",
  "algorithm": "kyber"
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

### Decapsulate

Decapsulate a shared secret using a private key.

**Endpoint:** `POST /quantum/decapsulate`

**Request:**
```json
{
  "ciphertext": "base64_encoded_ciphertext",
  "private_key": "base64_encoded_private_key",
  "algorithm": "kyber"
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

### Sign

Sign data using a post-quantum signature algorithm.

**Endpoint:** `POST /quantum/sign`

**Request:**
```json
{
  "data": "base64_encoded_data",
  "private_key": "base64_encoded_private_key",
  "algorithm": "dilithium"
}
```

**Response:**
```json
{
  "success": true,
  "data": {
    "signature": "base64_encoded_signature"
  }
}
```

### Verify

Verify a signature using a public key.

**Endpoint:** `POST /quantum/verify`

**Request:**
```json
{
  "data": "base64_encoded_data",
  "signature": "base64_encoded_signature",
  "public_key": "base64_encoded_public_key",
  "algorithm": "dilithium"
}
```

**Response:**
```json
{
  "success": true,
  "data": {
    "valid": true
  }
}
```

### Hybrid Encrypt

Encrypt data using hybrid encryption (classical + post-quantum).

**Endpoint:** `POST /quantum/hybrid/encrypt`

**Request:**
```json
{
  "data": "base64_encoded_data",
  "public_key_classical": "base64_encoded_public_key",
  "public_key_quantum": "base64_encoded_public_key"
}
```

**Response:**
```json
{
  "success": true,
  "data": {
    "ciphertext_classical": "base64_encoded_ciphertext",
    "ciphertext_quantum": "base64_encoded_ciphertext",
    "nonce": "base64_encoded_nonce"
  }
}
```

### Hybrid Decrypt

Decrypt data using hybrid encryption.

**Endpoint:** `POST /quantum/hybrid/decrypt`

**Request:**
```json
{
  "ciphertext_classical": "base64_encoded_ciphertext",
  "ciphertext_quantum": "base64_encoded_ciphertext",
  "nonce": "base64_encoded_nonce",
  "private_key_classical": "base64_encoded_private_key",
  "private_key_quantum": "base64_encoded_private_key"
}
```

**Response:**
```json
{
  "success": true,
  "data": {
    "data": "base64_encoded_data"
  }
}
```

---

## Behavioral Analysis APIs

### Monitor Behavior

Start monitoring behavior of a process.

**Endpoint:** `POST /behavioral/monitor`

**Request:**
```json
{
  "pid": 1234,
  "process_name": "unknown.exe",
  "monitor_level": "high"
}
```

**Response:**
```json
{
  "success": true,
  "data": {
    "monitor_id": "monitor-001",
    "pid": 1234,
    "process_name": "unknown.exe",
    "status": "monitoring",
    "monitor_level": "high"
  }
}
```

### Detect Pattern

Detect if a behavioral pattern matches known malicious patterns.

**Endpoint:** `POST /behavioral/pattern/detect`

**Request:**
```json
{
  "events": [
    {
      "type": "file_created",
      "timestamp": "2026-01-15T10:30:00Z",
      "details": {
        "file_path": "/path/to/file.exe",
        "file_size": 1024000
      }
    },
    {
      "type": "process_created",
      "timestamp": "2026-01-15T10:30:01Z",
      "details": {
        "process_name": "cmd.exe",
        "command_line": "cmd.exe /c malicious.bat"
      }
    }
  ]
}
```

**Response:**
```json
{
  "success": true,
  "data": {
    "pattern_detected": true,
    "pattern_name": "ransomware",
    "confidence": 0.92,
    "risk_level": "high",
    "matched_events": [0, 1],
    "recommendation": "quarantine"
  }
}
```

### Detect Anomaly

Detect if behavior is anomalous compared to baseline.

**Endpoint:** `POST /behavioral/anomaly/detect`

**Request:**
```json
{
  "pid": 1234,
  "process_name": "unknown.exe",
  "metrics": {
    "file_operations_per_minute": 150,
    "network_connections_per_minute": 20,
    "registry_changes_per_minute": 10
  }
}
```

**Response:**
```json
{
  "success": true,
  "data": {
    "anomaly_detected": true,
    "anomaly_score": 0.85,
    "risk_level": "high",
    "baseline": {
      "file_operations_per_minute": 10,
      "network_connections_per_minute": 2,
      "registry_changes_per_minute": 0
    },
    "deviation": {
      "file_operations_per_minute": 1400,
      "network_connections_per_minute": 900,
      "registry_changes_per_minute": 1000
    }
  }
}
```

### Calculate Risk Score

Calculate a risk score based on behavior.

**Endpoint:** `POST /behavioral/risk/calculate`

**Request:**
```json
{
  "pid": 1234,
  "process_name": "unknown.exe",
  "factors": {
    "pattern_matches": ["ransomware", "trojan"],
    "anomaly_score": 0.85,
    "reputation_score": 0.3,
    "file_hash_reputation": "malicious"
  }
}
```

**Response:**
```json
{
  "success": true,
  "data": {
    "risk_score": 0.92,
    "risk_level": "high",
    "factors": {
      "pattern_matches": {
        "weight": 0.4,
        "score": 0.95
      },
      "anomaly_score": {
        "weight": 0.3,
        "score": 0.85
      },
      "reputation_score": {
        "weight": 0.2,
        "score": 0.3
      },
      "file_hash_reputation": {
        "weight": 0.1,
        "score": 1.0
      }
    }
  }
}
```

---

## Threat Intelligence APIs

### Share Threat

Share a threat with the global threat intelligence network.

**Endpoint:** `POST /threat-intel/share`

**Request:**
```json
{
  "threat": {
    "type": "malware",
    "name": "Emotet",
    "hash": "sha256:abc123...",
    "severity": "high",
    "description": "Banking trojan",
    "indicators": [
      {
        "type": "domain",
        "value": "malicious-domain.com"
      },
      {
        "type": "ip",
        "value": "192.168.1.100"
      }
    ]
  }
}
```

**Response:**
```json
{
  "success": true,
  "data": {
    "threat_id": "threat-001",
    "status": "shared",
    "shared_at": "2026-01-15T10:30:00Z",
    "peers_notified": 150
  }
}
```

### Query Threat Database

Query the threat database for threats.

**Endpoint:** `POST /threat-intel/query`

**Request:**
```json
{
  "query": {
    "type": "hash",
    "value": "sha256:abc123..."
  }
}
```

**Response:**
```json
{
  "success": true,
  "data": {
    "threats": [
      {
        "threat_id": "threat-001",
        "type": "malware",
        "name": "Emotet",
        "hash": "sha256:abc123...",
        "severity": "high",
        "first_seen": "2026-01-10T00:00:00Z",
        "last_seen": "2026-01-15T10:30:00Z",
        "confidence": 0.95
      }
    ],
    "total": 1
  }
}
```

### Get Threat Statistics

Get statistics about threats in the database.

**Endpoint:** `GET /threat-intel/statistics`

**Response:**
```json
{
  "success": true,
  "data": {
    "total_threats": 1000000,
    "threats_by_type": {
      "malware": 500000,
      "ransomware": 200000,
      "trojan": 150000,
      "spyware": 100000,
      "adware": 50000
    },
    "threats_by_severity": {
      "critical": 100000,
      "high": 300000,
      "medium": 400000,
      "low": 200000
    },
    "recent_threats": {
      "last_24h": 1500,
      "last_7d": 10000,
      "last_30d": 40000
    }
  }
}
```

---

## SIEM Integration APIs

### Send Event

Send a security event to a SIEM platform.

**Endpoint:** `POST /siem/event/send`

**Request:**
```json
{
  "platform": "splunk",
  "event": {
    "timestamp": "2026-01-15T10:30:00Z",
    "event_type": "threat_detected",
    "severity": "high",
    "source": "sentinel",
    "details": {
      "threat_type": "malware",
      "confidence": 0.95,
      "file_hash": "sha256:abc123..."
    }
  }
}
```

**Response:**
```json
{
  "success": true,
  "data": {
    "event_id": "event-001",
    "platform": "splunk",
    "status": "sent",
    "sent_at": "2026-01-15T10:30:00Z"
  }
}
```

### Send Alert

Send a security alert to a SIEM platform.

**Endpoint:** `POST /siem/alert/send`

**Request:**
```json
{
  "platform": "splunk",
  "alert": {
    "timestamp": "2026-01-15T10:30:00Z",
    "alert_type": "critical_threat",
    "severity": "critical",
    "source": "sentinel",
    "title": "Critical malware detected",
    "description": "Malware detected with high confidence",
    "details": {
      "threat_type": "malware",
      "confidence": 0.95,
      "file_hash": "sha256:abc123..."
    }
  }
}
```

**Response:**
```json
{
  "success": true,
  "data": {
    "alert_id": "alert-001",
    "platform": "splunk",
    "status": "sent",
    "sent_at": "2026-01-15T10:30:00Z"
  }
}
```

### Get Supported Platforms

Get list of supported SIEM platforms.

**Endpoint:** `GET /siem/platforms`

**Response:**
```json
{
  "success": true,
  "data": {
    "platforms": [
      {
        "name": "splunk",
        "display_name": "Splunk",
        "supported": true,
        "features": ["events", "alerts"]
      },
      {
        "name": "qradar",
        "display_name": "IBM QRadar",
        "supported": true,
        "features": ["events", "alerts"]
      },
      {
        "name": "microsoft_sentinel",
        "display_name": "Microsoft Sentinel",
        "supported": true,
        "features": ["events", "alerts"]
      }
    ]
  }
}
```

---

## Error Codes

### Common Error Codes

| Code | HTTP Status | Description |
|------|-------------|-------------|
| `INVALID_REQUEST` | 400 | Invalid request parameters |
| `UNAUTHORIZED` | 401 | Authentication required |
| `FORBIDDEN` | 403 | Insufficient permissions |
| `NOT_FOUND` | 404 | Resource not found |
| `CONFLICT` | 409 | Resource conflict |
| `RATE_LIMITED` | 429 | Too many requests |
| `INTERNAL_ERROR` | 500 | Internal server error |
| `SERVICE_UNAVAILABLE` | 503 | Service temporarily unavailable |

### Core Error Codes

| Code | Description |
|------|-------------|
| `HYPERVISOR_NOT_INITIALIZED` | Hypervisor not initialized |
| `VM_NOT_FOUND` | Virtual machine not found |
| `VM_ALREADY_RUNNING` | Virtual machine already running |
| `VM_ALREADY_STOPPED` | Virtual machine already stopped |
| `MEMORY_PROTECTION_FAILED` | Memory protection failed |
| `PROCESS_NOT_FOUND` | Process not found |
| `PROCESS_ALREADY_MONITORING` | Process already being monitored |

### AI Error Codes

| Code | Description |
|------|-------------|
| `MODEL_NOT_LOADED` | AI model not loaded |
| `MODEL_LOAD_FAILED` | Failed to load AI model |
| `PREDICTION_FAILED` | Prediction failed |
| `FEATURE_EXTRACTION_FAILED` | Feature extraction failed |
| `INVALID_INPUT_TYPE` | Invalid input type for prediction |

### Gaming Error Codes

| Code | Description |
|------|-------------|
| `GAME_NOT_DETECTED` | Game not detected |
| `HANDSHAKE_FAILED` | Trusted handshake failed |
| `HANDSHAKE_EXPIRED` | Handshake session expired |
| `ZERO_SCAN_ALREADY_ACTIVE` | Zero-scan mode already active |
| `ZERO_SCAN_NOT_ACTIVE` | Zero-scan mode not active |
| `ATTACK_NOT_DETECTED` | No attack detected |

### Quantum Error Codes

| Code | Description |
|------|-------------|
| `UNSUPPORTED_ALGORITHM` | Unsupported quantum algorithm |
| `KEY_GENERATION_FAILED` | Key generation failed |
| `ENCAPSULATION_FAILED` | Encapsulation failed |
| `DECAPSULATION_FAILED` | Decapsulation failed |
| `SIGNING_FAILED` | Signing failed |
| `VERIFICATION_FAILED` | Verification failed |

---

## Rate Limiting

### Rate Limit Policy

| Tier | Requests per Minute | Requests per Hour |
|------|---------------------|-------------------|
| Free | 60 | 1000 |
| Basic | 600 | 10000 |
| Pro | 6000 | 100000 |
| Enterprise | Unlimited | Unlimited |

### Rate Limit Headers

All API responses include rate limit headers:

```http
X-RateLimit-Limit: 6000
X-RateLimit-Remaining: 5999
X-RateLimit-Reset: 1642237200
```

### Rate Limit Exceeded Response

```json
{
  "success": false,
  "data": null,
  "error": {
    "code": "RATE_LIMITED",
    "message": "Rate limit exceeded",
    "details": {
      "limit": 6000,
      "remaining": 0,
      "reset_at": "2026-01-15T11:00:00Z"
    }
  }
}
```

---

## API Versioning

### Versioning Strategy

SENTINEL uses URL-based versioning. The current version is `v1`.

### Version Format

```
https://api.sentinel.security/v{version}/{resource}
```

### Supported Versions

| Version | Status | Deprecation Date |
|---------|--------|------------------|
| v1 | Current | - |
| v0 | Deprecated | 2026-06-01 |

### Version Deprecation

When a version is deprecated:
- A `Deprecation` header is included in responses
- A `Sunset` header indicates the sunset date
- Clients are notified 6 months in advance

```http
Deprecation: true
Sunset: Sun, 01 Jun 2026 00:00:00 GMT
Link: <https://api.sentinel.security/v1>; rel="successor-version"
```

---

## SDKs

### Official SDKs

SENTINEL provides official SDKs for the following languages:

- **Python** - `pip install sentinel-sdk`
- **JavaScript/TypeScript** - `npm install @sentinel/sdk`
- **Go** - `go get github.com/sentinel/sdk-go`
- **Rust** - `cargo add sentinel-sdk`
- **Java** - Maven dependency
- **C#** - NuGet package

### SDK Example (Python)

```python
from sentinel import SentinelClient

# Initialize client
client = SentinelClient(
    api_key="your_api_key",
    base_url="https://api.sentinel.security/v1"
)

# Predict threat
result = client.ai.predict(
    type="file",
    data={
        "file_path": "/path/to/file.exe",
        "file_hash": "sha256:abc123...",
        "file_size": 1024000
    }
)

print(result.prediction.threat_type)  # "malware"
print(result.prediction.confidence)   # 0.95
```

---

## Support

### Documentation

- **API Reference**: https://docs.sentinel.security/api
- **SDK Documentation**: https://docs.sentinel.security/sdk
- **Examples**: https://github.com/sentinel/examples

### Support Channels

- **Email**: support@sentinel.security
- **Slack**: https://sentinel.slack.com
- **Discord**: https://discord.gg/sentinel
- **Status Page**: https://status.sentinel.security

### API Status

Check the current status of all API endpoints:
- **Status Page**: https://status.sentinel.security
- **API Health**: `GET /health`

---

## Changelog

### v1.0.0 (2026-01-15)
- Initial release of SENTINEL API v1
- Core APIs (Hypervisor, Memory, Process)
- AI Prediction APIs
- Gaming APIs (Trusted Handshake, Anti-DDoS)
- Quantum Cryptography APIs
- Behavioral Analysis APIs
- Threat Intelligence APIs
- SIEM Integration APIs
- Monitoring APIs
- Configuration APIs
- Audit APIs
- Performance APIs
- Security APIs

---

## License

© 2026 Vantis Corp. All rights reserved.

For more information, visit https://sentinel.security