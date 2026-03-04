# V-Sentinel Developer Cheat Sheet

## Quick Commands

### Building
```bash
# Build project
cargo build

# Build with optimizations
cargo build --release

# Build documentation
cargo doc --open

# Clean build artifacts
cargo clean
```

### Testing
```bash
# Run all tests
cargo test

# Run specific test
cargo test test_name

# Run tests with output
cargo test -- --nocapture

# Run ignored tests
cargo test -- --ignored
```

### Development
```bash
# Check formatting
cargo fmt -- --check

# Format code
cargo fmt

# Run linter
cargo clippy

# Run clippy with fixes
cargo clippy --fix
```

### Running
```bash
# Run application
cargo run

# Run with release optimizations
cargo run --release

# Run with specific log level
RUST_LOG=debug cargo run
```

---

## Module Quick Reference

### Privacy Module
```rust
use sentinel_privacy::{PrivacyEngine, ZkpBackend};

// Create privacy engine
let privacy = PrivacyEngine::new(ZkpBackend::Bulletproofs);

// Generate zero-knowledge proof
let proof = privacy.generate_zkp(
    "age >= 18",
    &json!({"age": 25}),
).await?;

// Verify proof
let verified = privacy.verify_zkp(&proof, &public_inputs).await?;
```

### Quantum Module
```rust
use sentinel_quantum::{QuantumEngine, Algorithm};

// Create quantum engine
let quantum = QuantumEngine::new(Algorithm::CrystalsKyber);

// Encrypt data
let ciphertext = quantum.encrypt("secret message").await?;

// Decrypt data
let plaintext = quantum.decrypt(&ciphertext).await?;
```

### Biometrics Module
```rust
use sentinel_biometrics::{BiometricEngine, BiometricType};

// Create biometric engine
let bio = BiometricEngine::new();

// Authenticate user
let result = bio.authenticate(
    &user_id,
    BiometricType::MultiModal {
        fingerprint: fingerprint_data,
        facial: facial_data,
        voice: voice_data,
    },
).await?;
```

### Neural Module
```rust
use sentinel_neural::{NeuralEngine, ModelType};

// Create neural engine
let neural = NeuralEngine::new(ModelType::Transformer);

// Detect threats
let threats = neural.detect_threats(&input_data).await?;

// Train model
neural.train_model(&training_data).await?;
```

### Autonomous Module
```rust
use sentinel_autonomous::{AutonomousEngine, AgentConfig};

// Create autonomous engine
let autonomous = AutonomousEngine::new();

// Deploy agent
let agent = autonomous.deploy_agent(AgentConfig {
    agent_type: AgentType::ThreatResponder,
    autonomous_level: AutonomousLevel::High,
}).await?;
```

---

## API Endpoints

### Authentication
```bash
# Get token
curl -X POST https://api.v-sentinel.io/v1/auth/token \
  -H "Content-Type: application/json" \
  -d '{"client_id":"id","client_secret":"secret"}'

# Health check
curl -H "Authorization: Bearer $TOKEN" \
  https://api.v-sentinel.io/v1/health
```

### Privacy
```bash
# Generate ZKP
curl -X POST https://api.v-sentinel.io/v1/privacy/zkp/generate \
  -H "Authorization: Bearer $TOKEN" \
  -H "Content-Type: application/json" \
  -d '{"statement":"age >= 18","witness":{"age":25}}'

# Apply differential privacy
curl -X POST https://api.v-sentinel.io/v1/privacy/dp/apply \
  -H "Authorization: Bearer $TOKEN" \
  -H "Content-Type: application/json" \
  -d '{"data":[1,2,3],"epsilon":1.0}'
```

### Quantum
```bash
# Encrypt
curl -X POST https://api.v-sentinel.io/v1/quantum/encrypt \
  -H "Authorization: Bearer $TOKEN" \
  -H "Content-Type: application/json" \
  -d '{"plaintext":"secret","algorithm":"crystals-kyber"}'
```

---

## Docker Commands

```bash
# Build image
docker build -t vsentinel:latest .

# Run container
docker run -d -p 8080:8080 --name vsentinel vsentinel:latest

# View logs
docker logs -f vsentinel

# Execute command in container
docker exec -it vsentinel bash

# Stop container
docker stop vsentinel

# Remove container
docker rm vsentinel
```

---

## Kubernetes Commands

```bash
# Apply manifest
kubectl apply -f deployment.yaml

# Get pods
kubectl get pods -n vsentinel

# Get logs
kubectl logs -f vsentinel-xxx -n vsentinel

# Port forward
kubectl port-forward svc/vsentinel 8080:80 -n vsentinel

# Scale deployment
kubectl scale deployment vsentinel --replicas=5 -n vsentinel

# Get events
kubectl get events -n vsentinel
```

---

## Database Commands

```bash
# Connect to PostgreSQL
psql -U vsentinel -d vsentinel

# Backup database
pg_dump -U vsentinel vsentinel > backup.sql

# Restore database
psql -U vsentinel vsentinel < backup.sql

# Check connections
psql -U vsentinel -d vsentinel -c "SELECT count(*) FROM pg_stat_activity;"
```

---

## Redis Commands

```bash
# Connect to Redis
redis-cli -a password

# Monitor commands
redis-cli -a password MONITOR

# Get all keys
redis-cli -a password KEYS "*"

# Flush database
redis-cli -a password FLUSHDB
```

---

## Monitoring Commands

```bash
# Check Prometheus metrics
curl http://localhost:9090/metrics

# Query Prometheus
curl -G 'http://localhost:9090/api/v1/query' \
  --data-urlencode 'query=sentinel_requests_total'

# Check health
curl http://localhost:8080/health

# Check readiness
curl http://localhost:8080/ready
```

---

## Git Commands

```bash
# Commit changes
git add .
git commit -m "description"

# Push to remote
git push origin main

# Pull latest changes
git pull origin main

# Create branch
git checkout -b feature-name

# Merge branch
git merge feature-name

# View log
git log --oneline --graph
```

---

## Configuration File Locations

| File | Location |
|------|----------|
| Main config | `/etc/vsentinel/sentinel.toml` |
| Logs | `/var/log/vsentinel/` |
| Data | `/var/lib/vsentinel/` |
| Models | `/opt/vsentinel/models/` |

---

## Environment Variables

```bash
# Set log level
export RUST_LOG=debug

# Set API key
export SENTINEL_API_KEY=your-key

# Set database URL
export DATABASE_URL=postgresql://...

# Set Redis URL
export REDIS_URL=redis://...
```

---

## Common Error Codes

| Code | Description | Solution |
|------|-------------|----------|
| AUTH_001 | Invalid API key | Check token validity |
| AUTH_002 | Token expired | Refresh token |
| REQ_001 | Invalid request | Check request format |
| SRV_001 | Internal error | Check logs |
| SEC_001 | Security violation | Review security policy |

---

## Performance Tips

1. **Enable release mode**: `cargo build --release`
2. **Use connection pooling**: Configure database pool size
3. **Enable caching**: Configure Redis for frequently accessed data
4. **Use async/await**: Leverage Rust's async capabilities
5. **Profile regularly**: Use `cargo flamegraph` for optimization

---

## Security Best Practices

1. **Always use HTTPS** in production
2. **Rotate API keys** regularly
3. **Enable audit logging**
4. **Use strong passwords** for database
5. **Keep dependencies** updated
6. **Enable rate limiting**
7. **Monitor logs** for suspicious activity

---

*Quick reference for V-Sentinel developers*