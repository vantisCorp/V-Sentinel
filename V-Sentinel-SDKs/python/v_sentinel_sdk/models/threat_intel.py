"""
Threat Intelligence Models
~~~~~~~~~~~~~~~~~~~~~~~~~~

Pydantic models for V-Sentinel threat intelligence.
"""

from datetime import datetime
from enum import Enum
from typing import Any, Dict, List, Optional

from pydantic import BaseModel, Field, field_validator


class IOCType(str, Enum):
    """Indicator of Compromise types."""
    
    IP = "ip"
    DOMAIN = "domain"
    URL = "url"
    HASH_MD5 = "hash_md5"
    HASH_SHA1 = "hash_sha1"
    HASH_SHA256 = "hash_sha256"
    EMAIL = "email"
    CERTIFICATE = "certificate"
    MUTEX = "mutex"
    REGISTRY = "registry"
    FILE_PATH = "file_path"
    
    def __str__(self) -> str:
        return self.value


class IOCConfidence(str, Enum):
    """IOC confidence levels."""
    
    LOW = "low"
    MEDIUM = "medium"
    HIGH = "high"
    
    def __str__(self) -> str:
        return self.value


class ThreatActorType(str, Enum):
    """Threat actor types."""
    
    APT = "apt"
    CRIME_GROUP = "crime_group"
    HACKTIVIST = "hacktivist"
    INSIDER = "insider"
    UNKNOWN = "unknown"


class ThreatActor(BaseModel):
    """Threat actor information."""
    
    name: str = Field(..., description="Primary name")
    aliases: List[str] = Field(default_factory=list, description="Known aliases")
    country: Optional[str] = Field(None, description="Attribution country")
    motivation: Optional[str] = Field(None, description="Primary motivation")
    type: ThreatActorType = Field(default=ThreatActorType.UNKNOWN, description="Actor type")
    
    mitre_id: Optional[str] = Field(None, description="MITRE ATT&CK group ID")
    first_seen: Optional[datetime] = Field(None, description="First known activity")
    
    description: Optional[str] = Field(None, description="Detailed description")
    references: List[str] = Field(default_factory=list, description="Reference URLs")
    
    model_config = {
        "json_schema_extra": {
            "examples": [
                {
                    "name": "APT29",
                    "aliases": ["Cozy Bear", "The Dukes"],
                    "country": "Russia",
                    "motivation": "Espionage",
                    "type": "apt",
                    "mitre_id": "G0016"
                }
            ]
        }
    }


class ThreatCampaign(BaseModel):
    """Threat campaign information."""
    
    name: str = Field(..., description="Campaign name")
    threat_actor: Optional[str] = Field(None, description="Attributed threat actor")
    
    start_date: Optional[datetime] = Field(None, description="Campaign start")
    end_date: Optional[datetime] = Field(None, description="Campaign end")
    
    targets: List[str] = Field(default_factory=list, description="Target sectors/regions")
    malware_families: List[str] = Field(default_factory=list, description="Associated malware")
    
    description: Optional[str] = Field(None, description="Campaign description")
    references: List[str] = Field(default_factory=list, description="Reference URLs")


class IOC(BaseModel):
    """
    Indicator of Compromise.
    
    Represents a threat intelligence indicator.
    
    Example:
        >>> ioc = IOC(
        ...     type=IOCType.IP,
        ...     value="192.0.2.1",
        ...     confidence=IOCConfidence.HIGH,
        ...     threat_actor="APT29"
        ... )
    """
    
    id: Optional[str] = Field(None, description="Unique IOC ID")
    type: IOCType = Field(..., description="IOC type")
    value: str = Field(..., description="IOC value")
    confidence: IOCConfidence = Field(default=IOCConfidence.MEDIUM, description="Confidence level")
    
    # Attribution
    threat_actor: Optional[str] = Field(None, description="Attributed threat actor")
    campaign: Optional[str] = Field(None, description="Associated campaign")
    malware_family: Optional[str] = Field(None, description="Associated malware family")
    
    # Timestamps
    first_seen: Optional[datetime] = Field(None, description="First seen timestamp")
    last_seen: Optional[datetime] = Field(None, description="Last seen timestamp")
    
    # Context
    description: Optional[str] = Field(None, description="IOC description")
    context: Optional[str] = Field(None, description="Context (c2, phishing, malware_delivery, etc.)")
    tags: List[str] = Field(default_factory=list, description="IOC tags")
    
    # Source information
    source: Optional[str] = Field(None, description="Source of intelligence")
    references: List[str] = Field(default_factory=list, description="Reference URLs")
    
    # Additional metadata
    metadata: Dict[str, Any] = Field(default_factory=dict, description="Additional metadata")
    
    model_config = {
        "json_schema_extra": {
            "examples": [
                {
                    "type": "ip",
                    "value": "185.141.63.22",
                    "confidence": "high",
                    "threat_actor": "APT29",
                    "campaign": "SolarWinds Supply Chain Attack",
                    "description": "Command and control server",
                    "tags": ["c2", "solarwinds", "apt"]
                }
            ]
        }
    }
    
    @field_validator('value')
    @classmethod
    def validate_value(cls, v: str, info) -> str:
        """Validate IOC value based on type."""
        # Basic validation - can be enhanced
        if not v or not v.strip():
            raise ValueError("IOC value cannot be empty")
        return v.strip()
    
    @property
    def is_high_confidence(self) -> bool:
        """Check if IOC has high confidence."""
        return self.confidence == IOCConfidence.HIGH


class IOCCheckResult(BaseModel):
    """Result of IOC check."""
    
    ioc: IOC = Field(..., description="IOC details")
    is_malicious: bool = Field(..., description="Whether IOC is known malicious")
    
    # Additional context if malicious
    threat_actor: Optional[ThreatActor] = Field(None, description="Threat actor details")
    campaigns: List[ThreatCampaign] = Field(default_factory=list, description="Associated campaigns")
    related_iocs: List[IOC] = Field(default_factory=list, description="Related IOCs")
    
    # Detection context
    detection_count: int = Field(default=0, description="Detection count in V-Sentinel")
    first_detected: Optional[datetime] = Field(None, description="First detection timestamp")
    last_detected: Optional[datetime] = Field(None, description="Last detection timestamp")


class IOCList(BaseModel):
    """List of IOCs with pagination."""
    
    iocs: List[IOC] = Field(..., description="List of IOCs")
    total: int = Field(..., description="Total number of IOCs")
    page: int = Field(default=1, description="Current page")
    page_size: int = Field(default=50, description="Items per page")
    has_more: bool = Field(default=False, description="More results available")


class IOCQuery(BaseModel):
    """Query parameters for searching IOCs."""
    
    query: Optional[str] = Field(None, description="Search query")
    type: Optional[List[IOCType]] = Field(None, description="Filter by IOC type")
    confidence: Optional[List[IOCConfidence]] = Field(None, description="Filter by confidence")
    
    threat_actor: Optional[str] = Field(None, description="Filter by threat actor")
    campaign: Optional[str] = Field(None, description="Filter by campaign")
    malware_family: Optional[str] = Field(None, description="Filter by malware family")
    
    time_range: Optional[str] = Field(None, description="Time range (e.g., '24h', '7d', '30d')")
    start_time: Optional[datetime] = Field(None, description="Start timestamp")
    end_time: Optional[datetime] = Field(None, description="End timestamp")
    
    sort_by: str = Field(default="last_seen", description="Sort field")
    sort_order: str = Field(default="desc", description="Sort order (asc/desc)")
    limit: int = Field(default=50, ge=1, le=1000, description="Max results")
    offset: int = Field(default=0, ge=0, description="Result offset")