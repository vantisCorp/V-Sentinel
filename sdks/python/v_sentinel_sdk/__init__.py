"""
V-Sentinel Python SDK
~~~~~~~~~~~~~~~~~~~~~~

Official Python SDK for V-Sentinel - Next-generation AI-native security system
with quantum-ready cryptography.

Features:
    - Threat Intelligence API client
    - Malware Detection integration
    - Zero Trust Architecture support
    - IOC (Indicator of Compromise) management
    - Real-time alert streaming
    - MCP (Model Context Protocol) integration

Example:
    >>> from v_sentinel_sdk import VSentinelClient
    >>> 
    >>> # Initialize client
    >>> client = VSentinelClient(api_key="your-api-key")
    >>> 
    >>> # Get detections
    >>> detections = await client.detections.list(limit=10)
    >>> 
    >>> # Check IOC
    >>> ioc_result = await client.threat_intel.check_ioc("192.0.2.1")
    >>> print(ioc_result)
"""

__version__ = "1.0.0"
__author__ = "V-Sentinel Security Team"
__email__ = "security@vantis.ai"
__license__ = "MIT"

from v_sentinel_sdk.client import VSentinelClient
from v_sentinel_sdk.models.detections import Detection, DetectionSeverity
from v_sentinel_sdk.models.hosts import Host, HostState
from v_sentinel_sdk.models.incidents import Incident, IncidentStatus
from v_sentinel_sdk.models.threat_intel import IOC, IOCType, ThreatActor
from v_sentinel_sdk.exceptions import (
    VSentinelError,
    AuthenticationError,
    RateLimitError,
    ResourceNotFoundError,
    ValidationError,
)

__all__ = [
    # Client
    "VSentinelClient",
    # Models
    "Detection",
    "DetectionSeverity",
    "Host",
    "HostState",
    "Incident",
    "IncidentStatus",
    "IOC",
    "IOCType",
    "ThreatActor",
    # Exceptions
    "VSentinelError",
    "AuthenticationError",
    "RateLimitError",
    "ResourceNotFoundError",
    "ValidationError",
    # Metadata
    "__version__",
    "__author__",
    "__email__",
    "__license__",
]