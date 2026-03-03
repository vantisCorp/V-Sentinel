# SENTINEL Sample Applications

This directory contains practical examples and sample applications demonstrating how to integrate SENTINEL Security System into various real-world scenarios.

## 📋 Table of Contents

- [Overview](#overview)
- [Getting Started](#getting-started)
- [Sample Applications](#sample-applications)
- [Integration Guides](#integration-guides)
- [Running the Examples](#running-the-examples)
- [Contributing](#contributing)

## 🎯 Overview

These examples demonstrate key features and integrations:
- **Threat Detection API** - Real-time threat prediction and blocking
- **File Scanning Integration** - Automated malware scanning workflows
- **Network Protection** - DDoS protection and intrusion detection
- **Process Monitoring** - Behavioral analysis and anomaly detection
- **Gaming Optimization** - Trusted Handshake protocol implementation
- **SIEM Integration** - Centralized security monitoring

## 🚀 Getting Started

### Prerequisites

- SENTINEL API key (get from [https://api.sentinel.ai](https://api.sentinel.ai))
- Python 3.8+ or Node.js 16+
- Basic understanding of REST APIs

### Installation

```bash
# Clone the repository
git clone https://github.com/vantisCorp/V-Sentinel.git
cd V-Sentinel/examples

# Install Python dependencies (for Python examples)
pip install -r requirements.txt

# Install Node.js dependencies (for JavaScript examples)
cd javascript-examples && npm install
```

## 💼 Sample Applications

### 1. Web Application Protection (Python)
**Location:** `web_app_protection/`

A Flask web application that integrates SENTINEL for:
- File upload scanning
- Request threat scoring
- Automated malware blocking
- Real-time security logging

**Features:**
- Upload file scanning before processing
- Request validation and threat scoring
- Customizable threat thresholds
- Detailed security events logging

### 2. Gaming Server Protection (Node.js)
**Location:** `gaming_server_protection/`

A Node.js game server implementing:
- Trusted Handshake protocol
- Real-time player verification
- Anti-DDoS protection
- RAM defolding optimization

**Features:**
- Zero-latency player authentication
- DDoS attack mitigation
- Resource optimization for gaming
- Real-time threat blocking

### 3. API Gateway Security (Python)
**Location:** `api_gateway_security/`

An API gateway that adds security layer:
- Request authentication and validation
- Rate limiting and abuse prevention
- Threat prediction for API calls
- Automated response blocking

**Features:**
- JWT and API key authentication
- Intelligent rate limiting
- SQL injection prevention
- XSS attack detection

### 4. Cloud Infrastructure Protection (Terraform)
**Location:** `cloud_protection/`

Terraform templates for deploying:
- Kubernetes cluster with SENTINEL
- Cloud-native security policies
- Automated threat response
- Federated learning nodes

**Features:**
- Infrastructure as Code
- Auto-scaling security
- Multi-cloud deployment
- Policy-as-code security

### 5. Mobile App Security (React Native)
**Location:** `mobile_app_security/`

React Native application with:
- Runtime protection
- API communication security
- Device integrity checks
- Behavioral authentication

**Features:**
- Zero-code integration
- Real-time threat detection
- Low battery consumption
- Offline protection

### 6. Enterprise SIEM Integration (Python)
**Location:** `siem_integration/`

Python scripts for integrating with:
- Splunk
- Elasticsearch
- Azure Sentinel
- AWS Security Hub

**Features:**
- Real-time threat forwarding
- Automated alert enrichment
- Custom dashboard creation
- Incident response automation

## 📚 Integration Guides

### Web Application Integration
**Guide:** `docs/web_app_integration.md`

Learn how to:
- Protect file uploads
- Validate incoming requests
- Block malicious traffic
- Monitor security events

### Gaming Server Integration
**Guide:** `docs/gaming_server_integration.md`

Learn how to:
- Implement Trusted Handshake
- Protect against DDoS attacks
- Optimize gaming performance
- Verify player authenticity

### API Security Integration
**Guide:** `docs/api_security_integration.md`

Learn how to:
- Secure REST endpoints
- Implement rate limiting
- Prevent common attacks
- Monitor API threats

### Cloud Integration
**Guide:** `docs/cloud_integration.md`

Learn how to:
- Deploy to Kubernetes
- Configure cloud policies
- Set up auto-scaling
- Monitor cloud threats

## 🏃 Running the Examples

### Running Python Examples

```bash
# Navigate to the example directory
cd web_app_protection

# Set your API key
export SENTINEL_API_KEY="your-api-key-here"

# Run the application
python app.py

# Access the application
open http://localhost:5000
```

### Running Node.js Examples

```bash
# Navigate to the example directory
cd gaming_server_protection

# Set your API key
export SENTINEL_API_KEY="your-api-key-here"

# Install dependencies
npm install

# Run the server
npm start

# Connect to the server
telnet localhost 3000
```

### Running Terraform Examples

```bash
# Navigate to the example directory
cd cloud_protection

# Configure your credentials
export AWS_ACCESS_KEY_ID="your-access-key"
export AWS_SECRET_ACCESS_KEY="your-secret-key"

# Initialize Terraform
terraform init

# Review the plan
terraform plan

# Apply the configuration
terraform apply
```

## 🔧 Configuration

All examples use environment variables for configuration:

```bash
# Required
SENTINEL_API_KEY="your-api-key"
SENTINEL_API_URL="https://api.sentinel.ai/v1"

# Optional
SENTINEL_LOG_LEVEL="info"
SENTINEL_TIMEOUT=30
SENTINEL_MAX_RETRIES=3
```

## 📖 Documentation

- **API Documentation:** [https://docs.sentinel.ai/api](https://docs.sentinel.ai/api)
- **SDK Documentation:** [https://docs.sentinel.ai/sdk](https://docs.sentinel.ai/sdk)
- **Getting Started:** [https://docs.sentinel.ai/getting-started](https://docs.sentinel.ai/getting-started)

## 🤝 Contributing

We welcome contributions! To add a new example:

1. Create a new directory under `examples/`
2. Add your application code
3. Include a `README.md` with setup instructions
4. Update this `README.md` with your example
5. Submit a pull request

## 📝 License

All examples are provided under the MIT License. See LICENSE file for details.

## 🆘 Support

- **Documentation:** [https://docs.sentinel.ai](https://docs.sentinel.ai)
- **GitHub Issues:** [https://github.com/vantisCorp/V-Sentinel/issues](https://github.com/vantisCorp/V-Sentinel/issues)
- **Community Forum:** [https://community.sentinel.ai](https://community.sentinel.ai)
- **Email:** support@sentinel.ai

## 🔗 Related Resources

- [SENTINEL Official Website](https://sentinel.ai)
- [Developer Portal](https://developers.sentinel.ai)
- [API Reference](https://api.sentinel.ai/docs)
- [Blog](https://blog.sentinel.ai)
- [YouTube Channel](https://youtube.com/@sentinel-ai)