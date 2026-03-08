# V-Sentinel Python SDK

[![Python 3.9+](https://img.shields.io/badge/python-3.9+-blue.svg)](https://www.python.org/downloads/)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Code style: black](https://img.shields.io/badge/code%20style-black-000000.svg)](https://github.com/psf/black)

Official Python SDK for the V-Sentinel security operations platform. Provides async-first API client with comprehensive type hints, Pydantic models, and seamless integration with modern Python applications.

## Features

- **Async-first design** - Built on httpx for high-performance async operations
- **Type-safe** - Full type hints with Pydantic v2 models for runtime validation
- **Comprehensive API coverage** - Detections, Hosts, Incidents, and Threat Intelligence
- **Automatic retries** - Configurable retry logic with exponential backoff
- **Structured logging** - JSON-formatted logs for easy integration
- **MCP Support** - Model Context Protocol integration for AI assistants
- **Modern Python** - Supports Python 3.9-3.12 with modern packaging

## Installation

```bash
# Using pip
pip install v-sentinel-sdk

# Using poetry
poetry add v-sentinel-sdk

# Using pipenv
pipenv install v-sentinel-sdk
```

## Quick Start

```python
import asyncio
from v_sentinel_sdk import VSentinelClient
from v_sentinel_sdk.models.detections import DetectionSeverity

async def main():
    # Initialize client (async context manager recommended)
    async with VSentinelClient(api_key="your-api-key") as client:
        # List recent critical detections
        detections = await client.detections.list(
            severity=DetectionSeverity.CRITICAL,
            time_range="24h",
            limit=10
        )
        
        for detection in detections.items:
            print(f"[{detection.severity.value}] {detection.title}")
            print(f"  Host: {detection.host.hostname}")
            print(f"  MITRE Techniques: {', '.join(detection.techniques)}")
        
        # Check an IOC against threat intelligence
        result = await client.threat_intel.check_ioc(
            value="192.0.2.1",
        )
        
        if result.is_malicious:
            print(f"Malicious IOC detected!")
            print(f"  Threat Actor: {result.threat_actor}")
            print(f"  Confidence: {result.confidence}")

asyncio.run(main())
```

## Authentication

The SDK supports multiple authentication methods:

### API Key (Recommended)

```python
from v_sentinel_sdk import VSentinelClient

client = VSentinelClient(
    api_key="your-api-key",
    base_url="https://api.vantis.ai/v1"  # Optional, defaults to production
)
```

### Environment Variable

```python
import os
from v_sentinel_sdk import VSentinelClient

# Set V_SENTINEL_API_KEY environment variable
client = VSentinelClient(api_key=os.environ["V_SENTINEL_API_KEY"])
```

### Configuration File

```python
from v_sentinel_sdk.models.config import ClientConfig

config = ClientConfig(
    api_key="your-api-key",
    base_url="https://api.vantis.ai/v1",
    timeout=60,
    max_retries=5,
    verify_ssl=True,
)

client = VSentinelClient.from_config(config)
```

## API Reference

### Detections

Manage security detections and alerts.

```python
# List detections with filters
detections = await client.detections.list(
    severity=DetectionSeverity.HIGH,
    status=DetectionStatus.NEW,
    time_range="7d",
    host_id="host-123",
    limit=50
)

# Get a specific detection
detection = await client.detections.get("detection-456")

# Update detection status
updated = await client.detections.update_status(
    detection_id="detection-456",
    status=DetectionStatus.IN_PROGRESS,
    notes="Investigating potential false positive"
)

# Assign detection to analyst
assigned = await client.detections.assign(
    detection_id="detection-456",
    assignee="analyst@company.com"
)

# Add note to detection
noted = await client.detections.add_note(
    detection_id="detection-456",
    note="Confirmed as true positive. Escalating to incident."
)
```

### Hosts

Manage endpoint inventory and isolation.

```python
from v_sentinel_sdk.models.hosts import HostState, HostPlatform

# List hosts
hosts = await client.hosts.list(
    platform=HostPlatform.WINDOWS,
    state=HostState.ONLINE,
    is_isolated=False,
    limit=100
)

# Get host details
host = await client.hosts.get("host-123")

# Search hosts by hostname
results = await client.hosts.search("workstation-")

# Isolate a host
isolated = await client.hosts.isolate(
    host_id="host-123",
    reason="Malware detected - immediate containment required"
)

# Get host detections
host_detections = await client.hosts.get_detections("host-123", limit=20)

# Add tag to host
tagged = await client.hosts.add_tag("host-123", "investigated")
```

### Incidents

Manage security incident lifecycle.

```python
from v_sentinel_sdk.models.incidents import IncidentSeverity, IncidentStatus, IncidentPhase

# Create an incident
incident = await client.incidents.create(
    title="Ransomware Detection on Finance Server",
    description="LockBit ransomware detected on FIN-SRV-01",
    severity=IncidentSeverity.CRITICAL,
    hosts=["host-123", "host-456"],
    detections=["detection-789"],
    tags=["ransomware", "lockbit", "finance"]
)

# List incidents
incidents = await client.incidents.list(
    severity=IncidentSeverity.CRITICAL,
    status=IncidentStatus.IN_PROGRESS,
    time_range="30d"
)

# Update incident phase
updated = await client.incidents.update_phase(
    incident_id=incident.id,
    phase=IncidentPhase.CONTAINMENT,
    notes="Hosts isolated, ransomware binaries removed"
)

# Add hosts to incident
updated = await client.incidents.add_hosts(
    incident_id=incident.id,
    host_ids=["host-789"]
)

# Add timeline entry
timeline = await client.incidents.get_timeline(incident.id)

# Close incident
closed = await client.incidents.close(
    incident_id=incident.id,
    resolution="Ransomware contained and eradicated. Systems restored from backup.",
    lessons_learned="Consider adding additional email filtering for phishing attachments."
)
```

### Threat Intelligence

Manage IOCs and threat actor data.

```python
from v_sentinel_sdk.models.threat_intel import IOCType, IOCConfidence

# Check an IOC
result = await client.threat_intel.check_ioc("malicious.example.com")

if result.is_malicious:
    print(f"Threat Actor: {result.threat_actor}")
    print(f"Related IOCs: {len(result.related_iocs)}")

# Bulk check IOCs
results = await client.threat_intel.check_iocs_bulk([
    "192.0.2.1",
    "malware.hash.example",
    "https://malicious.url/path"
])

# Add an IOC
ioc = await client.threat_intel.add_ioc(
    value="192.0.2.100",
    ioc_type=IOCType.IP,
    confidence=IOCConfidence.HIGH,
    threat_actor="APT29",
    campaign="CozyBear-2024",
    malware_family="SUNBURST",
    tags=["apt29", "supply-chain"],
    description="C2 server identified in SolarWinds campaign"
)

# Get threat actor information
actor = await client.threat_intel.get_threat_actor("APT29")
print(f"Aliases: {', '.join(actor.aliases)}")
print(f"MITRE ID: {actor.mitre_id}")

# Get actor's IOCs
actor_iocs = await client.threat_intel.get_threat_actor_iocs("APT29")

# Export IOCs for SIEM
siem_rules = await client.threat_intel.export_for_siEM(
    siem_type="splunk",
    ioc_types=[IOCType.IP, IOCType.DOMAIN, IOCType.URL]
)
```

## Advanced Configuration

### Custom Retry Configuration

```python
from v_sentinel_sdk import VSentinelClient
from v_sentinel_sdk.utils.retry import RetryConfig

retry_config = RetryConfig(
    max_attempts=5,
    base_delay=2.0,
    max_delay=120.0,
    exponential_base=2.0,
    jitter=True,
    retryable_status_codes=[429, 500, 502, 503, 504]
)

client = VSentinelClient(
    api_key="your-api-key",
    max_retries=5,
    timeout=60
)
```

### Structured Logging

```python
from v_sentinel_sdk.utils.logging import configure_logging, LogLevel

# Enable JSON logging
configure_logging(
    level=LogLevel.DEBUG,
    json_format=True,
    output="stderr"
)

# Or use simple logging
configure_logging(level="INFO")
```

### Proxy Configuration

```python
client = VSentinelClient(
    api_key="your-api-key",
    proxy="http://proxy.company.com:8080"
)
```

### Custom SSL Verification

```python
client = VSentinelClient(
    api_key="your-api-key",
    verify_ssl=False  # Not recommended for production
)

# Or with custom CA bundle
client = VSentinelClient(
    api_key="your-api-key",
    verify_ssl="/path/to/ca-bundle.crt"
)
```

## Error Handling

The SDK provides specific exception types for different error scenarios:

```python
from v_sentinel_sdk.exceptions import (
    VSentinelError,
    AuthenticationError,
    RateLimitError,
    ResourceNotFoundError,
    ValidationError,
    ConnectionError,
    TimeoutError,
)

try:
    detection = await client.detections.get("invalid-id")
except ResourceNotFoundError as e:
    print(f"Resource not found: {e.resource_id}")
except AuthenticationError:
    print("Invalid API key")
except RateLimitError as e:
    print(f"Rate limited. Retry after: {e.retry_after} seconds")
except TimeoutError:
    print("Request timed out")
except VSentinelError as e:
    print(f"SDK error: {e}")
```

## Async and Sync Usage

The SDK is async-first, but you can use it with `asyncio.run()`:

```python
import asyncio
from v_sentinel_sdk import VSentinelClient

async def get_detection(detection_id: str):
    async with VSentinelClient(api_key="key") as client:
        return await client.detections.get(detection_id)

# In async context
detection = await get_detection("det-123")

# In sync context
detection = asyncio.run(get_detection("det-123"))
```

## Type Hints

All methods return properly typed Pydantic models:

```python
from v_sentinel_sdk.models.detections import Detection, DetectionList
from v_sentinel_sdk.models.hosts import Host, HostList
from v_sentinel_sdk.models.incidents import Incident, IncidentList
from v_sentinel_sdk.models.threat_intel import IOC, IOCList, IOCCheckResult

# IDE autocomplete and type checking work out of the box
detection: Detection = await client.detections.get("det-123")
hosts: HostList = await client.hosts.list()
```

## Development

### Setup Development Environment

```bash
# Clone the repository
git clone https://github.com/vantis-ai/v-sentinel-sdks.git
cd v-sentinel-sdks/python

# Create virtual environment
python -m venv .venv
source .venv/bin/activate  # or .venv\Scripts\activate on Windows

# Install development dependencies
pip install -e ".[dev]"

# Run tests
pytest

# Run type checking
mypy v_sentinel_sdk

# Format code
black v_sentinel_sdk
ruff check v_sentinel_sdk
```

### Running Tests

```bash
# Run all tests
pytest

# Run with coverage
pytest --cov=v_sentinel_sdk --cov-report=html

# Run specific test file
pytest tests/test_detections.py

# Run with verbose output
pytest -v
```

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Support

- **Documentation**: [https://docs.vantis.ai/sdks/python](https://docs.vantis.ai/sdks/python)
- **API Reference**: [https://api.vantis.ai/docs](https://api.vantis.ai/docs)
- **GitHub Issues**: [https://github.com/vantis-ai/v-sentinel-sdks/issues](https://github.com/vantis-ai/v-sentinel-sdks/issues)
- **Security Issues**: security@vantis.ai

## Contributing

We welcome contributions! Please see our [Contributing Guide](CONTRIBUTING.md) for details.

---

Built with ❤️ by the Vantis AI team