"""
Tests for V-Sentinel SDK Pydantic models.
"""

import pytest
from datetime import datetime
from pydantic import ValidationError

from v_sentinel_sdk.models.detections import (
    Detection,
    DetectionSeverity,
    DetectionStatus,
    DetectionQuery,
)
from v_sentinel_sdk.models.hosts import (
    Host,
    HostState,
    HostPlatform,
)
from v_sentinel_sdk.models.incidents import (
    Incident,
    IncidentSeverity,
    IncidentStatus,
    IncidentPhase,
)
from v_sentinel_sdk.models.threat_intel import (
    IOC,
    IOCType,
    IOCConfidence,
    ThreatActor,
    IOCCheckResult,
)


class TestDetectionModel:
    """Tests for Detection model."""
    
    def test_detection_from_dict(self, detection_response):
        """Test creating Detection from dictionary."""
        detection = Detection(**detection_response)
        
        assert detection.id == "det-12345"
        assert detection.title == "Suspicious PowerShell Execution"
        assert detection.severity == DetectionSeverity.HIGH
        assert detection.status == DetectionStatus.NEW
        assert len(detection.techniques) == 2
        assert "T1059.001" in detection.techniques
    
    def test_detection_severity_enum(self):
        """Test DetectionSeverity enum values."""
        assert DetectionSeverity.CRITICAL.value == "CRITICAL"
        assert DetectionSeverity.HIGH.value == "HIGH"
        assert DetectionSeverity.MEDIUM.value == "MEDIUM"
        assert DetectionSeverity.LOW.value == "LOW"
        assert DetectionSeverity.INFORMATIONAL.value == "INFORMATIONAL"
    
    def test_detection_status_enum(self):
        """Test DetectionStatus enum values."""
        assert DetectionStatus.NEW.value == "NEW"
        assert DetectionStatus.IN_PROGRESS.value == "IN_PROGRESS"
        assert DetectionStatus.RESOLVED.value == "RESOLVED"
        assert DetectionStatus.FALSE_POSITIVE.value == "FALSE_POSITIVE"
        assert DetectionStatus.IGNORED.value == "IGNORED"
    
    def test_detection_required_fields(self):
        """Test that required fields are validated."""
        with pytest.raises(ValidationError):
            Detection(id="det-001")  # Missing required fields
    
    def test_detection_query_defaults(self):
        """Test DetectionQuery default values."""
        query = DetectionQuery(query="test")
        
        assert query.limit == 50
        assert query.offset == 0
        assert query.sort_by is None


class TestHostModel:
    """Tests for Host model."""
    
    def test_host_from_dict(self, host_response):
        """Test creating Host from dictionary."""
        host = Host(**host_response)
        
        assert host.id == "host-001"
        assert host.hostname == "WORKSTATION-01"
        assert host.platform == HostPlatform.WINDOWS
        assert host.state == HostState.ONLINE
        assert len(host.ip_addresses) == 2
        assert host.is_isolated is False
    
    def test_host_state_enum(self):
        """Test HostState enum values."""
        assert HostState.ONLINE.value == "ONLINE"
        assert HostState.OFFLINE.value == "OFFLINE"
        assert HostState.ISOLATED.value == "ISOLATED"
        assert HostState.UNKNOWN.value == "UNKNOWN"
    
    def test_host_platform_enum(self):
        """Test HostPlatform enum values."""
        assert HostPlatform.WINDOWS.value == "WINDOWS"
        assert HostPlatform.LINUX.value == "LINUX"
        assert HostPlatform.MACOS.value == "MACOS"
        assert HostPlatform.UNKNOWN.value == "UNKNOWN"


class TestIncidentModel:
    """Tests for Incident model."""
    
    def test_incident_from_dict(self, incident_response):
        """Test creating Incident from dictionary."""
        incident = Incident(**incident_response)
        
        assert incident.id == "inc-001"
        assert incident.title == "Ransomware Attack Detected"
        assert incident.severity == IncidentSeverity.CRITICAL
        assert incident.status == IncidentStatus.IN_PROGRESS
        assert incident.phase == IncidentPhase.CONTAINMENT
        assert len(incident.hosts) == 2
        assert len(incident.detections) == 3
    
    def test_incident_severity_enum(self):
        """Test IncidentSeverity enum values."""
        assert IncidentSeverity.CRITICAL.value == "CRITICAL"
        assert IncidentSeverity.HIGH.value == "HIGH"
        assert IncidentSeverity.MEDIUM.value == "MEDIUM"
        assert IncidentSeverity.LOW.value == "LOW"
    
    def test_incident_status_enum(self):
        """Test IncidentStatus enum values."""
        assert IncidentStatus.NEW.value == "NEW"
        assert IncidentStatus.IN_PROGRESS.value == "IN_PROGRESS"
        assert IncidentStatus.ON_HOLD.value == "ON_HOLD"
        assert IncidentStatus.RESOLVED.value == "RESOLVED"
        assert IncidentStatus.CLOSED.value == "CLOSED"
        assert IncidentStatus.FALSE_POSITIVE.value == "FALSE_POSITIVE"
    
    def test_incident_phase_enum(self):
        """Test IncidentPhase enum values."""
        assert IncidentPhase.IDENTIFICATION.value == "IDENTIFICATION"
        assert IncidentPhase.CONTAINMENT.value == "CONTAINMENT"
        assert IncidentPhase.ERADICATION.value == "ERADICATION"
        assert IncidentPhase.RECOVERY.value == "RECOVERY"
        assert IncidentPhase.LESSONS_LEARNED.value == "LESSONS_LEARNED"


class TestIOCModel:
    """Tests for IOC model."""
    
    def test_ioc_from_dict(self, ioc_response):
        """Test creating IOC from dictionary."""
        ioc = IOC(**ioc_response)
        
        assert ioc.id == "ioc-001"
        assert ioc.type == IOCType.IP
        assert ioc.value == "192.0.2.1"
        assert ioc.confidence == IOCConfidence.HIGH
        assert ioc.threat_actor == "APT29"
    
    def test_ioc_type_enum(self):
        """Test IOCType enum values."""
        assert IOCType.IP.value == "IP"
        assert IOCType.DOMAIN.value == "DOMAIN"
        assert IOCType.URL.value == "URL"
        assert IOCType.HASH_MD5.value == "HASH_MD5"
        assert IOCType.HASH_SHA1.value == "HASH_SHA1"
        assert IOCType.HASH_SHA256.value == "HASH_SHA256"
        assert IOCType.EMAIL.value == "EMAIL"
    
    def test_ioc_confidence_enum(self):
        """Test IOCConfidence enum values."""
        assert IOCConfidence.LOW.value == "LOW"
        assert IOCConfidence.MEDIUM.value == "MEDIUM"
        assert IOCConfidence.HIGH.value == "HIGH"


class TestThreatActorModel:
    """Tests for ThreatActor model."""
    
    def test_threat_actor_creation(self):
        """Test creating ThreatActor."""
        actor = ThreatActor(
            name="APT29",
            aliases=["Cozy Bear", "The Dukes"],
            country="Russia",
            motivation="Espionage",
            mitre_id="G0036",
            first_seen="2014-01-01T00:00:00Z",
            last_seen="2024-01-15T00:00:00Z",
            tags=["apt", "nation-state"],
            created_at="2024-01-01T00:00:00Z",
            updated_at="2024-01-15T00:00:00Z",
        )
        
        assert actor.name == "APT29"
        assert len(actor.aliases) == 2
        assert actor.mitre_id == "G0036"


class TestIOCCheckResult:
    """Tests for IOCCheckResult model."""
    
    def test_ioc_check_result_from_dict(self, ioc_check_result_response):
        """Test creating IOCCheckResult from dictionary."""
        result = IOCCheckResult(**ioc_check_result_response)
        
        assert result.is_malicious is True
        assert result.confidence == IOCConfidence.HIGH
        assert result.threat_actor == "APT29"
        assert len(result.related_iocs) == 1
        assert result.related_iocs[0].value == "malicious.example.com"