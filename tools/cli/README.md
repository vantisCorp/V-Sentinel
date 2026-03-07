# V-Sentinel CLI

A powerful command-line interface for V-Sentinel threat intelligence and security operations.

## Installation

### npm
```bash
npm install -g @vantis/vsentinel-cli
```

### From Source
```bash
cd tools/cli
npm install
npm run build
npm link
```

## Getting Started

### Initialize Configuration
```bash
vsentinel init
```

This will prompt for:
- API URL (default: https://api.vantis.ai/v1)
- API Key
- Request timeout

### Verify Configuration
```bash
vsentinel config show
```

## Commands

### IOC Lookup
Look up indicators of compromise against the threat intelligence database.

```bash
# Basic lookup
vsentinel lookup 192.168.1.1

# With related IOCs
vsentinel lookup malicious.com --related

# JSON output
vsentinel lookup abc123hash --json

# YAML output
vsentinel lookup https://malicious.url --yaml
```

### File Scanning
Scan files for indicators of compromise.

```bash
# Scan a file
vsentinel scan suspicious.txt

# Output as JSON
vsentinel scan logfile.txt --format json

# Save results to file
vsentinel scan file.txt --output report.md

# Filter by IOC types
vsentinel scan file.txt --types ip,domain,url

# Minimum confidence level
vsentinel scan file.txt --min-confidence high
```

### Detections Management
View and manage security detections.

```bash
# List detections
vsentinel detections list

# Filter by status
vsentinel detections list --status new

# Filter by severity
vsentinel detections list --severity critical

# Get specific detection
vsentinel detections get --id <detection-id>

# Update detection status
vsentinel detections update --id <detection-id> --status resolved
```

### Hosts Management
View and manage endpoint hosts.

```bash
# List hosts
vsentinel hosts list

# Filter by platform
vsentinel hosts list --platform windows

# Filter by state
vsentinel hosts list --state online

# Get host details
vsentinel hosts get --id <host-id>

# Isolate a host
vsentinel hosts isolate --id <host-id>

# Remove isolation
vsentinel hosts unisolate --id <host-id>
```

### Incidents Management
Manage security incidents.

```bash
# List incidents
vsentinel incidents list

# Filter by status
vsentinel incidents list --status in_progress

# Get incident details
vsentinel incidents get --id <incident-id>

# Create incident
vsentinel incidents create --title "Suspicious Activity" --description "Detected anomaly" --severity high

# Update incident status
vsentinel incidents update --id <incident-id> --status resolved
```

### Threat Intelligence
Access threat intelligence data.

```bash
# List threat actors
vsentinel threat-intel actors

# Search for specific actor
vsentinel threat-intel actors --name "APT29"

# Get MITRE ATT&CK techniques
vsentinel threat-intel techniques

# Get specific technique
vsentinel threat-intel techniques --id T1566

# List IOCs
vsentinel threat-intel iocs
```

### Configuration
Manage CLI configuration.

```bash
# Show configuration
vsentinel config show

# Set API key
vsentinel config set --api-key your-api-key

# Set API URL
vsentinel config set --api-url https://api.vantis.ai/v1

# Reset configuration
vsentinel config reset
```

## Output Formats

All commands support multiple output formats:

### Table (default)
Human-readable table format with colors and formatting.

### JSON
```bash
vsentinel lookup 192.168.1.1 --json
```

### YAML
```bash
vsentinel lookup 192.168.1.1 --yaml
```

## Environment Variables

You can also configure the CLI using environment variables:

| Variable | Description |
|----------|-------------|
| `VSENTINEL_API_URL` | API base URL |
| `VSENTINEL_API_KEY` | API authentication key |
| `VSENTINEL_TIMEOUT` | Request timeout in seconds |

## Examples

### Quick Threat Intelligence Check
```bash
# Check an IP address
vsentinel lookup 45.155.205.233

# Check a domain
vsentinel lookup malicious-site.com

# Check a file hash
vsentinel lookup 5d41402abc4b2a76b9719d911017c592
```

### Bulk Analysis
```bash
# Scan a log file for IOCs
vsentinel scan /var/log/auth.log --output ioc-report.md

# Get all critical detections
vsentinel detections list --severity critical --json > critical-detections.json
```

### Incident Response Workflow
```bash
# List active incidents
vsentinel incidents list --status in_progress

# Get incident details
vsentinel incidents get --id inc-abc123

# Isolate affected host
vsentinel hosts isolate --id host-xyz789

# Update incident status
vsentinel incidents update --id inc-abc123 --status resolved
```

## Command Aliases

Short aliases are available for common commands:

| Alias | Command |
|-------|---------|
| `l` | `lookup` |
| `s` | `scan` |
| `d` | `detections` |
| `h` | `hosts` |
| `i` | `incidents` |
| `ti` | `threat-intel` |

Example:
```bash
vsentinel l 192.168.1.1
vsentinel s suspicious-file.txt
vsentinel d list
```

## Configuration File

Configuration is stored in:
- **Linux/macOS**: `~/.config/configstore/vsentinel.json`
- **Windows**: `%APPDATA%\configstore\vsentinel.json`

## Shell Completion

To enable shell completion, add to your `.bashrc` or `.zshrc`:

```bash
# Bash
source <(vsentinel --help | grep -E '^  [a-z]' | awk '{print $1}' | while read cmd; do complete -W "$cmd" vsentinel; done)

# Zsh
autoload -U compinit && compinit
```

## Exit Codes

| Code | Description |
|------|-------------|
| 0 | Success |
| 1 | Error (API error, not found, etc.) |
| 2 | Invalid arguments |
| 3 | Configuration error |

## Troubleshooting

### API Key Not Set
```bash
vsentinel config set --api-key your-api-key
```

### Connection Refused
Check the API URL:
```bash
vsentinel config show
vsentinel config set --api-url https://api.vantis.ai/v1
```

### Timeout Errors
Increase timeout:
```bash
vsentinel config set --timeout 60
```

## Contributing

See the main [CONTRIBUTING.md](../../CONTRIBUTING.md) for guidelines.

## License

BSD-2-Clause License - see [LICENSE](../../LICENSE) for details.