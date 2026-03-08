# PQC Configuration Examples

This document provides various PQC (Post-Quantum Cryptography) configuration examples for the V-Sentinel platform.

## Table of Contents

1. [Development Configuration](#development-configuration)
2. [Staging Configuration](#staging-configuration)
3. [Production Configuration](#production-configuration)
4. [CNSA 2.0 Compliant Configuration](#cnsa-20-compliant-configuration)
5. [Hybrid Mode Configuration](#hybrid-mode-configuration)
6. [High-Security Configuration](#high-security-configuration)
7. [Migration Configuration](#migration-configuration)

---

## Development Configuration

**Use Case**: Development and testing environments where PQC is enabled for experimentation.

**Characteristics**:
- PQC enabled with flexible settings
- Lower security levels acceptable for testing
- Fallback to classical enabled
- Hybrid mode enabled

```toml
[network]
# Network settings
listen_address = "127.0.0.1"
listen_port = 8080
enable_tls = true
tls_cert_path = "/etc/sentinel/certs/dev_tls.crt"
tls_key_path = "/etc/sentinel/certs/dev_tls.key"
max_connections = 100
connection_timeout_secs = 30
enable_rate_limiting = false

# PQC settings
enable_pqc = true
pqc_kem_algorithm = "CrystalsKyber512"
pqc_signature_algorithm = "CrystalsDilithium2"
pqc_hybrid_mode = true
pqc_fallback_to_classical = true
pqc_cert_path = "/etc/sentinel/certs/dev_pqc.crt"
pqc_key_path = "/etc/sentinel/certs/dev_pqc.key"
pqc_min_security_level = 1

[quantum]
enabled = true
kem_algorithm = "crystals_kyber512"
signature_algorithm = "crystals_dilithium2"
hybrid_mode = true
key_rotation_hours = 24
```

---

## Staging Configuration

**Use Case**: Staging environment for pre-production testing with production-like settings.

**Characteristics**:
- PQC enabled with medium security levels
- Hybrid mode required
- Fallback enabled for compatibility testing
- Moderate security level (3)

```toml
[network]
# Network settings
listen_address = "0.0.0.0"
listen_port = 8443
enable_tls = true
tls_cert_path = "/etc/sentinel/certs/staging_tls.crt"
tls_key_path = "/etc/sentinel/certs/staging_tls.key"
max_connections = 500
connection_timeout_secs = 30
enable_rate_limiting = true
rate_limit_rpm = 120

# PQC settings
enable_pqc = true
pqc_kem_algorithm = "CrystalsKyber768"
pqc_signature_algorithm = "CrystalsDilithium3"
pqc_hybrid_mode = true
pqc_fallback_to_classical = true
pqc_cert_path = "/etc/sentinel/certs/staging_pqc.crt"
pqc_key_path = "/etc/sentinel/certs/staging_pqc.key"
pqc_min_security_level = 3

[quantum]
enabled = true
kem_algorithm = "crystals_kyber768"
signature_algorithm = "crystals_dilithium3"
hybrid_mode = true
key_rotation_hours = 168
```

---

## Production Configuration

**Use Case**: Production environment with optimized PQC settings for performance and security.

**Characteristics**:
- PQC enabled with high security levels
- Hybrid mode enabled
- Fallback enabled but tightly controlled
- Security level 3 (NIST Level 3)

```toml
[network]
# Network settings
listen_address = "0.0.0.0"
listen_port = 443
enable_tls = true
tls_cert_path = "/etc/sentinel/certs/production_tls.crt"
tls_key_path = "/etc/sentinel/certs/production_tls.key"
max_connections = 10000
connection_timeout_secs = 30
enable_rate_limiting = true
rate_limit_rpm = 60

# PQC settings
enable_pqc = true
pqc_kem_algorithm = "CrystalsKyber768"
pqc_signature_algorithm = "CrystalsDilithium3"
pqc_hybrid_mode = true
pqc_fallback_to_classical = true
pqc_cert_path = "/etc/sentinel/certs/production_pqc.crt"
pqc_key_path = "/etc/sentinel/certs/production_pqc.key"
pqc_min_security_level = 3

[quantum]
enabled = true
kem_algorithm = "crystals_kyber768"
signature_algorithm = "crystals_dilithium3"
hybrid_mode = true
key_rotation_hours = 168

[security]
enable_secure_boot = true
enable_immutable_partition = true
enable_hardware_attestation = true
enable_tpm = true
tpm_device_path = "/dev/tpm0"
enable_secure_enclave = true
audit_logging = true
audit_log_path = "/var/log/sentinel/audit.log"
```

---

## CNSA 2.0 Compliant Configuration

**Use Case**: Systems requiring compliance with NSA CNSA 2.0 (Commercial National Security Algorithm) requirements.

**Characteristics**:
- PQC mandatory
- Only CNSA 2.0 approved algorithms (Kyber-768/1024, Dilithium-3/5)
- Hybrid mode required
- Minimum security level 3

```toml
[network]
# Network settings
listen_address = "0.0.0.0"
listen_port = 443
enable_tls = true
tls_cert_path = "/etc/sentinel/certs/cnsa_tls.crt"
tls_key_path = "/etc/sentinel/certs/cnsa_tls.key"
max_connections = 5000
connection_timeout_secs = 30
enable_rate_limiting = true
rate_limit_rpm = 30

# PQC settings - CNSA 2.0 compliant
enable_pqc = true
pqc_kem_algorithm = "CrystalsKyber768"
pqc_signature_algorithm = "CrystalsDilithium3"
pqc_hybrid_mode = true
pqc_fallback_to_classical = true
pqc_cert_path = "/etc/sentinel/certs/cnsa_pqc.crt"
pqc_key_path = "/etc/sentinel/certs/cnsa_pqc.key"
pqc_min_security_level = 3

[quantum]
enabled = true
kem_algorithm = "crystals_kyber768"
signature_algorithm = "crystals_dilithium3"
hybrid_mode = true
key_rotation_hours = 168

[security]
enable_secure_boot = true
enable_immutable_partition = true
enable_hardware_attestation = true
enable_tpm = true
tpm_device_path = "/dev/tpm0"
enable_secure_enclave = true
audit_logging = true
audit_log_path = "/var/log/sentinel/audit.log"
```

**CNSA 2.0 Approved Algorithms:**

| Category | Algorithm | NIST Level |
|----------|-----------|------------|
| KEM | CRYSTALS-Kyber 768 | Level 3 |
| KEM | CRYSTALS-Kyber 1024 | Level 5 |
| Signature | CRYSTALS-Dilithium 3 | Level 3 |
| Signature | CRYSTALS-Dilithium 5 | Level 5 |

---

## Hybrid Mode Configuration

**Use Case**: Systems requiring both PQC and classical algorithms for maximum compatibility during transition.

**Characteristics**:
- Both PQC and classical algorithms enabled
- Hybrid mode enabled
- Balanced performance and security

```toml
[network]
# Network settings
listen_address = "0.0.0.0"
listen_port = 443
enable_tls = true
tls_cert_path = "/etc/sentinel/certs/hybrid_tls.crt"
tls_key_path = "/etc/sentinel/certs/hybrid_tls.key"
max_connections = 2000
connection_timeout_secs = 30
enable_rate_limiting = true
rate_limit_rpm = 60

# PQC settings
enable_pqc = true
pqc_kem_algorithm = "CrystalsKyber768"
pqc_signature_algorithm = "CrystalsDilithium3"
pqc_hybrid_mode = true
pqc_fallback_to_classical = true
pqc_cert_path = "/etc/sentinel/certs/hybrid_pqc.crt"
pqc_key_path = "/etc/sentinel/certs/hybrid_pqc.key"
pqc_min_security_level = 3

[quantum]
enabled = true
kem_algorithm = "crystals_kyber768"
signature_algorithm = "crystals_dilithium3"
hybrid_mode = true
key_rotation_hours = 168
```

**Hybrid Mode Benefits:**
- ✅ Quantum-resistant security with PQC algorithms
- ✅ Classical algorithm compatibility for older clients
- ✅ Defense in depth with multiple algorithms
- ✅ Graceful migration path

---

## High-Security Configuration

**Use Case**: Critical systems requiring maximum security, accepting performance trade-offs.

**Characteristics**:
- Highest security levels (NIST Level 5)
- Kyber-1024 and Dilithium-5
- Strict key rotation
- Hybrid mode enabled

```toml
[network]
# Network settings
listen_address = "0.0.0.0"
listen_port = 443
enable_tls = true
tls_cert_path = "/etc/sentinel/certs/highsec_tls.crt"
tls_key_path = "/etc/sentinel/certs/highsec_tls.key"
max_connections = 1000
connection_timeout_secs = 30
enable_rate_limiting = true
rate_limit_rpm = 30

# PQC settings - Maximum security
enable_pqc = true
pqc_kem_algorithm = "CrystalsKyber1024"
pqc_signature_algorithm = "CrystalsDilithium5"
pqc_hybrid_mode = true
pqc_fallback_to_classical = true
pqc_cert_path = "/etc/sentinel/certs/highsec_pqc.crt"
pqc_key_path = "/etc/sentinel/certs/highsec_pqc.key"
pqc_min_security_level = 5

[quantum]
enabled = true
kem_algorithm = "crystals_kyber1024"
signature_algorithm = "crystals_dilithium5"
hybrid_mode = true
key_rotation_hours = 72

[security]
enable_secure_boot = true
enable_immutable_partition = true
enable_hardware_attestation = true
enable_tpm = true
tpm_device_path = "/dev/tpm0"
enable_secure_enclave = true
audit_logging = true
audit_log_path = "/var/log/sentinel/audit.log"

[logging]
level = "Info"
format = "Json"
console = false
file = true
file_path = "/var/log/sentinel/sentinel.log"
max_file_size_mb = 100
max_files = 30
structured = true
```

---

## Migration Configuration

**Use Case**: Systems migrating from classical to PQC, requiring controlled transition phases.

**Phase 1 - PQC Enabled (Testing)**
```toml
[network]
enable_pqc = true
pqc_kem_algorithm = "CrystalsKyber768"
pqc_signature_algorithm = "CrystalsDilithium3"
pqc_hybrid_mode = true
pqc_fallback_to_classical = true
pqc_min_security_level = 3
```

**Phase 2 - PQC Preferred**
```toml
[network]
enable_pqc = true
pqc_kem_algorithm = "CrystalsKyber768"
pqc_signature_algorithm = "CrystalsDilithium3"
pqc_hybrid_mode = true
pqc_fallback_to_classical = false  # PQC only, no fallback
pqc_min_security_level = 3
```

**Phase 3 - PQC Only (Full Migration)**
```toml
[network]
enable_pqc = true
pqc_kem_algorithm = "CrystalsKyber768"
pqc_signature_algorithm = "CrystalsDilithium3"
pqc_hybrid_mode = false  # No hybrid mode
pqc_fallback_to_classical = false
pqc_min_security_level = 3
```

---

## PQC Algorithm Selection Guide

### NIST Security Levels

| Level | Security Strength | Recommended Use |
|-------|------------------|-----------------|
| Level 1 | ~128 bits | Development, non-critical systems |
| Level 3 | ~192 bits | Production, enterprise systems |
| Level 5 | ~256 bits | High-security, critical infrastructure |

### KEM Algorithm Comparison

| Algorithm | Security Level | Key Size | Performance | Use Case |
|-----------|---------------|----------|-------------|----------|
| Kyber-512 | Level 1 | 800 bytes | Fast | Development |
| Kyber-768 | Level 3 | 1184 bytes | Medium | Production (Recommended) |
| Kyber-1024 | Level 5 | 1568 bytes | Slow | High-security |

### Signature Algorithm Comparison

| Algorithm | Security Level | Signature Size | Performance | Use Case |
|-----------|---------------|----------------|-------------|----------|
| Dilithium-2 | Level 1 | ~1312 bytes | Fast | Development |
| Dilithium-3 | Level 3 | ~1952 bytes | Medium | Production (Recommended) |
| Dilithium-5 | Level 5 | ~2592 bytes | Medium | High-security |
| FALCON-512 | Level 1 | 666 bytes | Medium | Size-constrained |
| FALCON-1024 | Level 5 | 1280 bytes | Medium | High-security, size-sensitive |
| SPHINCS+ SHA2-128f | Level 1 | 7856 bytes | Slow | Stateless signatures |

---

## Configuration Validation

### Using the PQC Config Validator

```rust
use sentinel_config::{ConfigManager, pqc_validator::PqcConfigValidator};

// Create config manager
let config_manager = ConfigManager::new()?;

// Add PQC validator with production settings
let validator = PqcConfigValidator::production();
config_manager.add_validator(Box::new(validator)).await;

// Load configuration (will be validated)
config_manager.load_from_file("config.toml").await?;
```

### CNSA 2.0 Compliance Check

```rust
use sentinel_config::pqc_validator::PqcConfigValidator;

// Create CNSA 2.0 compliant validator
let validator = PqcConfigValidator::cnsa_2_0();

// Validate configuration
let result = validator.validate_pqc_config(
    true,  // enable_pqc
    Some("CrystalsKyber768"),  // kem_algorithm
    Some("CrystalsDilithium3"),  // signature_algorithm
    true,  // hybrid_mode
    true,  // fallback_to_classical
    3,  // min_security_level
)?;

if result.is_valid {
    println!("✅ Configuration is CNSA 2.0 compliant");
}
```

---

## Security Best Practices

### 1. Algorithm Selection
- **For Production**: Use Kyber-768 and Dilithium-3 (Level 3)
- **For High-Security**: Use Kyber-1024 and Dilithium-5 (Level 5)
- **For CNSA 2.0**: Use only approved algorithms

### 2. Hybrid Mode
- ✅ **Enable** during migration phase
- ✅ **Enable** for maximum compatibility
- ⚠️ **Disable** only after full PQC migration

### 3. Key Rotation
- **Kyber Keys**: Rotate every 7 days (168 hours)
- **Dilithium Keys**: Rotate every 30 days
- **High-Security**: Rotate every 72 hours

### 4. Certificate Management
- Use separate certificates for TLS and PQC
- Store certificates in secure locations
- Use TPM/HSM for private key storage
- Enable certificate rotation

### 5. Fallback Configuration
- **Development**: Enable fallback
- **Staging**: Enable fallback with monitoring
- **Production**: Enable fallback but monitor closely
- **Full PQC**: Disable fallback after migration

---

## Troubleshooting

### Issue: PQC Handshake Fails

**Possible Causes:**
1. Client doesn't support PQC algorithms
2. Hybrid mode disabled but fallback required
3. Algorithm not in allowed list

**Solutions:**
1. Enable `pqc_fallback_to_classical`
2. Enable `pqc_hybrid_mode`
3. Check algorithm compatibility

### Issue: Performance Degradation

**Possible Causes:**
1. Using Level 5 algorithms (Kyber-1024, Dilithium-5)
2. Too many concurrent PQC handshakes
3. No hardware acceleration

**Solutions:**
1. Use Level 3 algorithms (Kyber-768, Dilithium-3)
2. Increase connection timeout
3. Enable SIMD acceleration

### Issue: Configuration Validation Fails

**Possible Causes:**
1. Invalid algorithm name
2. Security level below minimum
3. Hybrid mode not enabled when required

**Solutions:**
1. Use valid algorithm names (case-insensitive)
2. Ensure security level >= minimum
3. Enable hybrid mode if required by validator

---

## References

- [NIST Post-Quantum Cryptography Standards](https://csrc.nist.gov/projects/post-quantum-cryptography)
- [NSA CNSA 2.0 Fact Sheet](https://media.defense.gov/2022/Sep/07/2003071838/-1/-1/0/CSA_CNSA_2.0_ALGORITHMS_.PDF)
- [IETF PQC TLS Drafts](https://datatracker.ietf.org/wg/tls/about/)
- [V-Sentinel PQC Implementation](./PHASE3_INTEGRATION_PLAN.md)

---

**Document Version**: 1.0
**Last Updated**: 2025-01-08
**Status**: Production Ready