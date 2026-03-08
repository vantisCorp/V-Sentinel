# Changelog

All notable changes to the V-Sentinel IOC Repository will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

---

## [1.0.0] - 2026-01-15

### Added
- Initial release of V-Sentinel Public IOC Repository
- 12,450+ Indicators of Compromise across multiple threat categories
- 850+ detection rules (YARA, Snort, Sigma)
- Complete automation scripts for IOC generation and validation
- Comprehensive documentation and integration guides

#### Threat Categories
- **APT Groups**: 4,200+ IOCs for APT29, Lazarus Group, APT28, APT33
- **Ransomware**: 2,800+ IOCs for LockBit, BlackCat, Hive, Conti
- **Botnets**: 1,650+ IOCs for Emotet, TrickBot, Mirai, QakBot
- **Trojans**: 1,800+ IOCs for Agent Tesla, RedLine Stealer, DarkComet

#### Detection Rules
- **YARA Rules**: 350+ rules for malware detection
- **Snort/Suricata Rules**: 280+ network detection rules
- **Sigma Rules**: 220+ SIEM correlation rules

#### Automation Tools
- `generate_iocs.py` - IOC generation from threat reports
- `validate_iocs.py` - IOC validation and quality checks
- `format_converter.py` - Format conversion (JSON, CSV, STIX, OpenIOC)
- `enrichment.py` - IOC enrichment from external sources

#### Documentation
- Attribution guide with confidence levels
- False positive management guidelines
- Integration guides for Splunk, ELK, QRadar
- API documentation for programmatic access

---

## [Unreleased]

### Planned
- Web dashboard for IOC visualization
- REST API for IOC queries
- Real-time IOC push notifications
- Machine learning-based IOC prioritization
- Community voting system for IOC confidence
- Automated threat hunting queries

---

## [0.9.0-beta] - 2025-12-20

### Added
- Beta release with 8,000+ IOCs
- Initial automation scripts
- Basic documentation

### Changed
- Improved IOC validation accuracy
- Enhanced rule performance

---

## Versioning

This project uses semantic versioning:

- **MAJOR**: Breaking changes, major reorganizations
- **MINOR**: New features, new IOCs, new rules (backward compatible)
- **PATCH**: Bug fixes, documentation updates, IOCs updates

---

## Release Cadence

- **Major Releases**: Quarterly (every 3 months)
- **Minor Releases**: Monthly
- **Patch Releases**: As needed (critical updates)

---

## Migration Guide

### Upgrading from 0.9.0-beta to 1.0.0

No breaking changes. Simply update:

```bash
git pull origin main
python scripts/validate_iocs.py --path iocs/
```

---

## Contributors

See [CONTRIBUTORS.md](CONTRIBUTORS.md) for the full list of contributors.

---

## License

This project is licensed under the BSD-2-Clause License - see [LICENSE](LICENSE) for details.

---

## Links

- [Repository](https://github.com/vantisCorp/V-Sentinel-IOCs)
- [Documentation](https://docs.vantis.ai/iocs)
- [Issues](https://github.com/vantisCorp/V-Sentinel-IOCs/issues)
- [Discussions](https://github.com/vantisCorp/V-Sentinel-IOCs/discussions)