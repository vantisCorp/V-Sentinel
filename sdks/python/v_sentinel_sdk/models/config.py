"""
Client Configuration Model
~~~~~~~~~~~~~~~~~~~~~~~~~~

Configuration model for V-Sentinel SDK client.
"""

from typing import Any, Dict, Optional

from pydantic import BaseModel, Field


class ClientConfig(BaseModel):
    """Configuration for V-Sentinel client."""
    
    api_key: str = Field(..., description="V-Sentinel API key")
    base_url: str = Field(
        default="https://api.vantis.ai/v1",
        description="Base URL for V-Sentinel API"
    )
    timeout: int = Field(default=30, ge=1, le=300, description="Request timeout in seconds")
    max_retries: int = Field(default=3, ge=0, le=10, description="Maximum retry attempts")
    verify_ssl: bool = Field(default=True, description="Verify SSL certificates")
    
    # Proxy settings
    proxy: Optional[str] = Field(None, description="Proxy URL")
    
    # Rate limiting
    rate_limit_enabled: bool = Field(default=True, description="Enable rate limiting")
    rate_limit_requests: int = Field(default=100, description="Max requests per minute")
    
    # Caching
    cache_enabled: bool = Field(default=True, description="Enable response caching")
    cache_ttl: int = Field(default=300, description="Cache TTL in seconds")
    
    # Logging
    log_level: str = Field(default="INFO", description="Log level")
    log_requests: bool = Field(default=False, description="Log HTTP requests")
    
    # Custom headers
    extra_headers: Dict[str, str] = Field(default_factory=dict, description="Extra HTTP headers")
    
    # Additional options
    options: Dict[str, Any] = Field(default_factory=dict, description="Additional options")
    
    model_config = {
        "extra": "allow",
        "json_schema_extra": {
            "examples": [
                {
                    "api_key": "your-api-key-here",
                    "base_url": "https://api.vantis.ai/v1",
                    "timeout": 30,
                    "max_retries": 3
                }
            ]
        }
    }