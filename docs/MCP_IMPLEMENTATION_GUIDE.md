# V-Sentinel MCP (Model Context Protocol) Implementation Guide

## Overview

This guide documents the implementation of the Model Context Protocol (MCP) server for V-Sentinel, enabling AI-driven security operations.

## Architecture

### Module Structure

```
src/mcp/
├── Cargo.toml              # Module dependencies
├── src/
│   ├── lib.rs             # Main library entry point
│   ├── error.rs           # Error types
│   ├── types.rs           # Core protocol types
│   ├── server.rs          # MCP server implementation
│   ├── tools/
│   │   ├── mod.rs         # Tool registry and manager
│   │   ├── detections.rs  # Detection search tool
│   │   ├── hosts.rs       # Host management tool
│   │   ├── incidents.rs   # Incident management tool
│   │   └── threat_intel.rs # Threat intelligence tool
│   ├── transport/
│   │   ├── mod.rs         # Transport trait
│   │   └── stdio.rs       # Stdio transport implementation
│   └── resources/
│       ├── api_docs.md    # API documentation
│       └── fql_guide.md   # FQL query guide
```

## Features

### 1. AI-Callable Security Tools

#### Detections Tool
- **Name**: `sentinel_search_detections`
- **Purpose**: Search and analyze security detections
- **Parameters**:
  - `query` (string): FQL-style query
  - `time_range` (string): Time range (e.g., "1h", "24h", "7d")
  - `limit` (integer): Maximum results (default: 100)
  - `sort_by` (string): Sort field (default: "timestamp")

#### Hosts Tool
- **Name**: `sentinel_list_hosts`
- **Purpose**: List and search hosts
- **Parameters**:
  - `host_id` (string): Specific host ID or hostname
  - `state` (string): Filter by state (online, offline, unknown)
  - `platform` (string): Filter by platform (Windows, Linux, macOS)

#### Incidents Tool
- **Name**: `sentinel_list_incidents`
- **Purpose**: List and manage security incidents
- **Parameters**:
  - `incident_id` (string): Specific incident ID
  - `severity` (string): Filter by severity (critical, high, medium, low)
  - `status` (string): Filter by status (new, in_progress, resolved, closed)
  - `limit` (integer): Maximum results (default: 50)

#### Threat Intelligence Tool
- **Name**: `sentinel_threat_intel`
- **Purpose**: Query threat intelligence database
- **Parameters**:
  - `ioc` (string): Indicator of Compromise
  - `ioc_type` (string): Type (ip, domain, hash, url)
  - `threat_actor` (string): Threat actor name
  - `malware_family` (string): Malware family name

### 2. Static Resources

#### API Documentation
- **URI**: `docs://api-reference`
- **Content**: Complete V-Sentinel API documentation
- **Purpose**: AI agents can reference API endpoints and usage

#### FQL Guide
- **URI**: `docs://fql-guide`
- **Content**: Comprehensive FQL query language guide
- **Purpose**: AI agents can learn FQL syntax and best practices

### 3. Transport Protocols

#### Stdio Transport
- Suitable for local development and testing
- JSON-RPC over stdin/stdout
- Easy to debug and test

#### Planned Transports
- SSE (Server-Sent Events): For real-time streaming
- HTTP: For web-based integration

## Usage

### Starting the MCP Server

```rust
use v_sentinel_mcp::MCPServer;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let server = MCPServer::new("V-Sentinel MCP Server").await?;
    
    // Start stdio transport
    let transport = v_sentinel_mcp::transport::stdio::StdioTransport::new(
        std::sync::Arc::new(server)
    );
    
    transport.run().await?;
    
    Ok(())
}
```

### Example Tool Call

```json
{
  "jsonrpc": "2.0",
  "id": "1",
  "method": "tools/call",
  "params": {
    "name": "sentinel_search_detections",
    "arguments": {
      "query": "severity:critical AND timestamp:>now-24h",
      "limit": 50,
      "sort_by": "timestamp"
    }
  }
}
```

### Response

```json
{
  "jsonrpc": "2.0",
  "id": "1",
  "result": {
    "result": {
      "success": true,
      "data": {
        "detections": [
          {
            "id": "det_001",
            "name": "Suspicious PowerShell Execution",
            "severity": "critical",
            "timestamp": "2026-03-06T10:30:00Z",
            "host_id": "host_001",
            "details": {
              "process": "powershell.exe",
              "command": "Invoke-Expression"
            }
          }
        ],
        "total": 1,
        "query": "severity:critical AND timestamp:>now-24h",
        "time_range": "24h"
      },
      "error": null,
      "metadata": {}
    }
  }
}
```

## Integration with AI Assistants

### Claude (Anthropic)

Claude can use the V-Sentinel MCP server to:
- Search for security detections
- Analyze threat intelligence
- Respond to natural language queries
- Automate incident investigation

### Example Interaction

**User**: "Find all critical detections from the last 24 hours"

**Claude (using MCP)**:
```json
{
  "name": "sentinel_search_detections",
  "arguments": {
    "query": "severity:critical",
    "time_range": "24h",
    "limit": 100
  }
}
```

**Response**: 
```
I found 3 critical detections in the last 24 hours:

1. Suspicious PowerShell Execution - WORKSTATION-001 (10:30 AM)
2. Potential Credential Dumping - SERVER-LINUX-001 (9:45 AM)
3. Malware Detection - MACBOOK-PRO-001 (8:20 AM)

Would you like me to investigate any of these further?
```

## Benefits

1. **AI-Driven Security Operations**: Automate threat hunting and incident response
2. **Natural Language Interface**: Security analysts can use natural language
3. **Reduced Workload**: Estimated 60-70% reduction in analyst workload
4. **Faster Response**: Real-time threat intelligence and analysis
5. **Consistency**: Standardized queries and responses

## Future Enhancements

1. **Additional Tools**:
   - Container security tool
   - Cloud security tool
   - Network analysis tool
   - Forensics tool

2. **Advanced Features**:
   - Multi-step workflows
   - Automated remediation
   - Threat hunting automation
   - Incident response automation

3. **Integration**:
   - Claude (Anthropic) integration
   - ChatGPT (OpenAI) integration
   - Custom AI agent integration

4. **Transport Protocols**:
   - SSE implementation
   - HTTP streaming
   - WebSocket support

## Testing

### Unit Tests
```bash
cargo test --package v-sentinel-mcp
```

### Integration Tests
```bash
cargo test --package v-sentinel-mcp --features integration
```

### Manual Testing
```bash
# Start MCP server
cargo run --package v-sentinel-mcp

# Send test request
echo '{"jsonrpc":"2.0","id":"1","method":"tools/list","params":{}}' | \
  cargo run --package v-sentinel-mcp
```

## Documentation

- [MCP Protocol Specification](https://modelcontextprotocol.io/)
- [V-Sentinel API Reference](docs://api-reference)
- [FQL Guide](docs://fql-guide)
- [Competitive Analysis](../docs/COMPETITIVE_ANALYSIS_STRATEGY.md)

## Contributing

When adding new tools:
1. Implement the `ToolTrait` in `src/tools/`
2. Register the tool in `MCPServer::register_default_tools()`
3. Add comprehensive documentation
4. Write unit tests
5. Update this guide

## License

AGPL-3.0-or-later - See LICENSE file for details.