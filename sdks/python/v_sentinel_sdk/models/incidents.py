"""
Incident Models
~~~~~~~~~~~~~~~

Pydantic models for V-Sentinel incidents.
"""

from datetime import datetime
from enum import Enum
from typing import Any, Dict, List, Optional

from pydantic import BaseModel, Field


class IncidentSeverity(str, Enum):
    """Incident severity levels."""
    
    CRITICAL = "critical"
    HIGH = "high"
    MEDIUM = "medium"
    LOW = "low"
    
    def __str__(self) -> str:
        return self.value


class IncidentStatus(str, Enum):
    """Incident status."""
    
    NEW = "new"
    IN_PROGRESS = "in_progress"
    ON_HOLD = "on_hold"
    RESOLVED = "resolved"
    CLOSED = "closed"
    FALSE_POSITIVE = "false_positive"
    
    def __str__(self) -> str:
        return self.value


class IncidentPhase(str, Enum):
    """Incident response phase."""
    
    IDENTIFICATION = "identification"
    CONTAINMENT = "containment"
    ERADICATION = "eradication"
    RECOVERY = "recovery"
    LESSONS_LEARNED = "lessons_learned"


class IncidentHost(BaseModel):
    """Host involved in an incident."""
    
    id: str = Field(..., description="Host ID")
    hostname: str = Field(..., description="Host name")
    role: str = Field(default="victim", description="Role in incident (victim/attacker/infrastructure)")


class IncidentDetection(BaseModel):
    """Detection associated with an incident."""
    
    id: str = Field(..., description="Detection ID")
    title: str = Field(..., description="Detection title")
    severity: str = Field(..., description="Detection severity")
    created_at: datetime = Field(..., description="Detection timestamp")


class IncidentComment(BaseModel):
    """Comment on an incident."""
    
    id: str = Field(..., description="Comment ID")
    author: str = Field(..., description="Comment author")
    content: str = Field(..., description="Comment content")
    created_at: datetime = Field(..., description="Comment timestamp")
    is_internal: bool = Field(default=False, description="Internal note flag")


class IncidentTimelineEntry(BaseModel):
    """Timeline entry for incident."""
    
    timestamp: datetime = Field(..., description="Event timestamp")
    event: str = Field(..., description="Event description")
    source: Optional[str] = Field(None, description="Event source")
    details: Optional[Dict[str, Any]] = Field(None, description="Event details")


class Incident(BaseModel):
    """
    V-Sentinel Incident.
    
    Represents a security incident in V-Sentinel.
    
    Example:
        >>> incident = Incident(
        ...     id="inc-001",
        ...     title="Ransomware Detection",
        ...     severity=IncidentSeverity.CRITICAL,
        ...     status=IncidentStatus.NEW
        ... )
    """
    
    id: str = Field(..., description="Unique incident ID")
    title: str = Field(..., description="Incident title")
    description: str = Field(..., description="Detailed description")
    severity: IncidentSeverity = Field(..., description="Severity level")
    status: IncidentStatus = Field(default=IncidentStatus.NEW, description="Current status")
    phase: IncidentPhase = Field(default=IncidentPhase.IDENTIFICATION, description="Current phase")
    
    # Classification
    incident_type: str = Field(default="security", description="Incident type")
    category: Optional[str] = Field(None, description="Incident category")
    attack_vector: Optional[str] = Field(None, description="Attack vector")
    
    # Timestamps
    created_at: datetime = Field(..., description="Incident creation timestamp")
    updated_at: Optional[datetime] = Field(None, description="Last update timestamp")
    detected_at: Optional[datetime] = Field(None, description="Detection timestamp")
    resolved_at: Optional[datetime] = Field(None, description="Resolution timestamp")
    closed_at: Optional[datetime] = Field(None, description="Closure timestamp")
    
    # Timing metrics
    time_to_detect: Optional[int] = Field(None, description="Time to detect (minutes)")
    time_to_respond: Optional[int] = Field(None, description="Time to respond (minutes)")
    time_to_resolve: Optional[int] = Field(None, description="Time to resolve (minutes)")
    
    # Related entities
    hosts: List[IncidentHost] = Field(default_factory=list, description="Involved hosts")
    detections: List[IncidentDetection] = Field(default_factory=list, description="Related detections")
    
    # Threat information
    threat_actor: Optional[str] = Field(None, description="Attributed threat actor")
    campaign: Optional[str] = Field(None, description="Associated campaign")
    malware_family: Optional[str] = Field(None, description="Malware family")
    
    # Response
    assigned_to: Optional[str] = Field(None, description="Assigned analyst")
    assignee_team: Optional[str] = Field(None, description="Assigned team")
    
    # Notes and timeline
    comments: List[IncidentComment] = Field(default_factory=list, description="Comments")
    timeline: List[IncidentTimelineEntry] = Field(default_factory=list, description="Timeline")
    
    # Impact assessment
    affected_assets: int = Field(default=0, description="Number of affected assets")
    affected_users: int = Field(default=0, description="Number of affected users")
    data_exfil_suspected: bool = Field(default=False, description="Data exfiltration suspected")
    
    # Resolution
    resolution_notes: Optional[str] = Field(None, description="Resolution notes")
    root_cause: Optional[str] = Field(None, description="Root cause analysis")
    lessons_learned: Optional[str] = Field(None, description="Lessons learned")
    
    # Additional data
    tags: List[str] = Field(default_factory=list, description="Incident tags")
    references: List[str] = Field(default_factory=list, description="Reference URLs")
    custom_fields: Dict[str, Any] = Field(default_factory=dict, description="Custom fields")
    
    model_config = {
        "json_schema_extra": {
            "examples": [
                {
                    "id": "inc-20240115-001",
                    "title": "Ransomware Detection on Finance Server",
                    "description": "LockBit ransomware detected on finance server with encryption activity",
                    "severity": "critical",
                    "status": "new",
                    "phase": "identification",
                    "incident_type": "ransomware",
                    "created_at": "2024-01-15T10:30:00Z",
                    "hosts": [
                        {
                            "id": "host-001",
                            "hostname": "finance-server-01",
                            "role": "victim"
                        }
                    ]
                }
            ]
        }
    }
    
    @property
    def is_open(self) -> bool:
        """Check if incident is still open."""
        return self.status not in (IncidentStatus.RESOLVED, IncidentStatus.CLOSED, IncidentStatus.FALSE_POSITIVE)
    
    @property
    def sla_breached(self) -> bool:
        """Check if SLA is breached based on severity."""
        # Define SLA thresholds in hours
        sla_thresholds = {
            IncidentSeverity.CRITICAL: 1,
            IncidentSeverity.HIGH: 4,
            IncidentSeverity.MEDIUM: 24,
            IncidentSeverity.LOW: 72,
        }
        
        if self.status == IncidentStatus.NEW:
            threshold_hours = sla_thresholds.get(self.severity, 24)
            elapsed = (datetime.utcnow() - self.created_at).total_seconds() / 3600
            return elapsed > threshold_hours
        
        return False


class IncidentList(BaseModel):
    """List of incidents with pagination."""
    
    incidents: List[Incident] = Field(..., description="List of incidents")
    total: int = Field(..., description="Total number of incidents")
    page: int = Field(default=1, description="Current page")
    page_size: int = Field(default=50, description="Items per page")
    has_more: bool = Field(default=False, description="More results available")


class IncidentQuery(BaseModel):
    """Query parameters for searching incidents."""
    
    query: Optional[str] = Field(None, description="Search query")
    severity: Optional[List[IncidentSeverity]] = Field(None, description="Filter by severity")
    status: Optional[List[IncidentStatus]] = Field(None, description="Filter by status")
    phase: Optional[List[IncidentPhase]] = Field(None, description="Filter by phase")
    
    threat_actor: Optional[str] = Field(None, description="Filter by threat actor")
    incident_type: Optional[str] = Field(None, description="Filter by incident type")
    host_id: Optional[str] = Field(None, description="Filter by host ID")
    assigned_to: Optional[str] = Field(None, description="Filter by assignee")
    
    time_range: Optional[str] = Field(None, description="Time range (e.g., '24h', '7d', '30d')")
    start_time: Optional[datetime] = Field(None, description="Start timestamp")
    end_time: Optional[datetime] = Field(None, description="End timestamp")
    
    sort_by: str = Field(default="created_at", description="Sort field")
    sort_order: str = Field(default="desc", description="Sort order (asc/desc)")
    limit: int = Field(default=50, ge=1, le=1000, description="Max results")
    offset: int = Field(default=0, ge=0, description="Result offset")