# SENTINEL - Quick Start Guide

## Getting Started in 5 Minutes

This guide will help you get SENTINEL up and running on your local machine in just 5 minutes.

---

## Prerequisites

Before you begin, make sure you have:

- **Docker** 20.10 or later
- **Docker Compose** 2.0 or later

That's it! Everything else is handled for you.

---

## Quick Start

### 1. Clone the Repository

```bash
git clone https://github.com/vantisCorp/V-Sentinel.git
cd V-Sentinel
```

### 2. Start Services

```bash
docker-compose up -d
```

This will start:
- SENTINEL API server
- PostgreSQL database
- Redis cache
- Monitoring services

### 3. Check Status

```bash
docker-compose ps
```

You should see all services running.

### 4. Test the API

```bash
# Check health endpoint
curl http://localhost:8080/health

# Check system status
curl http://localhost:8080/api/v1/system/status
```

### 5. View the Dashboard

Open your browser and navigate to:

```
http://localhost:8080
```

You'll see the SENTINEL dashboard with real-time metrics.

---

## What's Running?

After starting services, you'll have:

| Service | Port | Description |
|---------|------|-------------|
| SENTINEL API | 8080 | Main API server |
| PostgreSQL | 5432 | Database |
| Redis | 6379 | Cache |
| Prometheus | 9090 | Metrics |
| Grafana | 3000 | Visualization |

---

## Basic Usage

### Initialize Hypervisor

```bash
curl -X POST http://localhost:8080/api/v1/hypervisor/initialize \
  -H "Content-Type: application/json"
```

### Detect Threats

```bash
curl -X POST http://localhost:8080/api/v1/ai/predict \
  -H "Content-Type: application/json" \
  -d '{
    "features": {
      "process_behavior": [0.1, 0.2, 0.3, 0.4, 0.5],
      "file_operations": [0.6, 0.7, 0.8, 0.9, 1.0],
      "network_activity": [0.1, 0.2, 0.3, 0.4, 0.5],
      "system_calls": [0.6, 0.7, 0.8, 0.9, 1.0],
      "registry_changes": [0.1, 0.2, 0.3, 0.4, 0.5]
    }
  }'
```

### Monitor Gaming Traffic

```bash
curl http://localhost:8080/api/v1/gaming/traffic
```

---

## Development Setup

If you want to develop SENTINEL locally:

### 1. Install Rust

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

### 2. Install Dependencies

```bash
# Install Rust dependencies
cargo build

# Install Python dependencies
pip install -r requirements.txt
```

### 3. Run Tests

```bash
# Run all tests
cargo test

# Run with coverage
cargo tarpaulin --out Html
```

### 4. Run Development Server

```bash
cargo run
```

The API will be available at `http://localhost:8080`

---

## Configuration

SENTINEL uses environment variables for configuration. Create a `.env` file:

```env
# Database
DATABASE_URL=postgresql://sentinel:sentinel@localhost:5432/sentinel

# Redis
REDIS_URL=redis://localhost:6379

# API
API_HOST=0.0.0.0
API_PORT=8080
API_KEY=your_api_key_here

# Security
JWT_SECRET=your_jwt_secret_here
ENCRYPTION_KEY=your_encryption_key_here

# Logging
LOG_LEVEL=debug
LOG_FORMAT=pretty
```

---

## Monitoring

### View Metrics

Sentinel exposes Prometheus metrics at:

```
http://localhost:9090/metrics
```

### View Dashboard

Access the Grafana dashboard:

```
http://localhost:3000
```

Default credentials:
- Username: `admin`
- Password: `admin`

---

## Stopping Services

```bash
# Stop all services
docker-compose down

# Stop and remove volumes
docker-compose down -v
```

---

## Troubleshooting

### Services won't start

```bash
# Check logs
docker-compose logs

# Check specific service logs
docker-compose logs sentinel-api

# Restart services
docker-compose restart
```

### Port conflicts

If ports are already in use, edit `docker-compose.yml`:

```yaml
services:
  sentinel-api:
    ports:
      - "8081:8080"  # Change to different port
```

### Database connection errors

```bash
# Check if database is running
docker-compose ps postgres

# View database logs
docker-compose logs postgres

# Restart database
docker-compose restart postgres
```

---

## Next Steps

Now that you have SENTINEL running:

1. **Explore the API** - Try different endpoints
2. **Read the Documentation** - Check out `docs/` directory
3. **Review Examples** - Look at `examples/` directory
4. **Contribute** - See `CONTRIBUTING.md`

---

## Getting Help

- **Documentation:** https://docs.sentinel.ai
- **API Reference:** https://api.sentinel.ai
- **GitHub Issues:** https://github.com/vantisCorp/V-Sentinel/issues
- **Discord:** https://discord.gg/sentinel
- **Support:** support@sentinel.ai

---

## Quick Reference

### Common Commands

```bash
# Start services
docker-compose up -d

# Stop services
docker-compose down

# View logs
docker-compose logs -f

# Restart services
docker-compose restart

# Build image
docker build -t sentinel:latest .

# Run tests
cargo test

# Format code
cargo fmt

# Check code
cargo clippy
```

### API Endpoints

```bash
# Health check
GET /health

# System status
GET /api/v1/system/status

# Hypervisor
POST /api/v1/hypervisor/initialize

# AI Prediction
POST /api/v1/ai/predict

# Gaming
POST /api/v1/gaming/handshake

# Quantum
POST /api/v1/quantum/keypair
```

---

## What's Next?

- 📖 Read the full [Documentation](docs/)
- 💻 Check out [Examples](examples/)
- 🚀 Review the [Developer Guide](docs/SENTINEL_DEVELOPER_GUIDE.md)
- 🤝 Join our [Discord](https://discord.gg/sentinel)

---

**Happy Coding! 🎉**