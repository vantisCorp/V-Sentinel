# V-Sentinel - VS Code Extension

A comprehensive Visual Studio Code extension for threat intelligence and security operations, integrated with the V-Sentinel platform.

## Features

### IOC Lookup and Management
- **Quick IOC Lookup**: Select any text and instantly look it up against V-Sentinel's threat intelligence database
- **Multi-format Support**: Supports IPs, domains, URLs, file hashes (MD5, SHA1, SHA256), and email addresses
- **Batch Scanning**: Scan entire files for indicators of compromise
- **Hash Calculation**: Calculate and check file hashes directly from the editor

### Threat Intelligence Explorer
- **Tree View Panel**: Browse recent IOCs and active detections in a dedicated sidebar
- **Threat Actor Profiles**: View detailed information about threat actors, including aliases, campaigns, and MITRE ATT&CK techniques
- **MITRE ATT&CK Integration**: Browse and search MITRE ATT&CK techniques directly in VS Code

### Syntax Highlighting
- **IOC JSON**: Specialized syntax highlighting for IOC JSON files
- **YARA Rules**: Full syntax highlighting for YARA rule development
- **Semantic Highlighting**: Automatic detection and highlighting of IOCs in any file

### Code Snippets
- Pre-built snippets for creating IOC entries
- YARA rule templates for various malware types
- APT and ransomware detection rule templates

## Installation

### From VSIX
```bash
vsce package
code --install-extension vsentinel-1.0.0.vsix
```

### Development
```bash
cd tools/vscode-extension
npm install
npm run compile
```

Press F5 in VS Code to launch the extension in development mode.

## Configuration

Configure the extension in VS Code settings:

| Setting | Description | Default |
|---------|-------------|---------|
| `vsentinel.apiUrl` | V-Sentinel API base URL | `https://api.vantis.ai/v1` |
| `vsentinel.apiKey` | API key for authentication | `` |
| `vsentinel.autoScan` | Automatically scan files for IOCs | `false` |
| `vsentinel.highlightIOCs` | Highlight detected IOCs in editor | `true` |
| `vsentinel.iocColors` | Colors for different IOC types | See settings |
| `vsentinel.maxResults` | Maximum results to display | `100` |
| `vsentinel.timeout` | API request timeout (seconds) | `30` |

## Commands

| Command | Description |
|---------|-------------|
| `V-Sentinel: Lookup IOC` | Look up selected text or entered value |
| `V-Sentinel: Scan File for IOCs` | Scan current file for indicators |
| `V-Sentinel: Add IOC to Repository` | Add a new IOC to the repository |
| `V-Sentinel: Check File Hash` | Calculate and check file hash |
| `V-Sentinel: Search Threat Actor` | Search for threat actor information |
| `V-Sentinel: Get MITRE ATT&CK Techniques` | Browse MITRE techniques |
| `V-Sentinel: Refresh Explorer` | Refresh the tree view |

## Keyboard Shortcuts

Add these to your `keybindings.json`:

```json
[
    {
        "key": "ctrl+shift+i l",
        "command": "vsentinel.lookupIOC",
        "when": "editorTextFocus"
    },
    {
        "key": "ctrl+shift+i s",
        "command": "vsentinel.scanFile",
        "when": "editorTextFocus"
    }
]
```

## Context Menu

The extension adds context menu items:

- **Editor Context Menu**: Right-click selected text to look up IOC
- **Explorer Context Menu**: Right-click files to scan for IOCs

## IOC Types Supported

| Type | Example |
|------|---------|
| IP Address | `192.168.1.1`, `2001:db8::1` |
| Domain | `malicious.example.com` |
| URL | `https://example.com/payload` |
| MD5 Hash | `d41d8cd98f00b204e9800998ecf8427e` |
| SHA1 Hash | `da39a3ee5e6b4b0d3255bfef95601890afd80709` |
| SHA256 Hash | `e3b0c44298fc1c149afbf4c8996fb924...` |
| Email | `attacker@example.com` |

## Example Workflow

1. **Open a suspicious file** in VS Code
2. **Select an IP address** or domain
3. **Right-click** → **V-Sentinel: Lookup IOC**
4. View the threat intelligence report in a new document

Or:

1. **Open any file** in VS Code
2. Press **Ctrl+Shift+I S** to scan for IOCs
3. Review the IOC summary and detailed report

## YARA Rule Development

The extension provides excellent support for YARA rule development:

```yara
rule Example_Malware {
    meta:
        author = "Security Team"
        description = "Detects example malware"
    
    strings:
        $s1 = "malicious_string" nocase
        $h1 = { 4D 5A 90 00 }
    
    condition:
        any of them
}
```

Use the `yara-rule` snippet to quickly create new rules.

## Security Considerations

- API keys are stored in VS Code settings - consider using workspace settings for sensitive keys
- IOC values may contain sensitive data - be mindful when sharing files
- The extension makes network requests to the configured API endpoint

## Contributing

Contributions are welcome! Please see the main [CONTRIBUTING.md](../../CONTRIBUTING.md) for guidelines.

## License

BSD-2-Clause License - see [LICENSE](../../LICENSE) for details.

## Support

- **Issues**: [GitHub Issues](https://github.com/vantis-ai/v-sentinel/issues)
- **Documentation**: [V-Sentinel Docs](https://docs.vantis.ai/v-sentinel)
- **Community**: [Discord](https://discord.gg/vantis)