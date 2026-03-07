# Contributing to V-Sentinel IOCs

Thank you for your interest in contributing to the V-Sentinel Public IOC Repository! This document provides guidelines and best practices for contributing indicators, detection rules, and improvements.

---

## 🎯 Contribution Types

We welcome several types of contributions:

### 1. New Indicators of Compromise (IOCs)
- IP addresses, domain names, URLs, email addresses
- File hashes (MD5, SHA1, SHA256)
- Registry keys, file paths, mutexes
- Certificates and signing information

### 2. Detection Rules
- YARA rules for malware detection
- Snort/Suricata rules for network traffic
- Sigma rules for SIEM correlation
- OpenIOC XML definitions

### 3. Automation Scripts
- IOC generation tools
- Validation and testing scripts
- Format converters
- Enrichment utilities

### 4. Documentation
- Threat analysis reports
- Attribution information
- Usage examples
- Integration guides

---

## 📝 Getting Started

### Prerequisites

```bash
# Clone the repository
git clone https://github.com/vantisCorp/V-Sentinel-IOCs.git
cd V-Sentinel-IOCs

# Install Python dependencies
pip install -r requirements.txt

# Validate your environment
python scripts/validate_iocs.py --test
```

### Setting Up Your Development Environment

```bash
# Create a fork
gh repo fork vantisCorp/V-Sentinel-IOCs --clone

# Add upstream remote
git remote add upstream https://github.com/vantisCorp/V-Sentinel-IOCs.git

# Create a feature branch
git checkout -b feature/your-contribution
```

---

## 🎯 Adding New IOCs

### IOC Format Requirements

All IOCs must follow the JSON format:

```json
{
  "id": "ioc-001",
  "type": "ip",
  "value": "192.0.2.1",
  "threat_actor": "APT29",
  "campaign": "SolarWinds",
  "malware_family": "Sunburst",
  "confidence": "high",
  "first_seen": "2020-12-01T00:00:00Z",
  "last_seen": "2026-01-15T00:00:00Z",
  "description": "Command and control server",
  "source": "V-Sentinel Research",
  "references": [
    "https://attack.mitre.org/groups/G0016/",
    "https://www.fireeye.com/blog.html"
  ],
  "tags": ["c2", "apt", "espionage"],
  "attribution_confidence": "high"
}
```

### IOC Type Guidelines

#### IP Addresses
- Must be valid IPv4 or IPv6 addresses
- Exclude reserved/private ranges (RFC 1918)
- Provide confidence level (low, medium, high)
- Include port information if applicable

#### Domain Names
- Must be fully qualified domain names (FQDNs)
- Exclude wildcards unless specifically targeting subdomains
- Include WHOIS registration dates
- Provide DNS resolution history

#### File Hashes
- Must include at least SHA256
- Include MD5 and SHA1 if available
- Provide file size and type
- Include compilation timestamps if available

#### URLs
- Must include full protocol (http/https)
- Sanitize sensitive parameters
- Include URL path patterns
- Provide HTTP method if relevant

### IOC Submission Checklist

- [ ] IOC format is valid JSON
- [ ] All required fields are present
- [ ] Confidence level is specified
- [ ] Attribution information is included
- [ ] Source references are provided
- [ ] First and last seen dates are accurate
- [ ] IOC is validated using `scripts/validate_iocs.py`
- [ ] No duplicate entries exist
- [ ] No false positives identified

### Example Submission

```json
{
  "id": "ioc-001",
  "type": "domain",
  "value": "malicious-c2[.]example[.]com",
  "threat_actor": "Lazarus Group",
  "campaign": "Operation AppleJeus",
  "malware_family": "AppleJeus",
  "confidence": "high",
  "first_seen": "2024-06-15T00:00:00Z",
  "last_seen": "2026-01-10T00:00:00Z",
  "description": "Command and control domain used in financial theft campaigns targeting cryptocurrency exchanges",
  "source": "V-Sentinel Research",
  "references": [
    "https://attack.mitre.org/groups/G0032/",
    "https://www.kaspersky.com/resource-center/threats/applejeus"
  ],
  "tags": ["c2", "financial", "cryptocurrency", "lazarus"],
  "attribution_confidence": "high"
}
```

---

## 📜 Adding Detection Rules

### YARA Rules

```yara
rule Emotet_Dropper {
    meta:
        description = "Detects Emotet dropper payload"
        threat_actor = "TA542"
        malware_family = "Emotet"
        confidence = "high"
        date = "2026-01-15"
        reference = "https://attack.mitre.org/groups/G0095/"
    
    strings:
        $s1 = { 4D 5A 90 00 }  // MZ header
        $s2 = "powershell.exe" nocase
        $s3 = "Invoke-Expression" nocase
        $s4 = "https://api[.]ipify[.]org" ascii wide
    
    condition:
        uint16(0) == 0x5A4D and
        all of ($s*)
}
```

### Snort/Suricata Rules

```snort
alert tcp $EXTERNAL_NET any -> $HOME_NET 445 (msg:"ET TROJAN Emotet C2 Traffic"; flow:established,to_server; content:"|50 4F 53 54 20|"; depth:4; content:"User-Agent:"; content:"Microsoft-CryptoAPI"; distance:0; reference:url,attack.mitre.org/groups/G0095/; sid:2026001; rev:1;)
```

### Sigma Rules

```yaml
title: Potential Emotet Macro Execution
id: 1a0e5b1c-2d3e-4f5a-9b8c-7d6e5f4a3b2c
description: Detects potential Emotet macro execution in Office documents
status: stable
author: V-Sentinel Research
date: 2026/01/15
references:
    - https://attack.mitre.org/groups/G0095/
tags:
    - attack.initial_access
    - attack.t1566.001
logsource:
    product: windows
    service: security
detection:
    selection:
        EventID: 4688
        NewProcessName|endswith:
            - '\winword.exe'
            - '\excel.exe'
            - '\powerpnt.exe'
        CommandLine|contains:
            - 'powershell.exe'
            - 'cmd.exe /c'
    condition: selection
falsepositives:
    - Legitimate macro usage
level: high
```

### Rule Submission Checklist

- [ ] Rule syntax is valid
- [ ] Metadata is complete (description, author, date, references)
- [ ] MITRE ATT&CK techniques are mapped
- [ ] False positive rate is documented
- [ ] Rule has been tested against known samples
- [ ] Performance impact is assessed
- [ ] Rule follows naming conventions
- [ ] Documentation includes usage examples

---

## 🧪 Testing and Validation

### Running Validation

```bash
# Validate all IOCs
python scripts/validate_iocs.py --path iocs/

# Validate specific category
python scripts/validate_iocs.py --path iocs/apt/

# Generate detailed report
python scripts/validate_iocs.py --path iocs/ --report validation_report.txt

# Test rule syntax
python scripts/test_rules.py --path rules/yara/

# Run integration tests
pytest tests/
```

### Automated CI/CD

All contributions are automatically validated through CI/CD:
- JSON schema validation
- IOC format checking
- Rule syntax validation
- Duplicate detection
- False positive scanning

---

## 📊 Attribution Guidelines

### Confidence Levels

- **High**: Multiple independent sources, technical evidence, official attribution
- **Medium**: Technical evidence with some attribution uncertainty
- **Low**: Single source, circumstantial evidence, industry consensus

### Attribution Standards

1. **Technical Evidence**: Code similarities, infrastructure overlap, TTPs
2. **Victimology**: Target geography, industry sector, timing
3. **Operational Security**: Language, time zones, tool preferences
4. **Intelligence Sources**: Private threat feeds, government advisories, industry reports

### When NOT to Attribute

- Single-source attribution without corroboration
- Speculative connections without technical evidence
- Politically motivated attributions
- Commercially motivated claims

---

## 🚨 False Positive Management

### Reducing False Positives

1. **Specificity**: Use highly specific patterns
2. **Context**: Include contextual information
3. **Testing**: Test against legitimate traffic
4. **Documentation**: Document known FPs
5. **Tuning**: Provide tuning parameters

### Reporting False Positives

If you encounter false positives:

```bash
# Report via GitHub Issue
gh issue create --title "False Positive: [IOC/RULE ID]" \
  --body "Description of false positive scenario"
```

Include:
- IOC or rule ID
- False positive scenario
- Legitimate context
- Suggested improvements

---

## 📝 Documentation Standards

### Threat Analysis Reports

```markdown
# Threat Analysis: [Campaign Name]

## Executive Summary
Brief overview of the threat actor, objectives, and impact.

## Technical Analysis
### Infrastructure
- IPs, domains, certificates
- Hosting providers
- Network topology

### Malware Analysis
- Family description
- Capabilities
- Persistence mechanisms

### Tactics, Techniques, and Procedures
- MITRE ATT&CK mapping
- Kill chain analysis
- Timeline of activity

## Attribution
- Evidence supporting attribution
- Confidence level
- References

## IOCs
List of related indicators

## Detection
Recommended detection rules and hunting queries
```

---

## 🔄 Pull Request Process

### Before Submitting

1. **Validate**: Run `scripts/validate_iocs.py`
2. **Test**: Run `pytest tests/`
3. **Format**: Ensure consistent formatting
4. **Document**: Update relevant documentation
5. **Commit**: Use clear commit messages

### Commit Message Format

```
[TYPE]: Brief description

Detailed explanation of changes:
- What was changed
- Why it was changed
- How it was tested

Related issues: #123
```

Types: `feat`, `fix`, `docs`, `test`, `chore`

### Submitting PR

```bash
# Push your branch
git push origin feature/your-contribution

# Create pull request
gh pr create --title "Add IOCs for [Threat Actor]" \
  --body "Description of changes"
```

### PR Review Process

1. Automated checks must pass
2. Manual review by security researchers
3. Attribution verification
4. False positive assessment
5. Documentation review
6. Approval and merge

---

## 🎖️ Recognition

Contributors are recognized in:
- README.md contributor list
- CHANGELOG.md with attribution
- GitHub contributors graph
- Annual security research report

---

## 📞 Contact

- **Contributions**: security-research@vantis.ai
- **Questions**: Use GitHub Issues
- **Security Issues**: security@vantis.ai

---

## 📜 Code of Conduct

- Be respectful and constructive
- Provide evidence for claims
- Acknowledge limitations
- Collaborate openly
- Follow responsible disclosure

Thank you for contributing to V-Sentinel IOCs! 🙏