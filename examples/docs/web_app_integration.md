# Web Application Security Integration Guide

This comprehensive guide walks you through integrating SENTINEL Security System into your web applications for comprehensive protection.

## Table of Contents

- [Overview](#overview)
- [Prerequisites](#prerequisites)
- [Quick Start](#quick-start)
- [Authentication](#authentication)
- [File Upload Protection](#file-upload-protection)
- [Request Validation](#request-validation)
- [Threat Blocking](#threat-blocking)
- [Security Monitoring](#security-monitoring)
- [Best Practices](#best-practices)
- [Troubleshooting](#troubleshooting)

## Overview

SENTINEL provides web application protection through:

- **File Scanning**: Real-time malware detection for uploaded files
- **Request Analysis**: Threat scoring for incoming requests
- **Behavioral Detection**: Anomaly detection for suspicious patterns
- **Automated Blocking**: Instant threat neutralization
- **Security Logging**: Comprehensive event tracking

## Prerequisites

- SENTINEL API key from [https://api.sentinel.ai](https://api.sentinel.ai)
- Python 3.8+ or Node.js 16+ (for SDK)
- Basic understanding of web frameworks

## Quick Start

### Python (Flask)

```python
from sentinel_sdk import SentinelClient
from flask import Flask, request

app = Flask(__name__)
sentinel = SentinelClient(api_key="your-api-key")

@app.route('/upload', methods=['POST'])
def upload_file():
    file = request.files['file']
    
    # Scan file with SENTINEL
    result = sentinel.scan_file(file.stream)
    
    if result['is_malicious']:
        return {'error': 'Malicious file detected'}, 400
    
    # Process clean file
    return {'message': 'File is safe'}
```

### Node.js (Express)

```javascript
const { SentinelClient } = require('sentinel-sdk');
const express = require('express');

const app = express();
const sentinel = new SentinelClient({ apiKey: 'your-api-key' });

app.post('/upload', async (req, res) => {
    const file = req.files.file;
    
    // Scan file with SENTINEL
    const result = await sentinel.scanFile(file.data);
    
    if (result.isMalicious) {
        return res.status(400).json({ error: 'Malicious file detected' });
    }
    
    // Process clean file
    res.json({ message: 'File is safe' });
});
```

## Authentication

### API Key Authentication

```python
from sentinel_sdk import SentinelClient

# Initialize with API key
sentinel = SentinelClient(
    api_key="your-api-key",
    base_url="https://api.sentinel.ai/v1"
)

# Verify connection
health = sentinel.health_check()
print(f"Connection: {'Healthy' if health['status'] == 'ok' else 'Error'}")
```

### JWT Authentication

```python
# Generate JWT token
token = sentinel.generate_jwt(
    user_id="user-123",
    permissions=["scan:files", "predict:threats"]
)

# Use token for subsequent requests
result = sentinel.scan_file(
    file_path,
    auth_token=token
)
```

### Environment Variables

```bash
# .env file
SENTINEL_API_KEY=your-api-key
SENTINEL_API_URL=https://api.sentinel.ai/v1
SENTINEL_TIMEOUT=30
SENTINEL_MAX_RETRIES=3
```

```python
import os
from dotenv import load_dotenv

load_dotenv()

sentinel = SentinelClient(
    api_key=os.getenv('SENTINEL_API_KEY'),
    base_url=os.getenv('SENTINEL_API_URL')
)
```

## File Upload Protection

### Basic File Scanning

```python
@app.route('/upload', methods=['POST'])
def upload_file():
    if 'file' not in request.files:
        return {'error': 'No file provided'}, 400
    
    file = request.files['file']
    
    # Save temporarily
    temp_path = f"/tmp/{file.filename}"
    file.save(temp_path)
    
    try:
        # Scan file
        result = sentinel.scan_file(
            temp_path,
            options={
                'deep_scan': True,
                'timeout': 30
            }
        )
        
        if result['is_malicious']:
            log_security_event(
                'MALICIOUS_FILE',
                'HIGH',
                f"Malicious file blocked: {result['threat_name']}"
            )
            return {
                'error': 'Malicious file detected',
                'threat': result['threat_name']
            }, 400
        
        # Process clean file
        return {
            'message': 'File is safe',
            'scan_time_ms': result['scan_time_ms'],
            'confidence': result['confidence']
        }
    
    finally:
        # Clean up
        import os
        if os.path.exists(temp_path):
            os.remove(temp_path)
```

### Multiple File Scanning

```python
@app.route('/upload-multiple', methods=['POST'])
def upload_multiple():
    files = request.files.getlist('files')
    results = []
    
    for file in files:
        temp_path = f"/tmp/{file.filename}"
        file.save(temp_path)
        
        try:
            result = sentinel.scan_file(temp_path)
            results.append({
                'filename': file.filename,
                'is_malicious': result['is_malicious'],
                'threat_name': result.get('threat_name'),
                'confidence': result.get('confidence')
            })
        finally:
            os.remove(temp_path)
    
    # Check if any files were malicious
    if any(r['is_malicious'] for r in results):
        return {
            'error': 'Some files were malicious',
            'results': results
        }, 400
    
    return {'message': 'All files are safe', 'results': results}
```

### Streaming File Scanning

```python
@app.route('/upload-stream', methods=['POST'])
def upload_stream():
    file = request.files['file']
    
    # Scan without saving to disk
    result = sentinel.scan_stream(
        file.stream,
        filename=file.filename,
        chunk_size=8192
    )
    
    if result['is_malicious']:
        return {'error': 'Malicious file detected'}, 400
    
    return {'message': 'File is safe'}
```

## Request Validation

### Threat Score Middleware

```python
from functools import wraps

def check_threat(f):
    @wraps(f)
    def decorated_function(*args, **kwargs):
        request_data = {
            'method': request.method,
            'path': request.path,
            'headers': dict(request.headers),
            'args': dict(request.args),
            'form': dict(request.form),
            'remote_addr': request.remote_addr
        }
        
        threat_result = sentinel.analyze_request(request_data)
        
        if threat_result['risk_score'] > 0.8:
            log_security_event(
                'HIGH_RISK_REQUEST',
                'CRITICAL',
                f"High risk request blocked: {threat_result}"
            )
            return {'error': 'Request blocked by security policy'}, 403
        
        return f(*args, **kwargs)
    
    return decorated_function

@app.route('/api/endpoint', methods=['POST'])
@check_threat
def protected_endpoint():
    return {'message': 'Request processed'}
```

### SQL Injection Protection

```python
@app.route('/api/users/<user_id>', methods=['GET'])
@check_threat
def get_user(user_id):
    # SENTINEL has already validated the request
    # Now it's safe to process
    
    query = f"SELECT * FROM users WHERE id = {user_id}"
    # Process query safely
    
    return {'user': user_data}
```

### XSS Protection

```python
@app.route('/api/comment', methods=['POST'])
@check_threat
def add_comment():
    comment = request.json.get('comment', '')
    
    # SENTINEL validates request content
    # Additional HTML sanitization
    import bleach
    clean_comment = bleach.clean(comment)
    
    # Store clean comment
    return {'message': 'Comment added'}
```

## Threat Blocking

### IP Blocking

```python
blocked_ips = set()

@app.route('/api/block-ip', methods=['POST'])
def block_ip():
    data = request.json
    ip = data.get('ip')
    reason = data.get('reason', 'Manual block')
    
    blocked_ips.add(ip)
    
    # Notify SENTINEL
    sentinel.block_ip(ip, reason=reason)
    
    return {'success': True, 'ip': ip}

@app.before_request
def check_blocked_ip():
    if request.remote_addr in blocked_ips:
        log_security_event(
            'BLOCKED_IP_ACCESS',
            'HIGH',
            f'Blocked IP attempted access: {request.remote_addr}'
        )
        return {'error': 'Access denied'}, 403
```

### Rate Limiting with SENTINEL

```python
from flask_limiter import Limiter
from flask_limiter.util import get_remote_address

limiter = Limiter(
    app=app,
    key_func=get_remote_address,
    default_limits=["200 per day", "50 per hour"],
    storage_uri="memory://"
)

@app.route('/api/scan', methods=['POST'])
@limiter.limit("10 per minute")
@check_threat
def scan_endpoint():
    # Request processing
    pass
```

### Automated Threat Response

```python
@app.errorhandler(400)
def bad_request(error):
    # Log security events
    log_security_event(
        'BAD_REQUEST',
        'MEDIUM',
        f'Bad request: {error.description}'
    )
    
    # Check if it's a potential attack
    sentinel.report_potential_attack({
        'type': 'bad_request',
        'ip': request.remote_addr,
        'path': request.path,
        'description': error.description
    })
    
    return {'error': 'Bad request'}, 400
```

## Security Monitoring

### Security Event Logging

```python
import logging
from datetime import datetime

# Configure security logger
security_logger = logging.getLogger('sentinel_security')
security_handler = logging.FileHandler('security.log')
security_handler.setFormatter(logging.Formatter(
    '%(asctime)s - %(name)s - %(levelname)s - %(message)s'
))
security_logger.addHandler(security_handler)

def log_security_event(event_type, severity, details, metadata=None):
    event = {
        'timestamp': datetime.utcnow().isoformat(),
        'event_type': event_type,
        'severity': severity,
        'details': details,
        'ip': request.remote_addr,
        'user_agent': request.headers.get('User-Agent'),
        'path': request.path,
        'metadata': metadata or {}
    }
    
    security_logger.info(f"{event_type}: {details}")
    
    # Send to SENTINEL for analysis
    sentinel.log_security_event(event)
```

### Real-time Dashboard

```python
@app.route('/api/security/events', methods=['GET'])
def get_security_events():
    limit = request.args.get('limit', 50, type=int)
    severity = request.args.get('severity')
    
    # Get events from SENTINEL
    events = sentinel.get_security_events(
        limit=limit,
        severity=severity
    )
    
    return {'events': events}

@app.route('/api/security/stats', methods=['GET'])
def get_security_stats():
    stats = sentinel.get_security_statistics()
    
    return {
        'total_requests': stats['total_requests'],
        'threats_blocked': stats['threats_blocked'],
        'files_scanned': stats['files_scanned'],
        'avg_response_time_ms': stats['avg_response_time_ms']
    }
```

### Alert Configuration

```python
# Configure alerts
sentinel.configure_alerts({
    'webhooks': [
        {
            'url': 'https://your-webhook.com/alerts',
            'events': ['MALICIOUS_FILE', 'HIGH_RISK_REQUEST']
        }
    ],
    'email': [
        {
            'address': 'security@yourcompany.com',
            'severity': ['HIGH', 'CRITICAL']
        }
    ],
    'slack': {
        'webhook': 'https://hooks.slack.com/services/...',
        'channel': '#security-alerts'
    }
})
```

## Best Practices

### 1. Always Validate Files

```python
# Always scan files before processing
@app.route('/upload', methods=['POST'])
def upload_file():
    file = request.files['file']
    
    # Scan first
    result = sentinel.scan_file(file.stream)
    
    if result['is_malicious']:
        return {'error': 'Malicious file'}, 400
    
    # Then process
    process_file(file)
```

### 2. Use Threat Scoring

```python
# Use threat scoring for gradual blocking
threat_score = sentinel.predict_threat(features)

if threat_score > 0.9:
    # Block immediately
    block_user()
elif threat_score > 0.7:
    # Require additional verification
    request_verification()
elif threat_score > 0.5:
    # Log for monitoring
    log_for_review()
```

### 3. Implement Rate Limiting

```python
# Always rate limit scanning endpoints
@app.route('/scan', methods=['POST'])
@limiter.limit("10 per minute")
def scan():
    # Scan logic
    pass
```

### 4. Log Everything

```python
# Log all security events
try:
    result = sentinel.scan_file(file)
    log_security_event('FILE_SCAN', 'INFO', 'Scan completed')
except Exception as e:
    log_security_event('SCAN_ERROR', 'ERROR', str(e))
```

### 5. Handle Errors Gracefully

```python
# Always handle errors
try:
    result = sentinel.scan_file(file)
except SentinelAPIError as e:
    log_security_event('API_ERROR', 'HIGH', str(e))
    return {'error': 'Security service unavailable'}, 503
except Exception as e:
    log_security_event('UNKNOWN_ERROR', 'MEDIUM', str(e))
    return {'error': 'Internal error'}, 500
```

## Troubleshooting

### Issue: File Scanning is Slow

**Solution:** Use streaming scanning

```python
# Instead of:
result = sentinel.scan_file(file_path)

# Use:
result = sentinel.scan_stream(file.stream, filename=file.name)
```

### Issue: Too Many False Positives

**Solution:** Adjust threat thresholds

```python
result = sentinel.scan_file(
    file_path,
    options={
        'threshold': 0.8  # Higher threshold = fewer false positives
    }
)
```

### Issue: API Rate Limits Exceeded

**Solution:** Implement caching and batch processing

```python
from functools import lru_cache

@lru_cache(maxsize=1000)
def cached_file_scan(file_hash):
    # Only scan files once
    return sentinel.scan_file_by_hash(file_hash)
```

### Issue: Connection Timeouts

**Solution:** Increase timeout and implement retries

```python
sentinel = SentinelClient(
    api_key=api_key,
    timeout=60,
    max_retries=3,
    retry_delay=1
)
```

## Additional Resources

- [API Documentation](https://docs.sentinel.ai/api)
- [SDK Reference](https://docs.sentinel.ai/sdk)
- [Best Practices Guide](https://docs.sentinel.ai/best-practices)
- [Support Portal](https://support.sentinel.ai)
- [Community Forum](https://community.sentinel.ai)