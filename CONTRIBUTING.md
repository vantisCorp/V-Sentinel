# Contributing to V-Sentinel

Thank you for your interest in contributing to V-Sentinel! This document provides guidelines and instructions for contributing to the project.

## Table of Contents
- [Code of Conduct](#code-of-conduct)
- [Getting Started](#getting-started)
- [Development Workflow](#development-workflow)
- [Coding Standards](#coding-standards)
- [Testing](#testing)
- [Documentation](#documentation)
- [Pull Request Process](#pull-request-process)
- [Reporting Issues](#reporting-issues)

## Code of Conduct

We are committed to providing a welcoming and inclusive environment for all contributors. Please be respectful and constructive in all interactions.

## Getting Started

### Prerequisites

- Rust 1.70+ (latest stable recommended)
- Git
- Docker (optional, for containerized testing)

### Initial Setup

```bash
# Fork the repository
fork and clone your fork

# Add upstream remote
git remote add upstream https://github.com/vantisCorp/V-Sentinel.git

# Install development dependencies
cargo install cargo-audit cargo-tarpaulin cargo-deny cargo-outdated

# Run tests to verify setup
cargo test --all
```

## Development Workflow

1. **Create a branch** for your changes:
   ```bash
   git checkout -b feature/your-feature-name
   ```

2. **Make your changes** following the coding standards.

3. **Test your changes**:
   ```bash
   # Run all tests
   cargo test --all
   
   # Run integration tests
   cargo test --test integration_tests
   
   # Check formatting
   cargo fmt -- --check
   
   # Run linter
   cargo clippy --all-targets --all-features -- -D warnings
   ```

4. **Commit your changes** with clear messages:
   ```bash
   git commit -m "feat: add quantum-resistant encryption support"
   ```

5. **Push to your fork**:
   ```bash
   git push origin feature/your-feature-name
   ```

6. **Create a Pull Request** to the main repository.

## Coding Standards

### Rust Conventions

Follow the official [Rust Style Guide](https://rust-lang.github.io/api-guidelines/) and [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/).

### Code Formatting

All code must be formatted using `rustfmt`:
```bash
cargo fmt
```

### Linting

All code must pass `clippy` with no warnings:
```bash
cargo clippy --all-targets --all-features -- -D warnings
```

### Naming Conventions

- **Modules**: `snake_case`
- **Structs/Enums**: `PascalCase`
- **Functions/Methods**: `snake_case`
- **Constants**: `SCREAMING_SNAKE_CASE`
- **Type Parameters**: `PascalCase` (e.g., `T`, `U`, `K`)

### Documentation

All public items must have documentation comments:
```rust
/// Encrypts data using the specified algorithm.
///
/// # Arguments
/// * `data` - The data to encrypt
/// * `algorithm` - The encryption algorithm to use
///
/// # Returns
/// Encrypted data as a vector of bytes
///
/// # Errors
/// Returns an error if encryption fails
///
/// # Example
/// ```
/// let encrypted = encrypt(data, Algorithm::AES256)?;
/// ```
pub async fn encrypt(data: &[u8], algorithm: Algorithm) -> Result<Vec<u8>, Error> {
    // implementation
}
```

### Error Handling

Use the `Result` type for error handling:
```rust
pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Encryption failed: {0}")]
    EncryptionError(String),
    
    #[error("Invalid key")]
    InvalidKey,
    
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
}
```

### Async/Await

Use `async`/`await` for I/O operations and long-running tasks:
```rust
pub async fn process_data(data: &[u8]) -> Result<Vec<u8>> {
    // async implementation
}
```

## Testing

### Unit Tests

Write unit tests for all public functions:
```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_encryption() {
        let data = b"test data";
        let encrypted = encrypt(data, Algorithm::AES256).await.unwrap();
        assert!(!encrypted.is_empty());
    }
}
```

### Integration Tests

Add integration tests in the `tests/` directory:
```rust
// tests/integration_tests.rs

#[tokio::test]
async fn test_module_integration() {
    // integration test
}
```

### Test Coverage

Maintain test coverage above 80%:
```bash
# Run coverage
cargo tarpaulin --out Html --output-dir coverage

# View coverage report
open coverage/index.html
```

### Benchmarks

Add benchmarks for performance-critical code:
```rust
#[bench]
fn bench_encryption(b: &mut test::Bencher) {
    let data = vec![0u8; 1024];
    b.iter(|| encrypt(&data, Algorithm::AES256));
}
```

## Documentation

### Inline Documentation

Document all public APIs with clear examples:
```rust
/// Example usage of the encryption module
///
/// ```
/// use sentinel::crypto::encrypt;
/// let encrypted = encrypt(b"data", Algorithm::AES256)?;
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
```

### Module Documentation

Provide overview documentation at the module level:
```rust
//! # Quantum Cryptography Module
//!
//! This module provides post-quantum cryptographic algorithms
//! resistant to quantum computer attacks.
//!
//! ## Features
//! - Crystals-Kyber key encapsulation
//! - Crystals-Dilithium digital signatures
//! - Hybrid encryption modes
//!
//! ## Usage
//! ```
//! use sentinel_quantum::QuantumManager;
//! let manager = QuantumManager::new(config)?;
//! let keypair = manager.generate_keypair(KeyType::Kyber1024).await?;
//! ```
```

### README Updates

Update the README when adding new features or changing APIs.

## Pull Request Process

### Before Creating a PR

1. **Check for existing PRs**: Search for similar changes
2. **Update documentation**: Ensure docs are up to date
3. **Run all tests**: Ensure all tests pass
4. **Check formatting**: Run `cargo fmt`
5. **Run linter**: Run `cargo clippy`

### PR Description Format

Use the following format for PR descriptions:

```markdown
## Description
Brief description of what this PR does and why.

## Changes
- Added feature X
- Fixed bug Y
- Updated documentation for Z

## Testing
- Unit tests: ✅
- Integration tests: ✅
- Manual testing: ✅

## Checklist
- [x] Code follows project style guidelines
- [x] Tests added/updated
- [x] Documentation updated
- [x] All tests passing
- [x] No clippy warnings
```

### PR Review Process

1. **Automated checks**: CI/CD pipeline must pass
2. **Code review**: At least one maintainer approval required
3. **Testing**: Manual testing if needed
4. **Merge**: After all approvals and checks pass

## Reporting Issues

### Bug Reports

Use the bug report template:

```markdown
## Description
Clear description of the bug

## Steps to Reproduce
1. Step one
2. Step two
3. Step three

## Expected Behavior
What should happen

## Actual Behavior
What actually happens

## Environment
- OS: 
- Rust version: 
- V-Sentinel version: 

## Logs/Error Messages
```

### Feature Requests

Use the feature request template:

```markdown
## Problem Statement
What problem does this feature solve?

## Proposed Solution
How should this feature work?

## Alternatives
What alternatives have you considered?

## Additional Context
Any other relevant information
```

### Security Issues

For security vulnerabilities, please email **security@vsentinel.com** instead of using public issues.

## Recognition

Contributors are recognized in the project's CONTRIBUTORS.md file. All contributions are appreciated!

## Questions?

- Join our [Discord](https://discord.gg/vsentinel)
- Open a [discussion](https://github.com/vantisCorp/V-Sentinel/discussions)
- Email us at **contributions@vsentinel.com**

## License

By contributing to V-Sentinel, you agree that your contributions will be licensed under the MIT License.