"""
V-Sentinel Client
~~~~~~~~~~~~~~~~~

Main client for interacting with V-Sentinel API.
"""

import asyncio
from typing import Any, Dict, Optional, Union
from urllib.parse import urljoin

import httpx
from pydantic import ValidationError as PydanticValidationError

from v_sentinel_sdk.clients.detections import DetectionsClient
from v_sentinel_sdk.clients.hosts import HostsClient
from v_sentinel_sdk.clients.incidents import IncidentsClient
from v_sentinel_sdk.clients.threat_intel import ThreatIntelClient
from v_sentinel_sdk.exceptions import (
    AuthenticationError,
    RateLimitError,
    ResourceNotFoundError,
    VSentinelError,
    ValidationError,
)
from v_sentinel_sdk.models.config import ClientConfig
from v_sentinel_sdk.utils.retry import RetryHandler
from v_sentinel_sdk.utils.logging import get_logger

logger = get_logger(__name__)


class VSentinelClient:
    """
    Main client for V-Sentinel API.
    
    This client provides access to all V-Sentinel features:
    - Detections management
    - Host inventory
    - Incident response
    - Threat intelligence
    - MCP integration
    
    Args:
        api_key: Your V-Sentinel API key
        base_url: Base URL for V-Sentinel API (default: https://api.vantis.ai/v1)
        timeout: Request timeout in seconds (default: 30)
        max_retries: Maximum number of retries for failed requests (default: 3)
        verify_ssl: Whether to verify SSL certificates (default: True)
        
    Example:
        >>> client = VSentinelClient(api_key="your-api-key")
        >>> detections = await client.detections.list(limit=10)
    """
    
    def __init__(
        self,
        api_key: str,
        base_url: str = "https://api.vantis.ai/v1",
        timeout: int = 30,
        max_retries: int = 3,
        verify_ssl: bool = True,
        **kwargs: Any,
    ) -> None:
        self._config = ClientConfig(
            api_key=api_key,
            base_url=base_url.rstrip("/"),
            timeout=timeout,
            max_retries=max_retries,
            verify_ssl=verify_ssl,
            **kwargs,
        )
        
        self._http_client: Optional[httpx.AsyncClient] = None
        self._retry_handler = RetryHandler(max_retries=max_retries)
        
        # Initialize sub-clients
        self._detections: Optional[DetectionsClient] = None
        self._hosts: Optional[HostsClient] = None
        self._incidents: Optional[IncidentsClient] = None
        self._threat_intel: Optional[ThreatIntelClient] = None
    
    @property
    def detections(self) -> DetectionsClient:
        """Access the detections API."""
        if self._detections is None:
            self._detections = DetectionsClient(self)
        return self._detections
    
    @property
    def hosts(self) -> HostsClient:
        """Access the hosts API."""
        if self._hosts is None:
            self._hosts = HostsClient(self)
        return self._hosts
    
    @property
    def incidents(self) -> IncidentsClient:
        """Access the incidents API."""
        if self._incidents is None:
            self._incidents = IncidentsClient(self)
        return self._incidents
    
    @property
    def threat_intel(self) -> ThreatIntelClient:
        """Access the threat intelligence API."""
        if self._threat_intel is None:
            self._threat_intel = ThreatIntelClient(self)
        return self._threat_intel
    
    async def __aenter__(self) -> "VSentinelClient":
        """Async context manager entry."""
        await self._ensure_client()
        return self
    
    async def __aexit__(self, exc_type: Any, exc_val: Any, exc_tb: Any) -> None:
        """Async context manager exit."""
        await self.close()
    
    async def _ensure_client(self) -> httpx.AsyncClient:
        """Ensure HTTP client is initialized."""
        if self._http_client is None:
            self._http_client = httpx.AsyncClient(
                base_url=self._config.base_url,
                timeout=httpx.Timeout(self._config.timeout),
                verify=self._config.verify_ssl,
                headers={
                    "Authorization": f"Bearer {self._config.api_key}",
                    "Content-Type": "application/json",
                    "User-Agent": f"v-sentinel-sdk-python/{__import__('v_sentinel_sdk', fromlist=['__version__']).__version__}",
                },
            )
        return self._http_client
    
    async def close(self) -> None:
        """Close the HTTP client and cleanup resources."""
        if self._http_client is not None:
            await self._http_client.aclose()
            self._http_client = None
            logger.debug("HTTP client closed")
    
    async def request(
        self,
        method: str,
        path: str,
        **kwargs: Any,
    ) -> Dict[str, Any]:
        """
        Make an HTTP request to the V-Sentinel API.
        
        Args:
            method: HTTP method (GET, POST, PUT, DELETE, etc.)
            path: API endpoint path
            **kwargs: Additional arguments to pass to httpx
            
        Returns:
            Response data as dictionary
            
        Raises:
            AuthenticationError: If API key is invalid
            RateLimitError: If rate limit is exceeded
            ResourceNotFoundError: If resource is not found
            ValidationError: If request validation fails
            VSentinelError: For other API errors
        """
        client = await self._ensure_client()
        
        async def _make_request() -> Dict[str, Any]:
            response = await client.request(method, path, **kwargs)
            return self._handle_response(response)
        
        return await self._retry_handler.execute(_make_request)
    
    def _handle_response(self, response: httpx.Response) -> Dict[str, Any]:
        """Handle HTTP response and raise appropriate exceptions."""
        if response.status_code == 200:
            return response.json()
        
        if response.status_code == 201:
            return response.json()
        
        if response.status_code == 204:
            return {}
        
        if response.status_code == 400:
            error_data = response.json()
            raise ValidationError(
                message=error_data.get("message", "Validation error"),
                details=error_data.get("details"),
            )
        
        if response.status_code == 401:
            raise AuthenticationError("Invalid API key or unauthorized access")
        
        if response.status_code == 404:
            error_data = response.json()
            raise ResourceNotFoundError(
                resource=error_data.get("resource", "Unknown"),
                message=error_data.get("message", "Resource not found"),
            )
        
        if response.status_code == 429:
            retry_after = response.headers.get("Retry-After", "60")
            raise RateLimitError(
                retry_after=int(retry_after),
                message="Rate limit exceeded",
            )
        
        # Handle other errors
        try:
            error_data = response.json()
            message = error_data.get("message", f"API error: {response.status_code}")
        except Exception:
            message = f"API error: {response.status_code}"
        
        raise VSentinelError(message, status_code=response.status_code)
    
    # Convenience methods
    async def get(self, path: str, **kwargs: Any) -> Dict[str, Any]:
        """Make a GET request."""
        return await self.request("GET", path, **kwargs)
    
    async def post(self, path: str, **kwargs: Any) -> Dict[str, Any]:
        """Make a POST request."""
        return await self.request("POST", path, **kwargs)
    
    async def put(self, path: str, **kwargs: Any) -> Dict[str, Any]:
        """Make a PUT request."""
        return await self.request("PUT", path, **kwargs)
    
    async def delete(self, path: str, **kwargs: Any) -> Dict[str, Any]:
        """Make a DELETE request."""
        return await self.request("DELETE", path, **kwargs)
    
    async def health_check(self) -> bool:
        """
        Check if the V-Sentinel API is healthy.
        
        Returns:
            True if API is healthy, False otherwise
        """
        try:
            await self.get("/health")
            return True
        except Exception as e:
            logger.warning(f"Health check failed: {e}")
            return False
    
    async def get_version(self) -> Dict[str, Any]:
        """
        Get V-Sentinel API version information.
        
        Returns:
            Version information dictionary
        """
        return await self.get("/version")