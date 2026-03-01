#!/bin/bash
# SENTINEL Security System - Production Deployment Script

set -e

# Configuration
ENVIRONMENT="${ENVIRONMENT:-production}"
VERSION="${VERSION:-1.1.0}"
AWS_REGION="${AWS_REGION:-us-east-1}"
CLUSTER_NAME="${ENVIRONMENT}-cluster"
ECR_REPOSITORY_PREFIX="123456789012.dkr.ecr.${AWS_REGION}.amazonaws.com/sentinel"

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Functions
log_info() {
    echo -e "${GREEN}[INFO]${NC} $1"
}

log_warn() {
    echo -e "${YELLOW}[WARN]${NC} $1"
}

log_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

# Pre-deployment checks
pre_deployment_checks() {
    log_info "Running pre-deployment checks..."
    
    # Check if AWS CLI is installed
    if ! command -v aws &> /dev/null; then
        log_error "AWS CLI is not installed"
        exit 1
    fi
    
    # Check if Docker is installed
    if ! command -v docker &> /dev/null; then
        log_error "Docker is not installed"
        exit 1
    fi
    
    # Check if kubectl is installed
    if ! command -v kubectl &> /dev/null; then
        log_error "kubectl is not installed"
        exit 1
    fi
    
    # Check if terraform is installed
    if ! command -v terraform &> /dev/null; then
        log_error "terraform is not installed"
        exit 1
    fi
    
    # Verify AWS credentials
    log_info "Verifying AWS credentials..."
    aws sts get-caller-identity &> /dev/null
    if [ $? -ne 0 ]; then
        log_error "AWS credentials not configured"
        exit 1
    fi
    
    log_info "Pre-deployment checks passed"
}

# Build Docker images
build_images() {
    log_info "Building Docker images..."
    
    # Login to ECR
    log_info "Logging in to ECR..."
    aws ecr get-login-password --region ${AWS_REGION} | docker login --username AWS --password-stdin ${ECR_REPOSITORY_PREFIX%-sentinel*}
    
    # Build API image
    log_info "Building API image..."
    docker build -t ${ECR_REPOSITORY_PREFIX}-api:${VERSION} -f docker/Dockerfile.api .
    docker tag ${ECR_REPOSITORY_PREFIX}-api:${VERSION} ${ECR_REPOSITORY_PREFIX}-api:latest
    
    # Build Worker image
    log_info "Building Worker image..."
    docker build -t ${ECR_REPOSITORY_PREFIX}-worker:${VERSION} -f docker/Dockerfile.worker .
    docker tag ${ECR_REPOSITORY_PREFIX}-worker:${VERSION} ${ECR_REPOSITORY_PREFIX}-worker:latest
    
    # Build Web image
    log_info "Building Web image..."
    docker build -t ${ECR_REPOSITORY_PREFIX}-web:${VERSION} -f docker/Dockerfile.web .
    docker tag ${ECR_REPOSITORY_PREFIX}-web:${VERSION} ${ECR_REPOSITORY_PREFIX}-web:latest
    
    log_info "Docker images built successfully"
}

# Push images to ECR
push_images() {
    log_info "Pushing images to ECR..."
    
    docker push ${ECR_REPOSITORY_PREFIX}-api:${VERSION}
    docker push ${ECR_REPOSITORY_PREFIX}-api:latest
    
    docker push ${ECR_REPOSITORY_PREFIX}-worker:${VERSION}
    docker push ${ECR_REPOSITORY_PREFIX}-worker:latest
    
    docker push ${ECR_REPOSITORY_PREFIX}-web:${VERSION}
    docker push ${ECR_REPOSITORY_PREFIX}-web:latest
    
    log_info "Images pushed to ECR successfully"
}

# Deploy to ECS
deploy_ecs() {
    log_info "Deploying to ECS..."
    
    # Update ECS task definitions
    log_info "Updating ECS task definitions..."
    
    # API service
    aws ecs update-service \
        --cluster ${CLUSTER_NAME} \
        --service api \
        --task-definition sentinel-api:${VERSION} \
        --force-new-deployment \
        --region ${AWS_REGION}
    
    # Worker service
    aws ecs update-service \
        --cluster ${CLUSTER_NAME} \
        --service worker \
        --task-definition sentinel-worker:${VERSION} \
        --force-new-deployment \
        --region ${AWS_REGION}
    
    # Web service
    aws ecs update-service \
        --cluster ${CLUSTER_NAME} \
        --service web \
        --task-definition sentinel-web:${VERSION} \
        --force-new-deployment \
        --region ${AWS_REGION}
    
    log_info "ECS deployment initiated"
}

# Wait for deployment to complete
wait_for_deployment() {
    log_info "Waiting for deployment to complete..."
    
    local max_attempts=60
    local attempt=0
    
    while [ $attempt -lt $max_attempts ]; do
        # Check if all services are stable
        local api_stable=$(aws ecs describe-services \
            --cluster ${CLUSTER_NAME} \
            --services api \
            --region ${AWS_REGION} \
            --query 'services[0].deployments[0].rolloutState' \
            --output text)
        
        local worker_stable=$(aws ecs describe-services \
            --cluster ${CLUSTER_NAME} \
            --services worker \
            --region ${AWS_REGION} \
            --query 'services[0].deployments[0].rolloutState' \
            --output text)
        
        local web_stable=$(aws ecs describe-services \
            --cluster ${CLUSTER_NAME} \
            --services web \
            --region ${AWS_REGION} \
            --query 'services[0].deployments[0].rolloutState' \
            --output text)
        
        if [ "$api_stable" = "COMPLETED" ] && [ "$worker_stable" = "COMPLETED" ] && [ "$web_stable" = "COMPLETED" ]; then
            log_info "Deployment completed successfully"
            return 0
        fi
        
        log_info "Waiting for deployment... (attempt $((attempt + 1))/$max_attempts)"
        sleep 30
        attempt=$((attempt + 1))
    done
    
    log_error "Deployment timed out"
    return 1
}

# Run smoke tests
run_smoke_tests() {
    log_info "Running smoke tests..."
    
    # Test health endpoints
    log_info "Testing health endpoints..."
    
    local api_health=$(curl -s -o /dev/null -w "%{http_code}" https://api.sentinel.security/health)
    if [ "$api_health" != "200" ]; then
        log_error "API health check failed (HTTP $api_health)"
        return 1
    fi
    
    local web_health=$(curl -s -o /dev/null -w "%{http_code}" https://sentinel.security/health)
    if [ "$web_health" != "200" ]; then
        log_error "Web health check failed (HTTP $web_health)"
        return 1
    fi
    
    log_info "Smoke tests passed"
}

# Rollback deployment
rollback_deployment() {
    log_warn "Rolling back deployment..."
    
    # Get previous task definitions
    local previous_api=$(aws ecs describe-task-definition \
        --task-definition sentinel-api \
        --region ${AWS_REGION} \
        --query 'taskDefinition.revision' \
        --output text)
    
    local previous_worker=$(aws ecs describe-task-definition \
        --task-definition sentinel-worker \
        --region ${AWS_REGION} \
        --query 'taskDefinition.revision' \
        --output text)
    
    local previous_web=$(aws ecs describe-task-definition \
        --task-definition sentinel-web \
        --region ${AWS_REGION} \
        --query 'taskDefinition.revision' \
        --output text)
    
    # Rollback to previous version
    aws ecs update-service \
        --cluster ${CLUSTER_NAME} \
        --service api \
        --task-definition sentinel-api:${previous_api} \
        --force-new-deployment \
        --region ${AWS_REGION}
    
    aws ecs update-service \
        --cluster ${CLUSTER_NAME} \
        --service worker \
        --task-definition sentinel-worker:${previous_worker} \
        --force-new-deployment \
        --region ${AWS_REGION}
    
    aws ecs update-service \
        --cluster ${CLUSTER_NAME} \
        --service web \
        --task-definition sentinel-web:${previous_web} \
        --force-new-deployment \
        --region ${AWS_REGION}
    
    log_warn "Rollback initiated"
}

# Main deployment function
deploy() {
    log_info "Starting deployment of SENTINEL v${VERSION} to ${ENVIRONMENT}..."
    
    # Pre-deployment checks
    pre_deployment_checks
    
    # Build images
    build_images
    
    # Push images
    push_images
    
    # Deploy to ECS
    deploy_ecs
    
    # Wait for deployment
    if ! wait_for_deployment; then
        log_error "Deployment failed"
        rollback_deployment
        exit 1
    fi
    
    # Run smoke tests
    if ! run_smoke_tests; then
        log_error "Smoke tests failed"
        rollback_deployment
        exit 1
    fi
    
    log_info "Deployment completed successfully!"
}

# Parse command line arguments
case "${1:-deploy}" in
    deploy)
        deploy
        ;;
    rollback)
        rollback_deployment
        ;;
    smoke-test)
        run_smoke_tests
        ;;
    *)
        echo "Usage: $0 {deploy|rollback|smoke-test}"
        echo "Environment variables:"
        echo "  ENVIRONMENT - Environment name (default: production)"
        echo "  VERSION - Version to deploy (default: 1.1.0)"
        echo "  AWS_REGION - AWS region (default: us-east-1)"
        exit 1
        ;;
esac

exit 0