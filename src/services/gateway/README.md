# Sentinel API Gateway

A production-ready API Gateway with integrated Post-Quantum Cryptography (PQC) support for the V-Sentinel platform.

## Features

- ✅ Post-Quantum Cryptography (PQC) with NIST-standardized algorithms
- ✅ Hybrid mode (PQC + classical) for compatibility
- ✅ CRYSTALS-Kyber for key exchange
- ✅ CRYSTALS-Dilithium/FALCON for digital signatures
- ✅ TLS 1.3 with PQC cipher suites
- ✅ Quantum-resistant session management
- ✅ Post-compromise security (PCS)
- ✅ Automatic key rotation
- ✅ Health checks and metrics
- ✅ Rate limiting and CORS support

## Supported PQC Algorithms

### KEM Algorithms (Key Exchange)
- CRYSTALS-Kyber-512 (NIST Level 1)
- CRYSTALS-Kyber-768 (NIST Level 3) - Recommended for production
- CRYSTALS-Kyber-1024 (NIST Level 5)
- Hybrid modes: Kyber + X25519/X448

### Signature Algorithms (Authentication)
- CRYSTALS-Dilithium-2 (NIST Level 1)
- CRYSTALS-Dilithium-3 (NIST Level 3) - Recommended for production
- CRYSTALS-Dilithium-5 (NIST Level 5)
- FALCON-512 (NIST Level 1)
- FALCON-1024 (NIST Level 5)
- SPHINCS+ variants (Stateless signatures)
- Hybrid modes: Dilithium + ECDSA

## Configuration

Example configuration:

```toml
[general]
name = "sentinel-gateway"
address = "0.0.0.0"
port = 8080
environment = "Production"

[pqc]
enable_pqc = true
kem_algorithm = "HybridKyber768X25519"
signature_algorithm = "CrystalsDilithium3"
hybrid_mode = true
fallback_to_classical = true
min_security_level = 3
key_rotation_hours = 168
session_timeout_secs = 3600
enable_pcs = true

[tls]
enabled = true
cert_path = "/etc/sentinel/certs/gateway.crt"
key_path = "/etc/sentinel/certs/gateway.key"
min_tls_version = "1.3"

[rate_limiting]
enabled = true
requests_per_second = 60
burst_size = 100

[metrics]
enabled = true
endpoint = "/metrics"
include_pqc_metrics = true
```

## API Endpoints

### Health Check
```
GET /health
```

### PQC Status
```
GET /api/v1/pqc/status
```

### Initiate PQC Handshake
```
POST /api/v1/pqc/handshake
Content-Type: application/json

{
  "client_id": "client-123",
  "client_public_key": "...",
  "kem_ciphertext": "..."
}
```

### Metrics
```
GET /metrics
```

## Building

```bash
cd src/services/gateway
cargo build --release
```

## Running

```bash
cargo run -- --config config.toml
```

## Testing

```bash
cargo test
```

## Documentation

- [PQC Configuration Examples](../../../docs/pqc_config_examples.md)
- [Phase 3 Integration Plan](../../../docs/PHASE3_INTEGRATION_PLAN.md)

## License

Proprietary - VantisCorp