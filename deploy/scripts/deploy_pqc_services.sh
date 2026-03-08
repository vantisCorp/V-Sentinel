#!/bin/bash

# V-Sentinel PQC Services Deployment Script
# This script deploys PQC-enabled services to production environments

set -euo pipefail

# Configuration
ENVIRONMENT="${1:-staging}"
REGION="${2:-us-east-1}"
PROJECT_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
LOG_FILE="/var/log/v-sentinel/pqc_deploy_${ENVIRONMENT}_$(date +%Y%m%d_%H%M%S).log"

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Logging function
log() {
    local level=$1
    shift
    local message="$@"
    echo "$(date '+%Y-%m-%d %H:%M:%S') [${level}] ${message}" | tee -a "${LOG_FILE}"
}

log_info() {
    log "INFO" "${GREEN}$@${NC}"
}

log_warn() {
    log "WARN" "${YELLOW}$@${NC}"
}

log_error() {
    log "ERROR" "${RED}$@${NC}"
}

# Pre-flight checks
preflight_checks() {
    log_info "Starting pre-flight checks..."
    
    # Check if running as root
    if [[ $EUID -eq 0 ]]; then
        log_error "This script should not be run as root"
        exit 1
    fi
    
    # Check required commands
    local required_commands=("docker" "kubectl" "helm" "aws" "jq" "vault")
    for cmd in "${required_commands[@]}"; do
        if ! command -v "$cmd" &> /dev/null; then
            log_error "Required command not found: $cmd"
            exit 1
        fi
    done
    
    # Check environment file
    if [[ ! -f "${PROJECT_ROOT}/deploy/environments/${ENVIRONMENT}.env" ]]; then
        log_error "Environment file not found: ${ENVIRONMENT}.env"
        exit 1
    fi
    
    # Source environment variables
    source "${PROJECT_ROOT}/deploy/environments/${ENVIRONMENT}.env"
    
    log_info "Pre-flight checks completed successfully"
}

# Get secrets from vault
get_secrets() {
    log_info "Retrieving secrets from vault..."
    
    # Get PQC private keys
    vault kv get -format=json secret/v-sentinel/pqc/keys > /tmp/pqc_keys.json
    PQC_PRIVATE_KEY=$(jq -r '.data.data.private_key' /tmp/pqc_keys.json)
    
    # Get database credentials
    vault kv get -format=json secret/v-sentinel/database > /tmp/db_creds.json
    DB_PASSWORD=$(jq -r '.data.data.password' /tmp/db_creds.json)
    
    # Get TLS certificates
    vault kv get -format=json secret/v-sentinel/tls > /tmp/tls_certs.json
    TLS_CERT=$(jq -r '.data.data.certificate' /tmp/tls_certs.json)
    TLS_KEY=$(jq -r '.data.data.private_key' /tmp/tls_certs.json)
    
    # Get PQC certificates
    vault kv get -format=json secret/v-sentinel/pqc/certs > /tmp/pqc_certs.json
    PQC_CERT=$(jq -r '.data.data.certificate' /tmp/pqc_certs.json)
    PQC_KEY=$(jq -r '.data.data.private_key' /tmp/pqc_certs.json)
    
    log_info "Secrets retrieved successfully"
}

# Build Docker images
build_images() {
    log_info "Building Docker images..."
    
    local services=("gateway" "vpn" "messaging" "key-manager" "certificate-manager")
    
    for service in "${services[@]}"; do
        log_info "Building ${service} image..."
        cd "${PROJECT_ROOT}/src/services/${service}"
        
        docker build \
            --build-arg BUILDKIT_INLINE_CACHE=1 \
            --build-arg RUST_VERSION=1.75.0 \
            --tag "v-sentinel/${service}:${ENVIRONMENT}" \
            --tag "v-sentinel/${service}:${ENVIRONMENT}-$(git rev-parse --short HEAD)" \
            .
        
        log_info "Pushing ${service} image to registry..."
        docker tag "v-sentinel/${service}:${ENVIRONMENT}" "${DOCKER_REGISTRY}/v-sentinel/${service}:${ENVIRONMENT}"
        docker push "${DOCKER_REGISTRY}/v-sentinel/${service}:${ENVIRONMENT}"
    done
    
    log_info "All images built and pushed successfully"
}

# Deploy infrastructure with Terraform
deploy_infrastructure() {
    log_info "Deploying infrastructure with Terraform..."
    
    cd "${PROJECT_ROOT}/deploy/terraform"
    
    # Initialize Terraform
    terraform init \
        -backend-config="bucket=${TERRAFORM_STATE_BUCKET}" \
        -backend-config="key=v-sentinel/${ENVIRONMENT}/terraform.tfstate" \
        -backend-config="region=${REGION}"
    
    # Plan infrastructure
    terraform plan \
        -var-file="${ENVIRONMENT}.tfvars" \
        -out="tfplan-${ENVIRONMENT}"
    
    # Apply infrastructure
    terraform apply "tfplan-${ENVIRONMENT}"
    
    # Get outputs
    terraform output -json > "${PROJECT_ROOT}/deploy/outputs/terraform-outputs-${ENVIRONMENT}.json"
    
    log_info "Infrastructure deployed successfully"
}

# Deploy services with Helm
deploy_services() {
    log_info "Deploying services with Helm..."
    
    cd "${PROJECT_ROOT}/deploy/helm"
    
    local services=("gateway" "vpn" "messaging" "key-manager" "certificate-manager")
    
    for service in "${services[@]}"; do
        log_info "Deploying ${service}..."
        
        helm upgrade --install "v-sentinel-${service}" \
            "./${service}" \
            --namespace "v-sentinel-${ENVIRONMENT}" \
            --create-namespace \
            --values "./${service}/values-${ENVIRONMENT}.yaml" \
            --set "image.repository=${DOCKER_REGISTRY}/v-sentinel/${service}" \
            --set "image.tag=${ENVIRONMENT}" \
            --set "pqc.enabled=true" \
            --set "pqc.hybridMode=${PQC_HYBRID_MODE:-true}" \
            --set "pqc.kemAlgorithm=${PQC_KEM_ALGORITHM:-kyber1024}" \
            --set "pqc.signatureAlgorithm=${PQC_SIGNATURE_ALGORITHM:-dilithium5}" \
            --wait \
            --timeout 10m
        
        log_info "Waiting for ${service} to be ready..."
        kubectl rollout status deployment "v-sentinel-${service}" \
            -n "v-sentinel-${ENVIRONMENT}" \
            --timeout=5m
    done
    
    log_info "All services deployed successfully"
}

# Run database migrations
run_migrations() {
    log_info "Running database migrations..."
    
    cd "${PROJECT_ROOT}/src/database"
    
    # Get database connection details from Terraform outputs
    DB_HOST=$(jq -r '.database_host.value' "${PROJECT_ROOT}/deploy/outputs/terraform-outputs-${ENVIRONMENT}.json")
    DB_PORT=$(jq -r '.database_port.value' "${PROJECT_ROOT}/deploy/outputs/terraform-outputs-${ENVIRONMENT}.json")
    DB_NAME=$(jq -r '.database_name.value' "${PROJECT_ROOT}/deploy/outputs/terraform-outputs-${ENVIRONMENT}.json")
    
    # Run migrations
    DATABASE_URL="postgresql://${DB_USER}:${DB_PASSWORD}@${DB_HOST}:${DB_PORT}/${DB_NAME}" \
    cargo run --bin migrate -- --all
    
    log_info "Database migrations completed successfully"
}

# Configure monitoring
configure_monitoring() {
    log_info "Configuring monitoring..."
    
    cd "${PROJECT_ROOT}/deploy/monitoring"
    
    # Deploy Prometheus
    helm upgrade --install "v-sentinel-prometheus" \
        "./prometheus" \
        --namespace "monitoring" \
        --values "./prometheus/values-${ENVIRONMENT}.yaml"
    
    # Deploy Grafana
    helm upgrade --install "v-sentinel-grafana" \
        "./grafana" \
        --namespace "monitoring" \
        --values "./grafana/values-${ENVIRONMENT}.yaml" \
        --set "pqcDashboards.enabled=true"
    
    # Deploy Alertmanager
    helm upgrade --install "v-sentinel-alertmanager" \
        "./alertmanager" \
        --namespace "monitoring" \
        --values "./alertmanager/values-${ENVIRONMENT}.yaml" \
        --set "pqcAlerts.enabled=true"
    
    log_info "Monitoring configured successfully"
}

# Run health checks
health_checks() {
    log_info "Running health checks..."
    
    local services=("gateway" "vpn" "messaging" "key-manager" "certificate-manager")
    
    for service in "${services[@]}"; do
        log_info "Checking ${service} health..."
        
        local endpoint="http://v-sentinel-${service}.v-sentinel-${ENVIRONMENT}.svc.cluster.local:8080/health"
        
        local max_attempts=30
        local attempt=1
        
        while [[ $attempt -le $max_attempts ]]; do
            if curl -sf "$endpoint" > /dev/null; then
                log_info "${service} is healthy"
                break
            fi
            
            log_warn "${service} health check failed (attempt ${attempt}/${max_attempts})"
            sleep 10
            ((attempt++))
        done
        
        if [[ $attempt -gt $max_attempts ]]; then
            log_error "${service} failed health checks"
            exit 1
        fi
    done
    
    log_info "All health checks passed"
}

# Configure secrets in Kubernetes
configure_k8s_secrets() {
    log_info "Configuring Kubernetes secrets..."
    
    local namespace="v-sentinel-${ENVIRONMENT}"
    
    # Create PQC keys secret
    kubectl create secret generic "v-sentinel-pqc-keys" \
        --namespace "$namespace" \
        --from-literal=private-key="$PQC_PRIVATE_KEY" \
        --dry-run=client -o yaml | kubectl apply -f -
    
    # Create database secret
    kubectl create secret generic "v-sentinel-database" \
        --namespace "$namespace" \
        --from-literal=password="$DB_PASSWORD" \
        --dry-run=client -o yaml | kubectl apply -f -
    
    # Create TLS secrets
    kubectl create secret tls "v-sentinel-tls" \
        --namespace "$namespace" \
        --cert="$TLS_CERT" \
        --key="$TLS_KEY" \
        --dry-run=client -o yaml | kubectl apply -f -
    
    # Create PQC certificates secret
    kubectl create secret tls "v-sentinel-pqc-tls" \
        --namespace "$namespace" \
        --cert="$PQC_CERT" \
        --key="$PQC_KEY" \
        --dry-run=client -o yaml | kubectl apply -f -
    
    log_info "Kubernetes secrets configured successfully"
}

# Rollback function
rollback() {
    log_error "Initiating rollback..."
    
    local namespace="v-sentinel-${ENVIRONMENT}"
    
    # Rollback Helm releases
    helm rollback "v-sentinel-gateway" -n "$namespace"
    helm rollback "v-sentinel-vpn" -n "$namespace"
    helm rollback "v-sentinel-messaging" -n "$namespace"
    
    log_error "Rollback completed. Please investigate the issue."
    exit 1
}

# Main deployment function
main() {
    log_info "Starting V-Sentinel PQC deployment to ${ENVIRONMENT}..."
    log_info "Deployment log: ${LOG_FILE}"
    
    # Trap errors and rollback
    trap rollback ERR
    
    # Execute deployment steps
    preflight_checks
    get_secrets
    build_images
    deploy_infrastructure
    configure_k8s_secrets
    deploy_services
    run_migrations
    configure_monitoring
    health_checks
    
    log_info "Deployment completed successfully!"
    log_info "All PQC services are now running in ${ENVIRONMENT}"
    
    # Cleanup temporary files
    rm -f /tmp/pqc_keys.json /tmp/db_creds.json /tmp/tls_certs.json /tmp/pqc_certs.json
}

# Run main function
main "$@"