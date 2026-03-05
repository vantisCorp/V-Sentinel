# Sentinel Secure Messaging Service

A production-ready messaging service with integrated Post-Quantum Cryptography (PQC) support for the V-Sentinel platform.

## Features

- ✅ Post-Quantum Cryptography (PQC) with NIST-standardized algorithms
- ✅ End-to-end encryption with PQC
- ✅ CRYSTALS-Kyber for key exchange
- ✅ CRYSTALS-Dilithium/FALCON/SPHINCS+ for signatures
- ✅ Forward secrecy
- ✅ Message signing and verification
- ✅ Conversation management
- ✅ User key management

## Configuration

```toml
[pqc]
enable_pqc = true
kem_algorithm = "HybridKyber768X25519"
signature_algorithm = "CrystalsDilithium3"
hybrid_mode = true
min_security_level = 3
message_ttl_secs = 604800
enable_forward_secrecy = true
max_message_size = 10485760

[storage]
backend = "Memory"
```

## API

### Register User
```rust
messaging.register_user("user-1", public_key).await?;
```

### Create Conversation
```rust
messaging.create_conversation(vec!["user-1".to_string(), "user-2".to_string()]).await?;
```

### Send Message
```rust
messaging.send_message(
    "conv-1",
    "user-1",
    message_content,
    MessagePriority::Normal
).await?;
```

## Building

```bash
cd src/services/messaging
cargo build --release
```

## Testing

```bash
cargo test
```

## Documentation

- [PQC Configuration Examples](../../../docs/pqc_config_examples.md)
- [Phase 3 Integration Plan](../../../docs/PHASE3_INTEGRATION_PLAN.md)