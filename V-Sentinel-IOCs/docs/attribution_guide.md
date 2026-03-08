# Threat Attribution Methodology Guide

## Overview

This document outlines V-Sentinel's methodology for threat actor attribution, confidence levels, and the standards used to assign malicious activity to specific groups.

---

## Attribution Framework

### Confidence Levels

V-Sentinel uses three-tier confidence levels for threat attribution:

#### High Confidence
- **Definition**: Multiple independent sources corroborate attribution
- **Requirements**:
  - Technical evidence (code similarities, infrastructure overlap)
  - Official government advisories or private intelligence reports
  - Consistent Tactics, Techniques, and Procedures (TTPs)
  - Multiple independent security firm reports
- **Examples**: APT29 (SolarWinds), Lazarus Group (WannaCry)

#### Medium Confidence
- **Definition**: Technical evidence with some attribution uncertainty
- **Requirements**:
  - Code similarities or shared infrastructure
  - Industry consensus among security researchers
  - Overlapping TTPs but missing definitive proof
- **Examples**: Emotet (TA542), LockBit affiliates

#### Low Confidence
- **Definition**: Single source or circumstantial evidence
- **Requirements**:
  - Single security firm report
  - Limited technical evidence
  - Industry speculation without definitive proof
- **Examples**: Emerging threat groups, single incident analysis

---

## Attribution Criteria

### 1. Technical Evidence

#### Code Analysis
- **Source Code Similarities**: Identical or similar code segments
- **Compiler Timestamps**: Matching compilation dates
- **Code Signing Certificates**: Shared or related certificates
- **Obfuscation Techniques**: Similar encoding methods

#### Infrastructure Overlap
- **IP Address Ranges**: Related or adjacent IP blocks
- **Domain Registration**: Similar registrar patterns
- **Hosting Providers**: Shared hosting infrastructure
- **DNS Configuration**: Similar DNS setups

#### Malware Family
- **Family Relationship**: Variants of known malware families
- **Builder Tools**: Usage of common malware construction kits
- **Packaging**: Similar packing or obfuscation methods

### 2. Behavioral Patterns

#### Victimology
- **Target Geography**: Geographic targeting patterns
- **Industry Sector**: Specific industry focus
- **Organization Size**: Preference for certain organization sizes
- **Timing**: Attack timing patterns

#### Operational Security
- **Language**: Language used in code comments, UI, or communications
- **Time Zones**: Activity patterns matching specific time zones
- **Tool Preferences**: Preference for specific tools or frameworks
- **Error Messages**: Similar error messages or debug strings

### 3. Intelligence Sources

#### Government Advisories
- CISA (Cybersecurity and Infrastructure Security Agency)
- NSA (National Security Agency)
- FBI (Federal Bureau of Investigation)
- International partners (NCSC, ACSC, etc.)

#### Private Intelligence
- Mandiant
- CrowdStrike
- Kaspersky
- Proofpoint
- Group-IB

#### Industry Reports
- Microsoft Threat Intelligence
- Palo Alto Networks Unit 42
- Symantec
- McAfee Advanced Threat Research

---

## Attribution Process

### Phase 1: Data Collection
1. **IOC Extraction**: Extract all indicators of compromise
2. **Malware Analysis**: Analyze malware samples
3. **Network Analysis**: Examine network traffic and infrastructure
4. **Timeline Reconstruction**: Build attack timeline

### Phase 2: Evidence Correlation
1. **Cross-Reference**: Compare with known threat actor profiles
2. **TTP Matching**: Match to MITRE ATT&CK techniques
3. **Infrastructure Analysis**: Correlate with known infrastructure
4. **Code Comparison**: Compare with known malware samples

### Phase 3: Confidence Assessment
1. **Evidence Scoring**: Score each piece of evidence
2. **Source Evaluation**: Evaluate source reliability
3. **Independence Check**: Verify source independence
4. **Consistency Check**: Ensure consistency across sources

### Phase 4: Attribution Assignment
1. **Threat Actor Identification**: Identify most likely threat actor
2. **Confidence Assignment**: Assign appropriate confidence level
3. **Documentation**: Document all evidence and reasoning
4. **Peer Review**: Conduct internal peer review

---

## Common Attribution Pitfalls

### 1. Single Source Attribution
**Problem**: Relying on a single source without corroboration
**Solution**: Require multiple independent sources for high confidence

### 2. Infrastructure Reuse
**Problem**: Attributing based solely on shared infrastructure
**Solution**: Infrastructure reuse is common; require additional evidence

### 3. Malware-as-a-Service
**Problem**: Attributing to operator when malware is rented
**Solution**: Differentiate between malware developers and operators

### 4. False Flags
**Problem**: Threat actors planting false attribution clues
**Solution**: Be skeptical of obvious attribution markers

### 5. Political Bias
**Problem**: Politically motivated attributions
**Solution**: Maintain objectivity and follow evidence

---

## Case Studies

### Case 1: APT29 - SolarWinds Supply Chain Attack

**Evidence Collected**:
- Code similarities to previous APT29 operations
- Infrastructure overlap with known APT29 servers
- Targeting consistent with Russian state interests
- Multiple government attributions (NSA, CISA, UK NCSC)

**Confidence Level**: High

**Attribution Process**:
1. Identified Sunburst backdoor in SolarWinds Orion
2. Analyzed C2 infrastructure (avsvmcloud.com)
3. Correlated with previous APT29 TTPs
4. Reviewed multiple independent reports
5. Confirmed with government advisories

### Case 2: Lazarus Group - WannaCry Ransomware

**Evidence Collected**:
- Code similarities to Lazarus Group malware
- Infrastructure overlap with known Lazarus servers
- Timing consistent with North Korean operations
- Multiple private intelligence firm reports

**Confidence Level**: High

**Attribution Process**:
1. Analyzed WannaCry ransomware samples
2. Identified code segments matching Lazarus tools
3. Correlated with North Korean time zone activity
4. Reviewed intelligence from multiple firms
5. Confirmed with government advisories

---

## Attribution Ethics

### Responsible Attribution

1. **Evidence-Based**: Attribution must be based on technical evidence
2. **Transparency**: Clearly state confidence levels and evidence
3. **Independence**: Use multiple independent sources
4. **Avoid Speculation**: Don't speculate without evidence
5. **Update**: Update attributions as new evidence emerges

### When NOT to Attribute

1. **Single Source**: Without corroboration from other sources
2. **Circumstantial Evidence**: Without technical evidence
3. **Politically Motivated**: When attribution serves political agendas
4. **Commercially Motivated**: When attribution serves commercial interests
5. **Insufficient Evidence**: When evidence is inconclusive

---

## Attribution Standards

### MITRE ATT&CK Framework

V-Sentinel maps all attributed threats to MITRE ATT&CK techniques:
- **Tactics**: High-level goals
- **Techniques**: Methods to achieve tactics
- **Procedures**: Specific implementations

### STIX 2.1 Format

All attributed threats are documented in STIX 2.1 format:
- **Identity Objects**: Threat actor identities
- **Intrusion Set Objects**: Groupings of related attacks
- **Indicator Objects**: Technical indicators
- **Relationship Objects**: Connections between objects

### Industry Standards

V-Sentinel follows industry attribution standards:
- **FIRST**: Forum of Incident Response and Security Teams
- **ISAC**: Information Sharing and Analysis Centers
- **ENISA**: European Union Agency for Cybersecurity
- **ISO/IEC 27001**: Information security management

---

## Attribution Updates

### Review Process

Attributions are reviewed quarterly or when new evidence emerges:
1. **Evidence Review**: Review new evidence
2. **Confidence Reassessment**: Reassess confidence levels
3. **Attribution Updates**: Update or modify attributions
4. **Documentation**: Document changes and reasoning

### Version Control

All attributions are version controlled:
- **Major Updates**: Significant new evidence requiring reattribution
- **Minor Updates**: Additional evidence supporting existing attribution
- **Corrections**: Corrections to previous attributions

---

## Contact

For questions about threat attribution:
- **Security Research Team**: security-research@vantis.ai
- **Attribution Questions**: attribution@vantis.ai
- **Report Errors**: Use GitHub Issues

---

## References

- [MITRE ATT&CK](https://attack.mitre.org/)
- [CISA Advisory Catalog](https://www.cisa.gov/news-events/cybersecurity-advisories)
- [Mandiant Threat Intelligence](https://www.mandiant.com/resources/threat-intelligence)
- [CrowdStrike Threat Intelligence](https://www.crowdstrike.com/resources/)

---

**Last Updated**: 2026-01-15  
**Version**: 1.0.0  
**Maintained By**: V-Sentinel Security Research Team