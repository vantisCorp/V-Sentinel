"""
Tests for V-Sentinel SDK API clients.
"""

import pytest
from unittest.mock import AsyncMock, MagicMock, patch
import httpx

from v_sentinel_sdk.clients.detections import DetectionsClient
from v_sentinel_sdk.clients.hosts import HostsClient
from v_sentinel_sdk.clients.incidents import IncidentsClient
from v_sentinel_sdk.clients.threat_intel import ThreatIntelClient
from v_sentinel_sdk.models.detections import DetectionSeverity, DetectionStatus
from v_sentinel_sdk.models.hosts import HostState, HostPlatform
from v_sentinel_sdk.models.incidents import IncidentSeverity, IncidentStatus, IncidentPhase
from v_sentinel_sdk.models.threat_intel import IOCType, IOCConfidence
from v_sentinel_sdk.exceptions import ResourceNotFoundError, RateLimitError


class TestDetectionsClient:
    """Tests for DetectionsClient."""
    
    @pytest.fixture
    def client(self, mock_httpx_client, api_base_url):
        """Create DetectionsClient instance."""
        return DetectionsClient(mock_httpx_client, api_base_url)
    
    @pytest.mark.asyncio
    async def test_list_detections(self, client, mock_httpx_client, detection_list_response):
        """Test listing detections."""
        mock_response = MagicMock(spec=httpx.Response)
        mock_response.status_code = 200
        mock_response.json = MagicMock(return_value=detection_list_response)
        mock_httpx_client.get = AsyncMock(return_value=mock_response)
        
        result = await client.list()
        
        assert result.total == 2
        assert len(result.items) == 2
        mock_httpx_client.get.assert_called_once()
    
    @pytest.mark.asyncio
    async def test_list_detections_with_filters(self, client, mock_httpx_client, detection_list_response):
        """Test listing detections with filters."""
        mock_response = MagicMock(spec=httpx.Response)
        mock_response.status_code = 200
        mock_response.json = MagicMock(return_value=detection_list_response)
        mock_httpx_client.get = AsyncMock(return_value=mock_response)
        
        result = await client.list(
            severity=DetectionSeverity.HIGH,
            status=DetectionStatus.NEW,
            time_range="24h",
            limit=10
        )
        
        call_args = mock_httpx_client.get.call_args
        params = call_args.kwargs["params"]
        
        assert params["severity"] == "HIGH"
        assert params["status"] == "NEW"
        assert params["time_range"] == "24h"
        assert params["limit"] == 10
    
    @pytest.mark.asyncio
    async def test_get_detection(self, client, mock_httpx_client, detection_response):
        """Test getting a specific detection."""
        mock_response = MagicMock(spec=httpx.Response)
        mock_response.status_code = 200
        mock_response.json = MagicMock(return_value=detection_response)
        mock_httpx_client.get = AsyncMock(return_value=mock_response)
        
        result = await client.get("det-12345")
        
        assert result.id == "det-12345"
        assert result.title == "Suspicious PowerShell Execution"
    
    @pytest.mark.asyncio
    async def test_get_detection_not_found(self, client, mock_httpx_client):
        """Test getting non-existent detection."""
        mock_response = MagicMock(spec=httpx.Response)
        mock_response.status_code = 404
        mock_httpx_client.get = AsyncMock(return_value=mock_response)
        
        with pytest.raises(ResourceNotFoundError):
            await client.get("nonexistent")
    
    @pytest.mark.asyncio
    async def test_update_detection_status(self, client, mock_httpx_client, detection_response):
        """Test updating detection status."""
        updated_response = detection_response.copy()
        updated_response["status"] = "IN_PROGRESS"
        
        mock_response = MagicMock(spec=httpx.Response)
        mock_response.status_code = 200
        mock_response.json = MagicMock(return_value=updated_response)
        mock_httpx_client.patch = AsyncMock(return_value=mock_response)
        
        result = await client.update_status(
            detection_id="det-12345",
            status=DetectionStatus.IN_PROGRESS,
            notes="Investigating"
        )
        
        assert result.status == DetectionStatus.IN_PROGRESS


class TestHostsClient:
    """Tests for HostsClient."""
    
    @pytest.fixture
    def client(self, mock_httpx_client, api_base_url):
        """Create HostsClient instance."""
        return HostsClient(mock_httpx_client, api_base_url)
    
    @pytest.mark.asyncio
    async def test_list_hosts(self, client, mock_httpx_client):
        """Test listing hosts."""
        hosts_response = {
            "items": [
                {
                    "id": "host-001",
                    "hostname": "HOST-001",
                    "platform": "WINDOWS",
                    "state": "ONLINE",
                    "ip_addresses": ["192.168.1.1"],
                    "is_isolated": False,
                    "tags": [],
                    "created_at": "2024-01-15T10:00:00Z",
                    "updated_at": "2024-01-15T10:00:00Z",
                }
            ],
            "total": 1,
            "limit": 50,
            "offset": 0,
            "has_more": False,
        }
        
        mock_response = MagicMock(spec=httpx.Response)
        mock_response.status_code = 200
        mock_response.json = MagicMock(return_value=hosts_response)
        mock_httpx_client.get = AsyncMock(return_value=mock_response)
        
        result = await client.list()
        
        assert result.total == 1
        assert len(result.items) == 1
    
    @pytest.mark.asyncio
    async def test_isolate_host(self, client, mock_httpx_client, host_response):
        """Test isolating a host."""
        isolated_response = host_response.copy()
        isolated_response["is_isolated"] = True
        isolated_response["state"] = "ISOLATED"
        
        mock_response = MagicMock(spec=httpx.Response)
        mock_response.status_code = 200
        mock_response.json = MagicMock(return_value=isolated_response)
        mock_httpx_client.post = AsyncMock(return_value=mock_response)
        
        result = await client.isolate(
            host_id="host-001",
            reason="Malware detected"
        )
        
        assert result.is_isolated is True
        mock_httpx_client.post.assert_called_once()
    
    @pytest.mark.asyncio
    async def test_unisolate_host(self, client, mock_httpx_client, host_response):
        """Test unisolating a host."""
        mock_response = MagicMock(spec=httpx.Response)
        mock_response.status_code = 200
        mock_response.json = MagicMock(return_value=host_response)
        mock_httpx_client.post = AsyncMock(return_value=mock_response)
        
        result = await client.unisolate("host-001")
        
        mock_httpx_client.post.assert_called_once()


class TestIncidentsClient:
    """Tests for IncidentsClient."""
    
    @pytest.fixture
    def client(self, mock_httpx_client, api_base_url):
        """Create IncidentsClient instance."""
        return IncidentsClient(mock_httpx_client, api_base_url)
    
    @pytest.mark.asyncio
    async def test_create_incident(self, client, mock_httpx_client, incident_response):
        """Test creating an incident."""
        mock_response = MagicMock(spec=httpx.Response)
        mock_response.status_code = 201
        mock_response.json = MagicMock(return_value=incident_response)
        mock_httpx_client.post = AsyncMock(return_value=mock_response)
        
        result = await client.create(
            title="Ransomware Attack Detected",
            description="LockBit ransomware detected",
            severity=IncidentSeverity.CRITICAL
        )
        
        assert result.title == "Ransomware Attack Detected"
        mock_httpx_client.post.assert_called_once()
    
    @pytest.mark.asyncio
    async def test_update_incident_phase(self, client, mock_httpx_client, incident_response):
        """Test updating incident phase."""
        updated_response = incident_response.copy()
        updated_response["phase"] = "ERADICATION"
        
        mock_response = MagicMock(spec=httpx.Response)
        mock_response.status_code = 200
        mock_response.json = MagicMock(return_value=updated_response)
        mock_httpx_client.patch = AsyncMock(return_value=mock_response)
        
        result = await client.update_phase(
            incident_id="inc-001",
            phase=IncidentPhase.ERADICATION,
            notes="Malware removed"
        )
        
        assert result.phase == IncidentPhase.ERADICATION
    
    @pytest.mark.asyncio
    async def test_close_incident(self, client, mock_httpx_client, incident_response):
        """Test closing an incident."""
        closed_response = incident_response.copy()
        closed_response["status"] = "CLOSED"
        
        mock_response = MagicMock(spec=httpx.Response)
        mock_response.status_code = 200
        mock_response.json = MagicMock(return_value=closed_response)
        mock_httpx_client.post = AsyncMock(return_value=mock_response)
        
        result = await client.close(
            incident_id="inc-001",
            resolution="Incident resolved"
        )
        
        mock_httpx_client.post.assert_called_once()


class TestThreatIntelClient:
    """Tests for ThreatIntelClient."""
    
    @pytest.fixture
    def client(self, mock_httpx_client, api_base_url):
        """Create ThreatIntelClient instance."""
        return ThreatIntelClient(mock_httpx_client, api_base_url)
    
    @pytest.mark.asyncio
    async def test_check_ioc(self, client, mock_httpx_client, ioc_check_result_response):
        """Test checking an IOC."""
        mock_response = MagicMock(spec=httpx.Response)
        mock_response.status_code = 200
        mock_response.json = MagicMock(return_value=ioc_check_result_response)
        mock_httpx_client.get = AsyncMock(return_value=mock_response)
        
        result = await client.check_ioc("192.0.2.1")
        
        assert result.is_malicious is True
        assert result.threat_actor == "APT29"
    
    @pytest.mark.asyncio
    async def test_check_iocs_bulk(self, client, mock_httpx_client, ioc_check_result_response):
        """Test bulk checking IOCs."""
        bulk_response = {
            "results": [ioc_check_result_response]
        }
        
        mock_response = MagicMock(spec=httpx.Response)
        mock_response.status_code = 200
        mock_response.json = MagicMock(return_value=bulk_response)
        mock_httpx_client.post = AsyncMock(return_value=mock_response)
        
        result = await client.check_iocs_bulk(["192.0.2.1", "malicious.example.com"])
        
        assert len(result) == 1
        mock_httpx_client.post.assert_called_once()
    
    @pytest.mark.asyncio
    async def test_add_ioc(self, client, mock_httpx_client, ioc_response):
        """Test adding an IOC."""
        mock_response = MagicMock(spec=httpx.Response)
        mock_response.status_code = 201
        mock_response.json = MagicMock(return_value=ioc_response)
        mock_httpx_client.post = AsyncMock(return_value=mock_response)
        
        result = await client.add_ioc(
            value="192.0.2.1",
            ioc_type=IOCType.IP,
            confidence=IOCConfidence.HIGH,
            threat_actor="APT29"
        )
        
        assert result.value == "192.0.2.1"
        assert result.threat_actor == "APT29"
    
    @pytest.mark.asyncio
    async def test_export_iocs(self, client, mock_httpx_client):
        """Test exporting IOCs."""
        mock_response = MagicMock(spec=httpx.Response)
        mock_response.status_code = 200
        mock_response.text = '{"iocs": []}'
        mock_httpx_client.get = AsyncMock(return_value=mock_response)
        
        result = await client.export_iocs(format="json")
        
        assert result == '{"iocs": []}'
    
    @pytest.mark.asyncio
    async def test_get_threat_actor(self, client, mock_httpx_client):
        """Test getting threat actor information."""
        actor_response = {
            "name": "APT29",
            "aliases": ["Cozy Bear"],
            "country": "Russia",
            "motivation": "Espionage",
            "mitre_id": "G0036",
            "tags": ["apt"],
            "created_at": "2024-01-01T00:00:00Z",
            "updated_at": "2024-01-15T00:00:00Z",
        }
        
        mock_response = MagicMock(spec=httpx.Response)
        mock_response.status_code = 200
        mock_response.json = MagicMock(return_value=actor_response)
        mock_httpx_client.get = AsyncMock(return_value=mock_response)
        
        result = await client.get_threat_actor("APT29")
        
        assert result.name == "APT29"
        assert result.mitre_id == "G0036"