# Sentinel VPN Service

A production-ready VPN service with integrated Post-Quantum Cryptography (PQC) support for the V-Sentinel platform.

## Features

- ✅ Post-Quantum Cryptography (PQC) with NIST-standardized algorithms
- ✅ Hybrid VPN mode (PQC + classical) for compatibility
- ✅ CRYSTALS-Kyber for key exchange
- ✅ CRYSTALS-Dilithium/FALCON for digital signatures
- ✅ Perfect forward secrecy (PFS)
- ✅ Automatic rekeying
- ✅ Tunnel management
- ✅ Client authentication

## Configuration

```toml
[server]
address = "0.0.0.0:1194"
network_cidr = "10.8.0.0/24"

[pqc]
enable_pqc = true
kem_algorithm = "HybridKyber768X25519"
signature_algorithm = "CrystalsDilithium3"
hybrid_mode = true
fallback_to_classical = true
min_security_level = 3
rekey_interval_secs = 3600
mtu = 1500
enable_pfs = true
compression = true
```

## Building

```bash
cd src/services/vpn
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