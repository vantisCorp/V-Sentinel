# V-Sentinel Go SDK

[![Go Reference](https://pkg.go.dev/badge/github.com/vantis-ai/v-sentinel-sdk-go.svg)](https://pkg.go.dev/github.com/vantis-ai/v-sentinel-sdk-go)
[![Go Report Card](https://goreportcard.com/badge/github.com/vantis-ai/v-sentinel-sdk-go)](https://goreportcard.com/report/github.com/vantis-ai/v-sentinel-sdk-go)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

Official Go SDK for the V-Sentinel security operations platform. Provides a comprehensive client for interacting with V-Sentinel APIs, including detections, hosts, incidents, and threat intelligence operations.

## Features

- **Simple and intuitive API** - Clean Go idioms with proper error handling
- **Context support** - All operations support context for cancellation and timeouts
- **Automatic retries** - Built-in retry logic with exponential backoff
- **Type-safe** - Strong typing with Go structs for all API responses
- **Comprehensive coverage** - Full API coverage for all V-Sentinel operations
- **Zero dependencies** - Minimal external dependencies (just go-resty)

## Installation

```bash
go get github.com/vantis-ai/v-sentinel-sdk-go
```

## Quick Start

```go
package main

import (
    "context"
    "fmt"
    "log"

    vsentinel "github.com/vantis-ai/v-sentinel-sdk-go"
)

func main() {
    // Create client
    client, err := vsentinel.NewClient("your-api-key")
    if err != nil {
        log.Fatal(err)
    }

    ctx := context.Background()

    // List recent critical detections
    detections, err := client.Detections.List(ctx, &vsentinel.DetectionListOptions{
        Severity: vsentinel.SeverityCritical,
        TimeRange: "24h",
        Limit:    10,
    })
    if err != nil {
        log.Fatal(err)
    }

    for _, det := range detections.Items {
        fmt.Printf("[%s] %s\n", det.Severity, det.Title)
        fmt.Printf("  Host: %s\n", det.Host.Hostname)
        fmt.Printf("  MITRE Techniques: %v\n", det.Techniques)
    }

    // Check an IOC against threat intelligence
    result, err := client.ThreatIntel.CheckIOC(ctx, "192.0.2.1", vsentinel.IOCTypeIP)
    if err != nil {
        log.Fatal(err)
    }

    if result.IsMalicious {
        fmt.Printf("Malicious IOC detected!\n")
        fmt.Printf("  Threat Actor: %s\n", result.ThreatActor)
        fmt.Printf("  Confidence: %s\n", result.Confidence)
    }
}
```

## Authentication

### API Key

```go
client, err := vsentinel.NewClient("your-api-key")
```

### Environment Variable

```go
import "os"

client, err := vsentinel.NewClient(os.Getenv("V_SENTINEL_API_KEY"))
```

### Custom Configuration

```go
client, err := vsentinel.NewClient(
    "your-api-key",
    vsentinel.WithBaseURL("https://api.custom.com/v1"),
    vsentinel.WithTimeout(60 * time.Second),
    vsentinel.WithMaxRetries(5),
    vsentinel.WithProxy("http://proxy.company.com:8080"),
    vsentinel.WithDebug(true),
)
```

## API Reference

### Detections

Manage security detections and alerts.

```go
// List detections with filters
detections, err := client.Detections.List(ctx, &vsentinel.DetectionListOptions{
    Severity:  vsentinel.SeverityHigh,
    Status:    vsentinel.DetectionStatusNew,
    TimeRange: "7d",
    HostID:    "host-123",
    Limit:     50,
})

// Get a specific detection
detection, err := client.Detections.Get(ctx, "detection-456")

// Update detection status
updated, err := client.Detections.UpdateStatus(ctx, "detection-456", &vsentinel.UpdateStatusOptions{
    Status: vsentinel.DetectionStatusInProgress,
    Notes:  "Investigating potential false positive",
})

// Assign detection to analyst
assigned, err := client.Detections.Assign(ctx, "detection-456", "analyst@company.com")

// Add note to detection
note, err := client.Detections.AddNote(ctx, "detection-456", "Confirmed as true positive")
```

### Hosts

Manage endpoint inventory and isolation.

```go
// List hosts
hosts, err := client.Hosts.List(ctx, &vsentinel.HostListOptions{
    Platform: vsentinel.PlatformWindows,
    State:    vsentinel.HostStateOnline,
    Limit:    100,
})

// Get host details
host, err := client.Hosts.Get(ctx, "host-123")

// Search hosts by hostname
results, err := client.Hosts.Search(ctx, "workstation-", 50)

// Isolate a host
isolated, err := client.Hosts.Isolate(ctx, "host-123", "Malware detected - immediate containment required")

// Get host detections
hostDetections, err := client.Hosts.GetDetections(ctx, "host-123", 20)

// Add tag to host
tagged, err := client.Hosts.AddTag(ctx, "host-123", "investigated")
```

### Incidents

Manage security incident lifecycle.

```go
// Create an incident
incident, err := client.Incidents.Create(ctx, &vsentinel.IncidentCreateOptions{
    Title:       "Ransomware Detection on Finance Server",
    Description: "LockBit ransomware detected on FIN-SRV-01",
    Severity:    vsentinel.IncidentSeverityCritical,
    Hosts:       []string{"host-123", "host-456"},
    Detections:  []string{"detection-789"},
    Tags:        []string{"ransomware", "lockbit", "finance"},
})

// List incidents
incidents, err := client.Incidents.List(ctx, &vsentinel.IncidentListOptions{
    Severity:  vsentinel.IncidentSeverityCritical,
    Status:    vsentinel.IncidentStatusInProgress,
    TimeRange: "30d",
})

// Update incident phase
updated, err := client.Incidents.UpdatePhase(ctx, incident.ID, &vsentinel.IncidentUpdatePhaseOptions{
    Phase: vsentinel.PhaseContainment,
    Notes: "Hosts isolated, ransomware binaries removed",
})

// Add hosts to incident
updated, err := client.Incidents.AddHosts(ctx, incident.ID, []string{"host-789"})

// Close incident
closed, err := client.Incidents.Close(ctx, incident.ID, &vsentinel.IncidentCloseOptions{
    Resolution:     "Ransomware contained and eradicated. Systems restored from backup.",
    LessonsLearned: "Consider adding additional email filtering for phishing attachments.",
})
```

### Threat Intelligence

Manage IOCs and threat actor data.

```go
// Check an IOC
result, err := client.ThreatIntel.CheckIOC(ctx, "malicious.example.com", vsentinel.IOCTypeDomain)

if result.IsMalicious {
    fmt.Printf("Threat Actor: %s\n", result.ThreatActor)
    fmt.Printf("Related IOCs: %d\n", len(result.RelatedIOCs))
}

// Bulk check IOCs
results, err := client.ThreatIntel.CheckIOCsBulk(ctx, []string{
    "192.0.2.1",
    "malware.hash.example",
    "https://malicious.url/path",
}, vsentinel.IOCTypeIP)

// Add an IOC
ioc, err := client.ThreatIntel.AddIOC(ctx, &vsentinel.AddIOCOptions{
    Value:         "192.0.2.100",
    Type:          vsentinel.IOCTypeIP,
    Confidence:    vsentinel.ConfidenceHigh,
    ThreatActor:   "APT29",
    Campaign:      "CozyBear-2024",
    MalwareFamily: "SUNBURST",
    Tags:          []string{"apt29", "supply-chain"},
    Description:   "C2 server identified in SolarWinds campaign",
})

// Get threat actor information
actor, err := client.ThreatIntel.GetThreatActor(ctx, "APT29")
fmt.Printf("Aliases: %v\n", actor.Aliases)
fmt.Printf("MITRE ID: %s\n", actor.MITREID)

// Get actor's IOCs
actorIOCs, err := client.ThreatIntel.GetThreatActorIOCs(ctx, "APT29", 100)

// Export IOCs for SIEM
siemRules, err := client.ThreatIntel.ExportForSIEM(ctx, "splunk", 
    []vsentinel.IOCType{vsentinel.IOCTypeIP, vsentinel.IOCTypeDomain, vsentinel.IOCTypeURL}, 
    "")
```

## Error Handling

The SDK provides specific error types for different scenarios:

```go
import (
    "errors"
    vsentinel "github.com/vantis-ai/v-sentinel-sdk-go"
)

detection, err := client.Detections.Get(ctx, "invalid-id")
if err != nil {
    var notFound *vsentinel.ResourceNotFoundError
    var authError *vsentinel.AuthenticationError
    var rateLimit *vsentinel.RateLimitError
    
    switch {
    case errors.As(err, &notFound):
        fmt.Printf("Resource not found: %s\n", notFound.ResourceID)
    case errors.As(err, &authError):
        fmt.Println("Invalid API key")
    case errors.As(err, &rateLimit):
        fmt.Printf("Rate limited. Retry after: %s\n", rateLimit.RetryAfter)
    default:
        fmt.Printf("Error: %v\n", err)
    }
}

// Helper functions
if vsentinel.IsNotFoundError(err) {
    // Handle not found
}
```

## Context and Timeouts

All operations support context for cancellation and timeouts:

```go
// With timeout
ctx, cancel := context.WithTimeout(context.Background(), 30*time.Second)
defer cancel()

detections, err := client.Detections.List(ctx, nil)

// With cancellation
ctx, cancel := context.WithCancel(context.Background())
go func() {
    time.Sleep(5 * time.Second)
    cancel() // Cancel after 5 seconds
}()

detections, err := client.Detections.List(ctx, nil)
if errors.Is(err, context.Canceled) {
    // Handle cancellation
}
```

## Retries

The SDK automatically retries on transient errors:

```go
client, err := vsentinel.NewClient(
    "your-api-key",
    vsentinel.WithMaxRetries(5),
)
```

Default retry behavior:
- Retries on: 429, 500, 502, 503, 504 status codes
- Exponential backoff: 1s → 2s → 4s → ... (max 30s)
- Respects `Retry-After` header for 429 responses

## Proxy Configuration

```go
// HTTP proxy
client, err := vsentinel.NewClient(
    "your-api-key",
    vsentinel.WithProxy("http://proxy.company.com:8080"),
)

// SOCKS5 proxy
client, err := vsentinel.NewClient(
    "your-api-key",
    vsentinel.WithProxy("socks5://proxy.company.com:1080"),
)
```

## Development

### Requirements

- Go 1.21 or later

### Running Tests

```bash
go test ./...
```

### Linting

```bash
go vet ./...
golangci-lint run
```

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Support

- **Documentation**: [https://docs.vantis.ai/sdks/go](https://docs.vantis.ai/sdks/go)
- **API Reference**: [https://pkg.go.dev/github.com/vantis-ai/v-sentinel-sdk-go](https://pkg.go.dev/github.com/vantis-ai/v-sentinel-sdk-go)
- **GitHub Issues**: [https://github.com/vantis-ai/v-sentinel-sdks/issues](https://github.com/vantis-ai/v-sentinel-sdks/issues)
- **Security Issues**: security@vantis.ai

## Contributing

We welcome contributions! Please see our [Contributing Guide](CONTRIBUTING.md) for details.

---

Built with ❤️ by the Vantis AI team