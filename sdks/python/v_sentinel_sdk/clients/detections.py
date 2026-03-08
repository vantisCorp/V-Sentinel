"""
Detections API Client
~~~~~~~~~~~~~~~~~~~~~

Client for V-Sentinel detections API.
"""

from typing import TYPE_CHECKING, Any, Dict, List, Optional

from v_sentinel_sdk.models.detections import Detection, DetectionList, DetectionQuery

if TYPE_CHECKING:
    from v_sentinel_sdk.client import VSentinelClient


class DetectionsClient:
    """
    Client for V-Sentinel detections API.
    
    This client provides methods to manage and query security detections.
    
    Example:
        >>> async with VSentinelClient(api_key="key") as client:
        ...     detections = await client.detections.list(limit=10)
        ...     for det in detections.detections:
        ...         print(f"{det.title}: {det.severity}")
    """
    
    def __init__(self, client: "VSentinelClient") -> None:
        self._client = client
    
    async def list(
        self,
        query: Optional[str] = None,
        severity: Optional[List[str]] = None,
        status: Optional[List[str]] = None,
        host_id: Optional[str] = None,
        time_range: Optional[str] = None,
        limit: int = 50,
        offset: int = 0,
        **kwargs: Any,
    ) -> DetectionList:
        """
        List detections with optional filters.
        
        Args:
            query: Search query string
            severity: Filter by severity levels
            status: Filter by status values
            host_id: Filter by host ID
            time_range: Time range filter (e.g., '24h', '7d', '30d')
            limit: Maximum number of results
            offset: Result offset for pagination
            **kwargs: Additional query parameters
            
        Returns:
            DetectionList with detections and pagination info
            
        Example:
            >>> detections = await client.detections.list(
            ...     severity=["high", "critical"],
            ...     time_range="24h",
            ...     limit=100
            ... )
        """
        params: Dict[str, Any] = {
            "limit": limit,
            "offset": offset,
        }
        
        if query:
            params["query"] = query
        if severity:
            params["severity"] = ",".join(severity)
        if status:
            params["status"] = ",".join(status)
        if host_id:
            params["host_id"] = host_id
        if time_range:
            params["time_range"] = time_range
        
        params.update(kwargs)
        
        response = await self._client.get("/detections", params=params)
        return DetectionList(**response)
    
    async def get(self, detection_id: str) -> Detection:
        """
        Get a specific detection by ID.
        
        Args:
            detection_id: Unique detection identifier
            
        Returns:
            Detection details
            
        Raises:
            ResourceNotFoundError: If detection not found
            
        Example:
            >>> detection = await client.detections.get("det-001")
            >>> print(detection.title)
        """
        response = await self._client.get(f"/detections/{detection_id}")
        return Detection(**response)
    
    async def search(self, query: DetectionQuery) -> DetectionList:
        """
        Search detections with advanced query.
        
        Args:
            query: DetectionQuery with search parameters
            
        Returns:
            DetectionList with matching detections
            
        Example:
            >>> query = DetectionQuery(
            ...     query="powershell",
            ...     severity=[DetectionSeverity.HIGH],
            ...     time_range="7d"
            ... )
            >>> results = await client.detections.search(query)
        """
        response = await self._client.post("/detections/search", json=query.model_dump(exclude_none=True))
        return DetectionList(**response)
    
    async def update_status(
        self,
        detection_id: str,
        status: str,
        notes: Optional[str] = None,
    ) -> Detection:
        """
        Update detection status.
        
        Args:
            detection_id: Detection ID to update
            status: New status value
            notes: Optional notes for the status change
            
        Returns:
            Updated detection
            
        Example:
            >>> detection = await client.detections.update_status(
            ...     "det-001",
            ...     "resolved",
            ...     notes="False positive - legitimate admin activity"
            ... )
        """
        data: Dict[str, Any] = {"status": status}
        if notes:
            data["notes"] = notes
        
        response = await self._client.patch(f"/detections/{detection_id}", json=data)
        return Detection(**response)
    
    async def assign(
        self,
        detection_id: str,
        assignee: str,
    ) -> Detection:
        """
        Assign detection to an analyst.
        
        Args:
            detection_id: Detection ID to assign
            assignee: Username or ID of the analyst
            
        Returns:
            Updated detection
        """
        response = await self._client.patch(
            f"/detections/{detection_id}",
            json={"assigned_to": assignee}
        )
        return Detection(**response)
    
    async def add_note(
        self,
        detection_id: str,
        note: str,
    ) -> Detection:
        """
        Add a note to a detection.
        
        Args:
            detection_id: Detection ID
            note: Note content to add
            
        Returns:
            Updated detection
        """
        response = await self._client.post(
            f"/detections/{detection_id}/notes",
            json={"content": note}
        )
        return Detection(**response)
    
    async def get_related(
        self,
        detection_id: str,
    ) -> List[Detection]:
        """
        Get related detections.
        
        Args:
            detection_id: Detection ID
            
        Returns:
            List of related detections
        """
        response = await self._client.get(f"/detections/{detection_id}/related")
        return [Detection(**d) for d in response.get("detections", [])]
    
    async def statistics(
        self,
        time_range: str = "7d",
    ) -> Dict[str, Any]:
        """
        Get detection statistics.
        
        Args:
            time_range: Time range for statistics
            
        Returns:
            Detection statistics dictionary
        """
        return await self._client.get("/detections/statistics", params={"time_range": time_range})