# PQC Migration Guide

This guide provides step-by-step instructions for migrating V-Sentinel from classical cryptography to Post-Quantum Cryptography (PQC).

## Table of Contents

1. [Overview](#overview)
2. [Prerequisites](#prerequisites)
3. [Phase 1: Assessment](#phase-1-assessment)
4. [Phase 2: Infrastructure Preparation](#phase-2-infrastructure-preparation)
5. [Phase 3: Service Migration](#phase-3-service-migration)
6. [Phase 4: Testing and Validation](#phase-4-testing-and-validation)
7. [Phase 5: Production Deployment](#phase-5-production-deployment)
8. [Rollback Procedures](#rollback-procedures)
9. [Best Practices](#best-practices)
10. [Troubleshooting](#troubleshooting)

---

## Overview

### Why Migrate to PQC?

With the advent of quantum computing, classical cryptographic algorithms (RSA, ECC, DH) will become vulnerable. The V-Sentinel platform now supports NIST-standardized PQC algorithms to ensure quantum-resistant security.

### Migration Timeline

| Phase | Duration | Description |
|-------|----------|-------------|
| Assessment | 1-2 weeks | Inventory and risk assessment |
| Infrastructure | 2-4 weeks | Hardware and software preparation |
| Service Migration | 4-6 weeks | Incremental service updates |
| Testing | 2-3 weeks | Comprehensive validation |
| Production | 2-4 weeks | Phased deployment |

### Supported Algorithms

| Algorithm Type | NIST Standard | Recommended for Production |
|----------------|---------------|---------------------------|
| KEM | FIPS 203 (ML-KEM) | CRYSTALS-Kyber-768 |
| Signature | FIPS 204 (ML-DSA) | CRYSTALS-Dilithium-3 |
| Signature | FIPS 205 (SLH-DSA) | SPHINCS+ (stateless) |
| Signature | FIPS 206 (FN-DSA) | FALCON-1024 |

---

## Prerequisites

### System Requirements

- **Operating System**: Linux (kernel 5.x+), Windows Server 2019+, or macOS 12+
- **Memory**: Minimum 4GB RAM (8GB recommended for key generation)
- **Storage**: 1GB for keys and certificates
- **CPU**: Modern multi-core processor with AES-NI support

### Software Dependencies

- Rust 1.70+ (for V-Sentinel core)
- OpenSSL 3.0+ (for classical crypto fallback)
- TPM 2.0 (recommended for key storage)

### Access Requirements

- Administrative access to all V-Sentinel services
- Access to certificate authority (for PQC certificates)
- Monitoring and logging access

---

## Phase 1: Assessment

### Step 1.1: Inventory Current Cryptography

Run the cryptographic inventory tool:

```bash
# Inventory all cryptographic usage
sentinel crypto inventory --output inventory.json
```

This generates a report of:
- TLS certificates in use
- Key exchange algorithms
- Signature algorithms
- Encryption algorithms
- Key storage locations

### Step 1.2: Identify Quantum-Vulnerable Components

```bash
# Identify vulnerable algorithms
sentinel crypto audit --check-quantum-vulnerable
```

Expected output:
```
Quantum-Vulnerable Algorithms Detected:
┌─────────────────┬──────────────┬─────────────────────┐
│ Component       │ Algorithm    │ Location            │
├─────────────────┼──────────────┼─────────────────────┤
│ TLS Server      │ RSA-2048     │ /etc/sentinel/tls/  │
│ API Gateway     │ ECDHE-P256   │ Gateway config      │
│ VPN Service     │ RSA-4096     │ /etc/sentinel/vpn/  │
│ Messaging       │ Ed25519      │ /etc/sentinel/msg/  │
└─────────────────┴──────────────┴─────────────────────┘
```

### Step 1.3: Assess Migration Readiness

```bash
# Run readiness assessment
sentinel pqc assess --readiness
```

Output includes:
- Hardware compatibility check
- Software dependency check
- Key storage readiness
- Network compatibility

---

## Phase 2: Infrastructure Preparation

### Step 2.1: Generate PQC Keys

Generate PQC keypairs for all services:

```bash
# Generate Kyber-768 keypair for key exchange
sentinel pqc keygen --algorithm kyber768 --output /etc/sentinel/keys/kyber768.key

# Generate Dilithium-3 keypair for signatures
sentinel pqc keygen --algorithm dilithium3 --output /etc/sentinel/keys/dilithium3.key
```

### Step 2.2: Create PQC Certificates

Generate certificates using Dilithium:

```bash
# Generate PQC certificate
sentinel pqc cert create \
    --subject "CN=sentinel-gateway,O=VantisCorp,C=US" \
    --signing-key /etc/sentinel/keys/dilithium3.key \
    --output /etc/sentinel/certs/gateway-pqc.crt
```

### Step 2.3: Configure Key Storage

For production, use TPM or HSM:

```toml
# config.toml
[security]
enable_tpm = true
tpm_device_path = "/dev/tpm0"
key_storage = "tpm"
```

### Step 2.4: Update Configuration Files

Update all service configurations:

```toml
# Example PQC configuration
[pqc]
enable_pqc = true
kem_algorithm = "CrystalsKyber768"
signature_algorithm = "CrystalsDilithium3"
hybrid_mode = true
fallback_to_classical = true
min_security_level = 3
```

---

## Phase 3: Service Migration

### Step 3.1: API Gateway Migration

1. **Backup existing configuration:**
   ```bash
   cp /etc/sentinel/gateway/config.toml /etc/sentinel/gateway/config.toml.backup
   ```

2. **Update gateway configuration:**
   ```toml
   # /etc/sentinel/gateway/config.toml
   [pqc]
   enable_pqc = true
   kem_algorithm = "HybridKyber768X25519"
   signature_algorithm = "CrystalsDilithium3"
   hybrid_mode = true
   
   [tls]
   enabled = true
   cert_path = "/etc/sentinel/certs/gateway-pqc.crt"
   key_path = "/etc/sentinel/keys/kyber768.key"
   ```

3. **Restart gateway service:**
   ```bash
   systemctl restart sentinel-gateway
   ```

4. **Verify PQC is active:**
   ```bash
   curl -k https://localhost:8443/api/v1/pqc/status
   ```

### Step 3.2: VPN Service Migration

1. **Generate VPN-specific keys:**
   ```bash
   sentinel pqc keygen --algorithm kyber768 --output /etc/sentinel/vpn/server.key
   sentinel pqc keygen --algorithm dilithium3 --output /etc/sentinel/vpn/signing.key
   ```

2. **Update VPN configuration:**
   ```toml
   # /etc/sentinel/vpn/config.toml
   [pqc]
   enable_pqc = true
   kem_algorithm = "HybridKyber768X25519"
   signature_algorithm = "CrystalsDilithium3"
   rekey_interval_secs = 3600
   enable_pfs = true
   ```

3. **Restart VPN service:**
   ```bash
   systemctl restart sentinel-vpn
   ```

### Step 3.3: Messaging Service Migration

1. **Update messaging configuration:**
   ```toml
   # /etc/sentinel/messaging/config.toml
   [pqc]
   enable_pqc = true
   kem_algorithm = "HybridKyber768X25519"
   signature_algorithm = "CrystalsDilithium3"
   enable_forward_secrecy = true
   message_ttl_secs = 604800
   ```

2. **Restart messaging service:**
   ```bash
   systemctl restart sentinel-messaging
   ```

---

## Phase 4: Testing and Validation

### Step 4.1: Run Integration Tests

```bash
# Run PQC integration tests
cargo test --package sentinel-tests --test pqc_integration_tests
```

### Step 4.2: Run Performance Benchmarks

```bash
# Run PQC benchmarks
cargo bench --package sentinel-tests --bench pqc_benchmarks
```

Expected performance thresholds:
| Operation | Maximum Latency |
|-----------|-----------------|
| Key Generation | < 10ms |
| Key Exchange | < 5ms |
| Signature (Sign) | < 2ms |
| Signature (Verify) | < 1ms |
| TLS Handshake | < 50ms (hybrid) |

### Step 4.3: Validate CNSA 2.0 Compliance

```bash
# Check CNSA 2.0 compliance
sentinel pqc validate --cnsa-2-0
```

---

## Phase 5: Production Deployment

### Step 5.1: Staging Deployment

1. Deploy to staging environment
2. Monitor for 1-2 weeks
3. Collect performance metrics
4. Address any issues

### Step 5.2: Canary Deployment

1. Enable PQC for 10% of traffic:
   ```toml
   [pqc]
   enable_pqc = true
   traffic_percentage = 10
   ```

2. Monitor closely:
   ```bash
   sentinel monitor --pqc-metrics
   ```

3. Gradually increase to 50%, then 100%

### Step 5.3: Full Deployment

Once stable, remove fallback to classical:

```toml
[pqc]
enable_pqc = true
fallback_to_classical = false
hybrid_mode = false  # After full migration
```

---

## Rollback Procedures

### Quick Rollback

```bash
# Disable PQC immediately
sentinel config set pqc.enable_pqc false
sentinel config set pqc.fallback_to_classical true

# Restart all services
systemctl restart sentinel-gateway sentinel-vpn sentinel-messaging
```

### Full Rollback

1. Restore backup configurations:
   ```bash
   cp /etc/sentinel/gateway/config.toml.backup /etc/sentinel/gateway/config.toml
   ```

2. Restart services:
   ```bash
   systemctl restart sentinel-gateway sentinel-vpn sentinel-messaging
   ```

---

## Best Practices

### 1. Always Use Hybrid Mode During Migration

Hybrid mode ensures backward compatibility while providing quantum resistance:

```toml
[pqc]
hybrid_mode = true
fallback_to_classical = true
```

### 2. Implement Key Rotation

```toml
[pqc]
key_rotation_hours = 168  # Weekly rotation
```

### 3. Monitor Performance

Set up alerts for:
- PQC handshake failures
- Increased latency
- Memory usage spikes
- Certificate expiration

### 4. Maintain Backups

- Keep backup of all keys and certificates
- Document key rotation schedules
- Store configurations in version control

### 5. Security Auditing

Regularly audit:
- Key access logs
- Failed authentication attempts
- Certificate validity
- Algorithm usage

---

## Troubleshooting

### Issue: PQC Handshake Fails

**Symptoms:**
- Connection failures
- "Handshake failed" errors

**Solutions:**
1. Verify client supports PQC:
   ```bash
   openssl s_client -connect localhost:8443 -groups kyber768
   ```
2. Check certificate validity
3. Verify key formats

### Issue: Performance Degradation

**Symptoms:**
- Slow connections
- High CPU usage

**Solutions:**
1. Use Kyber-768 instead of Kyber-1024
2. Enable hardware acceleration
3. Increase system resources

### Issue: Certificate Verification Fails

**Symptoms:**
- "Certificate verify failed" errors

**Solutions:**
1. Check certificate chain
2. Verify signature algorithm
3. Ensure CA supports PQC certificates

---

## Checklist

### Pre-Migration
- [ ] Complete cryptographic inventory
- [ ] Identify all quantum-vulnerable components
- [ ] Assess hardware readiness
- [ ] Train team on PQC concepts

### During Migration
- [ ] Generate PQC keypairs
- [ ] Create PQC certificates
- [ ] Update configurations
- [ ] Test each service individually

### Post-Migration
- [ ] Run integration tests
- [ ] Run performance benchmarks
- [ ] Monitor for issues
- [ ] Document changes

### Production
- [ ] Staging deployment successful
- [ ] Canary deployment stable
- [ ] Full deployment complete
- [ ] Monitoring in place
- [ ] Rollback procedures tested

---

## References

- [NIST PQC Standards](https://csrc.nist.gov/projects/post-quantum-cryptography)
- [NSA CNSA 2.0 Fact Sheet](https://media.defense.gov/2022/Sep/07/2003071838/-1/-1/0/CSA_CNSA_2.0_ALGORITHMS_.PDF)
- [PQC Configuration Examples](./pqc_config_examples.md)
- [Phase 3 Integration Plan](./PHASE3_INTEGRATION_PLAN.md)

---

**Document Version**: 1.0  
**Last Updated**: 2025-01-08  
**Status**: Production Ready