"""
V-Sentinel SDK Exceptions
~~~~~~~~~~~~~~~~~~~~~~~~~

Custom exceptions for the V-Sentinel SDK.
"""

from typing import Any, Dict, Optional


class VSentinelError(Exception):
    """
    Base exception for all V-Sentinel SDK errors.
    
    Args:
        message: Error message
        status_code: HTTP status code (if applicable)
        details: Additional error details
    """
    
    def __init__(
        self,
        message: str,
        status_code: Optional[int] = None,
        details: Optional[Dict[str, Any]] = None,
    ) -> None:
        super().__init__(message)
        self.message = message
        self.status_code = status_code
        self.details = details or {}
    
    def __str__(self) -> str:
        if self.status_code:
            return f"[{self.status_code}] {self.message}"
        return self.message
    
    def __repr__(self) -> str:
        return f"{self.__class__.__name__}(message={self.message!r}, status_code={self.status_code})"


class AuthenticationError(VSentinelError):
    """
    Raised when authentication fails.
    
    This typically means:
    - Invalid API key
    - Expired API key
    - Insufficient permissions
    """
    
    def __init__(
        self,
        message: str = "Authentication failed",
        details: Optional[Dict[str, Any]] = None,
    ) -> None:
        super().__init__(message, status_code=401, details=details)


class RateLimitError(VSentinelError):
    """
    Raised when rate limit is exceeded.
    
    Args:
        retry_after: Seconds to wait before retrying
        message: Error message
    """
    
    def __init__(
        self,
        retry_after: int = 60,
        message: str = "Rate limit exceeded",
    ) -> None:
        super().__init__(message, status_code=429)
        self.retry_after = retry_after
    
    def __str__(self) -> str:
        return f"Rate limit exceeded. Retry after {self.retry_after} seconds."


class ResourceNotFoundError(VSentinelError):
    """
    Raised when a requested resource is not found.
    
    Args:
        resource: Type of resource that was not found
        resource_id: ID of the resource (if applicable)
        message: Error message
    """
    
    def __init__(
        self,
        resource: str = "Resource",
        resource_id: Optional[str] = None,
        message: Optional[str] = None,
    ) -> None:
        if message is None:
            if resource_id:
                message = f"{resource} with ID '{resource_id}' not found"
            else:
                message = f"{resource} not found"
        
        super().__init__(message, status_code=404)
        self.resource = resource
        self.resource_id = resource_id


class ValidationError(VSentinelError):
    """
    Raised when request validation fails.
    
    Args:
        message: Error message
        field: Field that failed validation
        details: Validation error details
    """
    
    def __init__(
        self,
        message: str = "Validation error",
        field: Optional[str] = None,
        details: Optional[Dict[str, Any]] = None,
    ) -> None:
        super().__init__(message, status_code=400, details=details)
        self.field = field


class ConnectionError(VSentinelError):
    """
    Raised when connection to V-Sentinel API fails.
    
    This typically means:
    - Network connectivity issues
    - API server is down
    - DNS resolution failure
    """
    
    def __init__(
        self,
        message: str = "Failed to connect to V-Sentinel API",
        details: Optional[Dict[str, Any]] = None,
    ) -> None:
        super().__init__(message, details=details)


class TimeoutError(VSentinelError):
    """
    Raised when a request times out.
    
    Args:
        timeout: Timeout duration in seconds
    """
    
    def __init__(
        self,
        timeout: int = 30,
        message: Optional[str] = None,
    ) -> None:
        if message is None:
            message = f"Request timed out after {timeout} seconds"
        super().__init__(message)
        self.timeout = timeout


class ConfigurationError(VSentinelError):
    """
    Raised when SDK configuration is invalid.
    
    This typically means:
    - Missing required configuration
    - Invalid configuration values
    - Environment variables not set
    """
    
    def __init__(
        self,
        message: str = "Invalid configuration",
        details: Optional[Dict[str, Any]] = None,
    ) -> None:
        super().__init__(message, details=details)


class MCPError(VSentinelError):
    """
    Raised when MCP (Model Context Protocol) operation fails.
    
    Args:
        tool: Name of the MCP tool that failed
        operation: Operation that failed
    """
    
    def __init__(
        self,
        tool: str,
        operation: str,
        message: Optional[str] = None,
        details: Optional[Dict[str, Any]] = None,
    ) -> None:
        if message is None:
            message = f"MCP operation '{operation}' failed for tool '{tool}'"
        super().__init__(message, details=details)
        self.tool = tool
        self.operation = operation