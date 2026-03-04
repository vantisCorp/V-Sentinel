#!/bin/bash
#
# V-Sentinel Deployment Script
#
# Automated deployment script for Docker Compose

set -e

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Configuration
COMPOSE_FILE="docker-compose.yml"
ENV_FILE=".env"
BACKUP_DIR="./backups"
TIMESTAMP=$(date +"%Y%m%d_%H%M%S")

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

check_prerequisites() {
    log_info "Checking prerequisites..."
    
    # Check Docker
    if ! command -v docker &> /dev/null; then
        log_error "Docker is not installed"
        exit 1
    fi
    
    # Check Docker Compose
    if ! command -v docker-compose &> /dev/null; then
        log_error "Docker Compose is not installed"
        exit 1
    fi
    
    log_success "Prerequisites check passed"
}

setup_environment() {
    log_info "Setting up environment..."
    
    # Create .env file if it doesn't exist
    if [ ! -f "$ENV_FILE" ]; then
        log_warning ".env file not found, creating from template..."
        cat > "$ENV_FILE" << EOF
# V-Sentinel Environment Configuration

# Database
DB_PASSWORD=$(openssl rand -base64 32)

# Redis
REDIS_PASSWORD=$(openssl rand -base64 32)

# Grafana
GRAFANA_ADMIN_USER=admin
GRAFANA_ADMIN_PASSWORD=$(openssl rand -base64 16)

# API Keys
SENTINEL_API_KEY=$(openssl rand -hex 32)
EOF
        log_success ".env file created"
    else
        log_info ".env file exists, using existing configuration"
    fi
}

backup_data() {
    log_info "Creating backup..."
    
    mkdir -p "$BACKUP_DIR"
    
    # Backup database
    docker exec vsentinel-postgres pg_dump -U vsentinel vsentinel > "$BACKUP_DIR/db_backup_${TIMESTAMP}.sql" 2>/dev/null || true
    
    # Backup volumes
    docker run --rm \
        -v vsentinel_postgres-data:/data \
        -v "$BACKUP_DIR:/backup" \
        alpine tar czf "/backup/postgres_${TIMESTAMP}.tar.gz" -C /data . 2>/dev/null || true
    
    log_success "Backup created in $BACKUP_DIR"
}

deploy() {
    log_info "Starting deployment..."
    
    # Pull latest images
    log_info "Pulling latest images..."
    docker-compose pull
    
    # Build images
    log_info "Building images..."
    docker-compose build
    
    # Stop existing services
    log_info "Stopping existing services..."
    docker-compose down
    
    # Start services
    log_info "Starting services..."
    docker-compose up -d
    
    # Wait for services to be healthy
    log_info "Waiting for services to be healthy..."
    sleep 30
    
    # Check service status
    log_info "Checking service status..."
    docker-compose ps
    
    log_success "Deployment completed successfully"
}

rollback() {
    log_info "Rolling back deployment..."
    
    # Stop services
    docker-compose down
    
    # Restore from backup if specified
    if [ -n "$BACKUP_FILE" ]; then
        log_info "Restoring from backup: $BACKUP_FILE"
        docker run --rm \
            -v vsentinel_postgres-data:/data \
            -v "$BACKUP_FILE:/backup/backup.tar.gz" \
            alpine tar xzf /backup/backup.tar.gz -C /data
    fi
    
    # Restart services
    docker-compose up -d
    
    log_success "Rollback completed"
}

health_check() {
    log_info "Performing health check..."
    
    # Check V-Sentinel
    if curl -f http://localhost:8080/health &> /dev/null; then
        log_success "V-Sentinel: OK"
    else
        log_error "V-Sentinel: FAILED"
        return 1
    fi
    
    # Check PostgreSQL
    if docker exec vsentinel-postgres pg_isready &> /dev/null; then
        log_success "PostgreSQL: OK"
    else
        log_error "PostgreSQL: FAILED"
        return 1
    fi
    
    # Check Redis
    if docker exec vsentinel-redis redis-cli ping &> /dev/null; then
        log_success "Redis: OK"
    else
        log_error "Redis: FAILED"
        return 1
    fi
    
    log_success "All services are healthy"
}

show_logs() {
    log_info "Showing logs..."
    docker-compose logs -f --tail=100
}

# Main
case "$1" in
    deploy)
        check_prerequisites
        setup_environment
        backup_data
        deploy
        health_check
        ;;
    
    update)
        log_info "Updating deployment..."
        deploy
        ;;
    
    rollback)
        BACKUP_FILE="$2"
        rollback
        ;;
    
    restart)
        log_info "Restarting services..."
        docker-compose restart
        health_check
        ;;
    
    stop)
        log_info "Stopping services..."
        docker-compose down
        ;;
    
    status)
        log_info "Service status:"
        docker-compose ps
        ;;
    
    logs)
        show_logs
        ;;
    
    health)
        health_check
        ;;
    
    backup)
        backup_data
        ;;
    
    *)
        echo "Usage: $0 {deploy|update|rollback|restart|stop|status|logs|health|backup}"
        echo ""
        echo "Commands:"
        echo "  deploy     - Deploy or update the application"
        echo "  update     - Update the application"
        echo "  rollback   - Rollback to previous version (usage: rollback [backup_file])"
        echo "  restart    - Restart all services"
        echo "  stop       - Stop all services"
        echo "  status     - Show service status"
        echo "  logs       - Show logs (tail -f)"
        echo "  health     - Perform health check"
        echo "  backup     - Create backup"
        exit 1
        ;;
esac

exit 0
