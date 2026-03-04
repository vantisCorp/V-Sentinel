# V-Sentinel Production Deployment Guide

## Table of Contents
1. [Overview](#overview)
2. [Prerequisites](#prerequisites)
3. [Deployment Options](#deployment-options)
4. [Docker Deployment](#docker-deployment)
5. [Kubernetes Deployment](#kubernetes-deployment)
6. [Bare Metal Deployment](#bare-metal-deployment)
7. [Configuration](#configuration)
8. [Monitoring](#monitoring)
9. [Security Hardening](#security-hardening)
10. [Troubleshooting](#troubleshooting)

---

## Overview

This guide provides comprehensive instructions for deploying V-Sentinel in production environments. V-Sentinel is designed for high availability, scalability, and security.

### System Requirements

| Component | Minimum | Recommended |
|-----------|---------|-------------|
| CPU | 4 cores | 16+ cores |
| RAM | 8 GB | 32+ GB |
| Storage | 100 GB SSD | 500 GB NVMe |
| Network | 1 Gbps | 10+ Gbps |
| OS | Linux (Ubuntu 20.04+) | Ubuntu 22.04 LTS |

---

## Prerequisites

### Software Requirements
- Docker 20.10+
- Docker Compose 2.0+
- Kubernetes 1.24+ (for K8s deployment)
- Helm 3.0+ (for Helm deployment)
- NGINX or Traefik (as reverse proxy)
- PostgreSQL 14+ (for data storage)
- Redis 7+ (for caching)
- Prometheus + Grafana (for monitoring)

### Network Requirements
- Open ports: 80, 443, 8080, 8443
- Static IP address recommended
- SSL/TLS certificate
- DNS configuration

---

## Deployment Options

V-Sentinel supports multiple deployment strategies:

1. **Docker Compose** - Simple, single-node deployment
2. **Kubernetes** - Scalable, multi-node deployment
3. **Bare Metal** - Direct server deployment
4. **Cloud Platform** - AWS, GCP, Azure

Choose the option that best fits your infrastructure and requirements.

---

## Docker Deployment

### Quick Start

1. **Clone the repository:**
```bash
git clone https://github.com/vantisCorp/V-Sentinel.git
cd V-Sentinel
```

2. **Create environment file:**
```bash
cp .env.example .env
# Edit .env with your configuration
```

3. **Start services:**
```bash
docker-compose up -d
```

4. **Verify deployment:**
```bash
docker-compose ps
curl http://localhost:8080/health
```

### Docker Compose Configuration

```yaml
version: '3.8'

services:
  sentinel:
    build:
      context: .
      dockerfile: Dockerfile
    container_name: vsentinel
    ports:
      - "8080:8080"
      - "8443:8443"
    environment:
      - RUST_LOG=info
      - SENTINEL_CONFIG=/config/sentinel.toml
    volumes:
      - ./config:/config:ro
      - ./data:/data
      - ./logs:/logs
    depends_on:
      - postgres
      - redis
    restart: unless-stopped
    networks:
      - sentinel-net
    healthcheck:
      test: ["CMD", "curl", "-f", "http://localhost:8080/health"]
      interval: 30s
      timeout: 10s
      retries: 3

  postgres:
    image: postgres:15-alpine
    container_name: vsentinel-db
    environment:
      - POSTGRES_DB=vsentinel
      - POSTGRES_USER=vsentinel
      - POSTGRES_PASSWORD=${DB_PASSWORD}
    volumes:
      - postgres-data:/var/lib/postgresql/data
    restart: unless-stopped
    networks:
      - sentinel-net

  redis:
    image: redis:7-alpine
    container_name: vsentinel-redis
    command: redis-server --requirepass ${REDIS_PASSWORD}
    volumes:
      - redis-data:/data
    restart: unless-stopped
    networks:
      - sentinel-net

  nginx:
    image: nginx:alpine
    container_name: vsentinel-nginx
    ports:
      - "80:80"
      - "443:443"
    volumes:
      - ./nginx/nginx.conf:/etc/nginx/nginx.conf:ro
      - ./nginx/ssl:/etc/nginx/ssl:ro
    depends_on:
      - sentinel
    restart: unless-stopped
    networks:
      - sentinel-net

volumes:
  postgres-data:
  redis-data:

networks:
  sentinel-net:
    driver: bridge
```

---

## Kubernetes Deployment

### Prerequisites

- Kubernetes cluster (1.24+)
- kubectl configured
- Helm 3.0+

### Using Helm

1. **Add the V-Sentinel Helm repository:**
```bash
helm repo add vsentinel https://charts.v-sentinel.io
helm repo update
```

2. **Install V-Sentinel:**
```bash
helm install vsentinel vsentinel/vsentinel \
  --namespace vsentinel \
  --create-namespace \
  --set config.database.url="postgresql://..." \
  --set config.redis.url="redis://..."
```

3. **Upgrade V-Sentinel:**
```bash
helm upgrade vsentinel vsentinel/vsentinel \
  --namespace vsentinel \
  --values custom-values.yaml
```

### Manual Kubernetes Deployment

#### Namespace
```yaml
apiVersion: v1
kind: Namespace
metadata:
  name: vsentinel
```

#### ConfigMap
```yaml
apiVersion: v1
kind: ConfigMap
metadata:
  name: vsentinel-config
  namespace: vsentinel
data:
  sentinel.toml: |
    [server]
    address = "0.0.0.0:8080"
    
    [database]
    url = "postgresql://vsentinel:password@postgres:5432/vsentinel"
    
    [redis]
    url = "redis://:password@redis:6379"
    
    [security]
    level = "high"
```

#### Secret
```yaml
apiVersion: v1
kind: Secret
metadata:
  name: vsentinel-secrets
  namespace: vsentinel
type: Opaque
data:
  db-password: <base64-encoded-password>
  redis-password: <base64-encoded-password>
  api-key: <base64-encoded-api-key>
```

#### Deployment
```yaml
apiVersion: apps/v1
kind: Deployment
metadata:
  name: vsentinel
  namespace: vsentinel
spec:
  replicas: 3
  selector:
    matchLabels:
      app: vsentinel
  template:
    metadata:
      labels:
        app: vsentinel
    spec:
      containers:
      - name: vsentinel
        image: vantis/vsentinel:latest
        ports:
        - containerPort: 8080
        env:
        - name: RUST_LOG
          value: "info"
        - name: SENTINEL_CONFIG
          value: "/config/sentinel.toml"
        volumeMounts:
        - name: config
          mountPath: /config
          readOnly: true
        - name: secrets
          mountPath: /secrets
          readOnly: true
        resources:
          requests:
            memory: "512Mi"
            cpu: "500m"
          limits:
            memory: "2Gi"
            cpu: "2000m"
        livenessProbe:
          httpGet:
            path: /health
            port: 8080
          initialDelaySeconds: 30
          periodSeconds: 10
        readinessProbe:
          httpGet:
            path: /ready
            port: 8080
          initialDelaySeconds: 10
          periodSeconds: 5
      volumes:
      - name: config
        configMap:
          name: vsentinel-config
      - name: secrets
        secret:
          secretName: vsentinel-secrets
```

#### Service
```yaml
apiVersion: v1
kind: Service
metadata:
  name: vsentinel
  namespace: vsentinel
spec:
  selector:
    app: vsentinel
  ports:
  - port: 80
    targetPort: 8080
    name: http
  type: ClusterIP
```

#### Ingress
```yaml
apiVersion: networking.k8s.io/v1
kind: Ingress
metadata:
  name: vsentinel-ingress
  namespace: vsentinel
  annotations:
    cert-manager.io/cluster-issuer: letsencrypt-prod
    nginx.ingress.kubernetes.io/ssl-redirect: "true"
spec:
  tls:
  - hosts:
    - sentinel.example.com
    secretName: vsentinel-tls
  rules:
  - host: sentinel.example.com
    http:
      paths:
      - path: /
        pathType: Prefix
        backend:
          service:
            name: vsentinel
            port:
              number: 80
```

### HorizontalPodAutoscaler

```yaml
apiVersion: autoscaling/v2
kind: HorizontalPodAutoscaler
metadata:
  name: vsentinel-hpa
  namespace: vsentinel
spec:
  scaleTargetRef:
    apiVersion: apps/v1
    kind: Deployment
    name: vsentinel
  minReplicas: 3
  maxReplicas: 10
  metrics:
  - type: Resource
    resource:
      name: cpu
      target:
        type: Utilization
        averageUtilization: 70
  - type: Resource
    resource:
      name: memory
      target:
        type: Utilization
        averageUtilization: 80
```

---

## Bare Metal Deployment

### System Setup

1. **Update system:**
```bash
apt update && apt upgrade -y
```

2. **Install dependencies:**
```bash
apt install -y \
    build-essential \
    pkg-config \
    libssl-dev \
    postgresql-client \
    redis-tools \
    nginx \
    certbot
```

3. **Install Rust:**
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env
```

4. **Build V-Sentinel:**
```bash
git clone https://github.com/vantisCorp/V-Sentinel.git
cd V-Sentinel
cargo build --release
```

### Service Configuration

Create a systemd service file:

```ini
[Unit]
Description=V-Sentinel Security System
After=network.target postgresql.service redis.service

[Service]
Type=simple
User=vsentinel
Group=vsentinel
WorkingDirectory=/opt/vsentinel
ExecStart=/opt/vsentinel/target/release/sentinel
Restart=always
RestartSec=10
Environment="RUST_LOG=info"
Environment="SENTINEL_CONFIG=/etc/vsentinel/sentinel.toml"

# Security
NoNewPrivileges=true
PrivateTmp=true
ProtectSystem=strict
ProtectHome=true
ReadWritePaths=/var/lib/vsentinel /var/log/vsentinel

# Resource limits
LimitNOFILE=65536
LimitNPROC=4096

[Install]
WantedBy=multi-user.target
```

Enable and start the service:

```bash
systemctl enable vsentinel
systemctl start vsentinel
systemctl status vsentinel
```

---

## Configuration

### Configuration File (`sentinel.toml`)

```toml
[server]
address = "0.0.0.0:8080"
tls_address = "0.0.0.0:8443"
tls_cert = "/etc/vsentinel/cert.pem"
tls_key = "/etc/vsentinel/key.pem"

[database]
url = "postgresql://vsentinel:password@localhost:5432/vsentinel"
max_connections = 100
min_connections = 10

[redis]
url = "redis://:password@localhost:6379/0"
max_connections = 50

[security]
level = "high"
enable_encryption = true
enable_audit_log = true
session_timeout = 3600

[privacy]
enable_zkp = true
zkp_backend = "bulletproofs"
dp_epsilon = 1.0

[quantum]
algorithm = "crystals-kyber"
key_rotation_interval = 86400

[ai]
model_path = "/models"
inference_threads = 4
enable_federated_learning = true

[monitoring]
enable_metrics = true
metrics_port = 9090
enable_tracing = true

[logging]
level = "info"
format = "json"
output = "both"
file = "/var/log/vsentinel/sentinel.log"
```

### Environment Variables

```bash
# Server
SENTINEL_SERVER_ADDRESS=0.0.0.0:8080
SENTINEL_TLS_ADDRESS=0.0.0.0:8443

# Database
DATABASE_URL=postgresql://vsentinel:password@localhost:5432/vsentinel
DATABASE_MAX_CONNECTIONS=100

# Redis
REDIS_URL=redis://:password@localhost:6379/0

# Security
SENTINEL_SECURITY_LEVEL=high
SENTINEL_API_KEY=your-api-key

# Logging
RUST_LOG=info
```

---

## Monitoring

### Prometheus Metrics

V-Sentinel exposes Prometheus metrics on port 9090:

```yaml
scrape_configs:
  - job_name: 'vsentinel'
    static_configs:
      - targets: ['localhost:9090']
    metrics_path: /metrics
```

### Grafana Dashboard

Import the V-Sentinel dashboard for monitoring:

1. Install Grafana
2. Add Prometheus data source
3. Import dashboard ID: `12345` (V-Sentinel)

### Key Metrics to Monitor

| Metric | Description | Threshold |
|--------|-------------|-----------|
| `sentinel_requests_total` | Total requests | - |
| `sentinel_request_duration` | Request duration | < 100ms p95 |
| `sentinel_threats_blocked` | Threats blocked | - |
| `sentinel_security_score` | Security score | > 95% |
| `sentinel_cpu_usage` | CPU usage | < 70% |
| `sentinel_memory_usage` | Memory usage | < 80% |

### Alerting Rules

```yaml
groups:
  - name: vsentinel
    rules:
      - alert: HighSecurityThreatLevel
        expr: sentinel_threat_level > 0.8
        for: 5m
        labels:
          severity: critical
        annotations:
          summary: "High security threat level detected"
          
      - alert: HighErrorRate
        expr: rate(sentinel_errors_total[5m]) > 0.1
        for: 5m
        labels:
          severity: warning
        annotations:
          summary: "High error rate detected"
          
      - alert: ServiceDown
        expr: up{job="vsentinel"} == 0
        for: 1m
        labels:
          severity: critical
        annotations:
          summary: "V-Sentinel service is down"
```

---

## Security Hardening

### TLS Configuration

```nginx
server {
    listen 443 ssl http2;
    server_name sentinel.example.com;

    ssl_certificate /etc/nginx/ssl/cert.pem;
    ssl_certificate_key /etc/nginx/ssl/key.pem;

    ssl_protocols TLSv1.2 TLSv1.3;
    ssl_ciphers ECDHE-RSA-AES256-GCM-SHA512:DHE-RSA-AES256-GCM-SHA512;
    ssl_prefer_server_ciphers off;
    ssl_session_cache shared:SSL:10m;
    ssl_session_timeout 10m;

    location / {
        proxy_pass http://localhost:8080;
        proxy_set_header Host $host;
        proxy_set_header X-Real-IP $remote_addr;
        proxy_set_header X-Forwarded-For $proxy_add_x_forwarded_for;
        proxy_set_header X-Forwarded-Proto $scheme;
    }
}
```

### Firewall Rules

```bash
# Allow SSH
ufw allow 22/tcp

# Allow HTTP/HTTPS
ufw allow 80/tcp
ufw allow 443/tcp

# Allow V-Sentinel ports
ufw allow 8080/tcp
ufw allow 8443/tcp

# Enable firewall
ufw enable
```

### Rate Limiting

```nginx
http {
    limit_req_zone $binary_remote_addr zone=sentinel_limit:10m rate=10r/s;
    
    server {
        location /api/ {
            limit_req zone=sentinel_limit burst=20 nodelay;
            proxy_pass http://localhost:8080;
        }
    }
}
```

---

## Troubleshooting

### Common Issues

#### Service Won't Start

```bash
# Check logs
journalctl -u vsentinel -n 100

# Check configuration
sentinel --check-config

# Check dependencies
systemctl status postgresql
systemctl status redis
```

#### High Memory Usage

```bash
# Check memory usage
free -h

# Check process memory
ps aux | grep sentinel

# Restart service
systemctl restart vsentinel
```

#### Database Connection Issues

```bash
# Test database connection
psql -U vsentinel -h localhost -d vsentinel

# Check connection count
psql -U vsentinel -h localhost -d vsentinel -c "SELECT count(*) FROM pg_stat_activity;"
```

### Health Checks

```bash
# Health check endpoint
curl http://localhost:8080/health

# Ready check
curl http://localhost:8080/ready

# Metrics endpoint
curl http://localhost:9090/metrics
```

### Logs

```bash
# Application logs
tail -f /var/log/vsentinel/sentinel.log

# System logs
journalctl -u vsentinel -f

# Docker logs
docker-compose logs -f sentinel
```

---

## Backup and Recovery

### Database Backup

```bash
# Backup
pg_dump -U vsentinel vsentinel > backup_$(date +%Y%m%d).sql

# Restore
psql -U vsentinel vsentinel < backup_20240304.sql
```

### Configuration Backup

```bash
# Backup configuration
tar -czf config_backup_$(date +%Y%m%d).tar.gz /etc/vsentinel

# Restore
tar -xzf config_backup_20240304.tar.gz -C /
```

---

## Support

For additional support:
- Documentation: https://docs.v-sentinel.io
- GitHub Issues: https://github.com/vantisCorp/V-Sentinel/issues
- Email: support@v-sentinel.io

---

*Last Updated: March 4, 2024*
*Version: 1.0.0*