#!/bin/bash
#
# V-Sentinel Security Audit Script
#
# Automated security audit and vulnerability scanning

set -e

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m'

# Configuration
REPORTS_DIR="security_reports"
TIMESTAMP=$(date +"%Y%m%d_%H%M%S")
REPORT_FILE="${REPORTS_DIR}/audit_${TIMESTAMP}.md"

# Create reports directory
mkdir -p "${REPORTS_DIR}"

# Functions
log_info() {
    echo -e "${BLUE}[INFO]${NC} $1"
}

log_success() {
    echo -e "${GREEN}[SUCCESS]${NC} $1"
}

log_warning() {
    echo -e "${YELLOW}[WARNING]${NC} $1"
}

log_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

# Initialize report
init_report() {
    cat > "${REPORT_FILE}" << EOF
# V-Sentinel Security Audit Report

**Date:** $(date +"%Y-%m-%d %H:%M:%S")
**Auditor:** Automated Security Audit
**Version:** 1.0.0

---

## Executive Summary

This report contains the results of the automated security audit for V-Sentinel.

---

## 1. Code Security Analysis

EOF
}

# Check for hardcoded secrets
check_secrets() {
    log_info "Checking for hardcoded secrets..."
    
    local findings=0
    local output_file="${REPORTS_DIR}/secrets_${TIMESTAMP}.txt"
    
    # Check for potential secrets
    grep -rn --include="*.rs" --include="*.toml" --include="*.yaml" --include="*.json" \
        -E "(password|secret|api_key|token|private_key)\s*=\s*[&quot;'][^&quot;']+[&quot;']" \
        . 2>/dev/null | grep -v ".git" | grep -v "example" | grep -v "test" > "${output_file}" || true
    
    findings=$(wc -l < "${output_file}")
    
    echo "### 1.1 Hardcoded Secrets Check" >> "${REPORT_FILE}"
    echo "" >> "${REPORT_FILE}"
    
    if [ "$findings" -gt 0 ]; then
        echo "⚠️ **Potential hardcoded secrets found: ${findings}**" >> "${REPORT_FILE}"
        echo "" >> "${REPORT_FILE}"
        echo "\`\`\`" >> "${REPORT_FILE}"
        head -20 "${output_file}" >> "${REPORT_FILE}"
        echo "\`\`\`" >> "${REPORT_FILE}"
        log_warning "Found ${findings} potential hardcoded secrets"
    else
        echo "✅ **No hardcoded secrets detected**" >> "${REPORT_FILE}"
        log_success "No hardcoded secrets found"
    fi
    
    echo "" >> "${REPORT_FILE}"
}

# Check dependencies for vulnerabilities
check_dependencies() {
    log_info "Checking dependencies for vulnerabilities..."
    
    echo "### 1.2 Dependency Vulnerability Check" >> "${REPORT_FILE}"
    echo "" >> "${REPORT_FILE}"
    
    if [ -f "Cargo.lock" ]; then
        # Run cargo audit if available
        if command -v cargo-audit &> /dev/null; then
            cargo audit > "${REPORTS_DIR}/cargo_audit_${TIMESTAMP}.txt" 2>&1 || true
            if grep -q "No known security vulnerabilities" "${REPORTS_DIR}/cargo_audit_${TIMESTAMP}.txt"; then
                echo "✅ **No vulnerable dependencies found**" >> "${REPORT_FILE}"
                log_success "No vulnerable dependencies"
            else
                echo "⚠️ **Vulnerable dependencies found**" >> "${REPORT_FILE}"
                echo "" >> "${REPORT_FILE}"
                echo "\`\`\`" >> "${REPORT_FILE}"
                cat "${REPORTS_DIR}/cargo_audit_${TIMESTAMP}.txt" >> "${REPORT_FILE}"
                echo "\`\`\`" >> "${REPORT_FILE}"
                log_warning "Vulnerable dependencies found"
            fi
        else
            echo "ℹ️ cargo-audit not available, skipping dependency check" >> "${REPORT_FILE}"
            log_warning "cargo-audit not installed"
        fi
    else
        echo "ℹ️ Cargo.lock not found, skipping dependency check" >> "${REPORT_FILE}"
    fi
    
    echo "" >> "${REPORT_FILE}"
}

# Check for insecure code patterns
check_code_patterns() {
    log_info "Checking for insecure code patterns..."
    
    echo "### 1.3 Insecure Code Pattern Check" >> "${REPORT_FILE}"
    echo "" >> "${REPORT_FILE}"
    
    local findings=0
    local output_file="${REPORTS_DIR}/patterns_${TIMESTAMP}.txt"
    
    # Check for insecure patterns
    grep -rn --include="*.rs" \
        -E "(unwrap\(\)|expect\(\)|panic!|unsafe)" \
        src/ 2>/dev/null > "${output_file}" || true
    
    findings=$(wc -l < "${output_file}")
    
    echo "Found ${findings} potentially unsafe code patterns:" >> "${REPORT_FILE}"
    echo "" >> "${REPORT_FILE}"
    echo "- unwrap() calls: $(grep -c "unwrap()" "${output_file}" || echo 0)" >> "${REPORT_FILE}"
    echo "- expect() calls: $(grep -c "expect(" "${output_file}" || echo 0)" >> "${REPORT_FILE}"
    echo "- panic! macros: $(grep -c "panic!" "${output_file}" || echo 0)" >> "${REPORT_FILE}"
    echo "- unsafe blocks: $(grep -c "unsafe" "${output_file}" || echo 0)" >> "${REPORT_FILE}"
    
    log_info "Found ${findings} potentially unsafe patterns"
    
    echo "" >> "${REPORT_FILE}"
}

# Check file permissions
check_permissions() {
    log_info "Checking file permissions..."
    
    echo "### 1.4 File Permission Check" >> "${REPORT_FILE}"
    echo "" >> "${REPORT_FILE}"
    
    local issues=0
    
    # Check for world-writable files
    local writable=$(find . -type f -perm -002 -not -path "./.git/*" 2>/dev/null | wc -l)
    
    if [ "$writable" -gt 0 ]; then
        echo "⚠️ Found ${writable} world-writable files" >> "${REPORT_FILE}"
        issues=$((issues + writable))
    else
        echo "✅ No world-writable files found" >> "${REPORT_FILE}"
    fi
    
    # Check for sensitive files with loose permissions
    if [ -f "config/sentinel.toml" ]; then
        local config_perms=$(stat -c %a config/sentinel.toml 2>/dev/null || echo "644")
        echo "Configuration file permissions: ${config_perms}" >> "${REPORT_FILE}"
    fi
    
    log_info "File permission check completed"
    
    echo "" >> "${REPORT_FILE}"
}

# Check SSL/TLS configuration
check_ssl_tls() {
    log_info "Checking SSL/TLS configuration..."
    
    echo "### 1.5 SSL/TLS Configuration Check" >> "${REPORT_FILE}"
    echo "" >> "${REPORT_FILE}"
    
    if [ -f "deploy/nginx/nginx.conf" ]; then
        local ssl_protocols=$(grep -E "ssl_protocols" deploy/nginx/nginx.conf || echo "Not found")
        local ssl_ciphers=$(grep -E "ssl_ciphers" deploy/nginx/nginx.conf || echo "Not found")
        
        echo "**Configured SSL Protocols:**" >> "${REPORT_FILE}"
        echo "\`\`\`" >> "${REPORT_FILE}"
        echo "${ssl_protocols}" >> "${REPORT_FILE}"
        echo "\`\`\`" >> "${REPORT_FILE}"
        echo "" >> "${REPORT_FILE}"
        
        echo "**Configured Cipher Suites:**" >> "${REPORT_FILE}"
        echo "\`\`\`" >> "${REPORT_FILE}"
        echo "${ssl_ciphers}" >> "${REPORT_FILE}"
        echo "\`\`\`" >> "${REPORT_FILE}"
        
        # Check for insecure protocols
        if echo "${ssl_protocols}" | grep -qE "(SSLv2|SSLv3|TLSv1[^\.])"; then
            echo "⚠️ Insecure SSL/TLS protocols detected" >> "${REPORT_FILE}"
            log_warning "Insecure SSL/TLS protocols found"
        else
            echo "✅ SSL/TLS configuration appears secure" >> "${REPORT_FILE}"
            log_success "SSL/TLS configuration is secure"
        fi
    else
        echo "ℹ️ NGINX configuration not found" >> "${REPORT_FILE}"
    fi
    
    echo "" >> "${REPORT_FILE}"
}

# Check API security
check_api_security() {
    log_info "Checking API security..."
    
    echo "### 1.6 API Security Check" >> "${REPORT_FILE}"
    echo "" >> "${REPORT_FILE}"
    
    if [ -f "docs/API_DOCUMENTATION.md" ]; then
        # Check for authentication requirements
        if grep -q "Authorization: Bearer" docs/API_DOCUMENTATION.md; then
            echo "✅ API endpoints require authentication" >> "${REPORT_FILE}"
        else
            echo "⚠️ Some API endpoints may not require authentication" >> "${REPORT_FILE}"
        fi
        
        # Check for rate limiting
        if grep -q "rate" docs/API_DOCUMENTATION.md; then
            echo "✅ Rate limiting is documented" >> "${REPORT_FILE}"
        else
            echo "⚠️ Rate limiting documentation not found" >> "${REPORT_FILE}"
        fi
    fi
    
    # Check NGINX rate limiting configuration
    if [ -f "deploy/nginx/nginx.conf" ]; then
        if grep -q "limit_req_zone" deploy/nginx/nginx.conf; then
            echo "✅ Rate limiting is configured in NGINX" >> "${REPORT_FILE}"
        fi
        
        if grep -q "limit_conn_zone" deploy/nginx/nginx.conf; then
            echo "✅ Connection limiting is configured in NGINX" >> "${REPORT_FILE}"
        fi
    fi
    
    log_info "API security check completed"
    
    echo "" >> "${REPORT_FILE}"
}

# Generate summary
generate_summary() {
    log_info "Generating audit summary..."
    
    cat >> "${REPORT_FILE}" << EOF

---

## 2. Security Recommendations

Based on the audit findings, the following recommendations are provided:

### 2.1 Immediate Actions
- Review all hardcoded secrets and move to environment variables
- Update vulnerable dependencies
- Fix insecure code patterns (unwrap, expect, unsafe)

### 2.2 Short-term Actions
- Implement automated dependency scanning in CI/CD
- Add security linters to development workflow
- Review and update cryptographic configurations

### 2.3 Long-term Actions
- Conduct regular penetration testing
- Implement security monitoring and alerting
- Establish security incident response procedures

---

## 3. Compliance Checklist

- [ ] OWASP Top 10 reviewed
- [ ] Security headers implemented
- [ ] Authentication mechanisms verified
- [ ] Authorization controls tested
- [ ] Input validation implemented
- [ ] Output encoding implemented
- [ ] Error handling secure
- [ ] Logging and monitoring configured
- [ ] Data protection measures in place
- [ ] Cryptographic standards verified

---

## 4. Audit Trail

| Check | Status | Findings |
|-------|--------|----------|
| Hardcoded Secrets | Completed | See report |
| Dependency Vulnerabilities | Completed | See report |
| Insecure Patterns | Completed | See report |
| File Permissions | Completed | See report |
| SSL/TLS Config | Completed | See report |
| API Security | Completed | See report |

---

**Report Generated:** $(date +"%Y-%m-%d %H:%M:%S")
**Next Audit Recommended:** $(date -d "+30 days" +"%Y-%m-%d" 2>/dev/null || date -v+30d +"%Y-%m-%d" 2>/dev/null || echo "In 30 days")

EOF

    log_success "Audit report saved to: ${REPORT_FILE}"
}

# Main execution
main() {
    echo -e "${BLUE}======================================${NC}"
    echo -e "${BLUE}  V-Sentinel Security Audit${NC}"
    echo -e "${BLUE}======================================${NC}"
    echo ""
    
    init_report
    check_secrets
    check_dependencies
    check_code_patterns
    check_permissions
    check_ssl_tls
    check_api_security
    generate_summary
    
    echo ""
    echo -e "${GREEN}Security audit completed!${NC}"
    echo -e "Report: ${REPORT_FILE}"
}

main "$@"