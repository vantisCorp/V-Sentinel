package vsentinel

import "fmt"

// APIError represents a generic API error.
type APIError struct {
	StatusCode int
	Message    string
}

func (e *APIError) Error() string {
	return fmt.Sprintf("API error (status %d): %s", e.StatusCode, e.Message)
}

// AuthenticationError indicates an authentication failure.
type AuthenticationError struct {
	Message string
}

func (e *AuthenticationError) Error() string {
	return fmt.Sprintf("authentication error: %s", e.Message)
}

// ResourceNotFoundError indicates a resource was not found.
type ResourceNotFoundError struct {
	Message     string
	ResourceID  string
	ResourceType string
}

func (e *ResourceNotFoundError) Error() string {
	if e.ResourceType != "" && e.ResourceID != "" {
		return fmt.Sprintf("%s not found: %s", e.ResourceType, e.ResourceID)
	}
	return e.Message
}

// ValidationError indicates a validation failure.
type ValidationError struct {
	Message string
	Details map[string]string
}

func (e *ValidationError) Error() string {
	if len(e.Details) > 0 {
		return fmt.Sprintf("validation error: %s (details: %v)", e.Message, e.Details)
	}
	return fmt.Sprintf("validation error: %s", e.Message)
}

// RateLimitError indicates rate limiting.
type RateLimitError struct {
	Message    string
	RetryAfter string
}

func (e *RateLimitError) Error() string {
	if e.RetryAfter != "" {
		return fmt.Sprintf("rate limit exceeded (retry after %s)", e.RetryAfter)
	}
	return "rate limit exceeded"
}

// ConnectionError indicates a connection failure.
type ConnectionError struct {
	Message string
	Cause   error
}

func (e *ConnectionError) Error() string {
	if e.Cause != nil {
		return fmt.Sprintf("connection error: %s (cause: %v)", e.Message, e.Cause)
	}
	return fmt.Sprintf("connection error: %s", e.Message)
}

func (e *ConnectionError) Unwrap() error {
	return e.Cause
}

// TimeoutError indicates a request timeout.
type TimeoutError struct {
	Message string
}

func (e *TimeoutError) Error() string {
	return fmt.Sprintf("timeout: %s", e.Message)
}

// IsNotFoundError checks if an error is a ResourceNotFoundError.
func IsNotFoundError(err error) bool {
	_, ok := err.(*ResourceNotFoundError)
	return ok
}

// IsAuthError checks if an error is an AuthenticationError.
func IsAuthError(err error) bool {
	_, ok := err.(*AuthenticationError)
	return ok
}

// IsRateLimitError checks if an error is a RateLimitError.
func IsRateLimitError(err error) bool {
	_, ok := err.(*RateLimitError)
	return ok
}

// IsValidationError checks if an error is a ValidationError.
func IsValidationError(err error) bool {
	_, ok := err.(*ValidationError)
	return ok
}