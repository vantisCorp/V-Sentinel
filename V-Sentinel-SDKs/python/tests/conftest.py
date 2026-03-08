"""
Pytest configuration and fixtures for V-Sentinel SDK tests.
"""

import asyncio
from typing import AsyncGenerator, Generator
from unittest.mock import AsyncMock, MagicMock, patch

import pytest
import httpx


@pytest.fixture(scope="session")
def event_loop() -> Generator[asyncio.AbstractEventLoop, None, None]:
    """Create event loop for async tests."""
    loop = asyncio.new_event_loop()
    yield loop
    loop.close()


@pytest.fixture
def mock_httpx_client() -> AsyncMock:
    """Create a mock httpx AsyncClient."""
    mock = AsyncMock(spec=httpx.AsyncClient)
    return mock


@pytest.fixture
def api_base_url() -> str:
    """Return test API base URL."""
    return "https://api.test.vantis.ai/v1"


@pytest.fixture
def api_key() -> str:
    """Return test API key."""
    return "test-api-key-12345"


@pytest.fixture
def mock_response() -> MagicMock:
    """Create a mock HTTP response."""
    response = MagicMock(spec=httpx.Response)
    response.status_code = 200
    response.json = MagicMock(return_value={})
    return response


@pytest.fixture
def detection_response() -> dict:
    """Return sample detection response data."""
    return {
        "id": "det-12345",
        "title": "Suspicious PowerShell Execution",
        "description": "PowerShell script with encoded command detected",
        "severity": "HIGH",
        "status": "NEW",
        "techniques": ["T1059.001", "T1027"],
        "tactics": ["EXECUTION", "DEFENSE_EVASION"],
        "host": {
            "id": "host-001",
            "hostname": "WORKSTATION-01",
            "platform": "WINDOWS",
            "state": "ONLINE",
            "ip_addresses": ["192.168.1.100"],
            "is_isolated": False,
            "tags": [],
            "created_at": "2024-01-15T10:00:00Z",
            "updated_at": "2024-01-15T10:00:00Z",
        },
        "indicators": [
            {
                "type": "COMMAND",
                "value": "powershell -enc UGF5bG9hZA==",
                "description": "Encoded PowerShell command"
            }
        ],
        "raw_data": {},
        "assignee": None,
        "notes": [],
        "tags": ["powershell", "suspicious"],
        "created_at": "2024-01-15T10:30:00Z",
        "updated_at": "2024-01-15T10:30:00Z",
    }


@pytest.fixture
def detection_list_response() -> dict:
    """Return sample detection list response data."""
    return {
        "items": [
            {
                "id": "det-001",
                "title": "Detection 1",
                "description": "Description 1",
                "severity": "HIGH",
                "status": "NEW",
                "techniques": [],
                "tactics": [],
                "host": {
                    "id": "host-001",
                    "hostname": "HOST-001",
                    "platform": "WINDOWS",
                    "state": "ONLINE",
                    "ip_addresses": ["192.168.1.1"],
                    "is_isolated": False,
                    "tags": [],
                    "created_at": "2024-01-15T10:00:00Z",
                    "updated_at": "2024-01-15T10:00:00Z",
                },
                "indicators": [],
                "raw_data": {},
                "assignee": None,
                "notes": [],
                "tags": [],
                "created_at": "2024-01-15T10:00:00Z",
                "updated_at": "2024-01-15T10:00:00Z",
            },
            {
                "id": "det-002",
                "title": "Detection 2",
                "description": "Description 2",
                "severity": "MEDIUM",
                "status": "IN_PROGRESS",
                "techniques": [],
                "tactics": [],
                "host": {
                    "id": "host-002",
                    "hostname": "HOST-002",
                    "platform": "LINUX",
                    "state": "ONLINE",
                    "ip_addresses": ["192.168.1.2"],
                    "is_isolated": False,
                    "tags": [],
                    "created_at": "2024-01-15T10:00:00Z",
                    "updated_at": "2024-01-15T10:00:00Z",
                },
                "indicators": [],
                "raw_data": {},
                "assignee": "analyst@test.com",
                "notes": [],
                "tags": [],
                "created_at": "2024-01-15T10:00:00Z",
                "updated_at": "2024-01-15T10:00:00Z",
            },
        ],
        "total": 2,
        "limit": 50,
        "offset": 0,
        "has_more": False,
    }


@pytest.fixture
def host_response() -> dict:
    """Return sample host response data."""
    return {
        "id": "host-001",
        "hostname": "WORKSTATION-01",
        "platform": "WINDOWS",
        "state": "ONLINE",
        "ip_addresses": ["192.168.1.100", "fe80::1"],
        "os_version": "Windows 10 Enterprise",
        "agent_version": "2.1.0",
        "is_isolated": False,
        "last_seen_at": "2024-01-15T10:00:00Z",
        "first_seen_at": "2024-01-01T08:00:00Z",
        "tags": ["finance", "workstation"],
        "created_at": "2024-01-01T08:00:00Z",
        "updated_at": "2024-01-15T10:00:00Z",
    }


@pytest.fixture
def incident_response() -> dict:
    """Return sample incident response data."""
    return {
        "id": "inc-001",
        "title": "Ransomware Attack Detected",
        "description": "LockBit ransomware detected on FIN-SRV-01",
        "severity": "CRITICAL",
        "status": "IN_PROGRESS",
        "phase": "CONTAINMENT",
        "hosts": ["host-001", "host-002"],
        "detections": ["det-001", "det-002", "det-003"],
        "assignee": "analyst@company.com",
        "timeline": [
            {
                "timestamp": "2024-01-15T10:00:00Z",
                "action": "incident_created",
                "description": "Incident created from detection",
                "user": "system",
            }
        ],
        "notes": [],
        "tags": ["ransomware", "lockbit"],
        "created_at": "2024-01-15T10:00:00Z",
        "updated_at": "2024-01-15T12:00:00Z",
    }


@pytest.fixture
def ioc_response() -> dict:
    """Return sample IOC response data."""
    return {
        "id": "ioc-001",
        "type": "IP",
        "value": "192.0.2.1",
        "confidence": "HIGH",
        "threat_actor": "APT29",
        "campaign": "CozyBear-2024",
        "malware_family": "SUNBURST",
        "tags": ["apt", "c2"],
        "description": "C2 server",
        "references": ["https://example.com/report"],
        "first_seen": "2024-01-01T00:00:00Z",
        "last_seen": "2024-01-15T00:00:00Z",
        "expires_at": "2024-07-15T00:00:00Z",
        "created_at": "2024-01-01T00:00:00Z",
        "updated_at": "2024-01-15T00:00:00Z",
    }


@pytest.fixture
def ioc_check_result_response() -> dict:
    """Return sample IOC check result response data."""
    return {
        "ioc": {
            "id": "ioc-001",
            "type": "IP",
            "value": "192.0.2.1",
            "confidence": "HIGH",
            "threat_actor": "APT29",
            "campaign": None,
            "malware_family": None,
            "tags": ["c2"],
            "description": "Known C2 server",
            "references": [],
            "first_seen": "2024-01-01T00:00:00Z",
            "last_seen": "2024-01-15T00:00:00Z",
            "expires_at": None,
            "created_at": "2024-01-01T00:00:00Z",
            "updated_at": "2024-01-15T00:00:00Z",
        },
        "is_malicious": True,
        "confidence": "HIGH",
        "threat_actor": "APT29",
        "malware_family": None,
        "related_iocs": [
            {
                "id": "ioc-002",
                "type": "DOMAIN",
                "value": "malicious.example.com",
                "confidence": "MEDIUM",
                "threat_actor": "APT29",
                "campaign": None,
                "malware_family": None,
                "tags": [],
                "description": None,
                "references": [],
                "first_seen": "2024-01-01T00:00:00Z",
                "last_seen": "2024-01-15T00:00:00Z",
                "expires_at": None,
                "created_at": "2024-01-01T00:00:00Z",
                "updated_at": "2024-01-15T00:00:00Z",
            }
        ],
    }