"""
V-Sentinel SDK Retry Utilities

Provides retry logic with exponential backoff and jitter.
"""

import asyncio
import random
import time
from dataclasses import dataclass, field
from enum import Enum
from functools import wraps
from typing import (
    Any,
    Callable,
    List,
    Optional,
    Type,
    TypeVar,
    Union,
    ParamSpec,
)

from ..exceptions import (
    RateLimitError,
    ConnectionError as VSConnectionError,
    TimeoutError as VSTimeoutError,
)

P = ParamSpec("P")
T = TypeVar("T")


class RetryState(Enum):
    """State of a retry operation."""
    PENDING = "pending"
    RETRYING = "retrying"
    SUCCESS = "success"
    FAILED = "failed"


@dataclass
class RetryConfig:
    """
    Configuration for retry behavior.
    
    Attributes:
        max_attempts: Maximum number of retry attempts
        base_delay: Base delay in seconds between retries
        max_delay: Maximum delay between retries
        exponential_base: Base for exponential backoff
        jitter: Whether to add random jitter to delays
        jitter_range: Jitter range as a fraction (0.0-1.0)
        retryable_exceptions: List of exception types to retry on
        retryable_status_codes: List of HTTP status codes to retry on
    """
    max_attempts: int = 3
    base_delay: float = 1.0
    max_delay: float = 60.0
    exponential_base: float = 2.0
    jitter: bool = True
    jitter_range: float = 0.5
    retryable_exceptions: List[Type[Exception]] = field(
        default_factory=lambda: [
            VSConnectionError,
            VSTimeoutError,
            ConnectionError,
            TimeoutError,
        ]
    )
    retryable_status_codes: List[int] = field(
        default_factory=lambda: [429, 500, 502, 503, 504]
    )
    
    def __post_init__(self):
        """Validate configuration."""
        if self.max_attempts < 1:
            raise ValueError("max_attempts must be at least 1")
        if self.base_delay <= 0:
            raise ValueError("base_delay must be positive")
        if self.max_delay < self.base_delay:
            raise ValueError("max_delay must be >= base_delay")
        if self.exponential_base <= 1:
            raise ValueError("exponential_base must be > 1")
        if not 0 <= self.jitter_range <= 1:
            raise ValueError("jitter_range must be between 0 and 1")


def calculate_delay(
    attempt: int,
    config: RetryConfig,
) -> float:
    """
    Calculate delay for a given attempt.
    
    Uses exponential backoff with optional jitter.
    
    Args:
        attempt: Current attempt number (0-indexed)
        config: Retry configuration
        
    Returns:
        Delay in seconds
    """
    # Calculate exponential delay
    delay = config.base_delay * (config.exponential_base ** attempt)
    
    # Apply maximum cap
    delay = min(delay, config.max_delay)
    
    # Add jitter if enabled
    if config.jitter:
        jitter_amount = delay * config.jitter_range
        delay += random.uniform(-jitter_amount, jitter_amount)
    
    # Ensure non-negative
    return max(0, delay)


def is_retryable_error(
    error: Exception,
    config: RetryConfig,
) -> bool:
    """
    Check if an error should trigger a retry.
    
    Args:
        error: The exception that occurred
        config: Retry configuration
        
    Returns:
        True if the error is retryable
    """
    # Check if exception type is retryable
    for retryable_type in config.retryable_exceptions:
        if isinstance(error, retryable_type):
            return True
    
    # Check for rate limit errors
    if isinstance(error, RateLimitError):
        return True
    
    # Check for HTTP status codes in certain exceptions
    if hasattr(error, "status_code"):
        status_code = getattr(error, "status_code")
        if status_code in config.retryable_status_codes:
            return True
    
    return False


def with_retry(
    config: Optional[RetryConfig] = None,
    *,
    max_attempts: Optional[int] = None,
    base_delay: Optional[float] = None,
    on_retry: Optional[Callable[[int, Exception, float], None]] = None,
) -> Callable:
    """
    Decorator for automatic retry with exponential backoff.
    
    Can be used with both sync and async functions.
    
    Args:
        config: RetryConfig instance (uses defaults if not provided)
        max_attempts: Override max_attempts from config
        base_delay: Override base_delay from config
        on_retry: Callback function called on each retry (attempt, error, delay)
        
    Returns:
        Decorated function with retry logic
        
    Example:
        @with_retry(max_attempts=3)
        async def fetch_data():
            return await api.get("/data")
    """
    # Create or update config
    if config is None:
        config = RetryConfig()
    elif max_attempts is not None or base_delay is not None:
        # Create a copy with overrides
        config = RetryConfig(
            max_attempts=max_attempts or config.max_attempts,
            base_delay=base_delay or config.base_delay,
            max_delay=config.max_delay,
            exponential_base=config.exponential_base,
            jitter=config.jitter,
            jitter_range=config.jitter_range,
            retryable_exceptions=config.retryable_exceptions,
            retryable_status_codes=config.retryable_status_codes,
        )
    
    def decorator(func: Callable[P, T]) -> Callable[P, T]:
        # Check if function is async
        if asyncio.iscoroutinefunction(func):
            @wraps(func)
            async def async_wrapper(*args: P.args, **kwargs: P.kwargs) -> T:
                last_error: Optional[Exception] = None
                
                for attempt in range(config.max_attempts):
                    try:
                        return await func(*args, **kwargs)
                    except Exception as e:
                        last_error = e
                        
                        # Check if we should retry
                        if not is_retryable_error(e, config):
                            raise
                        
                        # Check if we've exhausted attempts
                        if attempt >= config.max_attempts - 1:
                            raise
                        
                        # Calculate delay
                        delay = calculate_delay(attempt, config)
                        
                        # Handle rate limit retry-after header
                        if isinstance(e, RateLimitError) and e.retry_after:
                            delay = max(delay, e.retry_after)
                        
                        # Call retry callback
                        if on_retry:
                            on_retry(attempt + 1, e, delay)
                        
                        # Wait before retrying
                        await asyncio.sleep(delay)
                
                # Should not reach here, but raise last error just in case
                if last_error:
                    raise last_error
                raise RuntimeError("Unexpected state in retry logic")
            
            return async_wrapper  # type: ignore
        else:
            @wraps(func)
            def sync_wrapper(*args: P.args, **kwargs: P.kwargs) -> T:
                last_error: Optional[Exception] = None
                
                for attempt in range(config.max_attempts):
                    try:
                        return func(*args, **kwargs)
                    except Exception as e:
                        last_error = e
                        
                        # Check if we should retry
                        if not is_retryable_error(e, config):
                            raise
                        
                        # Check if we've exhausted attempts
                        if attempt >= config.max_attempts - 1:
                            raise
                        
                        # Calculate delay
                        delay = calculate_delay(attempt, config)
                        
                        # Handle rate limit retry-after header
                        if isinstance(e, RateLimitError) and e.retry_after:
                            delay = max(delay, e.retry_after)
                        
                        # Call retry callback
                        if on_retry:
                            on_retry(attempt + 1, e, delay)
                        
                        # Wait before retrying
                        time.sleep(delay)
                
                # Should not reach here, but raise last error just in case
                if last_error:
                    raise last_error
                raise RuntimeError("Unexpected state in retry logic")
            
            return sync_wrapper  # type: ignore
    
    return decorator


class Retrier:
    """
    Context manager for retry operations.
    
    Provides more control over retry behavior with
    manual iteration support.
    
    Example:
        async with Retrier(config) as retry_ctx:
            for attempt in retry_ctx:
                try:
                    result = await some_operation()
                    retry_ctx.success()
                    break
                except Exception as e:
                    retry_ctx.set_error(e)
    """
    
    def __init__(self, config: Optional[RetryConfig] = None):
        """
        Initialize retrier.
        
        Args:
            config: Retry configuration
        """
        self.config = config or RetryConfig()
        self.state = RetryState.PENDING
        self.current_attempt = 0
        self.last_error: Optional[Exception] = None
        self._delay_task: Optional[asyncio.Task] = None
    
    def __iter__(self):
        """Return self as iterator."""
        return self
    
    def __next__(self) -> int:
        """Get next attempt number."""
        if self.state == RetryState.SUCCESS:
            raise StopIteration
        
        if self.current_attempt >= self.config.max_attempts:
            self.state = RetryState.FAILED
            raise StopIteration
        
        self.current_attempt += 1
        self.state = RetryState.RETRYING
        return self.current_attempt
    
    async def __aenter__(self) -> "Retrier":
        """Enter async context."""
        return self
    
    async def __aexit__(self, exc_type, exc_val, exc_tb) -> bool:
        """Exit async context."""
        if exc_type is not None:
            self.last_error = exc_val
            
            if is_retryable_error(exc_val, self.config):
                if self.current_attempt < self.config.max_attempts:
                    delay = calculate_delay(self.current_attempt - 1, self.config)
                    await asyncio.sleep(delay)
                    return True  # Suppress exception and retry
            
            return False  # Let exception propagate
        return False
    
    def success(self) -> None:
        """Mark operation as successful."""
        self.state = RetryState.SUCCESS
    
    def set_error(self, error: Exception) -> None:
        """Set the last error."""
        self.last_error = error
    
    def should_retry(self) -> bool:
        """Check if should retry."""
        return (
            self.current_attempt < self.config.max_attempts and
            self.last_error is not None and
            is_retryable_error(self.last_error, self.config)
        )
    
    async def wait(self) -> None:
        """Wait before next attempt."""
        if self.last_error:
            delay = calculate_delay(self.current_attempt - 1, self.config)
            await asyncio.sleep(delay)


# Convenience function for quick retries
def retry(
    func: Callable[P, T],
    *args: P.args,
    max_attempts: int = 3,
    base_delay: float = 1.0,
    **kwargs: P.kwargs,
) -> T:
    """
    Execute a function with retry logic.
    
    Convenience function for one-off retry operations.
    
    Args:
        func: Function to execute
        *args: Function arguments
        max_attempts: Maximum retry attempts
        base_delay: Base delay between retries
        **kwargs: Function keyword arguments
        
    Returns:
        Function result
        
    Example:
        result = retry(api_call, param1="value", max_attempts=5)
    """
    config = RetryConfig(max_attempts=max_attempts, base_delay=base_delay)
    
    decorated = with_retry(config)(func)
    return decorated(*args, **kwargs)