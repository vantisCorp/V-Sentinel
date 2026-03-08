/**
 * V-Sentinel SDK Error Types
 *
 * Provides specific error types for different API error scenarios.
 */

/** Base error class for all V-Sentinel SDK errors */
export class VSentinelError extends Error {
  constructor(message: string) {
    super(message);
    this.name = 'VSentinelError';
    Object.setPrototypeOf(this, VSentinelError.prototype);
  }
}

/** API error with status code and response details */
export class APIError extends VSentinelError {
  readonly statusCode: number;
  readonly response?: unknown;

  constructor(statusCode: number, message: string, response?: unknown) {
    super(message);
    this.name = 'APIError';
    this.statusCode = statusCode;
    this.response = response;
    Object.setPrototypeOf(this, APIError.prototype);
  }
}

/** Authentication failure (401 or 403) */
export class AuthenticationError extends VSentinelError {
  constructor(message: string = 'Authentication failed') {
    super(message);
    this.name = 'AuthenticationError';
    Object.setPrototypeOf(this, AuthenticationError.prototype);
  }
}

/** Resource not found (404) */
export class ResourceNotFoundError extends VSentinelError {
  readonly resourceType?: string;
  readonly resourceId?: string;

  constructor(
    message: string,
    resourceType?: string,
    resourceId?: string
  ) {
    super(message);
    this.name = 'ResourceNotFoundError';
    this.resourceType = resourceType;
    this.resourceId = resourceId;
    Object.setPrototypeOf(this, ResourceNotFoundError.prototype);
  }
}

/** Validation error (400) */
export class ValidationError extends VSentinelError {
  readonly details?: Record<string, string>;

  constructor(message: string, details?: Record<string, string>) {
    super(message);
    this.name = 'ValidationError';
    this.details = details;
    Object.setPrototypeOf(this, ValidationError.prototype);
  }
}

/** Rate limit exceeded (429) */
export class RateLimitError extends VSentinelError {
  readonly retryAfter?: number;

  constructor(message: string = 'Rate limit exceeded', retryAfter?: number) {
    super(message);
    this.name = 'RateLimitError';
    this.retryAfter = retryAfter;
    Object.setPrototypeOf(this, RateLimitError.prototype);
  }
}

/** Connection error */
export class ConnectionError extends VSentinelError {
  readonly cause?: Error;

  constructor(message: string, cause?: Error) {
    super(message);
    this.name = 'ConnectionError';
    this.cause = cause;
    Object.setPrototypeOf(this, ConnectionError.prototype);
  }
}

/** Request timeout */
export class TimeoutError extends VSentinelError {
  constructor(message: string = 'Request timed out') {
    super(message);
    this.name = 'TimeoutError';
    Object.setPrototypeOf(this, TimeoutError.prototype);
  }
}

/** Error response from the API */
export interface ErrorResponse {
  message: string;
  details?: Record<string, string>;
  code?: string;
}

/** Type guard to check if error is an AuthenticationError */
export function isAuthenticationError(error: unknown): error is AuthenticationError {
  return error instanceof AuthenticationError;
}

/** Type guard to check if error is a ResourceNotFoundError */
export function isResourceNotFoundError(error: unknown): error is ResourceNotFoundError {
  return error instanceof ResourceNotFoundError;
}

/** Type guard to check if error is a RateLimitError */
export function isRateLimitError(error: unknown): error is RateLimitError {
  return error instanceof RateLimitError;
}

/** Type guard to check if error is a ValidationError */
export function isValidationError(error: unknown): error is ValidationError {
  return error instanceof ValidationError;
}