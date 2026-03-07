# Falcon Query Language (FQL) Guide

FQL (Falcon Query Language) is a powerful query language for searching and filtering security data in V-Sentinel.

## Basic Syntax

FQL uses a simple field:value syntax:

```
field:value
```

## Supported Operators

- `:` Equals
- `!=` Not equals
- `>` Greater than
- `<` Less than
- `>=` Greater than or equal
- `<=` Less than or equal
- `*` Wildcard
- `~` Regex match

## Common Fields

### Detections

| Field | Type | Description |
|-------|------|-------------|
| `severity` | string | Severity level (critical, high, medium, low) |
| `status` | string | Detection status (new, in_progress, resolved) |
| `hostname` | string | Host where detection occurred |
| `filename` | string | Process filename |
| `command_line` | string | Process command line |
| `behaviors` | string | Detection behaviors |
| `timestamp` | timestamp | Detection time |

### Examples

#### Search by severity
```
severity:critical
```

#### Search by multiple severities
```
severity:critical OR severity:high
```

#### Search by hostname
```
hostname:WORKSTATION-001
```

#### Search by process name
```
filename:powershell.exe
```

#### Search by command line
```
command_line:*Invoke-Expression*
```

#### Search by behaviors
```
behaviors:*
```

#### Search by time range
```
timestamp:>2026-03-06T00:00:00Z
timestamp:<2026-03-07T00:00:00Z
```

#### Complex query
```
severity:critical AND filename:powershell.exe AND command_line:*Invoke-Expression*
```

#### Wildcard search
```
command_line:*.exe
```

#### Regex search
```
hostname:~WORKSTATION-\d+
```

## Logical Operators

Combine multiple conditions with:

- `AND` - All conditions must match
- `OR` - Any condition must match
- `NOT` - Condition must not match

## Grouping

Use parentheses to group conditions:

```
(severity:critical OR severity:high) AND hostname:WORKSTATION-*
```

## Time Range Syntax

Absolute time:
```
timestamp:2026-03-06T10:00:00Z
```

Relative time:
```
timestamp:>now-1h    # Last hour
timestamp:>now-24h   # Last 24 hours
timestamp:>now-7d    # Last 7 days
```

## Best Practices

1. **Use specific fields** - More specific queries are faster
2. **Avoid broad wildcards** - Use `*` only when necessary
3. **Combine filters** - Use multiple conditions to narrow results
4. **Use time ranges** - Always specify a time range when possible
5. **Test queries** - Start with broad queries, then refine

## Advanced Examples

#### Critical detections in last hour on Windows hosts
```
severity:critical AND timestamp:>now-1h AND platform:Windows
```

#### PowerShell with suspicious commands
```
filename:powershell.exe AND (command_line:*Invoke-Expression* OR command_line:*DownloadString*)
```

#### Suspicious network activity
```
behaviors:*network* AND (destination_ip:192.168.1.0/24 OR destination_ip:10.0.0.0/8)
```

#### Malware detections with specific behaviors
```
malware:* AND (behaviors:*suspicious* OR behaviors:*malicious*)
```

## Performance Tips

1. Index fields are faster: `severity`, `timestamp`, `hostname`
2. Avoid leading wildcards: `*test` is slower than `test*`
3. Use time ranges to limit results
4. Limit result sets with `limit` parameter

## MCP Integration

When using FQL with the V-Sentinel MCP server tools:

```json
{
  "name": "sentinel_search_detections",
  "arguments": {
    "query": "severity:critical AND timestamp:>now-24h",
    "limit": 100,
    "sort_by": "timestamp"
  }
}
```

## Troubleshooting

### No results
- Check field names are correct
- Verify time range
- Try broadening the query

### Too many results
- Add more specific filters
- Reduce time range
- Use `limit` parameter

### Slow queries
- Add time range
- Use indexed fields
- Avoid broad wildcards

## Additional Resources

- [V-Sentinel API Reference](docs://api-reference)
- [MCP Tools Documentation](/docs/mcp-tools)
- [FQL Examples](/examples/fql)