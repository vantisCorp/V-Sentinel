# V-Sentinel API Reference

This document provides comprehensive API documentation for the V-Sentinel security platform, designed for AI agents and developers.

## Overview

V-Sentinel provides a comprehensive RESTful API for interacting with all security capabilities, including threat detection, incident response, and threat intelligence.

## Base URL

```
https://api.vsentinel.com/v1
```

## Authentication

All API requests require authentication using Bearer tokens:

```
Authorization: Bearer <your_api_token>
```

## Rate Limiting

- Standard: 1000 requests per minute
- Premium: 5000 requests per minute

## Endpoints

### Detections

#### Search Detections

```
GET /detections
```

Query parameters:
- `query` (string): FQL-style query
- `time_range` (string): Time range (e.g., "1h", "24h", "7d")
- `limit` (integer): Maximum results (default: 100, max: 1000)
- `sort_by` (string): Sort field (default: "timestamp")

Example:
```bash
curl -H "Authorization: Bearer <token>" \
  "https://api.vsentinel.com/v1/detections?query=severity:critical&time_range=24h"
```

Response:
```json
{
  "detections": [
    {
      "id": "det_001",
      "name": "Suspicious PowerShell Execution",
      "severity": "critical",
      "timestamp": "2026-03-06T10:30:00Z",
      "host_id": "host_001",
      "details": {
        "process": "powershell.exe",
        "command": "Invoke-Expression"
      }
    }
  ],
  "total": 1
}
```

### Hosts

#### List Hosts

```
GET /hosts
```

Query parameters:
- `host_id` (string): Specific host ID or hostname
- `state` (string): Filter by state (online, offline, unknown)
- `platform` (string): Filter by platform (Windows, Linux, macOS)

Example:
```bash
curl -H "Authorization: Bearer <token>" \
  "https://api.vsentinel.com/v1/host?state=online&platform=Windows"
```

### Incidents

#### List Incidents

```
GET /incidents
```

Query parameters:
- `incident_id` (string): Specific incident ID
- `severity` (string): Filter by severity (critical, high, medium, low)
- `status` (string): Filter by status (new, in_progress, resolved, closed)
- `limit` (integer): Maximum results (default: 50, max: 500)

### Threat Intelligence

#### Query Threat Intel

```
GET /threat-intel
```

Query parameters:
- `ioc` (string): Indicator of Compromise
- `ioc_type` (string): Type (ip, domain, hash, url)
- `threat_actor` (string): Threat actor name
- `malware_family` (string): Malware family name

Example:
```bash
curl -H "Authorization: Bearer <token>" \
  "https://api.vsentinel.com/v1/threat-intel?ioc=192.168.1.100&ioc_type=ip"
```

## Error Codes

| Code | Description |
|------|-------------|
| 400 | Bad Request |
| 401 | Unauthorized |
| 403 | Forbidden |
| 404 | Not Found |
| 429 | Rate Limit Exceeded |
| 500 | Internal Server Error |

## Webhooks

V-Sentinel supports webhooks for real-time notifications:

### Create Webhook

```
POST /webhooks
```

Request body:
```json
{
  "url": "https://your-webhook-url.com",
  "events": ["detection.created", "incident.updated"],
  "secret": "your_webhook_secret"
}
```

## SDKs

Official SDKs are available for:
- Python
- Go
- TypeScript/JavaScript
- Rust

See the [V-Sentinel GitHub organization](https://github.com/vantisCorp) for SDK repositories.