# Security Policy

## Supported Versions

| Version | Supported          |
| ------- | ------------------ |
| 2.1.x   | :white_check_mark: |
| 2.0.x   | :white_check_mark: |
| < 2.0   | :x:                |

## Reporting a Vulnerability

**Please do NOT report security vulnerabilities through public GitHub issues.**

Instead, please report them via email to: **security@vantiscorp.com**

You should receive a response within 48 hours. If for some reason you do not, please follow up via email to ensure we received your original message.

### What to Include

- Type of issue (e.g., buffer overflow, SQL injection, cross-site scripting, etc.)
- Full paths of source file(s) related to the manifestation of the issue
- The location of the affected source code (tag/branch/commit or direct URL)
- Any special configuration required to reproduce the issue
- Step-by-step instructions to reproduce the issue
- Proof-of-concept or exploit code (if possible)
- Impact of the issue, including how an attacker might exploit it

### Response Timeline

- **48 hours**: Initial acknowledgment of your report
- **1 week**: Preliminary assessment and severity classification
- **2 weeks**: Detailed response with planned remediation
- **30 days**: Fix deployed (for critical vulnerabilities)

### Scope

This security policy applies to the V-Sentinel core framework, all workspace crates, official SDKs, CLI tools, installers, Docker images, deployment configurations, and GitHub Actions workflows.

## Security Best Practices

1. Always use the latest stable release
2. Enable all security features including PQC (Post-Quantum Cryptography)
3. Follow the hardening guide in `docs/security/`
4. Regularly update dependencies
5. Use the built-in security audit tools (`scripts/security_audit.sh`)
6. Enable monitoring and alerting for security events