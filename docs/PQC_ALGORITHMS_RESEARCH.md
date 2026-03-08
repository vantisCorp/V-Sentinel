# PQC Algorithm Research & Implementation Plan

## Overview

This document details the research and implementation plan for adding real post-quantum cryptography (PQC) algorithms to V-Sentinel, replacing the placeholder implementations.

## Identified Rust PQC Crates

### 1. CRYSTALS-Kyber (KEM)

**Crate**: `pqc_kyber` v0.7.1
**Repository**: https://github.com/Argyle-Software/kyber
**Documentation**: https://docs.rs/pqc_kyber

**Features**:
- Pure Rust implementation (no unsafe code)
- no_std compatible (suitable for embedded devices)
- AVX2 optimized version for x86_64
- WASM support
- Multiple security levels: kyber512, kyber768 (default), kyber1024
- 90s mode (AES256 + SHA2) for hardware acceleration
- Unilateral and Mutually Authenticated Key Exchange
- Zeroization support

**Installation**:
```toml
pqc_kyber = "0.7"
```

**Usage Example**:
```rust
use pqc_kyber::*;

// Generate Keypair
let keys = keypair(&mut rng)?;

// Encapsulate shared secret
let (ciphertext, shared_secret_alice) = encapsulate(&keys.public, &mut rng)?;

// Decapsulate shared secret
let shared_secret_bob = decapsulate(&ciphertext, &keys.secret)?;
```

**Security Levels**:
- Kyber512 ~ AES-128 (NIST Security Level 1)
- Kyber768 ~ AES-192 (NIST Security Level 3) - Recommended
- Kyber1024 ~ AES-256 (NIST Security Level 5)

### 2. CRYSTALS-Dilithium (Signatures)

**Crate**: `pqcrypto-dilithium` v0.5.0
**Repository**: https://github.com/rustpq/pqcrypto
**Documentation**: https://docs.rs/pqcrypto-dilithium

**Features**:
- Bindings to PQClean C implementations
- Multiple security levels: dilithium2, dilithium3, dilithium5
- Optimized implementations: clean, AVX2, AArch64
- Serde serialization support
- no_std compatible

**Installation**:
```toml
pqcrypto-dilithium = "0.5"
```

**Security Levels**:
- Dilithium2 ~ AES-128 (NIST Security Level 2)
- Dilithium3 ~ AES-192 (NIST Security Level 3) - Recommended
- Dilithium5 ~ AES-256 (NIST Security Level 5)

### 3. SPHINCS+ (Hash-Based Signatures)

**Crate**: `pqcrypto-sphincsplus` v0.5.0
**Repository**: https://github.com/rustpq/pqcrypto
**Documentation**: https://docs.rs/pqcrypto-sphincsplus

**Features**:
- Stateful hash-based signatures
- Multiple parameter sets
- PQClean implementations
- Serde serialization support

**Installation**:
```toml
pqcrypto-sphincsplus = "0.5"
```

**Note**: SPHINCS+ has larger signature sizes but provides conservative security as a backup signature scheme.

### 4. FALCON (Lattice-Based Signatures)

**Crate**: `pqcrypto-falcon` v0.5.0
**Repository**: https://github.com/rustpq/pqcrypto
**Documentation**: https://docs.rs/pqcrypto-falcon

**Features**:
- Lattice-based signatures
- Small signature sizes
- Multiple security levels
- Optimized implementations

**Installation**:
```toml
pqcrypto-falcon = "0.5"
```

## Implementation Plan

### Phase 2.1: Update Dependencies ✅

**Completed**:
- [x] Added pqc_kyber = "0.7"
- [x] Added pqcrypto-dilithium = "0.5"
- [x] Added pqcrypto-falcon = "0.5"
- [x] Added pqcrypto-sphincsplus = "0.5"

### Phase 2.2: Implement Real PQC Algorithms

#### Task 1: Replace CRYSTALS-Kyber Placeholder

**File**: `src/quantum/src/lib.rs`

**Current**: Placeholder implementation in `CrystalsKyber` struct

**Changes Required**:
1. Import pqc_kyber crate
2. Replace keypair() method to use `pqc_kyber::keypair()`
3. Replace encapsulate() method to use `pqc_kyber::encapsulate()`
4. Replace decapsulate() method to use `pqc_kyber::decapsulate()`
5. Update key_size() and ciphertext_size() with actual values
6. Add error handling for pqc_kyber::KyberError

#### Task 2: Replace CRYSTALS-Dilithium Placeholder

**File**: `src/quantum/src/lib.rs`

**Current**: Placeholder implementation in `CrystalsDilithium` struct

**Changes Required**:
1. Import pqcrypto_dilithium crate
2. Replace keypair() method to use real Dilithium key generation
3. Replace sign() method to use real Dilithium signing
4. Replace verify() method to use real Dilithium verification
5. Update key_size() and signature_size() with actual values
6. Add error handling for Dilithium-specific errors

#### Task 3: Add SPHINCS+ Implementation

**File**: `src/quantum/src/lib.rs`

**New Structure**:
```rust
pub struct SphincsPlus {
    algorithm: SigAlgorithm,
}

impl Signature for SphincsPlus {
    fn keypair(&self) -> Result<(Keypair, SigAlgorithm)>;
    fn sign(&self, private_key: &[u8], message: &[u8]) -> Result<Vec<u8>>;
    fn verify(&self, public_key: &[u8], message: &[u8], signature: &[u8]) -> Result<bool>;
    fn algorithm(&self) -> SigAlgorithm;
}
```

#### Task 4: Add FALCON Implementation

**File**: `src/quantum/src/lib.rs`

**New Structure**:
```rust
pub struct Falcon {
    algorithm: SigAlgorithm,
}

impl Signature for FALCON {
    fn keypair(&self) -> Result<(Keypair, SigAlgorithm)>;
    fn sign(&self, private_key: &[u8], message: &[u8]) -> Result<Vec<u8>>;
    fn verify(&self, public_key: &[u8], message: &[u8], signature: &[u8]) -> Result<bool>;
    fn algorithm(&self) -> SigAlgorithm;
}
```

### Phase 2.3: PQC Configuration System

Create a configuration system to allow runtime algorithm selection:

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PQCConfig {
    pub kem_algorithm: KemAlgorithm,
    pub signature_algorithm: SigAlgorithm,
    pub use_hybrid: bool,
    pub enable_avx2: bool,
}

impl Default for PQCConfig {
    fn default() -> Self {
        Self {
            kem_algorithm: KemAlgorithm::CrystalsKyber768,
            signature_algorithm: SigAlgorithm::CrystalsDilithium3,
            use_hybrid: true,
            enable_avx2: true,
        }
    }
}
```

### Phase 2.4: Update QuantumCryptoManager

**Changes**:
1. Add PQCConfig support
2. Initialize algorithms based on configuration
3. Add algorithm switching at runtime
4. Update statistics tracking

### Phase 2.5: Testing

**Unit Tests**:
- Test all PQC algorithm implementations
- Test key generation, encapsulation/decapsulation, signing/verification
- Test hybrid crypto operations
- Test algorithm switching

**Integration Tests**:
- Test with real use cases
- Test performance benchmarks
- Test interoperability

## Algorithm Comparison

| Algorithm | Type | Security Level | Key Size | Signature/CT Size | Notes |
|-----------|------|---------------|----------|-------------------|-------|
| Kyber512 | KEM | NIST-1 (AES-128) | 800 | 768 | Faster, lower security |
| Kyber768 | KEM | NIST-3 (AES-192) | 1184 | 1088 | **Recommended** |
| Kyber1024 | KEM | NIST-5 (AES-256) | 1568 | 1568 | Highest security |
| Dilithium2 | Signature | NIST-2 (AES-128) | 1312 | 2420 | Faster signatures |
| Dilithium3 | Signature | NIST-3 (AES-192) | 1952 | 3293 | **Recommended** |
| Dilithium5 | Signature | NIST-5 (AES-256) | 2592 | 4595 | Highest security |
| SPHINCS+ | Signature | NIST-3/5 | Varies | ~10K-50K | Conservative, large sigs |
| FALCON | Signature | NIST-1/5 | Varies | ~700-1500 | Small signatures |

## Security Considerations

### Hybrid Approach

**Recommendation**: Use hybrid crypto combining classical and PQC algorithms

**Rationale**:
1. Defense in depth: Even if PQC is broken, classical crypto remains
2. Smooth transition: Gradual migration without breaking compatibility
3. Risk mitigation: Reduces risk of PQC vulnerabilities

**Implementation**:
```rust
// Hybrid KEM: X25519 + Kyber768
// Hybrid Signature: Ed25519 + Dilithium3
```

### Performance Impact

Expected performance characteristics:
- **Key Generation**: PQC slower than classical (2-10x)
- **Encapsulation**: Kyber comparable to X25519
- **Decapsulation**: Kyber comparable to X25519
- **Signing**: Dilithium slower than Ed25519 (2-5x)
- **Verification**: Dilithium comparable to Ed25519

### Side-Channel Resistance

- pqc_kyber: Pure Rust, no unsafe code
- pqcrypto crates: Bindings to PQClean, includes side-channel resistant implementations
- Enable AVX2 optimizations where available
- Use zeroization features

## Migration Path

### Phase 1: Discovery ✅
- Cryptographic inventory tool
- Vulnerability assessment

### Phase 2: Implementation (Current)
- Add real PQC algorithms
- Replace placeholders
- Create configuration system

### Phase 3: Integration
- Integrate with TLS/SSL
- Update key exchange protocols
- Migrate digital signatures

### Phase 4: Crypto Agility
- Runtime algorithm switching
- Automated migration tools
- Performance benchmarks

## References

- **NIST PQC Standards**: https://www.nist.gov/news-events/news/2024/08/nist-releases-first-3-finalized-post-quantum-encryption-standards
- **PQ-Crystals**: https://pq-crystals.org/
- **PQClean**: https://github.com/PQClean/PQClean/
- **pqc_kyber**: https://github.com/Argyle-Software/kyber
- **pqcrypto**: https://github.com/rustpq/pqcrypto

## Conclusion

The identified Rust crates provide production-ready implementations of all NIST-standardized PQC algorithms. By replacing placeholder implementations with these real algorithms, V-Sentinel will achieve genuine post-quantum cryptography capabilities.

**Recommended Defaults**:
- KEM: CRYSTALS-Kyber768
- Signature: CRYSTALS-Dilithium3
- Hybrid: Yes (Kyber768 + X25519, Dilithium3 + Ed25519)

**Next Steps**:
1. Implement real PQC algorithms in quantum module
2. Add comprehensive testing
3. Create performance benchmarks
4. Document migration guide