# SENTINEL API Documentation

## Overview

The SENTINEL API provides comprehensive security capabilities including threat prediction, file scanning, process monitoring, network protection, quantum cryptography, gaming mode, threat intelligence, and SIEM integration.

## Base URL

```
https://api.sentinel.ai/v1
```

## Authentication

All API requests require authentication using an API key or JWT token.

### API Key Authentication

```http
X-API-Key: your-api-key
```

### JWT Authentication

```http
Authorization: Bearer your-jwt-token
```

## Rate Limits

- **Free Tier:** 100 requests/minute
- **Standard Tier:** 1,000 requests/minute
- **Enterprise Tier:** Unlimited

## API Endpoints

### Health

#### GET /health
Check the health status of the API.

**Response:**
```json
{
  "status": "healthy",
  "version": "1.0.0",
  "timestamp": "2025-03-03T12:00:00Z"
}
```

---

### Threat Prediction

#### POST /predict
Predict whether a file, process, or network connection is malicious using AI.

**Request Body:**
```json
{
  "features": {
    "process_name": "suspicious.exe",
    "file_hash": "sha256:abc123...",
    "file_size": 1024000,
    "network_connections": ["192.168.1.1:443"],
    "registry_keys": ["HKLM\\Software\\Suspicious"]
  }
}
```

**Response:**
```json
{
  "threat_level": "high",
  "confidence": 0.95,
  "threat_type": "malware",
  "recommendations": ["isolate", "quarantine"],
  "prediction_id": "pred_12345",
  "timestamp": "2025-03-03T12:00:00Z"
}
```

#### POST /predict/batch
Predict threats for multiple items in a single request.

**Request Body:**
```json
{
  "items": [
    {
      "features": {
        "process_name": "file1.exe",
        "file_hash": "sha256:abc123..."
      }
    },
    {
      "features": {
        "process_name": "file2.exe",
        "file_hash": "sha256:def456..."
      }
    }
  ]
}
```

**Response:**
```json
{
  "predictions": [
    {
      "threat_level": "high",
      "confidence": 0.95,
      "threat_type": "malware"
    }
  ],
  "batch_id": "batch_12345",
  "timestamp": "2025-03-03T12:00:00Z"
}
```

---

### File Scanning

#### POST /scan/file
Scan a file for malware and threats with deep analysis.

**Request Body (multipart/form-data):**
```
file: binary file data
deep_scan: false (optional)
heuristics: true (optional)
cloud_lookup: true (optional)
```

**Response:**
```json
{
  "scan_id": "scan_12345",
  "status": "completed",
  "file_info": {
    "name": "suspicious.exe",
    "size": 1024000,
    "hash": "sha256:abc123..."
  },
  "threats_found": 2,
  "threats": [
    {
      "type": "trojan",
      "name": "Trojan.GenericKD.123",
      "severity": "high",
      "description": "Generic trojan detected"
    }
  ],
  "scan_duration": 2.5,
  "timestamp": "2025-03-03T12:00:00Z"
}
```

#### GET /scan/file/{scan_id}
Get the result of a file scan by scan ID.

**Parameters:**
- `scan_id` (path) - Scan ID

**Response:**
```json
{
  "scan_id": "scan_12345",
  "status": "completed",
  "file_info": {
    "name": "suspicious.exe",
    "size": 1024000,
    "hash": "sha256:abc123..."
  },
  "threats_found": 2,
  "threats": [...],
  "scan_duration": 2.5,
  "timestamp": "2025-03-03T12:00:00Z"
}
```

---

### Process Monitoring

#### POST /monitor/start
Start real-time process monitoring with optional filters.

**Request Body:**
```json
{
  "filters": {
    "process_names": ["chrome.exe", "firefox.exe"],
    "monitor_children": true,
    "pid_filter": [1234, 5678]
  },
  "callback_url": "https://your-server.com/webhook",
  "event_types": ["process_start", "threat_detected"]
}
```

**Response:**
```json
{
  "monitoring_id": "monitor_12345",
  "status": "started",
  "timestamp": "2025-03-03T12:00:00Z"
}
```

#### POST /monitor/stop/{monitoring_id}
Stop a running process monitoring session.

**Parameters:**
- `monitoring_id` (path) - Monitoring ID

**Response:**
```json
{
  "monitoring_id": "monitor_12345",
  "status": "stopped",
  "timestamp": "2025-03-03T12:00:00Z"
}
```

---

### Network Protection

#### POST /network/protect
Enable network protection with configurable rules.

**Request Body:**
```json
{
  "rules": {
    "block_malicious_ips": true,
    "block_suspicious_domains": true,
    "ddos_protection": true,
    "rate_limit": {
      "enabled": true,
      "requests_per_second": 1000
    },
    "firewall_rules": [
      {
        "action": "allow",
        "source": "0.0.0.0/0",
        "destination": "192.168.1.0/24",
        "port": 80,
        "protocol": "tcp"
      }
    ]
  }
}
```

**Response:**
```json
{
  "protection_id": "protect_12345",
  "status": "enabled",
  "rules": {...},
  "timestamp": "2025-03-03T12:00:00Z"
}
```

#### GET /network/events
Retrieve network protection events.

**Query Parameters:**
- `limit` (optional) - Maximum number of events (default: 100, max: 1000)
- `offset` (optional) - Offset for pagination (default: 0)
- `event_type` (optional) - Filter by event type (blocked, allowed, suspicious, ddos)
- `start_time` (optional) - Start time filter (ISO 8601)
- `end_time` (optional) - End time filter (ISO 8601)

**Response:**
```json
{
  "events": [
    {
      "event_id": "evt_12345",
      "event_type": "blocked",
      "source_ip": "192.168.1.1",
      "destination_ip": "10.0.0.1",
      "port": 443,
      "protocol": "tcp",
      "timestamp": "2025-03-03T12:00:00Z"
    }
  ],
  "total": 50,
  "limit": 100,
  "offset": 0
}
```

---

## Error Responses

All error responses follow this format:

```json
{
  "error": {
    "code": "ERROR_CODE",
    "message": "Error description",
    "details": {}
  },
  "timestamp": "2025-03-03T12:00:00Z"
}
```

### Error Codes

| Code | Description |
|------|-------------|
| INVALID_REQUEST | Invalid request parameters |
| UNAUTHORIZED | Authentication required |
| FORBIDDEN | Insufficient permissions |
| NOT_FOUND | Resource not found |
| RATE_LIMIT_EXCEEDED | Rate limit exceeded |
| INTERNAL_ERROR | Internal server error |
| SERVICE_UNAVAILABLE | Service temporarily unavailable |

---

## Status Codes

| Code | Description |
|------|-------------|
| 200 | Success |
| 201 | Created |
| 400 | Bad Request |
| 401 | Unauthorized |
| 403 | Forbidden |
| 404 | Not Found |
| 429 | Too Many Requests |
| 500 | Internal Server Error |
| 503 | Service Unavailable |

---

## SDK Integration

### Python SDK

```python
from sentinel_sdk import SentinelClient

client = SentinelClient(api_key="your-api-key")

# Predict threat
result = client.predict_threat(
    features={
        "process_name": "test.exe",
        "file_hash": "sha256:abc123..."
    }
)
print(f"Threat: {result['threat_level']}")
```

### JavaScript/Node.js SDK

```javascript
const { SentinelClient } = require('@sentinel/sdk');

const client = new SentinelClient({
    apiKey: 'your-api-key'
});

const result = await client.predictThreat({
    processName: 'test.exe',
    fileHash: 'sha256:abc123...'
});
console.log(`Threat: ${result.threatLevel}`);
```

---

## Support

- **Documentation:** https://sentinel.ai/docs/api
- **Email:** support@sentinel.ai
- **Discord:** https://discord.gg/sentinel
- **GitHub:** https://github.com/vantisCorp/V-Sentinel
- **Status Page:** https://status.sentinel.ai

---

## Changelog

### Version 1.0.0 (March 2025)
- Initial release
- Health check endpoint
- Threat prediction APIs
- File scanning APIs
- Process monitoring APIs
- Network protection APIs
- Error handling

---

## License

MIT License - See LICENSE file for details.

---

*© 2025 SENTINEL. All rights reserved. | vantisCorp*

*Last Updated: March 3, 2025*