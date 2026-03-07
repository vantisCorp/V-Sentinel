// Package vsentinel provides a Go SDK for the V-Sentinel security operations platform.
//
// This SDK offers a comprehensive client for interacting with V-Sentinel APIs,
// including detections, hosts, incidents, and threat intelligence operations.
//
// Example usage:
//
//	client := vsentinel.NewClient("your-api-key")
//	detections, err := client.Detections.List(ctx, &vsentinel.DetectionListOptions{
//	    Severity: vsentinel.SeverityHigh,
//	    Limit:    10,
//	})
package vsentinel

import (
	"context"
	"fmt"
	"net/http"
	"net/url"
	"time"

	"github.com/go-resty/resty/v2"
)

const (
	// DefaultBaseURL is the default API base URL.
	DefaultBaseURL = "https://api.vantis.ai/v1"

	// DefaultTimeout is the default request timeout.
	DefaultTimeout = 30 * time.Second

	// DefaultMaxRetries is the default number of retries.
	DefaultMaxRetries = 3

	// Version is the SDK version.
	Version = "1.0.0"
)

// Client is the main V-Sentinel API client.
type Client struct {
	// HTTP client for making requests
	http *resty.Client

	// Base URL for API requests
	baseURL *url.URL

	// API key for authentication
	apiKey string

	// Services for different API areas
	Detections   *DetectionsService
	Hosts        *HostsService
	Incidents    *IncidentsService
	ThreatIntel  *ThreatIntelService
}

// ClientOption is a function that configures the Client.
type ClientOption func(*Client) error

// WithBaseURL sets a custom base URL for the API.
func WithBaseURL(baseURL string) ClientOption {
	return func(c *Client) error {
		parsedURL, err := url.Parse(baseURL)
		if err != nil {
			return fmt.Errorf("invalid base URL: %w", err)
		}
		c.baseURL = parsedURL
		return nil
	}
}

// WithTimeout sets the request timeout.
func WithTimeout(timeout time.Duration) ClientOption {
	return func(c *Client) error {
		c.http.SetTimeout(timeout)
		return nil
	}
}

// WithMaxRetries sets the maximum number of retries.
func WithMaxRetries(maxRetries int) ClientOption {
	return func(c *Client) error {
		c.http.SetRetryCount(maxRetries)
		return nil
	}
}

// WithHTTPClient sets a custom HTTP client.
func WithHTTPClient(httpClient *http.Client) ClientOption {
	return func(c *Client) error {
		c.http.SetTransport(httpClient.Transport)
		return nil
	}
}

// WithProxy sets a proxy for the HTTP client.
func WithProxy(proxyURL string) ClientOption {
	return func(c *Client) error {
		c.http.SetProxy(proxyURL)
		return nil
	}
}

// WithDebug enables debug logging.
func WithDebug(debug bool) ClientOption {
	return func(c *Client) error {
		c.http.SetDebug(debug)
		return nil
	}
}

// NewClient creates a new V-Sentinel API client.
func NewClient(apiKey string, opts ...ClientOption) (*Client, error) {
	if apiKey == "" {
		return nil, fmt.Errorf("API key is required")
	}

	// Parse default base URL
	baseURL, err := url.Parse(DefaultBaseURL)
	if err != nil {
		return nil, fmt.Errorf("failed to parse default base URL: %w", err)
	}

	// Create resty client
	httpClient := resty.New().
		SetTimeout(DefaultTimeout).
		SetRetryCount(DefaultMaxRetries).
		SetRetryWaitTime(1 * time.Second).
		SetRetryMaxWaitTime(30 * time.Second).
		SetHeader("Accept", "application/json").
		SetHeader("User-Agent", fmt.Sprintf("v-sentinel-sdk-go/%s", Version))

	// Add retry conditions for common retryable status codes
	httpClient.AddRetryCondition(func(r *resty.Response, err error) bool {
		return r.StatusCode() == http.StatusTooManyRequests ||
			r.StatusCode() == http.StatusInternalServerError ||
			r.StatusCode() == http.StatusBadGateway ||
			r.StatusCode() == http.StatusServiceUnavailable ||
			r.StatusCode() == http.StatusGatewayTimeout
	})

	client := &Client{
		http:    httpClient,
		baseURL: baseURL,
		apiKey:  apiKey,
	}

	// Apply options
	for _, opt := range opts {
		if err := opt(client); err != nil {
			return nil, err
		}
	}

	// Set authentication
	client.http.SetAuthToken(apiKey)

	// Initialize services
	client.Detections = &DetectionsService{client: client}
	client.Hosts = &HostsService{client: client}
	client.Incidents = &IncidentsService{client: client}
	client.ThreatIntel = &ThreatIntelService{client: client}

	return client, nil
}

// newRequest creates a new API request.
func (c *Client) newRequest(ctx context.Context) *resty.Request {
	return c.http.R().SetContext(ctx)
}

// get performs a GET request.
func (c *Client) get(ctx context.Context, path string, params map[string]string, result interface{}) error {
	req := c.newRequest(ctx).SetResult(result)

	if len(params) > 0 {
		req.SetQueryParams(params)
	}

	resp, err := req.Get(c.baseURL.String() + path)
	if err != nil {
		return fmt.Errorf("request failed: %w", err)
	}

	return checkResponse(resp)
}

// post performs a POST request.
func (c *Client) post(ctx context.Context, path string, body interface{}, result interface{}) error {
	req := c.newRequest(ctx).SetBody(body)

	if result != nil {
		req.SetResult(result)
	}

	resp, err := req.Post(c.baseURL.String() + path)
	if err != nil {
		return fmt.Errorf("request failed: %w", err)
	}

	return checkResponse(resp)
}

// patch performs a PATCH request.
func (c *Client) patch(ctx context.Context, path string, body interface{}, result interface{}) error {
	req := c.newRequest(ctx).SetBody(body)

	if result != nil {
		req.SetResult(result)
	}

	resp, err := req.Patch(c.baseURL.String() + path)
	if err != nil {
		return fmt.Errorf("request failed: %w", err)
	}

	return checkResponse(resp)
}

// delete performs a DELETE request.
func (c *Client) delete(ctx context.Context, path string, body interface{}, result interface{}) error {
	req := c.newRequest(ctx)

	if body != nil {
		req.SetBody(body)
	}

	if result != nil {
		req.SetResult(result)
	}

	resp, err := req.Delete(c.baseURL.String() + path)
	if err != nil {
		return fmt.Errorf("request failed: %w", err)
	}

	return checkResponse(resp)
}

// checkResponse checks the API response for errors.
func checkResponse(resp *resty.Response) error {
	if resp.IsSuccess() {
		return nil
	}

	switch resp.StatusCode() {
	case http.StatusUnauthorized:
		return &AuthenticationError{Message: "invalid or expired API key"}
	case http.StatusForbidden:
		return &AuthenticationError{Message: "access denied"}
	case http.StatusNotFound:
		return &ResourceNotFoundError{Message: "resource not found"}
	case http.StatusTooManyRequests:
		retryAfter := resp.Header().Get("Retry-After")
		return &RateLimitError{Message: "rate limit exceeded", RetryAfter: retryAfter}
	case http.StatusBadRequest:
		var errResp ErrorResponse
		if err := resp.UnmarshalJson(&errResp); err == nil {
			return &ValidationError{Message: errResp.Message, Details: errResp.Details}
		}
		return &ValidationError{Message: "bad request"}
	default:
		return &APIError{
			StatusCode: resp.StatusCode(),
			Message:    string(resp.Body()),
		}
	}
}

// ErrorResponse represents an API error response.
type ErrorResponse struct {
	Message string            `json:"message"`
	Details map[string]string `json:"details,omitempty"`
}