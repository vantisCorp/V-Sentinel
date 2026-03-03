"""
SENTINEL Web Application Protection Example

This Flask application demonstrates how to integrate SENTINEL Security System
for comprehensive web application protection including:
- File upload scanning
- Request threat scoring
- Automated malware blocking
- Real-time security logging

Author: SENTINEL Security Team
License: MIT
"""

import os
import hashlib
import logging
from datetime import datetime
from functools import wraps
from flask import Flask, request, jsonify, render_template, redirect, url_for
from flask_limiter import Limiter
from flask_limiter.util import get_remote_address

# Import SENTINEL SDK
try:
    from sentinel_sdk import SentinelClient
    SENTINEL_AVAILABLE = True
except ImportError:
    SENTINEL_AVAILABLE = False
    print("Warning: sentinel_sdk not installed. Using mock client for demo.")

# Configure logging
logging.basicConfig(
    level=logging.INFO,
    format='%(asctime)s - %(name)s - %(levelname)s - %(message)s',
    handlers=[
        logging.FileHandler('security.log'),
        logging.StreamHandler()
    ]
)
logger = logging.getLogger(__name__)

# Initialize Flask app
app = Flask(__name__)
app.config['MAX_CONTENT_LENGTH'] = 16 * 1024 * 1024  # 16MB max file size
app.config['UPLOAD_FOLDER'] = 'uploads'
app.config['ALLOWED_EXTENSIONS'] = {'txt', 'pdf', 'png', 'jpg', 'jpeg', 'gif', 'doc', 'docx'}

# Rate limiting
limiter = Limiter(
    app=app,
    key_func=get_remote_address,
    default_limits=["200 per day", "50 per hour"]
)

# Initialize SENTINEL client
SENTINEL_API_KEY = os.environ.get('SENTINEL_API_KEY', 'demo-key')
SENTINEL_API_URL = os.environ.get('SENTINEL_API_URL', 'https://api.sentinel.ai/v1')

if SENTINEL_AVAILABLE:
    sentinel = SentinelClient(
        api_key=SENTINEL_API_KEY,
        base_url=SENTINEL_API_URL
    )
else:
    # Mock client for demo purposes
    class MockSentinelClient:
        def predict_threat(self, **kwargs):
            return {'threat_score': 0.1, 'threat_type': 'benign', 'confidence': 0.95}
        
        def scan_file(self, file_path, **kwargs):
            return {
                'is_malicious': False,
                'threat_name': None,
                'confidence': 0.99,
                'scan_time_ms': 45
            }
        
        def analyze_request(self, request_data, **kwargs):
            return {'risk_score': 0.05, 'risk_factors': [], 'recommendation': 'allow'}
    
    sentinel = MockSentinelClient()

# Security event storage (in production, use a proper database)
security_events = []
blocked_ips = set()


def allowed_file(filename):
    """Check if file has allowed extension."""
    return '.' in filename and \
           filename.rsplit('.', 1)[1].lower() in app.config['ALLOWED_EXTENSIONS']


def log_security_event(event_type, severity, details, request_data=None):
    """Log security events for monitoring and analysis."""
    event = {
        'timestamp': datetime.utcnow().isoformat(),
        'event_type': event_type,
        'severity': severity,
        'details': details,
        'source_ip': get_remote_address(),
        'user_agent': request.headers.get('User-Agent', 'Unknown') if request else 'N/A',
        'request_path': request.path if request else 'N/A'
    }
    security_events.append(event)
    logger.info(f"Security Event: {event_type} - {severity} - {details}")
    return event


def threat_check(f):
    """Decorator to check request for threats before processing."""
    @wraps(f)
    def decorated_function(*args, **kwargs):
        # Check if IP is blocked
        client_ip = get_remote_address()
        if client_ip in blocked_ips:
            log_security_event(
                'BLOCKED_IP_ACCESS',
                'HIGH',
                f'Blocked IP {client_ip} attempted access'
            )
            return jsonify({'error': 'Access denied'}), 403
        
        # Analyze request for threats
        request_data = {
            'method': request.method,
            'path': request.path,
            'headers': dict(request.headers),
            'args': dict(request.args),
            'form': dict(request.form) if request.form else None,
            'remote_addr': client_ip
        }
        
        try:
            threat_result = sentinel.analyze_request(request_data)
            
            if threat_result.get('risk_score', 0) > 0.8:
                log_security_event(
                    'HIGH_RISK_REQUEST',
                    'CRITICAL',
                    f'High risk request detected: {threat_result}',
                    request_data
                )
                # Optionally block the IP
                # blocked_ips.add(client_ip)
                return jsonify({'error': 'Request blocked by security policy'}), 403
            
            if threat_result.get('risk_score', 0) > 0.5:
                log_security_event(
                    'MEDIUM_RISK_REQUEST',
                    'MEDIUM',
                    f'Medium risk request: {threat_result}',
                    request_data
                )
                # Log but allow the request
                request.threat_score = threat_result.get('risk_score')
            
        except Exception as e:
            logger.error(f"Error analyzing request: {e}")
        
        return f(*args, **kwargs)
    return decorated_function


@app.route('/')
def index():
    """Home page with security dashboard."""
    return render_template('index.html',
        total_requests=len(security_events),
        blocked_count=len([e for e in security_events if e['severity'] in ['HIGH', 'CRITICAL']]),
        recent_events=security_events[-10:]
    )


@app.route('/health')
def health():
    """Health check endpoint."""
    return jsonify({
        'status': 'healthy',
        'sentinel_connected': SENTINEL_AVAILABLE,
        'timestamp': datetime.utcnow().isoformat()
    })


@app.route('/api/scan', methods=['POST'])
@limiter.limit("10 per minute")
@threat_check
def scan_file():
    """
    Scan an uploaded file for malware.
    
    Request:
        - file: The file to scan (multipart/form-data)
    
    Response:
        - is_malicious: Boolean indicating if file is malicious
        - threat_name: Name of detected threat (if any)
        - confidence: Confidence score (0-1)
        - scan_time_ms: Time taken to scan in milliseconds
    """
    if 'file' not in request.files:
        return jsonify({'error': 'No file provided'}), 400
    
    file = request.files['file']
    
    if file.filename == '':
        return jsonify({'error': 'No file selected'}), 400
    
    if not allowed_file(file.filename):
        return jsonify({'error': 'File type not allowed'}), 400
    
    # Save file temporarily for scanning
    filename = os.path.basename(file.filename)
    filepath = os.path.join(app.config['UPLOAD_FOLDER'], filename)
    os.makedirs(app.config['UPLOAD_FOLDER'], exist_ok=True)
    file.save(filepath)
    
    try:
        # Calculate file hash
        with open(filepath, 'rb') as f:
            file_hash = hashlib.sha256(f.read()).hexdigest()
        
        # Scan with SENTINEL
        scan_result = sentinel.scan_file(
            filepath,
            options={
                'deep_scan': True,
                'timeout': 30
            }
        )
        
        # Log the scan event
        log_security_event(
            'FILE_SCAN',
            'HIGH' if scan_result.get('is_malicious') else 'LOW',
            f'File scanned: {filename}, Result: {"malicious" if scan_result.get("is_malicious") else "clean"}',
            {
                'filename': filename,
                'file_hash': file_hash,
                'scan_result': scan_result
            }
        )
        
        return jsonify({
            'filename': filename,
            'file_hash': file_hash,
            'is_malicious': scan_result.get('is_malicious', False),
            'threat_name': scan_result.get('threat_name'),
            'confidence': scan_result.get('confidence', 0),
            'scan_time_ms': scan_result.get('scan_time_ms', 0)
        })
    
    finally:
        # Clean up the temporary file
        if os.path.exists(filepath):
            os.remove(filepath)


@app.route('/api/threat-score', methods=['POST'])
@limiter.limit("60 per minute")
@threat_check
def get_threat_score():
    """
    Get threat score for a given input.
    
    Request body:
        - input_type: 'process', 'network', 'file', 'url'
        - data: The data to analyze
    
    Response:
        - threat_score: 0-1 score indicating threat level
        - threat_type: Classification of threat
        - confidence: Confidence score (0-1)
        - indicators: List of threat indicators
    """
    data = request.get_json()
    
    if not data or 'input_type' not in data or 'data' not in data:
        return jsonify({'error': 'Invalid request. Required: input_type, data'}), 400
    
    input_type = data['input_type']
    input_data = data['data']
    
    # Prepare features for threat prediction
    features = {
        'input_type': input_type,
        'data': input_data
    }
    
    # Add specific features based on input type
    if input_type == 'process':
        features.update({
            'process_name': input_data.get('name', 'unknown'),
            'process_path': input_data.get('path', ''),
            'memory_usage': input_data.get('memory_usage', 0),
            'cpu_usage': input_data.get('cpu_usage', 0),
            'parent_process': input_data.get('parent', ''),
            'network_connections': input_data.get('network_connections', 0)
        })
    elif input_type == 'network':
        features.update({
            'source_ip': input_data.get('source_ip', ''),
            'dest_ip': input_data.get('dest_ip', ''),
            'source_port': input_data.get('source_port', 0),
            'dest_port': input_data.get('dest_port', 0),
            'protocol': input_data.get('protocol', 'tcp'),
            'bytes_transferred': input_data.get('bytes', 0)
        })
    elif input_type == 'url':
        features.update({
            'url': input_data.get('url', ''),
            'domain': input_data.get('domain', ''),
            'path': input_data.get('path', '/'),
            'query_params': input_data.get('query', '')
        })
    
    # Get threat prediction from SENTINEL
    prediction = sentinel.predict_threat(features=features)
    
    # Log the prediction
    log_security_event(
        'THREAT_PREDICTION',
        'HIGH' if prediction.get('threat_score', 0) > 0.7 else 'LOW',
        f'Threat prediction for {input_type}: score={prediction.get("threat_score")}',
        prediction
    )
    
    return jsonify({
        'threat_score': prediction.get('threat_score', 0),
        'threat_type': prediction.get('threat_type', 'unknown'),
        'confidence': prediction.get('confidence', 0),
        'indicators': prediction.get('indicators', []),
        'recommendation': prediction.get('recommendation', 'monitor')
    })


@app.route('/api/events', methods=['GET'])
def get_security_events():
    """
    Get recent security events.
    
    Query parameters:
        - limit: Maximum number of events to return (default: 50)
        - severity: Filter by severity (LOW, MEDIUM, HIGH, CRITICAL)
        - type: Filter by event type
    """
    limit = request.args.get('limit', 50, type=int)
    severity = request.args.get('severity')
    event_type = request.args.get('type')
    
    events = security_events.copy()
    
    # Apply filters
    if severity:
        events = [e for e in events if e['severity'] == severity.upper()]
    if event_type:
        events = [e for e in events if e['event_type'] == event_type.upper()]
    
    # Sort by timestamp (newest first) and limit
    events = sorted(events, key=lambda x: x['timestamp'], reverse=True)[:limit]
    
    return jsonify({
        'total': len(security_events),
        'returned': len(events),
        'events': events
    })


@app.route('/api/block-ip', methods=['POST'])
@limiter.limit("10 per minute")
def block_ip():
    """
    Block an IP address.
    
    Request body:
        - ip: IP address to block
        - reason: Reason for blocking
        - duration: Duration in seconds (optional, 0 = permanent)
    """
    data = request.get_json()
    
    if not data or 'ip' not in data:
        return jsonify({'error': 'IP address required'}), 400
    
    ip = data['ip']
    reason = data.get('reason', 'Manual block')
    duration = data.get('duration', 0)
    
    blocked_ips.add(ip)
    
    log_security_event(
        'IP_BLOCKED',
        'HIGH',
        f'IP {ip} blocked. Reason: {reason}. Duration: {duration}s'
    )
    
    return jsonify({
        'success': True,
        'ip': ip,
        'reason': reason,
        'blocked_count': len(blocked_ips)
    })


@app.route('/api/unblock-ip', methods=['POST'])
def unblock_ip():
    """
    Unblock an IP address.
    
    Request body:
        - ip: IP address to unblock
    """
    data = request.get_json()
    
    if not data or 'ip' not in data:
        return jsonify({'error': 'IP address required'}), 400
    
    ip = data['ip']
    
    if ip in blocked_ips:
        blocked_ips.remove(ip)
        log_security_event('IP_UNBLOCKED', 'LOW', f'IP {ip} unblocked')
        return jsonify({'success': True, 'ip': ip})
    
    return jsonify({'error': 'IP not in blocklist'}), 404


@app.errorhandler(413)
def request_entity_too_large(error):
    """Handle file too large error."""
    log_security_event('FILE_TOO_LARGE', 'MEDIUM', f'File upload exceeded size limit')
    return jsonify({'error': 'File too large. Maximum size is 16MB'}), 413


@app.errorhandler(429)
def ratelimit_handler(e):
    """Handle rate limit exceeded."""
    log_security_event('RATE_LIMIT_EXCEEDED', 'MEDIUM', f'Rate limit exceeded: {e.description}')
    return jsonify({'error': 'Rate limit exceeded', 'retry_after': str(e.description)}), 429


if __name__ == '__main__':
    # Create upload directory
    os.makedirs(app.config['UPLOAD_FOLDER'], exist_ok=True)
    
    # Run the application
    port = int(os.environ.get('PORT', 5000))
    debug = os.environ.get('FLASK_DEBUG', 'false').lower() == 'true'
    
    print(f"""
    ╔══════════════════════════════════════════════════════════════╗
    ║          SENTINEL Web Application Protection Demo            ║
    ╠══════════════════════════════════════════════════════════════╣
    ║  Starting server on port {port}...                              ║
    ║  SENTINEL SDK: {'Connected' if SENTINEL_AVAILABLE else 'Mock Mode'}                                   ║
    ║  API Key: {SENTINEL_API_KEY[:8]}...                                        ║
    ╚══════════════════════════════════════════════════════════════╝
    """)
    
    app.run(host='0.0.0.0', port=port, debug=debug)