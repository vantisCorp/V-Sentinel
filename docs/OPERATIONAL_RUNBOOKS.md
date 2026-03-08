# V-Sentinel PQC Operational Runbooks

## Table of Contents

1. [Service Overview](#service-overview)
2. [Daily Operations](#daily-operations)
3. [Incident Response](#incident-response)
4. [Maintenance Procedures](#maintenance-procedures)
5. [Troubleshooting Guide](#troubleshooting-guide)
6. [Performance Tuning](#performance-tuning)
7. [Security Procedures](#security-procedures)

---

## Service Overview

### Architecture Components

| Service | Port | Health Check | Critical Metrics |
|---------|------|--------------|------------------|
| PQC Gateway | 8443 | `/health` | TLS handshake time, connection rate |
| PQC VPN | 1194 | `/health` | Tunnel count, encryption latency |
| PQC Messaging | 5671 | `/health` | Message throughput, encryption ops/sec |
| Key Manager | 8080 | `/health` | Key generation rate, key age distribution |
| Certificate Manager | 8081 | `/health` | Certificate expiration, renewal rate |

### Key Dependencies

- **Database**: PostgreSQL 15+ (PQC metadata, audit logs)
- **Cache**: Redis 7+ (Key caching, session data)
- **HSM**: Hardware Security Module (Production only)
- **Vault**: HashiCorp Vault (Secrets management)
- **Monitoring**: Prometheus + Grafana

---

## Daily Operations

### Morning Health Check (Daily @ 09:00 UTC)

```bash
#!/bin/bash
# Daily health check script

echo "=== V-Sentinel PQC Daily Health Check ==="
echo "Date: $(date)"

# Check service health
services=("gateway" "vpn" "messaging" "key-manager" "certificate-manager")
for service in "${services[@]}"; do
    echo "Checking ${service}..."
    kubectl get pods -n v-sentinel-production -l app=${service} -o wide
done

# Check PQC certificate expiration
echo "Checking PQC certificate expiration..."
kubectl exec -n v-sentinel-production deployment/certificate-manager -- \
    /app/certificate-manager check-expiration --days 30

# Check key rotation status
echo "Checking key rotation status..."
kubectl exec -n v-sentinel-production deployment/key-manager -- \
    /app/key-manager status --rotation

# Review alerts from last 24h
echo "Recent alerts..."
# Add alert review logic here
```

### Daily Metrics Review

Check these metrics daily in Grafana:

1. **PQC Operations**
   - Key generation success rate: Target > 99.9%
   - Encryption operations/sec: Monitor trends
   - TLS handshake latency: Target < 100ms p95

2. **Security Metrics**
   - Failed authentication attempts: Monitor spikes
   - Quantum threat indicators: Check new alerts
   - Key age distribution: Ensure rotation compliance

3. **Performance Metrics**
   - Service response times: Target p95 < 200ms
   - Error rates: Target < 0.1%
   - Resource utilization: CPU < 70%, Memory < 80%

---

## Incident Response

### Incident Severity Levels

| Severity | Response Time | Impact |
|----------|---------------|--------|
| P1 - Critical | 15 minutes | Complete service outage |
| P2 - High | 1 hour | Degraded service, security breach |
| P3 - Medium | 4 hours | Partial service impact |
| P4 - Low | 24 hours | Minor issues, no user impact |

### Common Incidents & Procedures

#### INC-001: PQC Gateway High Latency

**Symptoms:**
- TLS handshake latency > 500ms p95
- Increased connection timeout errors
- User complaints about slow connections

**Diagnosis:**
```bash
# Check gateway health
kubectl exec -n v-sentinel-production deployment/gateway -- \
    curl -s http://localhost:8080/health

# Check metrics
kubectl exec -n v-sentinel-production deployment/gateway -- \
    curl -s http://localhost:8080/metrics | grep tls_handshake_duration

# Check resource usage
kubectl top pods -n v-sentinel-production -l app=gateway
```

**Resolution:**
1. Increase HPA limits if resource constrained
2. Check database connectivity
3. Restart gateway pods if needed:
   ```bash
   kubectl rollout restart deployment/gateway -n v-sentinel-production
   ```
4. If persists, escalate to engineering team

**Prevention:**
- Set up alerts for latency > 300ms
- Regular capacity planning
- Load testing before releases

---

#### INC-002: Key Manager Rotation Failure

**Symptoms:**
- Alert: "Key rotation failed for X keys"
- Keys approaching expiration without renewal
- Audit logs showing rotation errors

**Diagnosis:**
```bash
# Check key manager logs
kubectl logs -n v-sentinel-production -l app=key-manager --tail=100

# Check rotation status
kubectl exec -n v-sentinel-production deployment/key-manager -- \
    /app/key-manager status --rotation

# Check HSM connectivity (production)
kubectl exec -n v-sentinel-production deployment/key-manager -- \
    /app/key-manager test-hsm-connection
```

**Resolution:**
1. Check HSM status (production) or vault connectivity
2. Manually rotate affected keys:
   ```bash
   kubectl exec -n v-sentinel-production deployment/key-manager -- \
       /app/key-manager rotate --key-id <key_id>
   ```
3. Review rotation logs for root cause
4. Update rotation policies if needed

**Prevention:**
- Monitor rotation success rates
- Test rotation procedures regularly
- Have backup rotation mechanisms

---

#### INC-003: Quantum Threat Detection

**Symptoms:**
- Alert: "Quantum computer capability detected"
- Unexpected decryption attempt patterns
- Security system flags quantum-resistance bypass

**Diagnosis:**
```bash
# Check threat detection logs
kubectl logs -n v-sentinel-production -l app=gateway --tail=200 | grep "quantum"

# Check security events
kubectl exec -n v-sentinel-production deployment/gateway -- \
    /app/gateway security-events --recent 1h

# Review authentication patterns
kubectl exec -n v-sentinel-production deployment/messaging -- \
    /app/messaging auth-stats --abnormal
```

**Resolution:**
1. Enable hybrid mode if not active
2. Increase key sizes temporarily
3. Force re-key all active sessions
4. Alert security team immediately
5. Document incident for post-mortem

**Prevention:**
- Continuous threat monitoring
- Regular security audits
- Keep quantum threat intelligence updated

---

#### INC-004: PQC Certificate Expiration Imminent

**Symptoms:**
- Alert: "Certificate expiring in < 7 days"
- TLS handshake failures due to expired certs
- Services rejecting connections

**Diagnosis:**
```bash
# Check certificate status
kubectl exec -n v-sentinel-production deployment/certificate-manager -- \
    /app/certificate-manager list --expiring-days 7

# Check renewal logs
kubectl logs -n v-sentinel-production -l app=certificate-manager --tail=50

# Check CA connectivity
kubectl exec -n v-sentinel-production deployment/certificate-manager -- \
    /app/certificate-manager test-ca-connection
```

**Resolution:**
1. Check CA status and connectivity
2. Manually renew certificates:
   ```bash
   kubectl exec -n v-sentinel-production deployment/certificate-manager -- \
       /app/certificate-manager renew --certificate-id <cert_id>
   ```
3. Reload services with new certificates
4. Monitor certificate validation

**Prevention:**
- Set expiration alerts at 30, 14, 7 days
- Automated renewal testing
- Certificate renewal playbook

---

## Maintenance Procedures

### Weekly Maintenance (Sundays @ 02:00 UTC)

1. **Backup Verification**
   ```bash
   # Verify recent backups
   /app/backup verify --days 7
   ```

2. **Log Rotation**
   ```bash
   # Rotate service logs
   kubectl exec -n v-sentinel-production -l app=gateway -- \
       logrotate -f /etc/logrotate.d/gateway
   ```

3. **Cache Cleanup**
   ```bash
   # Clean expired cache entries
   kubectl exec -n v-sentinel-production deployment/gateway -- \
       redis-cli FLUSHDB
   ```

4. **Security Updates**
   ```bash
   # Check for security updates
   apt list --upgradable | grep security
   ```

### Monthly Maintenance (1st Monday @ 00:00 UTC)

1. **Full System Backup**
   ```bash
   /app/backup full --compress --encrypt
   ```

2. **Performance Review**
   - Review monthly performance metrics
   - Identify optimization opportunities
   - Update capacity planning

3. **Security Audit**
   - Review access logs
   - Audit key usage patterns
   - Check compliance with security policies

4. **Documentation Update**
   - Update runbooks based on incidents
   - Document configuration changes
   - Review and update SOPs

### Quarterly Maintenance

1. **Disaster Recovery Test**
   ```bash
   # Test restore procedures
   /app/restore test --backup-id <latest>
   ```

2. **Capacity Planning**
   - Review growth trends
   - Plan infrastructure upgrades
   - Budget for upcoming needs

3. **Compliance Audit**
   - Verify NIST compliance
   - Review CNSA 2.0 requirements
   - Document compliance status

4. **Staff Training**
   - Update training materials
   - Conduct refresher sessions
   - Plan upcoming training

---

## Troubleshooting Guide

### Troubleshooting Checklist

1. **Service Not Responding**
   - [ ] Check pod status: `kubectl get pods -n v-sentinel-production`
   - [ ] Check pod logs: `kubectl logs <pod-name>`
   - [ ] Check health endpoint: `curl http://service:port/health`
   - [ ] Check resource usage: `kubectl top pods`
   - [ ] Check dependencies (database, cache)

2. **High Error Rate**
   - [ ] Review recent changes
   - [ ] Check configuration drift
   - [ ] Review dependency health
   - [ ] Check rate limiting rules
   - [ ] Review authentication failures

3. **Performance Degradation**
   - [ ] Check resource utilization
   - [ ] Review database queries
   - [ ] Check network latency
   - [ ] Review cache hit rates
   - [ ] Check for resource contention

### Common Issues & Solutions

#### Issue: TLS Handshake Failures

**Symptoms:**
- Increased TLS handshake errors
- Client connection failures

**Diagnosis:**
```bash
# Check TLS configuration
kubectl exec -n v-sentinel-production deployment/gateway -- \
    openssl s_client -connect localhost:8443 -tls1_3

# Check certificate status
kubectl exec -n v-sentinel-production deployment/certificate-manager -- \
    /app/certificate-manager status

# Check gateway logs
kubectl logs -n v-sentinel-production -l app=gateway | grep -i tls
```

**Solutions:**
1. Verify certificate validity
2. Check cipher suite compatibility
3. Verify PQC algorithm support
4. Check for handshake timeout configuration

---

#### Issue: Key Generation Slowness

**Symptoms:**
- Key generation takes > 1 second
- Delayed service availability

**Diagnosis:**
```bash
# Test key generation speed
kubectl exec -n v-sentinel-production deployment/key-manager -- \
    /app/key-manager benchmark --algorithm kyber1024 --iterations 10

# Check HSM status (production)
kubectl exec -n v-sentinel-production deployment/key-manager -- \
    /app/key-manager hsm-status

# Check CPU usage
kubectl top pods -n v-sentinel-production -l app=key-manager
```

**Solutions:**
1. Add more key manager instances
2. Optimize key generation parameters
3. Check HSM performance (production)
4. Consider key pre-generation

---

#### Issue: Memory Leaks

**Symptoms:**
- Steady memory increase over time
- OOM kills and pod restarts

**Diagnosis:**
```bash
# Monitor memory usage over time
kubectl top pods -n v-sentinel-production -l app=gateway --containers

# Check for memory leaks
kubectl exec -n v-sentinel-production deployment/gateway -- \
    /app/gateway memory-stats --detailed

# Review garbage collection
kubectl exec -n v-sentinel-production deployment/gateway -- \
    /app/gateway gc-stats
```

**Solutions:**
1. Identify and fix memory leak
2. Set appropriate memory limits
3. Implement periodic restarts
4. Optimize memory usage

---

## Performance Tuning

### Tuning Parameters

#### Gateway Service

| Parameter | Default | Recommended | Notes |
|-----------|---------|-------------|-------|
| max_connections | 10000 | 50000 | Based on load |
| connection_timeout | 30s | 60s | High latency networks |
| keepalive_timeout | 75s | 120s | Long-lived connections |
| worker_processes | 4 | 16 | CPU cores |
| cache_ttl | 3600s | 7200s | Key caching duration |

#### Key Manager

| Parameter | Default | Recommended | Notes |
|-----------|---------|-------------|-------|
| key_cache_size | 10000 | 50000 | Increase for high load |
| rotation_interval | 90d | 90d | Compliance requirement |
| key_generation_workers | 4 | 16 | Parallel key generation |
| hsm_connection_pool | 5 | 20 | Production only |

### Performance Optimization Checklist

- [ ] Enable connection pooling
- [ ] Optimize database queries
- [ ] Implement caching strategies
- [ ] Configure appropriate timeouts
- [ ] Set proper resource limits
- [ ] Enable HTTP/2 and HTTP/3
- [ ] Optimize TLS configuration
- [ ] Use hardware acceleration (if available)

---

## Security Procedures

### Incident Response Procedure

1. **Detection**
   - Monitor alerts
   - Review security logs
   - Check threat indicators

2. **Containment**
   - Isolate affected systems
   - Block malicious IPs
   - Revoke compromised credentials

3. **Eradication**
   - Remove threats
   - Patch vulnerabilities
   - Clean compromised data

4. **Recovery**
   - Restore from clean backups
   - Verify system integrity
   - Monitor for recurrence

5. **Lessons Learned**
   - Document incident
   - Update procedures
   - Train team

### Security Audit Checklist

- [ ] Review access logs for suspicious activity
- [ ] Audit key usage and access patterns
- [ ] Verify compliance with security policies
- [ ] Check for configuration drift
- [ ] Review user access rights
- [ ] Audit authentication mechanisms
- [ ] Review encryption key management
- [ ] Check for unauthorized changes

### Emergency Contacts

| Role | Contact | On-Call Hours |
|------|---------|---------------|
| Security Lead | security@v-sentinel.com | 24/7 |
| Operations Lead | ops@v-sentinel.com | 24/7 |
| Engineering Lead | eng@v-sentinel.com | Business hours |
| Database Admin | dba@v-sentinel.com | Business hours |

---

*Document Version: 1.0*  
*Last Updated: March 2026*  
*Classification: Internal Use - Confidential*