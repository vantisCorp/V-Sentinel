# SENTINEL Security Best Practices

This guide provides comprehensive best practices for implementing SENTINEL Security System in production environments.

## Table of Contents

- [General Security Principles](#general-security-principles)
- [Authentication Best Practices](#authentication-best-practices)
- [File Handling Security](#file-handling-security)
- [Network Security](#network-security)
- [API Security](#api-security)
- [Performance Optimization](#performance-optimization)
- [Monitoring and Logging](#monitoring-and-logging)
- [Incident Response](#incident-response)
- [Compliance and Privacy](#compliance-and-privacy)
- [Production Deployment](#production-deployment)

## General Security Principles

### Defense in Depth

Implement multiple layers of security:

```python
# Layer 1: Network-level protection
@app.before_request
def check_network_threats():
    if is_blocked_ip(request.remote_addr):
        return {'error': 'Access denied'}, 403

# Layer 2: Request-level protection
@app.before_request
def analyze_request():
    if request.method == 'POST':
        threat_score = sentinel.analyze_request(request.json)
        if threat_score > 0.8:
            return {'error': 'Request blocked'}, 403

# Layer 3: Application-level protection
@app.route('/upload', methods=['POST'])
def upload_file():
    result = sentinel.scan_file(request.files['file'])
    if result['is_malicious']:
        return {'error': 'Malicious file'}, 400
    # Process file
```

### Principle of Least Privilege

Only grant necessary permissions:

```python
# Bad: Granting too many permissions
admin_token = sentinel.generate_token(permissions=['all'])

# Good: Granting only needed permissions
read_token = sentinel.generate_token(
    permissions=['scan:files', 'predict:threats'],
    expires_in=3600  # 1 hour
)
```

### Zero Trust Architecture

Never trust, always verify:

```python
# Always verify, even for internal requests
@app.route('/internal/scan', methods=['POST'])
@require_internal_auth
def internal_scan():
    # Still verify the request
    threat = sentinel.analyze_request(request.json)
    if threat['risk_score'] > 0.5:
        log_security_event('INTERNAL_THREAT', 'HIGH', threat)
        return {'error': 'Request blocked'}, 403
    
    result = sentinel.scan_file(request.files['file'])
    return {'result': result}
```

## Authentication Best Practices

### API Key Management

```python
import os
from dotenv import load_dotenv

# Store API keys securely
load_dotenv()

SENTINEL_API_KEY = os.getenv('SENTINEL_API_KEY')

# Rotate keys regularly
def rotate_api_key():
    new_key = sentinel.rotate_key(SENTINEL_API_KEY)
    os.environ['SENTINEL_API_KEY'] = new_key
    return new_key

# Use different keys for different environments
api_keys = {
    'development': os.getenv('SENTINEL_DEV_KEY'),
    'staging': os.getenv('SENTINEL_STAGING_KEY'),
    'production': os.getenv('SENTINEL_PROD_KEY')
}

sentinel = SentinelClient(api_key=api_keys[os.getenv('ENV', 'development')])
```

### JWT Token Handling

```python
import jwt
from datetime import datetime, timedelta

def generate_jwt(user_id, permissions):
    """Generate a secure JWT token."""
    payload = {
        'sub': user_id,
        'permissions': permissions,
        'iat': datetime.utcnow(),
        'exp': datetime.utcnow() + timedelta(hours=1),
        'jti': str(uuid.uuid4())  # Unique token ID
    }
    
    return jwt.encode(payload, SECRET_KEY, algorithm='HS256')

def verify_jwt(token):
    """Verify and decode JWT token."""
    try:
        payload = jwt.decode(token, SECRET_KEY, algorithms=['HS256'])
        
        # Check if token is blacklisted
        if is_token_blacklisted(payload['jti']):
            raise jwt.InvalidTokenError('Token is blacklisted')
        
        return payload
    except jwt.ExpiredSignatureError:
        raise AuthenticationError('Token has expired')
    except jwt.InvalidTokenError as e:
        raise AuthenticationError(f'Invalid token: {e}')
```

### Session Management

```python
import secrets
from datetime import datetime, timedelta

class SessionManager:
    def __init__(self):
        self.sessions = {}
        self.session_timeout = timedelta(hours=24)
    
    def create_session(self, user_id):
        session_id = secrets.token_urlsafe(32)
        self.sessions[session_id] = {
            'user_id': user_id,
            'created_at': datetime.utcnow(),
            'last_activity': datetime.utcnow(),
            'ip_address': None,
            'user_agent': None
        }
        return session_id
    
    def validate_session(self, session_id, ip_address, user_agent):
        if session_id not in self.sessions:
            return False
        
        session = self.sessions[session_id]
        
        # Check timeout
        if datetime.utcnow() - session['last_activity'] > self.session_timeout:
            del self.sessions[session_id]
            return False
        
        # Check IP address (optional, can be relaxed for mobile users)
        if session['ip_address'] and session['ip_address'] != ip_address:
            # Potential session hijacking
            log_security_event('IP_MISMATCH', 'MEDIUM', {
                'session_id': session_id,
                'old_ip': session['ip_address'],
                'new_ip': ip_address
            })
        
        # Update activity
        session['last_activity'] = datetime.utcnow()
        return True
    
    def destroy_session(self, session_id):
        if session_id in self.sessions:
            del self.sessions[session_id]
```

## File Handling Security

### Secure File Upload

```python
import os
import hashlib
import magic
from werkzeug.utils import secure_filename

ALLOWED_EXTENSIONS = {'txt', 'pdf', 'png', 'jpg', 'jpeg', 'gif'}
MAX_FILE_SIZE = 16 * 1024 * 1024  # 16MB

def secure_upload(file):
    # Check file size
    file.seek(0, os.SEEK_END)
    size = file.tell()
    file.seek(0)
    
    if size > MAX_FILE_SIZE:
        raise ValidationError('File too large')
    
    # Check extension
    filename = secure_filename(file.filename)
    ext = filename.rsplit('.', 1)[1].lower() if '.' in filename else ''
    
    if ext not in ALLOWED_EXTENSIONS:
        raise ValidationError('File type not allowed')
    
    # Check actual file type (magic bytes)
    file_type = magic.from_buffer(file.stream.read(1024))
    file.stream.seek(0)
    
    if not is_allowed_mime_type(file_type):
        raise ValidationError('File type not allowed')
    
    # Scan with SENTINEL
    result = sentinel.scan_file(file.stream)
    
    if result['is_malicious']:
        log_security_event('MALICIOUS_FILE', 'HIGH', {
            'filename': filename,
            'threat': result['threat_name']
        })
        raise SecurityError('Malicious file detected')
    
    # Generate safe filename
    file_hash = hashlib.sha256(file.stream.read()).hexdigest()[:16]
    file.stream.seek(0)
    safe_filename = f"{file_hash}.{ext}"
    
    return safe_filename

def is_allowed_mime_type(file_type):
    allowed_types = [
        'text/plain',
        'application/pdf',
        'image/png',
        'image/jpeg',
        'image/gif'
    ]
    return any(t in file_type for t in allowed_types)
```

### Secure File Storage

```python
import os
from cryptography.fernet import Fernet

class SecureFileStorage:
    def __init__(self, storage_path, encryption_key):
        self.storage_path = storage_path
        self.cipher = Fernet(encryption_key)
        
        # Ensure storage directory exists
        os.makedirs(storage_path, mode=0o700, exist_ok=True)
    
    def store_file(self, filename, content):
        # Encrypt content
        encrypted = self.cipher.encrypt(content)
        
        # Store with restricted permissions
        filepath = os.path.join(self.storage_path, filename)
        with open(filepath, 'wb') as f:
            f.write(encrypted)
        
        # Set file permissions
        os.chmod(filepath, 0o600)
        
        return filepath
    
    def retrieve_file(self, filename):
        filepath = os.path.join(self.storage_path, filename)
        
        with open(filepath, 'rb') as f:
            encrypted = f.read()
        
        # Decrypt content
        decrypted = self.cipher.decrypt(encrypted)
        
        return decrypted
    
    def delete_file(self, filename):
        filepath = os.path.join(self.storage_path, filename)
        
        # Secure delete (overwrite before removal)
        if os.path.exists(filepath):
            size = os.path.getsize(filepath)
            with open(filepath, 'wb') as f:
                f.write(os.urandom(size))
            os.remove(filepath)
```

## Network Security

### TLS/SSL Configuration

```python
# Enforce HTTPS
@app.before_request
def enforce_https():
    if not request.is_secure:
        return redirect(request.url.replace('http://', 'https://'), code=301)

# Security headers
@app.after_request
def add_security_headers(response):
    # Prevent clickjacking
    response.headers['X-Frame-Options'] = 'DENY'
    
    # Prevent MIME type sniffing
    response.headers['X-Content-Type-Options'] = 'nosniff'
    
    # XSS Protection
    response.headers['X-XSS-Protection'] = '1; mode=block'
    
    # Content Security Policy
    response.headers['Content-Security-Policy'] = "default-src 'self'"
    
    # HSTS
    response.headers['Strict-Transport-Security'] = 'max-age=31536000; includeSubDomains'
    
    return response
```

### Rate Limiting

```python
from flask_limiter import Limiter
from flask_limiter.util import get_remote_address

# Configure rate limiting
limiter = Limiter(
    app=app,
    key_func=get_remote_address,
    default_limits=["200 per day", "50 per hour"],
    storage_uri="redis://localhost:6379"
)

# Custom rate limits for sensitive endpoints
@app.route('/api/login', methods=['POST'])
@limiter.limit("5 per minute")
def login():
    # Login logic
    pass

@app.route('/api/scan', methods=['POST'])
@limiter.limit("10 per minute")
def scan():
    # Scan logic
    pass

# Dynamic rate limiting based on threat score
@app.before_request
def dynamic_rate_limit():
    threat_score = get_threat_score(request.remote_addr)
    
    if threat_score > 0.8:
        # High risk - very limited
        limiter.limit("1 per minute")(lambda: None)()
    elif threat_score > 0.5:
        # Medium risk - limited
        limiter.limit("5 per minute")(lambda: None)()
```

### IP Blocking

```python
class IPBlocker:
    def __init__(self, redis_client):
        self.redis = redis_client
        self.block_duration = 3600  # 1 hour
    
    def block_ip(self, ip, reason, duration=None):
        duration = duration or self.block_duration
        key = f"blocked:{ip}"
        
        self.redis.setex(key, duration, json.dumps({
            'reason': reason,
            'blocked_at': datetime.utcnow().isoformat()
        }))
        
        log_security_event('IP_BLOCKED', 'HIGH', {
            'ip': ip,
            'reason': reason,
            'duration': duration
        })
    
    def is_blocked(self, ip):
        key = f"blocked:{ip}"
        return self.redis.exists(key)
    
    def get_block_info(self, ip):
        key = f"blocked:{ip}"
        data = self.redis.get(key)
        if data:
            return json.loads(data)
        return None
    
    def auto_block(self, ip, threat_type):
        # Automatic blocking for serious threats
        serious_threats = ['MALWARE', 'DDoS', 'BRUTE_FORCE', 'INJECTION']
        
        if threat_type in serious_threats:
            self.block_ip(ip, f'Auto-blocked: {threat_type}', duration=86400)
            return True
        
        return False
```

## API Security

### Input Validation

```python
from pydantic import BaseModel, validator, constr
from typing import Optional, List
import re

class ScanRequest(BaseModel):
    filename: constr(min_length=1, max_length=255)
    options: Optional[dict] = {}
    
    @validator('filename')
    def validate_filename(cls, v):
        # Check for path traversal
        if '..' in v or '/' in v or '\\' in v:
            raise ValueError('Invalid filename')
        return v
    
    @validator('options')
    def validate_options(cls, v):
        allowed_keys = {'deep_scan', 'timeout', 'threshold'}
        for key in v.keys():
            if key not in allowed_keys:
                raise ValueError(f'Invalid option: {key}')
        return v

class ThreatRequest(BaseModel):
    input_type: constr(regex='^(process|network|file|url)$')
    data: dict
    
    @validator('data')
    def validate_data(cls, v, values):
        input_type = values.get('input_type')
        
        if input_type == 'process':
            required = ['name', 'path']
            if not all(k in v for k in required):
                raise ValueError('Process requires name and path')
        
        elif input_type == 'url':
            url = v.get('url', '')
            if not re.match(r'^https?://', url):
                raise ValueError('Invalid URL format')
        
        return v

@app.route('/api/scan', methods=['POST'])
def scan():
    try:
        request_data = ScanRequest(**request.json)
    except ValueError as e:
        return {'error': str(e)}, 400
    
    # Process validated request
    result = sentinel.scan_file(request_data.filename, request_data.options)
    return {'result': result}
```

### Output Sanitization

```python
import bleach
from typing import Any

def sanitize_output(data: Any) -> Any:
    """Recursively sanitize output data."""
    if isinstance(data, str):
        # Sanitize HTML content
        return bleach.clean(data, strip=True)
    elif isinstance(data, dict):
        return {k: sanitize_output(v) for k, v in data.items()}
    elif isinstance(data, list):
        return [sanitize_output(item) for item in data]
    else:
        return data

@app.route('/api/results')
def get_results():
    results = fetch_results()
    
    # Sanitize before returning
    sanitized = sanitize_output(results)
    
    return {'results': sanitized}
```

### Error Handling

```python
import logging
from functools import wraps

logger = logging.getLogger('sentinel')

def handle_errors(f):
    @wraps(f)
    def decorated_function(*args, **kwargs):
        try:
            return f(*args, **kwargs)
        
        except ValidationError as e:
            logger.warning(f'Validation error: {e}')
            return {'error': str(e)}, 400
        
        except AuthenticationError as e:
            logger.warning(f'Authentication error: {e}')
            return {'error': 'Authentication failed'}, 401
        
        except RateLimitError as e:
            logger.warning(f'Rate limit exceeded: {e}')
            return {'error': 'Rate limit exceeded'}, 429
        
        except SentinelAPIError as e:
            logger.error(f'SENTINEL API error: {e}')
            return {'error': 'Security service unavailable'}, 503
        
        except Exception as e:
            logger.exception(f'Unexpected error: {e}')
            return {'error': 'Internal server error'}, 500
    
    return decorated_function
```

## Performance Optimization

### Caching

```python
from functools import lru_cache
import hashlib

# Cache threat scores
@lru_cache(maxsize=10000)
def get_cached_threat_score(file_hash: str) -> dict:
    """Cache threat scores by file hash."""
    return sentinel.scan_file_by_hash(file_hash)

def get_threat_score_with_cache(file_content: bytes) -> dict:
    """Get threat score with caching."""
    file_hash = hashlib.sha256(file_content).hexdigest()
    return get_cached_threat_score(file_hash)

# Cache allowed IPs
allowed_ips_cache = {}

def is_ip_allowed(ip: str) -> bool:
    """Check if IP is allowed with caching."""
    if ip in allowed_ips_cache:
        cache_entry = allowed_ips_cache[ip]
        if cache_entry['expires'] > time.time():
            return cache_entry['allowed']
    
    # Check from database
    allowed = check_ip_in_database(ip)
    
    allowed_ips_cache[ip] = {
        'allowed': allowed,
        'expires': time.time() + 300  # 5 minutes
    }
    
    return allowed
```

### Connection Pooling

```python
import requests
from requests.adapters import HTTPAdapter
from urllib3.util.retry import Retry

class SentinelClient:
    def __init__(self, api_key, base_url):
        self.api_key = api_key
        self.base_url = base_url
        
        # Create session with connection pooling
        self.session = requests.Session()
        
        # Configure retry strategy
        retry_strategy = Retry(
            total=3,
            backoff_factor=1,
            status_forcelist=[429, 500, 502, 503, 504]
        )
        
        adapter = HTTPAdapter(
            max_retries=retry_strategy,
            pool_connections=10,
            pool_maxsize=100
        )
        
        self.session.mount("https://", adapter)
        self.session.mount("http://", adapter)
    
    def scan_file(self, file_path):
        response = self.session.post(
            f"{self.base_url}/scan",
            files={'file': open(file_path, 'rb')},
            headers={'Authorization': f'Bearer {self.api_key}'}
        )
        return response.json()
```

### Asynchronous Processing

```python
import asyncio
from concurrent.futures import ThreadPoolExecutor
from functools import partial

executor = ThreadPoolExecutor(max_workers=10)

async def scan_files_async(files):
    """Scan multiple files asynchronously."""
    loop = asyncio.get_event_loop()
    
    tasks = [
        loop.run_in_executor(
            executor,
            partial(sentinel.scan_file, file)
        )
        for file in files
    ]
    
    results = await asyncio.gather(*tasks)
    return results

# Flask async route (Flask 2.0+)
@app.route('/api/scan-multiple', methods=['POST'])
async def scan_multiple():
    files = request.files.getlist('files')
    
    # Save files temporarily
    temp_files = []
    for file in files:
        temp_path = f"/tmp/{file.filename}"
        file.save(temp_path)
        temp_files.append(temp_path)
    
    try:
        results = await scan_files_async(temp_files)
        return {'results': results}
    finally:
        # Clean up
        for path in temp_files:
            os.remove(path)
```

## Monitoring and Logging

### Security Logging

```python
import logging
import json
from datetime import datetime

# Configure logging
logging.basicConfig(
    level=logging.INFO,
    handlers=[
        logging.FileHandler('security.log'),
        logging.StreamHandler()
    ]
)

security_logger = logging.getLogger('security')

def log_security_event(event_type, severity, details, metadata=None):
    """Log security events in structured format."""
    event = {
        'timestamp': datetime.utcnow().isoformat(),
        'event_type': event_type,
        'severity': severity,
        'details': details,
        'metadata': metadata or {},
        'source': {
            'ip': request.remote_addr,
            'user_agent': request.headers.get('User-Agent'),
            'path': request.path,
            'method': request.method
        }
    }
    
    # Log locally
    security_logger.info(json.dumps(event))
    
    # Send to SENTINEL
    sentinel.log_event(event)
    
    # Send to SIEM
    siem_client.send_event(event)
    
    return event
```

### Alerting

```python
import requests
from typing import Dict, List

class AlertManager:
    def __init__(self, config: Dict):
        self.config = config
        self.webhooks = config.get('webhooks', [])
        self.emails = config.get('emails', [])
        self.slack = config.get('slack')
    
    def send_alert(self, event: Dict):
        """Send alert through configured channels."""
        severity = event['severity']
        
        # Check severity threshold
        if severity in ['HIGH', 'CRITICAL']:
            self._send_webhooks(event)
            self._send_emails(event)
            self._send_slack(event)
        elif severity == 'MEDIUM':
            self._send_webhooks(event)
    
    def _send_webhooks(self, event: Dict):
        for webhook in self.webhooks:
            try:
                requests.post(
                    webhook['url'],
                    json=event,
                    timeout=5
                )
            except Exception as e:
                security_logger.error(f"Failed to send webhook: {e}")
    
    def _send_emails(self, event: Dict):
        for email in self.emails:
            if event['severity'] in email.get('severity', ['HIGH', 'CRITICAL']):
                send_email(
                    to=email['address'],
                    subject=f"[{event['severity']}] {event['event_type']}",
                    body=json.dumps(event, indent=2)
                )
    
    def _send_slack(self, event: Dict):
        if self.slack:
            color = {
                'CRITICAL': '#ff0000',
                'HIGH': '#ff6600',
                'MEDIUM': '#ffcc00',
                'LOW': '#36a64f'
            }.get(event['severity'], '#cccccc')
            
            payload = {
                'channel': self.slack['channel'],
                'attachments': [{
                    'color': color,
                    'title': event['event_type'],
                    'text': event['details'],
                    'fields': [
                        {'title': 'Severity', 'value': event['severity'], 'short': True},
                        {'title': 'Source IP', 'value': event['source']['ip'], 'short': True}
                    ]
                }]
            }
            
            requests.post(self.slack['webhook'], json=payload)

# Initialize alert manager
alert_manager = AlertManager({
    'webhooks': [{'url': 'https://your-webhook.com/alerts'}],
    'emails': [{'address': 'security@company.com', 'severity': ['HIGH', 'CRITICAL']}],
    'slack': {
        'webhook': 'https://hooks.slack.com/services/...',
        'channel': '#security-alerts'
    }
})
```

## Incident Response

### Automated Response

```python
class IncidentResponder:
    def __init__(self):
        self.responses = {
            'MALWARE': self.handle_malware,
            'DDoS': self.handle_ddos,
            'BRUTE_FORCE': self.handle_brute_force,
            'INJECTION': self.handle_injection
        }
    
    def respond(self, event: Dict):
        """Automated incident response."""
        threat_type = event.get('threat_type')
        
        if threat_type in self.responses:
            return self.responses[threat_type](event)
        
        return {'action': 'log_only'}
    
    def handle_malware(self, event: Dict):
        """Handle malware detection."""
        # Block the source
        ip_blocker.block_ip(
            event['source']['ip'],
            reason='Malware distribution',
            duration=86400
        )
        
        # Quarantine affected files
        quarantine_files(event.get('affected_files', []))
        
        # Notify security team
        alert_manager.send_alert({
            'severity': 'CRITICAL',
            'event_type': 'MALWARE_DETECTED',
            'details': event
        })
        
        return {'action': 'blocked', 'ip_blocked': True}
    
    def handle_ddos(self, event: Dict):
        """Handle DDoS attack."""
        # Enable rate limiting
        enable_strict_rate_limiting()
        
        # Block attacking IPs
        for ip in event.get('attacking_ips', []):
            ip_blocker.block_ip(ip, 'DDoS attack')
        
        # Enable additional protection
        sentinel.enable_ddos_protection()
        
        return {'action': 'mitigated', 'mode': 'ddos_protection'}
    
    def handle_brute_force(self, event: Dict):
        """Handle brute force attack."""
        ip = event['source']['ip']
        
        # Temporarily block
        ip_blocker.block_ip(
            ip,
            reason='Brute force attempt',
            duration=3600
        )
        
        # Require CAPTCHA for future attempts
        require_captcha(ip)
        
        return {'action': 'blocked', 'captcha_required': True}
    
    def handle_injection(self, event: Dict):
        """Handle injection attack."""
        ip = event['source']['ip']
        
        # Block the IP
        ip_blocker.block_ip(
            ip,
            reason='Injection attempt',
            duration=86400
        )
        
        # Log for forensic analysis
        log_for_forensics(event)
        
        return {'action': 'blocked', 'logged': True}

# Initialize responder
incident_responder = IncidentResponder()

# Use in event handler
def handle_security_event(event):
    # Log the event
    log_security_event(event['event_type'], event['severity'], event['details'])
    
    # Automated response
    response = incident_responder.respond(event)
    
    return response
```

## Compliance and Privacy

### Data Retention

```python
from datetime import datetime, timedelta

class DataRetentionManager:
    def __init__(self, retention_days=90):
        self.retention_days = retention_days
    
    def cleanup_old_data(self):
        """Remove data older than retention period."""
        cutoff = datetime.utcnow() - timedelta(days=self.retention_days)
        
        # Clean logs
        cleanup_logs(before=cutoff)
        
        # Clean security events
        cleanup_security_events(before=cutoff)
        
        # Clean session data
        cleanup_sessions(before=cutoff)
    
    def archive_old_data(self):
        """Archive old data before deletion."""
        cutoff = datetime.utcnow() - timedelta(days=self.retention_days)
        
        # Archive to cold storage
        archive_to_cold_storage(before=cutoff)

# GDPR compliance
def handle_data_deletion_request(user_id):
    """Handle GDPR right to be forgotten."""
    # Delete user data
    delete_user_data(user_id)
    
    # Delete logs containing user data
    delete_user_logs(user_id)
    
    # Delete from SENTINEL
    sentinel.delete_user_data(user_id)
    
    # Log the deletion
    log_security_event('GDPR_DELETION', 'INFO', {'user_id': user_id})
```

### Audit Logging

```python
def audit_log(action, user_id, details):
    """Log audit events for compliance."""
    event = {
        'timestamp': datetime.utcnow().isoformat(),
        'action': action,
        'user_id': user_id,
        'details': details,
        'source_ip': request.remote_addr,
        'user_agent': request.headers.get('User-Agent')
    }
    
    # Store in immutable audit log
    store_audit_event(event)
    
    return event

# Example usage
@app.route('/api/admin/users/<user_id>', methods=['DELETE'])
@require_admin
def delete_user(user_id):
    audit_log(
        action='USER_DELETE',
        user_id=current_user.id,
        details={'deleted_user_id': user_id}
    )
    
    # Delete user
    delete_user(user_id)
    
    return {'success': True}
```

## Production Deployment

### Configuration Management

```python
import os
from dataclasses import dataclass

@dataclass
class Config:
    """Production configuration."""
    # Environment
    ENV: str = os.getenv('ENV', 'development')
    DEBUG: bool = os.getenv('DEBUG', 'false').lower() == 'true'
    
    # SENTINEL
    SENTINEL_API_KEY: str = os.getenv('SENTINEL_API_KEY')
    SENTINEL_API_URL: str = os.getenv('SENTINEL_API_URL', 'https://api.sentinel.ai/v1')
    SENTINEL_TIMEOUT: int = int(os.getenv('SENTINEL_TIMEOUT', '30'))
    
    # Security
    SECRET_KEY: str = os.getenv('SECRET_KEY')
    ALLOWED_HOSTS: list = os.getenv('ALLOWED_HOSTS', '*').split(',')
    
    # Rate limiting
    RATE_LIMIT_REQUESTS: int = int(os.getenv('RATE_LIMIT_REQUESTS', '1000'))
    RATE_LIMIT_WINDOW: int = int(os.getenv('RATE_LIMIT_WINDOW', '60'))
    
    # Logging
    LOG_LEVEL: str = os.getenv('LOG_LEVEL', 'INFO')
    LOG_FILE: str = os.getenv('LOG_FILE', 'app.log')

config = Config()

# Validate configuration
def validate_config():
    if not config.SENTINEL_API_KEY:
        raise ValueError("SENTINEL_API_KEY is required")
    if not config.SECRET_KEY:
        raise ValueError("SECRET_KEY is required")
    if config.ENV == 'production' and config.DEBUG:
        raise ValueError("DEBUG must be false in production")
```

### Health Checks

```python
@app.route('/health')
def health_check():
    """Health check endpoint."""
    checks = {
        'status': 'healthy',
        'timestamp': datetime.utcnow().isoformat(),
        'checks': {}
    }
    
    # Check SENTINEL connection
    try:
        sentinel.ping()
        checks['checks']['sentinel'] = 'healthy'
    except Exception as e:
        checks['checks']['sentinel'] = f'unhealthy: {e}'
        checks['status'] = 'unhealthy'
    
    # Check database
    try:
        db.ping()
        checks['checks']['database'] = 'healthy'
    except Exception as e:
        checks['checks']['database'] = f'unhealthy: {e}'
        checks['status'] = 'unhealthy'
    
    # Check Redis
    try:
        redis.ping()
        checks['checks']['redis'] = 'healthy'
    except Exception as e:
        checks['checks']['redis'] = f'unhealthy: {e}'
        checks['status'] = 'unhealthy'
    
    status_code = 200 if checks['status'] == 'healthy' else 503
    return checks, status_code
```

### Graceful Shutdown

```python
import signal
import sys

def graceful_shutdown(signum, frame):
    """Handle graceful shutdown."""
    logging.info("Shutting down gracefully...")
    
    # Stop accepting new connections
    stop_accepting_connections()
    
    # Complete pending requests
    wait_for_pending_requests(timeout=30)
    
    # Close database connections
    close_database_connections()
    
    # Flush logs
    logging.shutdown()
    
    sys.exit(0)

# Register signal handlers
signal.signal(signal.SIGTERM, graceful_shutdown)
signal.signal(signal.SIGINT, graceful_shutdown)
```

## Additional Resources

- [API Documentation](https://docs.sentinel.ai/api)
- [SDK Reference](https://docs.sentinel.ai/sdk)
- [Security Whitepaper](https://docs.sentinel.ai/whitepaper)
- [Compliance Guide](https://docs.sentinel.ai/compliance)
- [Support Portal](https://support.sentinel.ai)