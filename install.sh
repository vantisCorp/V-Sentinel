#!/bin/bash

#===============================================================================
# V-Sentinel v2.1.0 - Installation Script
# Next-generation AI-native security system with quantum-ready cryptography
#===============================================================================

set -e

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
PURPLE='\033[0;35m'
CYAN='\033[0;36m'
NC='\033[0m' # No Color

# Version
VERSION="2.1.0"
INSTALL_DIR="/opt/v-sentinel"
CONFIG_DIR="/etc/v-sentinel"
LOG_DIR="/var/log/v-sentinel"
BIN_DIR="/usr/local/bin"

# Print banner
print_banner() {
    echo -e "${CYAN}"
    echo "╔═══════════════════════════════════════════════════════════════╗"
    echo "║                                                               ║"
    echo "║     ╦╔═╗╔═╗╔═╗╦═╗╔╦╗  ╔╦╗╔═╗╦═╗╦╔═╗╔═╗╦ ╦╔═╗╦═╗            ║"
    echo "║     ║║ ║╠═╝╠═╣╠╦╝ ║    ║ ║╣ ╠╦╝║╠╣ ╠═╣║║║╠═╣╠╦╝            ║"
    echo "║     ╩╚═╝╩  ╩ ╩╩╚═ ╩    ╩ ╚═╝╩╚═╩╚  ╩ ╩╚╩╝╩ ╩╩╚═            ║"
    echo "║                                                               ║"
    echo "║     Next-Generation AI-Native Security System                ║"
    echo "║     Version ${VERSION}                                           ║"
    echo "║                                                               ║"
    echo "╚═══════════════════════════════════════════════════════════════╝"
    echo -e "${NC}"
}

# Check if running as root
check_root() {
    if [[ $EUID -ne 0 ]]; then
        echo -e "${YELLOW}[!] This script must be run as root${NC}"
        echo -e "${CYAN}[i] Try: sudo ./install.sh${NC}"
        exit 1
    fi
}

# Detect OS
detect_os() {
    if [[ -f /etc/os-release ]]; then
        . /etc/os-release
        OS=$ID
        OS_VERSION=$VERSION_ID
    elif [[ -f /etc/redhat-release ]]; then
        OS="rhel"
    else
        OS="unknown"
    fi
    echo -e "${GREEN}[✓] Detected OS: ${OS} ${OS_VERSION}${NC}"
}

# Install dependencies
install_dependencies() {
    echo -e "${CYAN}[i] Installing dependencies...${NC}"
    
    case $OS in
        ubuntu|debian)
            apt-get update -qq
            apt-get install -y -qq build-essential pkg-config libssl-dev curl wget git cmake
            ;;
        rhel|centos|fedora|rocky|almalinux)
            yum install -y -q gcc gcc-c++ make pkg-config openssl-devel curl wget git cmake
            ;;
        arch|manjaro)
            pacman -Sy --noconfirm base-devel pkg-config openssl curl wget git cmake
            ;;
        *)
            echo -e "${YELLOW}[!] Unsupported OS: ${OS}. Installing minimal dependencies...${NC}"
            ;;
    esac
    
    # Install Rust if not present
    if ! command -v rustc &> /dev/null; then
        echo -e "${CYAN}[i] Installing Rust...${NC}"
        curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
        source "$HOME/.cargo/env"
    fi
    
    echo -e "${GREEN}[✓] Dependencies installed${NC}"
}

# Create directories
create_directories() {
    echo -e "${CYAN}[i] Creating directories...${NC}"
    
    mkdir -p "$INSTALL_DIR"
    mkdir -p "$CONFIG_DIR"
    mkdir -p "$LOG_DIR"
    mkdir -p "$INSTALL_DIR/modules"
    
    echo -e "${GREEN}[✓] Directories created${NC}"
}

# Build project
build_project() {
    echo -e "${CYAN}[i] Building V-Sentinel v${VERSION}...${NC}"
    
    # Check if we're in the project directory
    if [[ -f "Cargo.toml" ]]; then
        PROJECT_DIR=$(pwd)
    elif [[ -d "../V-Sentinel" ]]; then
        cd ../V-Sentinel
        PROJECT_DIR=$(pwd)
    else
        echo -e "${RED}[✗] Cargo.toml not found. Please run this script from the V-Sentinel directory${NC}"
        exit 1
    fi
    
    # Build release
    cargo build --release
    
    echo -e "${GREEN}[✓] Build completed${NC}"
}

# Install binaries
install_binaries() {
    echo -e "${CYAN}[i] Installing binaries...${NC}"
    
    # Copy main binary
    if [[ -f "target/release/sentinel" ]]; then
        cp target/release/sentinel "$BIN_DIR/v-sentinel"
        chmod +x "$BIN_DIR/v-sentinel"
    fi
    
    # Create symlink
    ln -sf "$BIN_DIR/v-sentinel" "$BIN_DIR/sentinel"
    
    echo -e "${GREEN}[✓] Binaries installed${NC}"
}

# Install configuration
install_config() {
    echo -e "${CYAN}[i] Installing configuration...${NC}"
    
    # Create default config if not exists
    if [[ ! -f "$CONFIG_DIR/config.toml" ]]; then
        cat > "$CONFIG_DIR/config.toml" << EOF
# V-Sentinel Configuration File
# Version: ${VERSION}

[general]
# Application name
app_name = "V-Sentinel"
# Log level: trace, debug, info, warn, error
log_level = "info"
# Environment: development, staging, production
environment = "production"

[security]
# Enable Zero Trust Architecture
zero_trust_enabled = true
# Enable Shadow AI Detection
shadow_ai_enabled = true
# Enable Deepfake Detection
deepfake_enabled = true
# Enable AI Security
ai_security_enabled = true

[cryptography]
# Post-quantum cryptography algorithm
pq_algorithm = "kyber768"
# Signature algorithm
sig_algorithm = "dilithium3"
# Enable hybrid encryption
hybrid_encryption = true

[api]
# API host
host = "0.0.0.0"
# API port
port = 8080
# Enable TLS
tls_enabled = true

[monitoring]
# Enable metrics
metrics_enabled = true
# Metrics port
metrics_port = 9090
# Enable health checks
health_checks_enabled = true

[logging]
# Log file path
log_file = "/var/log/v-sentinel/v-sentinel.log"
# Max log file size (MB)
max_log_size = 100
# Number of backup files
backup_count = 5
EOF
    fi
    
    echo -e "${GREEN}[✓] Configuration installed${NC}"
}

# Create systemd service
create_systemd_service() {
    echo -e "${CYAN}[i] Creating systemd service...${NC}"
    
    cat > /etc/systemd/system/v-sentinel.service << EOF
[Unit]
Description=V-Sentinel Security System
Documentation=https://github.com/vantisCorp/V-Sentinel
After=network.target

[Service]
Type=simple
User=root
ExecStart=$BIN_DIR/v-sentinel --config $CONFIG_DIR/config.toml
Restart=on-failure
RestartSec=5
StandardOutput=journal
StandardError=journal

[Install]
WantedBy=multi-user.target
EOF
    
    systemctl daemon-reload
    systemctl enable v-sentinel
    
    echo -e "${GREEN}[✓] Systemd service created${NC}"
}

# Print post-install instructions
print_post_install() {
    echo ""
    echo -e "${GREEN}╔═══════════════════════════════════════════════════════════════╗${NC}"
    echo -e "${GREEN}║                  Installation Complete!                       ║${NC}"
    echo -e "${GREEN}╠═══════════════════════════════════════════════════════════════╣${NC}"
    echo -e "${GREEN}║${NC}                                                               ${GREEN}║${NC}"
    echo -e "${GREEN}║${NC}  V-Sentinel v${VERSION} has been installed successfully!        ${GREEN}║${NC}"
    echo -e "${GREEN}║${NC}                                                               ${GREEN}║${NC}"
    echo -e "${GREEN}║${NC}  ${CYAN}Quick Start:${NC}                                                ${GREEN}║${NC}"
    echo -e "${GREEN}║${NC}    sudo systemctl start v-sentinel                           ${GREEN}║${NC}"
    echo -e "${GREEN}║${NC}    sudo systemctl status v-sentinel                          ${GREEN}║${NC}"
    echo -e "${GREEN}║${NC}                                                               ${GREEN}║${NC}"
    echo -e "${GREEN}║${NC}  ${CYAN}Configuration:${NC}                                             ${GREEN}║${NC}"
    echo -e "${GREEN}║${NC}    $CONFIG_DIR/config.toml                                   ${GREEN}║${NC}"
    echo -e "${GREEN}║${NC}                                                               ${GREEN}║${NC}"
    echo -e "${GREEN}║${NC}  ${CYAN}Logs:${NC}                                                     ${GREEN}║${NC}"
    echo -e "${GREEN}║${NC}    $LOG_DIR/                                                 ${GREEN}║${NC}"
    echo -e "${GREEN}║${NC}                                                               ${GREEN}║${NC}"
    echo -e "${GREEN}║${NC}  ${CYAN}Documentation:${NC}                                            ${GREEN}║${NC}"
    echo -e "${GREEN}║${NC}    https://github.com/vantisCorp/V-Sentinel                  ${GREEN}║${NC}"
    echo -e "${GREEN}║${NC}                                                               ${GREEN}║${NC}"
    echo -e "${GREEN}╚═══════════════════════════════════════════════════════════════╝${NC}"
    echo ""
}

# Main installation
main() {
    print_banner
    check_root
    detect_os
    install_dependencies
    create_directories
    build_project
    install_binaries
    install_config
    create_systemd_service
    print_post_install
}

# Run main
main "$@"