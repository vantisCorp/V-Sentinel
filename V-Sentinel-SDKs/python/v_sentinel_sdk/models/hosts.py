"""
Host Models
~~~~~~~~~~~

Pydantic models for V-Sentinel hosts.
"""

from datetime import datetime
from enum import Enum
from typing import Any, Dict, List, Optional

from pydantic import BaseModel, Field


class HostState(str, Enum):
    """Host operational state."""
    
    ONLINE = "online"
    OFFLINE = "offline"
    ISOLATED = "isolated"
    UNKNOWN = "unknown"
    
    def __str__(self) -> str:
        return self.value


class HostPlatform(str, Enum):
    """Host operating system platform."""
    
    WINDOWS = "windows"
    LINUX = "linux"
    MACOS = "macos"
    UNKNOWN = "unknown"
    
    def __str__(self) -> str:
        return self.value


class HostGroup(BaseModel):
    """Host group membership."""
    
    id: str = Field(..., description="Group ID")
    name: str = Field(..., description="Group name")
    type: str = Field(default="static", description="Group type (static/dynamic)")


class HostPolicy(BaseModel):
    """Applied security policy."""
    
    id: str = Field(..., description="Policy ID")
    name: str = Field(..., description="Policy name")
    applied_at: datetime = Field(..., description="Application timestamp")


class Host(BaseModel):
    """
    V-Sentinel Host.
    
    Represents a managed host/endpoint in V-Sentinel.
    
    Example:
        >>> host = Host(
        ...     id="host-001",
        ...     hostname="workstation-01",
        ...     platform=HostPlatform.WINDOWS,
        ...     state=HostState.ONLINE
        ... )
    """
    
    id: str = Field(..., description="Unique host ID")
    hostname: str = Field(..., description="Host name")
    platform: HostPlatform = Field(..., description="Operating system platform")
    state: HostState = Field(default=HostState.UNKNOWN, description="Current state")
    
    # Network information
    ip_addresses: List[str] = Field(default_factory=list, description="IP addresses")
    mac_addresses: List[str] = Field(default_factory=list, description="MAC addresses")
    domain: Optional[str] = Field(None, description="Domain name")
    
    # OS details
    os_version: Optional[str] = Field(None, description="OS version string")
    os_build: Optional[str] = Field(None, description="OS build number")
    kernel_version: Optional[str] = Field(None, description="Kernel version (Linux)")
    
    # Hardware information
    manufacturer: Optional[str] = Field(None, description="Hardware manufacturer")
    model: Optional[str] = Field(None, description="Hardware model")
    serial_number: Optional[str] = Field(None, description="Serial number")
    
    # Agent information
    agent_version: Optional[str] = Field(None, description="V-Sentinel agent version")
    agent_installed_at: Optional[datetime] = Field(None, description="Agent installation date")
    last_seen: Optional[datetime] = Field(None, description="Last communication timestamp")
    
    # Security status
    is_isolated: bool = Field(default=False, description="Network isolation status")
    isolation_reason: Optional[str] = Field(None, description="Reason for isolation")
    
    # Groups and policies
    groups: List[HostGroup] = Field(default_factory=list, description="Group memberships")
    policies: List[HostPolicy] = Field(default_factory=list, description="Applied policies")
    
    # Metrics
    detection_count: int = Field(default=0, description="Total detection count")
    critical_detections: int = Field(default=0, description="Critical detection count")
    high_detections: int = Field(default=0, description="High severity detection count")
    
    # Additional data
    tags: List[str] = Field(default_factory=list, description="Host tags")
    custom_properties: Dict[str, Any] = Field(default_factory=dict, description="Custom properties")
    
    # Timestamps
    created_at: Optional[datetime] = Field(None, description="Registration timestamp")
    updated_at: Optional[datetime] = Field(None, description="Last update timestamp")
    
    model_config = {
        "json_schema_extra": {
            "examples": [
                {
                    "id": "host-001",
                    "hostname": "workstation-01",
                    "platform": "windows",
                    "state": "online",
                    "ip_addresses": ["192.168.1.100"],
                    "os_version": "Windows 11 Pro",
                    "agent_version": "1.0.0",
                    "is_isolated": False
                }
            ]
        }
    }
    
    @property
    def needs_attention(self) -> bool:
        """Check if host needs security attention."""
        return self.critical_detections > 0 or self.high_detections > 0


class HostList(BaseModel):
    """List of hosts with pagination."""
    
    hosts: List[Host] = Field(..., description="List of hosts")
    total: int = Field(..., description="Total number of hosts")
    page: int = Field(default=1, description="Current page")
    page_size: int = Field(default=50, description="Items per page")
    has_more: bool = Field(default=False, description="More results available")


class HostQuery(BaseModel):
    """Query parameters for searching hosts."""
    
    query: Optional[str] = Field(None, description="Search query")
    platform: Optional[List[HostPlatform]] = Field(None, description="Filter by platform")
    state: Optional[List[HostState]] = Field(None, description="Filter by state")
    group_id: Optional[str] = Field(None, description="Filter by group ID")
    
    is_isolated: Optional[bool] = Field(None, description="Filter by isolation status")
    has_detections: Optional[bool] = Field(None, description="Filter by detection presence")
    
    sort_by: str = Field(default="hostname", description="Sort field")
    sort_order: str = Field(default="asc", description="Sort order (asc/desc)")
    limit: int = Field(default=50, ge=1, le=1000, description="Max results")
    offset: int = Field(default=0, ge=0, description="Result offset")