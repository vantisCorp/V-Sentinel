"""
Detection Models
~~~~~~~~~~~~~~~~

Pydantic models for V-Sentinel detections.
"""

from datetime import datetime
from enum import Enum
from typing import Any, Dict, List, Optional

from pydantic import BaseModel, Field


class DetectionSeverity(str, Enum):
    """Detection severity levels."""
    
    CRITICAL = "critical"
    HIGH = "high"
    MEDIUM = "medium"
    LOW = "low"
    INFORMATIONAL = "informational"
    
    def __str__(self) -> str:
        return self.value
    
    @property
    def priority(self) -> int:
        """Get numeric priority (higher = more severe)."""
        priorities = {
            DetectionSeverity.CRITICAL: 5,
            DetectionSeverity.HIGH: 4,
            DetectionSeverity.MEDIUM: 3,
            DetectionSeverity.LOW: 2,
            DetectionSeverity.INFORMATIONAL: 1,
        }
        return priorities[self]


class DetectionStatus(str, Enum):
    """Detection status."""
    
    NEW = "new"
    IN_PROGRESS = "in_progress"
    RESOLVED = "resolved"
    FALSE_POSITIVE = "false_positive"
    IGNORED = "ignored"
    
    def __str__(self) -> str:
        return self.value


class DetectionTactic(str, Enum):
    """MITRE ATT&CK tactics."""
    
    INITIAL_ACCESS = "initial_access"
    EXECUTION = "execution"
    PERSISTENCE = "persistence"
    PRIVILEGE_ESCALATION = "privilege_escalation"
    DEFENSE_EVASION = "defense_evasion"
    CREDENTIAL_ACCESS = "credential_access"
    DISCOVERY = "discovery"
    LATERAL_MOVEMENT = "lateral_movement"
    COLLECTION = "collection"
    COMMAND_AND_CONTROL = "command_and_control"
    EXFILTRATION = "exfiltration"
    IMPACT = "impact"


class DetectionTechnique(BaseModel):
    """MITRE ATT&CK technique reference."""
    
    id: str = Field(..., description="Technique ID (e.g., T1059)")
    name: str = Field(..., description="Technique name")
    tactics: List[DetectionTactic] = Field(default_factory=list, description="Associated tactics")
    url: Optional[str] = Field(None, description="Reference URL")


class DetectionIndicator(BaseModel):
    """Indicator of Compromise associated with detection."""
    
    type: str = Field(..., description="Indicator type (ip, domain, url, hash, etc.)")
    value: str = Field(..., description="Indicator value")
    confidence: str = Field(default="medium", description="Confidence level")
    threat_actor: Optional[str] = Field(None, description="Associated threat actor")
    campaign: Optional[str] = Field(None, description="Associated campaign")


class Detection(BaseModel):
    """
    V-Sentinel Detection.
    
    Represents a security detection/alert in V-Sentinel.
    
    Example:
        >>> detection = Detection(
        ...     id="det-001",
        ...     title="Suspicious PowerShell Execution",
        ...     severity=DetectionSeverity.HIGH,
        ...     description="PowerShell script with obfuscated content detected"
        ... )
    """
    
    id: str = Field(..., description="Unique detection ID")
    title: str = Field(..., description="Detection title")
    description: str = Field(..., description="Detailed description")
    severity: DetectionSeverity = Field(..., description="Severity level")
    status: DetectionStatus = Field(default=DetectionStatus.NEW, description="Current status")
    
    # Timestamps
    created_at: datetime = Field(..., description="Detection creation timestamp")
    updated_at: Optional[datetime] = Field(None, description="Last update timestamp")
    first_seen: Optional[datetime] = Field(None, description="First occurrence timestamp")
    last_seen: Optional[datetime] = Field(None, description="Most recent occurrence timestamp")
    
    # Host and process information
    host_id: Optional[str] = Field(None, description="Associated host ID")
    host_name: Optional[str] = Field(None, description="Host name")
    process_name: Optional[str] = Field(None, description="Process name")
    process_id: Optional[int] = Field(None, description="Process ID")
    command_line: Optional[str] = Field(None, description="Command line")
    
    # MITRE ATT&CK mapping
    techniques: List[DetectionTechnique] = Field(
        default_factory=list,
        description="MITRE ATT&CK techniques"
    )
    
    # Indicators
    indicators: List[DetectionIndicator] = Field(
        default_factory=list,
        description="Associated indicators of compromise"
    )
    
    # Additional data
    raw_data: Optional[Dict[str, Any]] = Field(None, description="Raw detection data")
    tags: List[str] = Field(default_factory=list, description="Detection tags")
    references: List[str] = Field(default_factory=list, description="Reference URLs")
    
    # Metadata
    confidence: str = Field(default="medium", description="Detection confidence")
    false_positive_probability: Optional[float] = Field(
        None,
        ge=0.0,
        le=1.0,
        description="Probability of being a false positive"
    )
    
    # Assignment and notes
    assigned_to: Optional[str] = Field(None, description="Assigned analyst")
    notes: List[str] = Field(default_factory=list, description="Analysis notes")
    
    model_config = {
        "json_schema_extra": {
            "examples": [
                {
                    "id": "det-20240115-001",
                    "title": "Suspicious PowerShell Execution",
                    "description": "PowerShell script with base64 encoded content attempting to download payload",
                    "severity": "high",
                    "status": "new",
                    "created_at": "2024-01-15T10:30:00Z",
                    "host_id": "host-001",
                    "host_name": "workstation-01",
                    "process_name": "powershell.exe",
                    "techniques": [
                        {
                            "id": "T1059.001",
                            "name": "PowerShell",
                            "tactics": ["execution"]
                        }
                    ],
                    "confidence": "high"
                }
            ]
        }
    }
    
    @property
    def is_critical(self) -> bool:
        """Check if detection is critical severity."""
        return self.severity == DetectionSeverity.CRITICAL
    
    @property
    def is_open(self) -> bool:
        """Check if detection is still open (not resolved)."""
        return self.status in (DetectionStatus.NEW, DetectionStatus.IN_PROGRESS)


class DetectionList(BaseModel):
    """List of detections with pagination."""
    
    detections: List[Detection] = Field(..., description="List of detections")
    total: int = Field(..., description="Total number of detections")
    page: int = Field(default=1, description="Current page")
    page_size: int = Field(default=50, description="Items per page")
    has_more: bool = Field(default=False, description="More results available")


class DetectionQuery(BaseModel):
    """Query parameters for searching detections."""
    
    query: Optional[str] = Field(None, description="Search query")
    severity: Optional[List[DetectionSeverity]] = Field(None, description="Filter by severity")
    status: Optional[List[DetectionStatus]] = Field(None, description="Filter by status")
    host_id: Optional[str] = Field(None, description="Filter by host ID")
    technique: Optional[str] = Field(None, description="Filter by MITRE technique")
    threat_actor: Optional[str] = Field(None, description="Filter by threat actor")
    
    time_range: Optional[str] = Field(None, description="Time range (e.g., '24h', '7d', '30d')")
    start_time: Optional[datetime] = Field(None, description="Start timestamp")
    end_time: Optional[datetime] = Field(None, description="End timestamp")
    
    sort_by: str = Field(default="created_at", description="Sort field")
    sort_order: str = Field(default="desc", description="Sort order (asc/desc)")
    limit: int = Field(default=50, ge=1, le=1000, description="Max results")
    offset: int = Field(default=0, ge=0, description="Result offset")