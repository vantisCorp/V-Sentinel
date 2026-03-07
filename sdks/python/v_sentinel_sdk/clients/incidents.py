"""
V-Sentinel Incidents API Client

Provides async methods for incident response operations.
"""

from typing import Optional, List, Dict, Any
import httpx

from ..models.incidents import (
    Incident,
    IncidentList,
    IncidentStatus,
    IncidentSeverity,
    IncidentPhase,
    IncidentTimelineEntry,
    IncidentNote,
)
from ..exceptions import (
    ValidationError,
    ResourceNotFoundError,
    APIError,
)


class IncidentsClient:
    """
    Client for managing security incidents in V-Sentinel.
    
    Provides methods for incident lifecycle management including
    creation, tracking, and resolution.
    """
    
    def __init__(self, client: httpx.AsyncClient, base_url: str):
        """
        Initialize the incidents client.
        
        Args:
            client: HTTP client instance
            base_url: Base API URL
        """
        self._client = client
        self._base_url = base_url.rstrip("/")
        self._endpoint = f"{self._base_url}/incidents"
    
    async def list(
        self,
        *,
        query: Optional[str] = None,
        severity: Optional[IncidentSeverity] = None,
        status: Optional[IncidentStatus] = None,
        phase: Optional[IncidentPhase] = None,
        assignee: Optional[str] = None,
        time_range: Optional[str] = None,
        sort_by: Optional[str] = None,
        sort_order: Optional[str] = "desc",
        limit: Optional[int] = 50,
        offset: Optional[int] = 0,
    ) -> IncidentList:
        """
        List incidents with optional filtering.
        
        Args:
            query: Search query string
            severity: Filter by severity level
            status: Filter by status
            phase: Filter by incident phase
            assignee: Filter by assignee ID
            time_range: Time range filter (e.g., '24h', '7d', '30d')
            sort_by: Field to sort by
            sort_order: Sort order ('asc' or 'desc')
            limit: Maximum number of results
            offset: Offset for pagination
            
        Returns:
            IncidentList with incidents and pagination info
        """
        params: Dict[str, Any] = {
            "limit": limit,
            "offset": offset,
        }
        
        if query:
            params["query"] = query
        if severity:
            params["severity"] = severity.value
        if status:
            params["status"] = status.value
        if phase:
            params["phase"] = phase.value
        if assignee:
            params["assignee"] = assignee
        if time_range:
            params["time_range"] = time_range
        if sort_by:
            params["sort_by"] = sort_by
        if sort_order:
            params["sort_order"] = sort_order
        
        response = await self._client.get(self._endpoint, params=params)
        response.raise_for_status()
        
        data = response.json()
        return IncidentList(**data)
    
    async def get(self, incident_id: str) -> Incident:
        """
        Get a specific incident by ID.
        
        Args:
            incident_id: Unique incident identifier
            
        Returns:
            Incident details
            
        Raises:
            ResourceNotFoundError: If incident not found
        """
        response = await self._client.get(f"{self._endpoint}/{incident_id}")
        
        if response.status_code == 404:
            raise ResourceNotFoundError(
                f"Incident not found: {incident_id}",
                resource_type="incident",
                resource_id=incident_id,
            )
        
        response.raise_for_status()
        
        data = response.json()
        return Incident(**data)
    
    async def create(
        self,
        *,
        title: str,
        description: str,
        severity: IncidentSeverity,
        hosts: Optional[List[str]] = None,
        detections: Optional[List[str]] = None,
        tags: Optional[List[str]] = None,
        assignee: Optional[str] = None,
    ) -> Incident:
        """
        Create a new incident.
        
        Args:
            title: Incident title
            description: Detailed description
            severity: Incident severity
            hosts: List of affected host IDs
            detections: List of related detection IDs
            tags: List of tags
            assignee: User ID to assign the incident to
            
        Returns:
            Created incident
        """
        payload: Dict[str, Any] = {
            "title": title,
            "description": description,
            "severity": severity.value,
        }
        
        if hosts:
            payload["hosts"] = hosts
        if detections:
            payload["detections"] = detections
        if tags:
            payload["tags"] = tags
        if assignee:
            payload["assignee"] = assignee
        
        response = await self._client.post(self._endpoint, json=payload)
        response.raise_for_status()
        
        data = response.json()
        return Incident(**data)
    
    async def update(
        self,
        incident_id: str,
        *,
        title: Optional[str] = None,
        description: Optional[str] = None,
        severity: Optional[IncidentSeverity] = None,
        tags: Optional[List[str]] = None,
    ) -> Incident:
        """
        Update an incident.
        
        Args:
            incident_id: Unique incident identifier
            title: New title
            description: New description
            severity: New severity
            tags: New tags list
            
        Returns:
            Updated incident
        """
        payload: Dict[str, Any] = {}
        
        if title is not None:
            payload["title"] = title
        if description is not None:
            payload["description"] = description
        if severity is not None:
            payload["severity"] = severity.value
        if tags is not None:
            payload["tags"] = tags
        
        response = await self._client.patch(
            f"{self._endpoint}/{incident_id}",
            json=payload,
        )
        
        if response.status_code == 404:
            raise ResourceNotFoundError(
                f"Incident not found: {incident_id}",
                resource_type="incident",
                resource_id=incident_id,
            )
        
        response.raise_for_status()
        
        data = response.json()
        return Incident(**data)
    
    async def update_status(
        self,
        incident_id: str,
        status: IncidentStatus,
        *,
        notes: Optional[str] = None,
    ) -> Incident:
        """
        Update incident status.
        
        Args:
            incident_id: Unique incident identifier
            status: New status
            notes: Optional notes for status change
            
        Returns:
            Updated incident
        """
        payload: Dict[str, Any] = {
            "status": status.value,
        }
        
        if notes:
            payload["notes"] = notes
        
        response = await self._client.patch(
            f"{self._endpoint}/{incident_id}/status",
            json=payload,
        )
        
        if response.status_code == 404:
            raise ResourceNotFoundError(
                f"Incident not found: {incident_id}",
                resource_type="incident",
                resource_id=incident_id,
            )
        
        response.raise_for_status()
        
        data = response.json()
        return Incident(**data)
    
    async def update_phase(
        self,
        incident_id: str,
        phase: IncidentPhase,
        *,
        notes: Optional[str] = None,
    ) -> Incident:
        """
        Update incident phase.
        
        Args:
            incident_id: Unique incident identifier
            phase: New phase
            notes: Optional notes for phase change
            
        Returns:
            Updated incident
        """
        payload: Dict[str, Any] = {
            "phase": phase.value,
        }
        
        if notes:
            payload["notes"] = notes
        
        response = await self._client.patch(
            f"{self._endpoint}/{incident_id}/phase",
            json=payload,
        )
        
        if response.status_code == 404:
            raise ResourceNotFoundError(
                f"Incident not found: {incident_id}",
                resource_type="incident",
                resource_id=incident_id,
            )
        
        response.raise_for_status()
        
        data = response.json()
        return Incident(**data)
    
    async def assign(
        self,
        incident_id: str,
        assignee: str,
    ) -> Incident:
        """
        Assign an incident to a user.
        
        Args:
            incident_id: Unique incident identifier
            assignee: User ID to assign to
            
        Returns:
            Updated incident
        """
        response = await self._client.patch(
            f"{self._endpoint}/{incident_id}/assign",
            json={"assignee": assignee},
        )
        
        if response.status_code == 404:
            raise ResourceNotFoundError(
                f"Incident not found: {incident_id}",
                resource_type="incident",
                resource_id=incident_id,
            )
        
        response.raise_for_status()
        
        data = response.json()
        return Incident(**data)
    
    async def add_note(
        self,
        incident_id: str,
        note: str,
        *,
        visibility: Optional[str] = "team",
    ) -> IncidentNote:
        """
        Add a note to an incident.
        
        Args:
            incident_id: Unique incident identifier
            note: Note content
            visibility: Note visibility ('team' or 'public')
            
        Returns:
            Created note
        """
        response = await self._client.post(
            f"{self._endpoint}/{incident_id}/notes",
            json={
                "content": note,
                "visibility": visibility,
            },
        )
        
        if response.status_code == 404:
            raise ResourceNotFoundError(
                f"Incident not found: {incident_id}",
                resource_type="incident",
                resource_id=incident_id,
            )
        
        response.raise_for_status()
        
        data = response.json()
        return IncidentNote(**data)
    
    async def list_notes(
        self,
        incident_id: str,
        *,
        limit: Optional[int] = 50,
        offset: Optional[int] = 0,
    ) -> List[IncidentNote]:
        """
        List notes for an incident.
        
        Args:
            incident_id: Unique incident identifier
            limit: Maximum number of results
            offset: Offset for pagination
            
        Returns:
            List of notes
        """
        response = await self._client.get(
            f"{self._endpoint}/{incident_id}/notes",
            params={"limit": limit, "offset": offset},
        )
        
        if response.status_code == 404:
            raise ResourceNotFoundError(
                f"Incident not found: {incident_id}",
                resource_type="incident",
                resource_id=incident_id,
            )
        
        response.raise_for_status()
        
        data = response.json()
        return [IncidentNote(**note) for note in data.get("notes", [])]
    
    async def get_timeline(
        self,
        incident_id: str,
    ) -> List[IncidentTimelineEntry]:
        """
        Get incident timeline.
        
        Args:
            incident_id: Unique incident identifier
            
        Returns:
            List of timeline entries
        """
        response = await self._client.get(
            f"{self._endpoint}/{incident_id}/timeline",
        )
        
        if response.status_code == 404:
            raise ResourceNotFoundError(
                f"Incident not found: {incident_id}",
                resource_type="incident",
                resource_id=incident_id,
            )
        
        response.raise_for_status()
        
        data = response.json()
        return [IncidentTimelineEntry(**entry) for entry in data.get("entries", [])]
    
    async def add_hosts(
        self,
        incident_id: str,
        host_ids: List[str],
    ) -> Incident:
        """
        Add hosts to an incident.
        
        Args:
            incident_id: Unique incident identifier
            host_ids: List of host IDs to add
            
        Returns:
            Updated incident
        """
        response = await self._client.post(
            f"{self._endpoint}/{incident_id}/hosts",
            json={"host_ids": host_ids},
        )
        
        if response.status_code == 404:
            raise ResourceNotFoundError(
                f"Incident not found: {incident_id}",
                resource_type="incident",
                resource_id=incident_id,
            )
        
        response.raise_for_status()
        
        data = response.json()
        return Incident(**data)
    
    async def remove_hosts(
        self,
        incident_id: str,
        host_ids: List[str],
    ) -> Incident:
        """
        Remove hosts from an incident.
        
        Args:
            incident_id: Unique incident identifier
            host_ids: List of host IDs to remove
            
        Returns:
            Updated incident
        """
        response = await self._client.delete(
            f"{self._endpoint}/{incident_id}/hosts",
            json={"host_ids": host_ids},
        )
        
        if response.status_code == 404:
            raise ResourceNotFoundError(
                f"Incident not found: {incident_id}",
                resource_type="incident",
                resource_id=incident_id,
            )
        
        response.raise_for_status()
        
        data = response.json()
        return Incident(**data)
    
    async def add_detections(
        self,
        incident_id: str,
        detection_ids: List[str],
    ) -> Incident:
        """
        Add detections to an incident.
        
        Args:
            incident_id: Unique incident identifier
            detection_ids: List of detection IDs to add
            
        Returns:
            Updated incident
        """
        response = await self._client.post(
            f"{self._endpoint}/{incident_id}/detections",
            json={"detection_ids": detection_ids},
        )
        
        if response.status_code == 404:
            raise ResourceNotFoundError(
                f"Incident not found: {incident_id}",
                resource_type="incident",
                resource_id=incident_id,
            )
        
        response.raise_for_status()
        
        data = response.json()
        return Incident(**data)
    
    async def remove_detections(
        self,
        incident_id: str,
        detection_ids: List[str],
    ) -> Incident:
        """
        Remove detections from an incident.
        
        Args:
            incident_id: Unique incident identifier
            detection_ids: List of detection IDs to remove
            
        Returns:
            Updated incident
        """
        response = await self._client.delete(
            f"{self._endpoint}/{incident_id}/detections",
            json={"detection_ids": detection_ids},
        )
        
        if response.status_code == 404:
            raise ResourceNotFoundError(
                f"Incident not found: {incident_id}",
                resource_type="incident",
                resource_id=incident_id,
            )
        
        response.raise_for_status()
        
        data = response.json()
        return Incident(**data)
    
    async def close(
        self,
        incident_id: str,
        *,
        resolution: str,
        lessons_learned: Optional[str] = None,
    ) -> Incident:
        """
        Close an incident.
        
        Args:
            incident_id: Unique incident identifier
            resolution: Resolution summary
            lessons_learned: Optional lessons learned
            
        Returns:
            Closed incident
        """
        payload: Dict[str, Any] = {
            "resolution": resolution,
        }
        
        if lessons_learned:
            payload["lessons_learned"] = lessons_learned
        
        response = await self._client.post(
            f"{self._endpoint}/{incident_id}/close",
            json=payload,
        )
        
        if response.status_code == 404:
            raise ResourceNotFoundError(
                f"Incident not found: {incident_id}",
                resource_type="incident",
                resource_id=incident_id,
            )
        
        response.raise_for_status()
        
        data = response.json()
        return Incident(**data)
    
    async def search(
        self,
        query: str,
        *,
        limit: Optional[int] = 50,
    ) -> IncidentList:
        """
        Search incidents by query.
        
        Args:
            query: Search query
            limit: Maximum number of results
            
        Returns:
            IncidentList with matching incidents
        """
        response = await self._client.get(
            f"{self._endpoint}/search",
            params={"query": query, "limit": limit},
        )
        response.raise_for_status()
        
        data = response.json()
        return IncidentList(**data)