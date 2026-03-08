# V-Sentinel Public IOC Repository

[![License](https://img.shields.io/badge/License-BSD--2--Clause-blue.svg)](LICENSE)
[![Contributions](https://img.shields.io/badge/Contributions-Welcome-green.svg)](CONTRIBUTING.md)
[![Last Updated](https://img.shields.io/badge/Last%20Updated-2026--01--15-informational.svg)]()
[![Stars](https://img.shields.io/github/stars/vantisCorp/V-Sentinel-IOCs?style=social)](https://github.com/vantisCorp/V-Sentinel-IOCs/stargazers)

> 🎯 **Intelligence-Driven Security Indicators**  
> High-fidelity IOCs, detection rules, and threat intelligence from the V-Sentinel security research team.

---

## 📋 Overview

This repository hosts publicly available **Indicators of Compromise (IOCs)**, detection rules, and threat intelligence data curated by the V-Sentinel security research team. All indicators are derived from real-world threat investigations and are continuously updated to reflect the evolving threat landscape.

### What You'll Find

- 🎯 **Threat Indicators** - IPs, domains, hashes, URLs, email addresses
- 📜 **Detection Rules** - YARA, Snort, Sigma rules for popular security tools
- 🧪 **Validation Scripts** - Python tools for IOC generation and validation
- 📚 **Documentation** - Detailed attribution information and analysis notes
- 🔄 **Automated Updates** - CI/CD pipelines for continuous IOC enrichment

---

## 🎯 Threat Categories

### APT Groups
- 🇨🇳 **APT29 (Cozy Bear)** - State-sponsored espionage
- 🇰🇵 **Lazarus Group** - Financial theft and espionage
- 🇷🇺 **APT28 (Fancy Bear)** - Military and government targeting
- 🇮🇷 **APT33** - Energy sector espionage

### Ransomware
- 💀 **LockBit 3.0** - Ransomware-as-a-service
- 🔒 **BlackCat/ALPHV** - Targeted enterprise attacks
- 🎭 **Hive** - Healthcare sector focus
- ⚡ **Conti** - Discontinued but still active

### Botnets
- 🤖 **Emotet** - Banking trojan botnet
- 🌐 **TrickBot** - Modular banking malware
- 📡 **Mirai** - IoT DDoS botnet
- 🎪 **QakBot** - Banking credential theft

### Trojans & Malware
- 💉 **Emotet** - Banking credential theft
- 🔓 **Agent Tesla** - Information stealer
- 📦 **RedLine Stealer** - Browser credential theft
- 🎯 **DarkComet** - Remote access trojan

---

## 📁 Repository Structure

```
V-Sentinel-IOCs/
├── README.md                 # This file
├── LICENSE                   # BSD-2-Clause license
├── CONTRIBUTING.md           # Contribution guidelines
├── CHANGELOG.md              # Version history
├── iocs/                     # Indicator data
│   ├── apt/                  # APT group IOCs
│   ├── ransomware/           # Ransomware IOCs
│   ├── botnets/              # Botnet IOCs
│   ├── trojans/              # Trojan/malware IOCs
│   └── espionage/            # Espionage campaign IOCs
├── rules/                    # Detection rules
│   ├── yara/                 # YARA rules
│   ├── snort/                # Snort/Suricata rules
│   ├── sigma/                # Sigma rules
│   └── openioc/              # OpenIOC format
├── scripts/                  # Automation scripts
│   ├── generate_iocs.py      # IOC generator
│   ├── validate_iocs.py      # IOC validator
│   ├── format_converter.py   # Format conversion
│   └── enrichment.py         # IOC enrichment
├── docs/                     # Documentation
│   ├── attribution_guide.md  # Threat attribution methodology
│   ├── false_positive_management.md # FP reduction
│   └── integration_guide.md  # Tool integration
└── tests/                    # Test suites
    ├── test_iocs.py          # IOC validation tests
    ├── test_rules.py         # Rule syntax tests
    └── integration_tests.py  # End-to-end tests
```

---

## 🚀 Quick Start

### Download All IOCs

```bash
# Clone the repository
git clone https://github.com/vantisCorp/V-Sentinel-IOCs.git
cd V-Sentinel-IOCs

# Download latest IOC data
python scripts/generate_iocs.py --download-all --output iocs/latest.json
```

### Validate IOCs

```bash
# Validate all IOC files
python scripts/validate_iocs.py --path iocs/

# Generate validation report
python scripts/validate_iocs.py --path iocs/ --report validation_report.txt
```

### Convert Formats

```bash
# Convert JSON to CSV
python scripts/format_converter.py --input iocs/latest.json --output iocs/latest.csv --format csv

# Convert to STIX 2.1
python scripts/format_converter.py --input iocs/latest.json --output iocs/latest.stix --format stix
```

### Load into Security Tools

```bash
# YARA rules
yara -r rules/yara/ /path/to/scan

# Snort/Suricata
snort -c rules/snort/ -i eth0

# Sigma (convert to Splunk)
sigma convert --target splunk --output rules/splunk/ rules/sigma/
```

---

## 📊 IOC Statistics

- **Total IOCs**: 12,450+
- **IP Addresses**: 4,200+
- **Domain Names**: 3,800+
- **File Hashes**: 2,800+
- **URLs**: 1,650+
- **Detection Rules**: 850+
- **Last Updated**: 2026-01-15

---

## 🔍 Usage Examples

### Python Integration

```python
import json
from vantis_iocs import IOCLoader

# Load IOCs
loader = IOCLoader()
iocs = loader.load_from_file('iocs/apt/apt29.json')

# Filter by type
malicious_ips = [ioc['value'] for ioc in iocs if ioc['type'] == 'ip']

# Check against network traffic
def is_malicious(ip):
    return ip in malicious_ips

# Validate domain
def check_domain(domain):
    for ioc in iocs:
        if ioc['type'] == 'domain' and ioc['value'] == domain:
            return True
    return False
```

### SIEM Integration (Splunk)

```splunk
index=network 
| lookup iocs.json indicator AS dest_ip OUTPUT threat_actor, campaign, severity
| where isnotnull(threat_actor)
| table _time, dest_ip, threat_actor, campaign, severity
```

### Firewall Integration (Palo Alto)

```bash
# Block malicious IPs
curl -X POST "https://firewall/api/?type=config&action=set&xpath=/config/devices/entry/vsys/entry/address-group/entry[@name='Malicious-IOCs']/member&element=<member>10.0.0.1</member>"

# Block malicious domains
curl -X POST "https://firewall/api/?type=config&action=set&xpath=/config/devices/entry/vsys/entry/profiles/entry[@name='DNS-Security']/custom/signature/entry[@name='Malicious-Domains']&element=<pattern>*.malicious.com</pattern>"
```

---

## 🤝 Contributing

We welcome contributions from the security community! Please follow these guidelines:

### Adding New IOCs

1. Fork the repository
2. Create a new branch for your contribution
3. Add IOCs to the appropriate category folder
4. Include attribution information
5. Validate using `scripts/validate_iocs.py`
6. Submit a pull request

### Adding Detection Rules

1. Follow the rule format guidelines
2. Include detailed descriptions
3. Test rules against known good and bad samples
4. Document false positive rates
5. Submit for review

### Code of Conduct

- Be respectful and constructive
- Provide evidence for attributions
- Avoid false positives
- Follow responsible disclosure

See [CONTRIBUTING.md](CONTRIBUTING.md) for detailed guidelines.

---

## 📜 License

This repository is licensed under the **BSD-2-Clause License**:

```
Redistribution and use in source and binary forms, with or without
modification, are permitted provided that the following conditions are met:

1. Redistributions of source code must retain the above copyright notice,
   this list of conditions and the following disclaimer.

2. Redistributions in binary form must reproduce the above copyright notice,
   this list of conditions and the following disclaimer in the documentation
   and/or other materials provided with the distribution.
```

See [LICENSE](LICENSE) for the full license text.

---

## 🔗 Related Projects

- [V-Sentinel Core](https://github.com/vantisCorp/V-Sentinel) - Main V-Sentinel repository
- [V-Sentinel Docs](https://github.com/vantisCorp/V-Sentinel-Docs) - Official documentation
- [V-Sentinel MCP](https://github.com/vantisCorp/V-Sentinel/tree/main/src/mcp) - Model Context Protocol integration

---

## 📞 Contact

- **Security Research Team**: security-research@vantis.ai
- **GitHub Issues**: [Report a bug](https://github.com/vantisCorp/V-Sentinel-IOCs/issues)
- **Discord**: [Join our community](https://discord.gg/vantis-sentinel)

---

## 🙏 Acknowledgments

This repository is maintained by the V-Sentinel security research team with contributions from the global cybersecurity community.

**Made with ❤️ by V-Sentinel**