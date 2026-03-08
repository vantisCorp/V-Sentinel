"""
V-Sentinel SDK Logging Utilities

Provides structured logging for the SDK.
"""

import logging
import sys
from dataclasses import dataclass, field
from datetime import datetime
from enum import Enum
from typing import Any, Dict, Optional, Union
import json


class LogLevel(Enum):
    """Log levels for SDK."""
    DEBUG = "DEBUG"
    INFO = "INFO"
    WARNING = "WARNING"
    ERROR = "ERROR"
    CRITICAL = "CRITICAL"


# Default log format
DEFAULT_FORMAT = "%(asctime)s - %(name)s - %(levelname)s - %(message)s"
DEFAULT_DATE_FORMAT = "%Y-%m-%d %H:%M:%S"


@dataclass
class LogConfig:
    """
    Configuration for SDK logging.
    
    Attributes:
        level: Minimum log level
        format: Log message format string
        date_format: Date format string
        include_timestamp: Include ISO timestamp in structured logs
        include_extra: Include extra fields in structured logs
        output: Output destination ('stdout', 'stderr', or file path)
        json_format: Use JSON structured logging
    """
    level: LogLevel = LogLevel.INFO
    format: str = DEFAULT_FORMAT
    date_format: str = DEFAULT_DATE_FORMAT
    include_timestamp: bool = True
    include_extra: bool = True
    output: str = "stderr"
    json_format: bool = False


class SDKLogger:
    """
    Structured logger for V-Sentinel SDK.
    
    Provides both traditional and JSON structured logging
    with support for extra context fields.
    
    Example:
        logger = SDKLogger("v_sentinel_sdk.client")
        logger.info("Request started", method="GET", endpoint="/hosts")
        logger.error("Request failed", error_code=500, duration_ms=150)
    """
    
    def __init__(
        self,
        name: str,
        config: Optional[LogConfig] = None,
    ):
        """
        Initialize SDK logger.
        
        Args:
            name: Logger name
            config: Logging configuration
        """
        self.name = name
        self.config = config or LogConfig()
        
        # Get or create Python logger
        self._logger = logging.getLogger(name)
        self._logger.setLevel(self.config.level.value)
        
        # Remove existing handlers
        self._logger.handlers.clear()
        
        # Add handler
        self._setup_handler()
    
    def _setup_handler(self) -> None:
        """Set up log handler based on configuration."""
        if self.config.output == "stdout":
            stream = sys.stdout
        elif self.config.output == "stderr":
            stream = sys.stderr
        else:
            # File output
            stream = open(self.config.output, "a")
        
        handler = logging.StreamHandler(stream)
        handler.setLevel(self.config.level.value)
        
        if self.config.json_format:
            formatter = JSONFormatter(self.config)
        else:
            formatter = logging.Formatter(
                self.config.format,
                datefmt=self.config.date_format,
            )
        
        handler.setFormatter(formatter)
        self._logger.addHandler(handler)
    
    def _log(
        self,
        level: int,
        message: str,
        **kwargs: Any,
    ) -> None:
        """
        Internal logging method.
        
        Args:
            level: Log level
            message: Log message
            **kwargs: Extra context fields
        """
        if kwargs:
            self._logger.log(level, message, extra=kwargs)
        else:
            self._logger.log(level, message)
    
    def debug(self, message: str, **kwargs: Any) -> None:
        """Log debug message."""
        self._log(logging.DEBUG, message, **kwargs)
    
    def info(self, message: str, **kwargs: Any) -> None:
        """Log info message."""
        self._log(logging.INFO, message, **kwargs)
    
    def warning(self, message: str, **kwargs: Any) -> None:
        """Log warning message."""
        self._log(logging.WARNING, message, **kwargs)
    
    def error(self, message: str, **kwargs: Any) -> None:
        """Log error message."""
        self._log(logging.ERROR, message, **kwargs)
    
    def critical(self, message: str, **kwargs: Any) -> None:
        """Log critical message."""
        self._log(logging.CRITICAL, message, **kwargs)
    
    def exception(self, message: str, **kwargs: Any) -> None:
        """Log exception with traceback."""
        self._logger.exception(message, extra=kwargs)
    
    def bind(self, **context: Any) -> "BoundLogger":
        """
        Create a bound logger with persistent context.
        
        Args:
            **context: Context fields to bind
            
        Returns:
            BoundLogger instance
            
        Example:
            request_logger = logger.bind(request_id="abc-123")
            request_logger.info("Processing request")
        """
        return BoundLogger(self, context)


class BoundLogger:
    """
    Logger with bound context fields.
    
    Created by SDKLogger.bind() - do not instantiate directly.
    """
    
    def __init__(
        self,
        parent: SDKLogger,
        context: Dict[str, Any],
    ):
        """
        Initialize bound logger.
        
        Args:
            parent: Parent SDKLogger instance
            context: Bound context fields
        """
        self._parent = parent
        self._context = context
    
    def _log(
        self,
        level: int,
        message: str,
        **kwargs: Any,
    ) -> None:
        """Log with merged context."""
        merged = {**self._context, **kwargs}
        self._parent._log(level, message, **merged)
    
    def debug(self, message: str, **kwargs: Any) -> None:
        """Log debug message with bound context."""
        self._log(logging.DEBUG, message, **kwargs)
    
    def info(self, message: str, **kwargs: Any) -> None:
        """Log info message with bound context."""
        self._log(logging.INFO, message, **kwargs)
    
    def warning(self, message: str, **kwargs: Any) -> None:
        """Log warning message with bound context."""
        self._log(logging.WARNING, message, **kwargs)
    
    def error(self, message: str, **kwargs: Any) -> None:
        """Log error message with bound context."""
        self._log(logging.ERROR, message, **kwargs)
    
    def critical(self, message: str, **kwargs: Any) -> None:
        """Log critical message with bound context."""
        self._log(logging.CRITICAL, message, **kwargs)
    
    def exception(self, message: str, **kwargs: Any) -> None:
        """Log exception with bound context."""
        merged = {**self._context, **kwargs}
        self._parent._logger.exception(message, extra=merged)
    
    def bind(self, **context: Any) -> "BoundLogger":
        """Add more context to bound logger."""
        merged = {**self._context, **context}
        return BoundLogger(self._parent, merged)


class JSONFormatter(logging.Formatter):
    """
    JSON formatter for structured logging.
    
    Outputs log records as JSON objects for easy parsing
    by log aggregation systems.
    """
    
    def __init__(self, config: LogConfig):
        """
        Initialize JSON formatter.
        
        Args:
            config: Logging configuration
        """
        self.config = config
        super().__init__()
    
    def format(self, record: logging.LogRecord) -> str:
        """
        Format log record as JSON.
        
        Args:
            record: Log record to format
            
        Returns:
            JSON formatted log string
        """
        log_obj: Dict[str, Any] = {
            "logger": record.name,
            "level": record.levelname,
            "message": record.getMessage(),
        }
        
        if self.config.include_timestamp:
            log_obj["timestamp"] = datetime.utcnow().isoformat() + "Z"
        
        # Add location info
        log_obj["location"] = {
            "file": record.filename,
            "line": record.lineno,
            "function": record.funcName,
        }
        
        # Add extra fields from record
        if self.config.include_extra:
            extra_fields = {
                k: v
                for k, v in record.__dict__.items()
                if k not in {
                    "name", "msg", "args", "created", "filename",
                    "funcName", "levelname", "levelno", "lineno",
                    "module", "msecs", "message", "pathname",
                    "process", "processName", "relativeCreated",
                    "thread", "threadName", "exc_info", "exc_text",
                    "stack_info", "asctime",
                }
            }
            if extra_fields:
                log_obj["context"] = extra_fields
        
        # Add exception info if present
        if record.exc_info:
            log_obj["exception"] = self.formatException(record.exc_info)
        
        return json.dumps(log_obj)


# Module-level logger registry
_loggers: Dict[str, SDKLogger] = {}
_global_config: Optional[LogConfig] = None


def get_logger(name: str) -> SDKLogger:
    """
    Get or create a logger by name.
    
    Uses global configuration if set via set_log_level().
    
    Args:
        name: Logger name
        
    Returns:
        SDKLogger instance
        
    Example:
        logger = get_logger("v_sentinel_sdk.client")
        logger.info("Connection established")
    """
    if name in _loggers:
        return _loggers[name]
    
    config = _global_config or LogConfig()
    logger = SDKLogger(name, config)
    _loggers[name] = logger
    return logger


def set_log_level(
    level: Union[LogLevel, str, int],
    *,
    json_format: bool = False,
    output: str = "stderr",
) -> None:
    """
    Set global logging level for SDK.
    
    Args:
        level: Log level (LogLevel enum, string, or int)
        json_format: Use JSON structured logging
        output: Output destination
        
    Example:
        set_log_level("DEBUG")
        set_log_level(LogLevel.WARNING, json_format=True)
    """
    global _global_config
    
    # Convert level to LogLevel enum
    if isinstance(level, str):
        level = LogLevel[level.upper()]
    elif isinstance(level, int):
        level_mapping = {
            logging.DEBUG: LogLevel.DEBUG,
            logging.INFO: LogLevel.INFO,
            logging.WARNING: LogLevel.WARNING,
            logging.ERROR: LogLevel.ERROR,
            logging.CRITICAL: LogLevel.CRITICAL,
        }
        level = level_mapping.get(level, LogLevel.INFO)
    
    _global_config = LogConfig(
        level=level,
        json_format=json_format,
        output=output,
    )
    
    # Update existing loggers
    for logger in _loggers.values():
        logger._logger.setLevel(level.value)


def configure_logging(
    *,
    level: Union[LogLevel, str] = LogLevel.INFO,
    json_format: bool = False,
    output: str = "stderr",
    format: str = DEFAULT_FORMAT,
    date_format: str = DEFAULT_DATE_FORMAT,
) -> None:
    """
    Configure SDK logging with full options.
    
    Args:
        level: Minimum log level
        json_format: Use JSON structured logging
        output: Output destination ('stdout', 'stderr', or file path)
        format: Log message format string
        date_format: Date format string
        
    Example:
        configure_logging(
            level="DEBUG",
            json_format=True,
            output="/var/log/v_sentinel.log",
        )
    """
    global _global_config
    
    if isinstance(level, str):
        level = LogLevel[level.upper()]
    
    _global_config = LogConfig(
        level=level,
        format=format,
        date_format=date_format,
        json_format=json_format,
        output=output,
    )
    
    # Update existing loggers
    for name, logger in _loggers.items():
        _loggers[name] = SDKLogger(name, _global_config)