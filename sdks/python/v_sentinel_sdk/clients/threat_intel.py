"""
V-Sentinel Threat Intelligence API Client

Provides async methods for threat intelligence operations.
"""

from typing import Optional, List, Dict, Any
import httpx

from ..models.threat_intel import (
    IOC,
    IOCList,
    IOCType,
    IOCConfidence,
    IOCCheckResult,
    ThreatActor,
    ThreatActorList,
    Campaign,
    CampaignList,
)
from ..exceptions import (
    ValidationError,
    ResourceNotFoundError,
    APIError,
)


class ThreatIntelClient:
    """
    Client for threat intelligence operations in V-Sentinel.
    
    Provides methods for IOC management, threat actor research,
    and campaign tracking.
    """
    
    def __init__(self, client: httpx.AsyncClient, base_url: str):
        """
        Initialize the threat intelligence client.
        
        Args:
            client: HTTP client instance
            base_url: Base API URL
        """
        self._client = client
        self._base_url = base_url.rstrip("/")
        self._endpoint = f"{self._base_url}/threat-intel"
    
    # ==================== IOC Operations ====================
    
    async def list_iocs(
        self,
        *,
        ioc_type: Optional[IOCType] = None,
        confidence: Optional[IOCConfidence] = None,
        threat_actor: Optional[str] = None,
        malware_family: Optional[str] = None,
        tags: Optional[List[str]] = None,
        time_range: Optional[str] = None,
        sort_by: Optional[str] = None,
        sort_order: Optional[str] = "desc",
        limit: Optional[int] = 50,
        offset: Optional[int] = 0,
    ) -> IOCList:
        """
        List IOCs with optional filtering.
        
        Args:
            ioc_type: Filter by IOC type
            confidence: Filter by confidence level
            threat_actor: Filter by threat actor name
            malware_family: Filter by malware family
            tags: Filter by tags
            time_range: Time range filter (e.g., '24h', '7d', '30d')
            sort_by: Field to sort by
            sort_order: Sort order ('asc' or 'desc')
            limit: Maximum number of results
            offset: Offset for pagination
            
        Returns:
            IOCList with IOCs and pagination info
        """
        params: Dict[str, Any] = {
            "limit": limit,
            "offset": offset,
        }
        
        if ioc_type:
            params["type"] = ioc_type.value
        if confidence:
            params["confidence"] = confidence.value
        if threat_actor:
            params["threat_actor"] = threat_actor
        if malware_family:
            params["malware_family"] = malware_family
        if tags:
            params["tags"] = ",".join(tags)
        if time_range:
            params["time_range"] = time_range
        if sort_by:
            params["sort_by"] = sort_by
        if sort_order:
            params["sort_order"] = sort_order
        
        response = await self._client.get(
            f"{self._endpoint}/iocs",
            params=params,
        )
        response.raise_for_status()
        
        data = response.json()
        return IOCList(**data)
    
    async def get_ioc(self, ioc_id: str) -> IOC:
        """
        Get a specific IOC by ID.
        
        Args:
            ioc_id: Unique IOC identifier
            
        Returns:
            IOC details
            
        Raises:
            ResourceNotFoundError: If IOC not found
        """
        response = await self._client.get(
            f"{self._endpoint}/iocs/{ioc_id}",
        )
        
        if response.status_code == 404:
            raise ResourceNotFoundError(
                f"IOC not found: {ioc_id}",
                resource_type="ioc",
                resource_id=ioc_id,
            )
        
        response.raise_for_status()
        
        data = response.json()
        return IOC(**data)
    
    async def check_ioc(
        self,
        value: str,
        *,
        ioc_type: Optional[IOCType] = None,
    ) -> IOCCheckResult:
        """
        Check if an indicator is malicious.
        
        This is the primary method for checking IOCs against
        the threat intelligence database.
        
        Args:
            value: IOC value to check
            ioc_type: Optional IOC type (auto-detected if not provided)
            
        Returns:
            IOCCheckResult with threat intelligence data
        """
        params: Dict[str, Any] = {"value": value}
        
        if ioc_type:
            params["type"] = ioc_type.value
        
        response = await self._client.get(
            f"{self._endpoint}/check",
            params=params,
        )
        response.raise_for_status()
        
        data = response.json()
        return IOCCheckResult(**data)
    
    async def check_iocs_bulk(
        self,
        values: List[str],
        *,
        ioc_type: Optional[IOCType] = None,
    ) -> List[IOCCheckResult]:
        """
        Check multiple IOCs in bulk.
        
        Args:
            values: List of IOC values to check
            ioc_type: Optional IOC type for all values
            
        Returns:
            List of IOCCheckResult for each value
        """
        payload: Dict[str, Any] = {"values": values}
        
        if ioc_type:
            payload["type"] = ioc_type.value
        
        response = await self._client.post(
            f"{self._endpoint}/check/bulk",
            json=payload,
        )
        response.raise_for_status()
        
        data = response.json()
        return [IOCCheckResult(**result) for result in data.get("results", [])]
    
    async def add_ioc(
        self,
        value: str,
        ioc_type: IOCType,
        *,
        confidence: IOCConfidence = IOCConfidence.MEDIUM,
        threat_actor: Optional[str] = None,
        campaign: Optional[str] = None,
        malware_family: Optional[str] = None,
        tags: Optional[List[str]] = None,
        description: Optional[str] = None,
        references: Optional[List[str]] = None,
        expiration_days: Optional[int] = None,
    ) -> IOC:
        """
        Add a new IOC to the database.
        
        Args:
            value: IOC value
            ioc_type: Type of IOC
            confidence: Confidence level (default: MEDIUM)
            threat_actor: Associated threat actor
            campaign: Associated campaign
            malware_family: Associated malware family
            tags: List of tags
            description: Description of the IOC
            references: List of reference URLs
            expiration_days: Days until IOC expires
            
        Returns:
            Created IOC
        """
        payload: Dict[str, Any] = {
            "value": value,
            "type": ioc_type.value,
            "confidence": confidence.value,
        }
        
        if threat_actor:
            payload["threat_actor"] = threat_actor
        if campaign:
            payload["campaign"] = campaign
        if malware_family:
            payload["malware_family"] = malware_family
        if tags:
            payload["tags"] = tags
        if description:
            payload["description"] = description
        if references:
            payload["references"] = references
        if expiration_days:
            payload["expiration_days"] = expiration_days
        
        response = await self._client.post(
            f"{self._endpoint}/iocs",
            json=payload,
        )
        response.raise_for_status()
        
        data = response.json()
        return IOC(**data)
    
    async def update_ioc(
        self,
        ioc_id: str,
        *,
        confidence: Optional[IOCConfidence] = None,
        threat_actor: Optional[str] = None,
        campaign: Optional[str] = None,
        malware_family: Optional[str] = None,
        tags: Optional[List[str]] = None,
        description: Optional[str] = None,
    ) -> IOC:
        """
        Update an existing IOC.
        
        Args:
            ioc_id: Unique IOC identifier
            confidence: New confidence level
            threat_actor: New threat actor
            campaign: New campaign
            malware_family: New malware family
            tags: New tags list
            description: New description
            
        Returns:
            Updated IOC
        """
        payload: Dict[str, Any] = {}
        
        if confidence is not None:
            payload["confidence"] = confidence.value
        if threat_actor is not None:
            payload["threat_actor"] = threat_actor
        if campaign is not None:
            payload["campaign"] = campaign
        if malware_family is not None:
            payload["malware_family"] = malware_family
        if tags is not None:
            payload["tags"] = tags
        if description is not None:
            payload["description"] = description
        
        response = await self._client.patch(
            f"{self._endpoint}/iocs/{ioc_id}",
            json=payload,
        )
        
        if response.status_code == 404:
            raise ResourceNotFoundError(
                f"IOC not found: {ioc_id}",
                resource_type="ioc",
                resource_id=ioc_id,
            )
        
        response.raise_for_status()
        
        data = response.json()
        return IOC(**data)
    
    async def delete_ioc(self, ioc_id: str) -> bool:
        """
        Delete an IOC.
        
        Args:
            ioc_id: Unique IOC identifier
            
        Returns:
            True if deleted successfully
        """
        response = await self._client.delete(
            f"{self._endpoint}/iocs/{ioc_id}",
        )
        
        if response.status_code == 404:
            raise ResourceNotFoundError(
                f"IOC not found: {ioc_id}",
                resource_type="ioc",
                resource_id=ioc_id,
            )
        
        response.raise_for_status()
        return True
    
    async def search_iocs(
        self,
        query: str,
        *,
        limit: Optional[int] = 50,
    ) -> IOCList:
        """
        Search IOCs by query.
        
        Args:
            query: Search query
            limit: Maximum number of results
            
        Returns:
            IOCList with matching IOCs
        """
        response = await self._client.get(
            f"{self._endpoint}/iocs/search",
            params={"query": query, "limit": limit},
        )
        response.raise_for_status()
        
        data = response.json()
        return IOCList(**data)
    
    # ==================== Threat Actor Operations ====================
    
    async def list_threat_actors(
        self,
        *,
        country: Optional[str] = None,
        motivation: Optional[str] = None,
        tags: Optional[List[str]] = None,
        limit: Optional[int] = 50,
        offset: Optional[int] = 0,
    ) -> ThreatActorList:
        """
        List threat actors with optional filtering.
        
        Args:
            country: Filter by country of origin
            motivation: Filter by motivation type
            tags: Filter by tags
            limit: Maximum number of results
            offset: Offset for pagination
            
        Returns:
            ThreatActorList with actors and pagination info
        """
        params: Dict[str, Any] = {
            "limit": limit,
            "offset": offset,
        }
        
        if country:
            params["country"] = country
        if motivation:
            params["motivation"] = motivation
        if tags:
            params["tags"] = ",".join(tags)
        
        response = await self._client.get(
            f"{self._endpoint}/actors",
            params=params,
        )
        response.raise_for_status()
        
        data = response.json()
        return ThreatActorList(**data)
    
    async def get_threat_actor(self, actor_id: str) -> ThreatActor:
        """
        Get a specific threat actor by ID or name.
        
        Args:
            actor_id: Threat actor ID or name
            
        Returns:
            ThreatActor details
        """
        response = await self._client.get(
            f"{self._endpoint}/actors/{actor_id}",
        )
        
        if response.status_code == 404:
            raise ResourceNotFoundError(
                f"Threat actor not found: {actor_id}",
                resource_type="threat_actor",
                resource_id=actor_id,
            )
        
        response.raise_for_status()
        
        data = response.json()
        return ThreatActor(**data)
    
    async def get_threat_actor_iocs(
        self,
        actor_id: str,
        *,
        limit: Optional[int] = 100,
    ) -> IOCList:
        """
        Get IOCs associated with a threat actor.
        
        Args:
            actor_id: Threat actor ID or name
            limit: Maximum number of results
            
        Returns:
            IOCList with actor's IOCs
        """
        response = await self._client.get(
            f"{self._endpoint}/actors/{actor_id}/iocs",
            params={"limit": limit},
        )
        
        if response.status_code == 404:
            raise ResourceNotFoundError(
                f"Threat actor not found: {actor_id}",
                resource_type="threat_actor",
                resource_id=actor_id,
            )
        
        response.raise_for_status()
        
        data = response.json()
        return IOCList(**data)
    
    # ==================== Campaign Operations ====================
    
    async def list_campaigns(
        self,
        *,
        threat_actor: Optional[str] = None,
        status: Optional[str] = None,
        time_range: Optional[str] = None,
        limit: Optional[int] = 50,
        offset: Optional[int] = 0,
    ) -> CampaignList:
        """
        List campaigns with optional filtering.
        
        Args:
            threat_actor: Filter by threat actor
            status: Filter by campaign status
            time_range: Time range filter
            limit: Maximum number of results
            offset: Offset for pagination
            
        Returns:
            CampaignList with campaigns and pagination info
        """
        params: Dict[str, Any] = {
            "limit": limit,
            "offset": offset,
        }
        
        if threat_actor:
            params["threat_actor"] = threat_actor
        if status:
            params["status"] = status
        if time_range:
            params["time_range"] = time_range
        
        response = await self._client.get(
            f"{self._endpoint}/campaigns",
            params=params,
        )
        response.raise_for_status()
        
        data = response.json()
        return CampaignList(**data)
    
    async def get_campaign(self, campaign_id: str) -> Campaign:
        """
        Get a specific campaign by ID.
        
        Args:
            campaign_id: Unique campaign identifier
            
        Returns:
            Campaign details
        """
        response = await self._client.get(
            f"{self._endpoint}/campaigns/{campaign_id}",
        )
        
        if response.status_code == 404:
            raise ResourceNotFoundError(
                f"Campaign not found: {campaign_id}",
                resource_type="campaign",
                resource_id=campaign_id,
            )
        
        response.raise_for_status()
        
        data = response.json()
        return Campaign(**data)
    
    async def get_campaign_iocs(
        self,
        campaign_id: str,
        *,
        limit: Optional[int] = 100,
    ) -> IOCList:
        """
        Get IOCs associated with a campaign.
        
        Args:
            campaign_id: Unique campaign identifier
            limit: Maximum number of results
            
        Returns:
            IOCList with campaign's IOCs
        """
        response = await self._client.get(
            f"{self._endpoint}/campaigns/{campaign_id}/iocs",
            params={"limit": limit},
        )
        
        if response.status_code == 404:
            raise ResourceNotFoundError(
                f"Campaign not found: {campaign_id}",
                resource_type="campaign",
                resource_id=campaign_id,
            )
        
        response.raise_for_status()
        
        data = response.json()
        return IOCList(**data)
    
    # ==================== Export Operations ====================
    
    async def export_iocs(
        self,
        *,
        ioc_type: Optional[IOCType] = None,
        threat_actor: Optional[str] = None,
        malware_family: Optional[str] = None,
        tags: Optional[List[str]] = None,
        format: str = "json",
        include_expired: bool = False,
    ) -> str:
        """
        Export IOCs in specified format.
        
        Args:
            ioc_type: Filter by IOC type
            threat_actor: Filter by threat actor
            malware_family: Filter by malware family
            tags: Filter by tags
            format: Export format ('json', 'stix', 'csv', 'txt')
            include_expired: Include expired IOCs
            
        Returns:
            Exported data as string
        """
        params: Dict[str, Any] = {
            "format": format,
            "include_expired": include_expired,
        }
        
        if ioc_type:
            params["type"] = ioc_type.value
        if threat_actor:
            params["threat_actor"] = threat_actor
        if malware_family:
            params["malware_family"] = malware_family
        if tags:
            params["tags"] = ",".join(tags)
        
        response = await self._client.get(
            f"{self._endpoint}/export",
            params=params,
        )
        response.raise_for_status()
        
        return response.text
    
    async def export_for_siEM(
        self,
        siem_type: str,
        *,
        ioc_types: Optional[List[IOCType]] = None,
        threat_actor: Optional[str] = None,
    ) -> str:
        """
        Export IOCs formatted for specific SIEM.
        
        Args:
            siem_type: SIEM type ('splunk', 'elastic', 'sentinel', 'qradar')
            ioc_types: Filter by IOC types
            threat_actor: Filter by threat actor
            
        Returns:
            SIEM-formatted data
        """
        params: Dict[str, Any] = {"siem": siem_type}
        
        if ioc_types:
            params["types"] = ",".join(t.value for t in ioc_types)
        if threat_actor:
            params["threat_actor"] = threat_actor
        
        response = await self._client.get(
            f"{self._endpoint}/export/siem",
            params=params,
        )
        response.raise_for_status()
        
        return response.text