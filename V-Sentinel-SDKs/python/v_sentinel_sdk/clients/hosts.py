"""
Hosts API Client
~~~~~~~~~~~~~~~~

Client for V-Sentinel hosts API.
"""

from typing import TYPE_CHECKING, Any, Dict, List, Optional

from v_sentinel_sdk.models.hosts import Host, HostList, HostQuery, HostState

if TYPE_CHECKING:
    from v_sentinel_sdk.client import VSentinelClient


class HostsClient:
    """
    Client for V-Sentinel hosts API.
    
    This client provides methods to manage and query host inventory.
    
    Example:
        >>> async with VSentinelClient(api_key="key") as client:
        ...     hosts = await client.hosts.list(limit=10)
        ...     for host in hosts.hosts:
        ...         print(f"{host.hostname}: {host.state}")
    """
    
    def __init__(self, client: "VSentinelClient") -> None:
        self._client = client
    
    async def list(
        self,
        query: Optional[str] = None,
        platform: Optional[List[str]] = None,
        state: Optional[List[str]] = None,
        group_id: Optional[str] = None,
        is_isolated: Optional[bool] = None,
        limit: int = 50,
        offset: int = 0,
        **kwargs: Any,
    ) -> HostList:
        """
        List hosts with optional filters.
        
        Args:
            query: Search query string
            platform: Filter by platform (windows, linux, macos)
            state: Filter by state (online, offline, isolated)
            group_id: Filter by group ID
            is_isolated: Filter by isolation status
            limit: Maximum number of results
            offset: Result offset for pagination
            **kwargs: Additional query parameters
            
        Returns:
            HostList with hosts and pagination info
        """
        params: Dict[str, Any] = {
            "limit": limit,
            "offset": offset,
        }
        
        if query:
            params["query"] = query
        if platform:
            params["platform"] = ",".join(platform)
        if state:
            params["state"] = ",".join(state)
        if group_id:
            params["group_id"] = group_id
        if is_isolated is not None:
            params["is_isolated"] = str(is_isolated).lower()
        
        params.update(kwargs)
        
        response = await self._client.get("/hosts", params=params)
        return HostList(**response)
    
    async def get(self, host_id: str) -> Host:
        """
        Get a specific host by ID.
        
        Args:
            host_id: Unique host identifier
            
        Returns:
            Host details
        """
        response = await self._client.get(f"/hosts/{host_id}")
        return Host(**response)
    
    async def search(self, query: HostQuery) -> HostList:
        """
        Search hosts with advanced query.
        
        Args:
            query: HostQuery with search parameters
            
        Returns:
            HostList with matching hosts
        """
        response = await self._client.post("/hosts/search", json=query.model_dump(exclude_none=True))
        return HostList(**response)
    
    async def isolate(
        self,
        host_id: str,
        reason: str,
    ) -> Host:
        """
        Isolate a host from the network.
        
        Args:
            host_id: Host ID to isolate
            reason: Reason for isolation
            
        Returns:
            Updated host
        """
        response = await self._client.post(
            f"/hosts/{host_id}/isolate",
            json={"reason": reason}
        )
        return Host(**response)
    
    async def unisolate(
        self,
        host_id: str,
    ) -> Host:
        """
        Remove network isolation from a host.
        
        Args:
            host_id: Host ID to unisolate
            
        Returns:
            Updated host
        """
        response = await self._client.post(f"/hosts/{host_id}/unisolate")
        return Host(**response)
    
    async def get_detections(
        self,
        host_id: str,
        limit: int = 50,
    ) -> Dict[str, Any]:
        """
        Get detections for a specific host.
        
        Args:
            host_id: Host ID
            limit: Maximum number of detections
            
        Returns:
            Detections for the host
        """
        return await self._client.get(f"/hosts/{host_id}/detections", params={"limit": limit})
    
    async def get_timeline(
        self,
        host_id: str,
        time_range: str = "24h",
    ) -> Dict[str, Any]:
        """
        Get activity timeline for a host.
        
        Args:
            host_id: Host ID
            time_range: Time range for timeline
            
        Returns:
            Activity timeline
        """
        return await self._client.get(
            f"/hosts/{host_id}/timeline",
            params={"time_range": time_range}
        )
    
    async def add_tag(
        self,
        host_id: str,
        tag: str,
    ) -> Host:
        """
        Add a tag to a host.
        
        Args:
            host_id: Host ID
            tag: Tag to add
            
        Returns:
            Updated host
        """
        response = await self._client.post(f"/hosts/{host_id}/tags", json={"tag": tag})
        return Host(**response)
    
    async def remove_tag(
        self,
        host_id: str,
        tag: str,
    ) -> Host:
        """
        Remove a tag from a host.
        
        Args:
            host_id: Host ID
            tag: Tag to remove
            
        Returns:
            Updated host
        """
        response = await self._client.delete(f"/hosts/{host_id}/tags/{tag}")
        return Host(**response)
    
    async def statistics(self) -> Dict[str, Any]:
        """
        Get host statistics.
        
        Returns:
            Host statistics dictionary
        """
        return await self._client.get("/hosts/statistics")