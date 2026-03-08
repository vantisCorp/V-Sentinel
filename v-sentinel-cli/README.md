# V-Sentinel CLI

A standalone command-line interface for V-Sentinel, the AI-Powered Security Sentinel System.

## Installation

### Download Binary

Download the latest release for your platform from the [Releases page](https://github.com/vantisCorp/V-Sentinel/releases).

### Build from Source

```bash
cd v-sentinel-cli
cargo build --release
```

The binary will be available at `target/release/v-sentinel`.

## Usage

```
V-Sentinel - AI-Powered Security Sentinel System CLI

Usage: v-sentinel [OPTIONS] <COMMAND>

Commands:
  start         Start the V-Sentinel security agent
  stop          Stop the V-Sentinel security agent
  status        Show current status of V-Sentinel
  scan          Run a security scan
  threat-intel  Threat intelligence operations
  deepfake      Deepfake detection operations
  shadow-ai     Shadow AI detection operations
  zero-trust    Zero trust network operations
  config        Configuration management
  version       Version information
  help          Print this message or the help of the given subcommand(s)

Options:
  -v, --verbose          Enable verbose output
  -c, --config <CONFIG>  Configuration file path
  -h, --help             Print help
  -V, --version          Print version
```

## Examples

### Start the Security Agent

```bash
# Start in foreground
v-sentinel start

# Start as daemon (background)
v-sentinel start --daemon
```

### Check Status

```bash
# Human-readable status
v-sentinel status

# JSON output for scripting
v-sentinel status --json
```

### Run a Security Scan

```bash
# Quick scan
v-sentinel scan

# Full scan of specific path
v-sentinel scan --path /home/user --scan-type full

# Save results to file
v-sentinel scan --output results.json
```

### Threat Intelligence

```bash
# Update threat database
v-sentinel threat-intel --update

# Search for specific threat
v-sentinel threat-intel --search "APT29"
```

### Deepfake Detection

```bash
# Analyze a media file
v-sentinel deepfake --input video.mp4

# Save analysis report
v-sentinel deepfake --input video.mp4 --output report.json
```

### Shadow AI Detection

```bash
# Scan network for unauthorized AI tools
v-sentinel shadow-ai --scan

# Scan specific network range
v-sentinel shadow-ai --scan --network 192.168.1.0/24
```

### Zero Trust Operations

```bash
# Verify all devices
v-sentinel zero-trust --verify

# Verify specific device
v-sentinel zero-trust --verify --device-id "device-123"
```

### Configuration

```bash
# Show current configuration
v-sentinel config --show

# Set configuration value
v-sentinel config --set log_level=debug

# Reset to defaults
v-sentinel config --reset
```

## Configuration

The CLI looks for configuration in the following locations:

- `/etc/v-sentinel/config.yaml` (system-wide)
- `~/.config/v-sentinel/config.yaml` (user)
- `./v-sentinel.yaml` (current directory)

You can specify a custom configuration file with the `-c` option:

```bash
v-sentinel -c /path/to/config.yaml status
```

## Shell Completion

The CLI supports shell completion for bash, zsh, fish, and PowerShell:

```bash
# Generate bash completion
v-sentinel --help | grep -A 100 "Commands:" > /dev/null
# Or use clap's completion generation
```

## License

MIT License - See [LICENSE](../LICENSE) for details.

## Related

- [V-Sentinel Main Repository](https://github.com/vantisCorp/V-Sentinel)
- [Documentation](https://github.com/vantisCorp/V-Sentinel/blob/main/docs/)
- [Installation Guide](https://github.com/vantisCorp/V-Sentinel/blob/main/INSTALL.md)