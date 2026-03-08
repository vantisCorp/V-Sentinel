# Cryptographic Inventory Tool

## Overview

The Cryptographic Inventory Tool is a security assessment utility designed to scan the V-Sentinel codebase and identify all cryptographic operations. It helps assess quantum vulnerability exposure by detecting quantum-vulnerable algorithms (RSA, ECC, Diffie-Hellman) and quantum-safe alternatives.

## Purpose

### Post-Quantum Cryptography (PQC) Migration Readiness

As quantum computing advances, classical cryptographic algorithms like RSA and ECC will become vulnerable. This tool helps organizations:

1. **Inventory**: Identify all cryptographic operations in the codebase
2. **Assess**: Determine quantum vulnerability exposure
3. **Prioritize**: Rank operations by risk level
4. **Plan**: Create a roadmap for PQC migration

### Key Capabilities

- **Pattern-Based Detection**: Uses regex patterns to find cryptographic operations
- **Algorithm Classification**: Identifies specific algorithms used
- **Vulnerability Assessment**: Classifies algorithms as quantum-vulnerable or quantum-safe
- **Risk Scoring**: Calculates overall quantum risk level (Critical, High, Medium, Low)
- **Detailed Reporting**: Generates comprehensive markdown reports
- **Actionable Recommendations**: Provides prioritized migration guidance

## Usage

### Running the Tool

```bash
# Run from project root
cd src/tools
cargo run --bin crypto-inventory

# Or compile and run directly
cargo build --release --bin crypto-inventory
./target/release/crypto-inventory
```

### Output

The tool generates:

1. **Console Summary**: Quick overview of findings
2. **Markdown Report**: Detailed analysis saved to `crypto_vulnerability_report.md`

## Detection Categories

### Cryptographic Operations Detected

| Operation Type | Description |
|----------------|-------------|
| Encryption | Data encryption operations (AES, RSA, etc.) |
| Decryption | Data decryption operations |
| Key Generation | Cryptographic key pair generation |
| Key Exchange | Key exchange protocols (DH, ECDH) |
| Digital Signature | Digital signature creation |
| Signature Verification | Signature verification operations |
| Hashing | Hash functions (SHA, BLAKE, etc.) |
| Key Derivation | Key derivation functions (HKDF, PBKDF2, etc.) |
| Random Number Generation | RNG operations |

### Quantum-Vulnerable Algorithms

The following algorithms are vulnerable to quantum attacks:

- **RSA** (all key sizes)
- **ECC** (Elliptic Curve Cryptography)
- **ECDH** (Elliptic Curve Diffie-Hellman)
- **ECDSA** (Elliptic Curve Digital Signature Algorithm)
- **Ed25519** (Edwards-curve Digital Signature)
- **Ed448**
- **X25519** (Curve25519 key exchange)
- **X448**
- **Diffie-Hellman** (classical)

### Quantum-Safe Algorithms

The following algorithms are considered quantum-safe:

- **CRYSTALS-Kyber** (PQC KEM)
- **CRYSTALS-Dilithium** (PQC Signatures)
- **SPHINCS+** (Hash-based signatures)
- **FALCON** (Lattice-based signatures)
- **AES** (Symmetric encryption)
- **ChaCha20** (Stream cipher)
- **SHA-256/SHA-512/SHA-3** (Hash functions)
- **BLAKE2** (Hash function)
- **HKDF/PBKDF2/Scrypt/Argon2** (Key derivation)

## Risk Assessment

### Risk Levels

| Level | Vulnerability Ratio | Description |
|-------|-------------------|-------------|
| **Critical** | >50% with RSA/ECC | Immediate action required |
| **High** | >50% overall | Significant vulnerabilities |
| **Medium** | >30% overall | Moderate vulnerabilities |
| **Low** | ≤30% overall | Good quantum readiness |

### Risk Factors

The tool considers:

1. **Vulnerability Ratio**: Percentage of operations using quantum-vulnerable algorithms
2. **Algorithm Criticality**: RSA and ECC operations weighted higher
3. **Operation Type**: Key exchange and signatures considered higher risk

## Report Structure

### Executive Summary

- Overall risk level
- Total cryptographic operations
- Quantum-vulnerable vs quantum-safe breakdown
- Algorithm distribution

### Detailed Analysis

- List of all quantum-vulnerable operations with:
  - File path and line number
  - Operation type
  - Algorithm used
  - Pattern matched

- Summary of quantum-safe operations

### Recommendations

Prioritized actions based on risk level:

1. **Critical**: Immediate PQC migration required
2. **High**: Urgent PQC migration planning
3. **Medium**: Begin PQC assessment
4. **Low**: Continue monitoring

## Integration with V-Sentinel

### Phase 1: Discovery

The cryptographic inventory tool is the first phase of V-Sentinel's PQC migration plan:

```
Phase 1: Cryptographic Discovery
├── Run inventory tool
├── Generate vulnerability report
├── Identify critical operations
└── Prioritize migration targets

Phase 2: PQC Implementation
├── Add CRYSTALS-Kyber (KEM)
├── Add CRYSTALS-Dilithium (Signatures)
├── Add SPHINCS+ (Hash-based)
├── Add FALCON (Lattice-based)
└── Create PQC configuration

Phase 3: Integration
├── Integrate with TLS/SSL
├── Update key exchange protocols
├── Migrate digital signatures
└── Create migration tools

Phase 4: Crypto Agility
├── Build crypto-agility framework
├── Enable rapid algorithm swapping
├── Create automated updates
└── Add performance benchmarks
```

## Future Enhancements

### Planned Features

1. **Binary Analysis**: Scan compiled binaries for cryptographic usage
2. **Dependency Scanning**: Analyze third-party dependencies for quantum vulnerabilities
3. **Configuration Analysis**: Scan configuration files for cryptographic settings
4. **Runtime Detection**: Monitor cryptographic operations at runtime
5. **Migration Assistant**: Automated code refactoring for PQC migration
6. **Performance Impact Analysis**: Estimate performance impact of PQC adoption

### Accuracy Improvements

- AST-based parsing instead of regex
- Machine learning for algorithm detection
- Cross-language support (C, C++, Go, Python)
- False positive reduction

## References

- **NIST PQC Standards**: https://www.nist.gov/news-events/news/2024/08/nist-releases-first-3-finalized-post-quantum-encryption-standards
- **Post-Quantum Cryptography**: https://pq-crystals.org/
- **Deloitte Tech Trends 2025**: Quantum computing and cybersecurity
- **IBM Quantum Safe**: https://www.ibm.com/quantum/quantum-safe

## Contributing

When extending the tool:

1. Add new cryptographic operation patterns
2. Update algorithm classification lists
3. Improve risk assessment algorithms
4. Add test cases for new patterns
5. Update documentation

## License

MIT License - See V-Sentinel project license file