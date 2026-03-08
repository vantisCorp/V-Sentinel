#!/bin/bash

# V-Sentinel v2.1.1 - Linux AppImage Builder
# Creates a portable AppImage that can run on any Linux distribution

set -e

VERSION="2.1.1"
APP_NAME="V-Sentinel"
APPIMAGE_NAME="V-Sentinel-${VERSION}-x86_64.AppImage"

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
CYAN='\033[0;36m'
NC='\033[0m'

echo -e "${CYAN}================================================================${NC}"
echo -e "${CYAN}  V-Sentinel Linux AppImage Builder v${VERSION}${NC}"
echo -e "${CYAN}================================================================${NC}"
echo ""

# Check for required tools
check_dependencies() {
    echo -e "${CYAN}[i] Checking dependencies...${NC}"
    
    # Check for cargo
    if ! command -v cargo &> /dev/null; then
        echo -e "${RED}[!] Rust/Cargo not found. Please install Rust first.${NC}"
        exit 1
    fi
    
    # Check for wget
    if ! command -v wget &> /dev/null; then
        echo -e "${YELLOW}[!] wget not found. Installing...${NC}"
        sudo apt-get install -y wget
    fi
    
    echo -e "${GREEN}[+] All dependencies satisfied${NC}"
}

# Build release binary
build_binary() {
    echo -e "${CYAN}[i] Building release binary...${NC}"
    cargo build --release
    echo -e "${GREEN}[+] Binary built successfully${NC}"
}

# Create AppDir structure
create_appdir() {
    echo -e "${CYAN}[i] Creating AppDir structure...${NC}"
    
    rm -rf AppDir
    mkdir -p AppDir/usr/bin
    mkdir -p AppDir/usr/lib
    mkdir -p AppDir/usr/share/applications
    mkdir -p AppDir/usr/share/icons/hicolor/256x256/apps
    mkdir -p AppDir/usr/share/icons/hicolor/scalable/apps
    mkdir -p AppDir/usr/share/metainfo
    mkdir -p AppDir/usr/share/doc/v-sentinel
    mkdir -p AppDir/etc/v-sentinel
    mkdir -p AppDir/var/log/v-sentinel
    
    # Copy binary
    cp target/release/sentinel AppDir/usr/bin/v-sentinel
    chmod +x AppDir/usr/bin/v-sentinel
    
    # Copy configuration
    cp config/default.toml AppDir/etc/v-sentinel/config.toml
    
    # Copy documentation
    cp README.md AppDir/usr/share/doc/v-sentinel/
    cp LICENSE AppDir/usr/share/doc/v-sentinel/
    cp INSTALL.md AppDir/usr/share/doc/v-sentinel/
    
    echo -e "${GREEN}[+] AppDir structure created${NC}"
}

# Create desktop entry
create_desktop_entry() {
    echo -e "${CYAN}[i] Creating desktop entry...${NC}"
    
    cat > AppDir/usr/share/applications/v-sentinel.desktop << EOF
[Desktop Entry]
Type=Application
Name=V-Sentinel
GenericName=Security System
Comment=Next-generation AI-native security system with quantum-ready cryptography
Exec=v-sentinel
Icon=v-sentinel
Terminal=true
Categories=Security;System;Network;
StartupNotify=true
Keywords=security;ai;quantum;protection;
EOF
    
    # Also create root desktop entry
    cat > AppDir/v-sentinel.desktop << EOF
[Desktop Entry]
Type=Application
Name=V-Sentinel
GenericName=Security System
Comment=Next-generation AI-native security system with quantum-ready cryptography
Exec=v-sentinel
Icon=v-sentinel
Terminal=true
Categories=Security;System;Network;
StartupNotify=true
Keywords=security;ai;quantum;protection;
EOF
    
    echo -e "${GREEN}[+] Desktop entry created${NC}"
}

# Create AppStream metadata
create_appstream() {
    echo -e "${CYAN}[i] Creating AppStream metadata...${NC}"
    
    cat > AppDir/usr/share/metainfo/v-sentinel.metainfo.xml << EOF
<?xml version="1.0" encoding="UTF-8"?>
<component type="desktop-application">
  <id>com.vantiscorp.v-sentinel</id>
  <name>V-Sentinel</name>
  <summary>Next-generation AI-native security system</summary>
  <description>
    <p>V-Sentinel is a revolutionary security framework powered by artificial intelligence, 
    designed for real-time applications, distributed systems, and gaming platforms.</p>
    <p>Features:</p>
    <ul>
      <li>Zero Trust Architecture (NIST SP 800-207)</li>
      <li>Shadow AI Detection and Governance</li>
      <li>Deepfake Detection and Media Forensics</li>
      <li>AI Security Module</li>
      <li>Post-Quantum Cryptography</li>
    </ul>
  </description>
  <metadata_license>MIT</metadata_license>
  <project_license>AGPL-3.0</project_license>
  <developer_name>VantisCorp</developer_name>
  <url type="homepage">https://github.com/vantisCorp/V-Sentinel</url>
  <url type="bugtracker">https://github.com/vantisCorp/V-Sentinel/issues</url>
  <url type="help">https://github.com/vantisCorp/V-Sentinel/wiki</url>
  <launchable type="desktop-id">v-sentinel.desktop</launchable>
  <releases>
    <release version="${VERSION}" date="$(date +%Y-%m-%d)">
      <description>
        <p>Latest stable release with installer support</p>
      </description>
    </release>
  </releases>
  <content_rating type="oars-1.1"/>
  <requires>
    <display_length compare="ge">360</display_length>
  </requires>
</component>
EOF
    
    echo -e "${GREEN}[+] AppStream metadata created${NC}"
}

# Create AppImage icon
create_icon() {
    echo -e "${CYAN}[i] Creating icon...${NC}"
    
    # Create a simple SVG icon if not exists
    if [[ ! -f "assets/icon.svg" ]]; then
        mkdir -p assets
        cat > assets/icon.svg << 'EOF'
<?xml version="1.0" encoding="UTF-8"?>
<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 256 256" width="256" height="256">
  <defs>
    <linearGradient id="shield" x1="0%" y1="0%" x2="100%" y2="100%">
      <stop offset="0%" style="stop-color:#ff4444"/>
      <stop offset="100%" style="stop-color:#cc0000"/>
    </linearGradient>
  </defs>
  <path d="M128 16 L224 64 L224 128 C224 192 176 240 128 256 C80 240 32 192 32 128 L32 64 Z" 
        fill="url(#shield)" stroke="#990000" stroke-width="4"/>
  <path d="M128 48 L192 80 L192 128 C192 176 160 208 128 224 C96 208 64 176 64 128 L64 80 Z" 
        fill="#ffffff" opacity="0.2"/>
  <text x="128" y="160" font-family="Arial Black" font-size="72" font-weight="bold" 
        fill="#ffffff" text-anchor="middle">V</text>
</svg>
EOF
    fi
    
    # Copy icons
    cp assets/icon.svg AppDir/usr/share/icons/hicolor/scalable/apps/v-sentinel.svg
    cp assets/icon.svg AppDir/v-sentinel.svg
    
    # Create PNG icon if possible
    if command -v convert &> /dev/null; then
        convert -background none -resize 256x256 assets/icon.svg AppDir/usr/share/icons/hicolor/256x256/apps/v-sentinel.png
        convert -background none -resize 256x256 assets/icon.svg AppDir/v-sentinel.png
    fi
    
    echo -e "${GREEN}[+] Icons created${NC}"
}

# Download linuxdeploy
download_linuxdeploy() {
    echo -e "${CYAN}[i] Downloading linuxdeploy...${NC}"
    
    if [[ ! -f "linuxdeploy-x86_64.AppImage" ]]; then
        wget -q https://github.com/linuxdeploy/linuxdeploy/releases/download/continuous/linuxdeploy-x86_64.AppImage
        chmod +x linuxdeploy-x86_64.AppImage
    fi
    
    echo -e "${GREEN}[+] linuxdeploy downloaded${NC}"
}

# Build AppImage
build_appimage() {
    echo -e "${CYAN}[i] Building AppImage...${NC}"
    
    export ARCH=x86_64
    export VERSION=${VERSION}
    export APP_NAME=${APP_NAME}
    
    # Run linuxdeploy
    ./linuxdeploy-x86_64.AppImage --appdir AppDir --output appimage
    
    echo -e "${GREEN}[+] AppImage built successfully${NC}"
}

# Create release package
create_package() {
    echo -e "${CYAN}[i] Creating release package...${NC}"
    
    mkdir -p releases
    
    # Move AppImage
    mv -f V-Sentinel-${VERSION}-x86_64.AppImage releases/ 2>/dev/null || true
    
    # Create tarball
    tar -czvf releases/V-Sentinel-${VERSION}-linux-x86_64.tar.gz \
        --transform 's,^,V-Sentinel-${VERSION}/,' \
        README.md LICENSE INSTALL.md config/default.toml
    
    echo -e "${GREEN}[+] Release package created${NC}"
}

# Print summary
print_summary() {
    echo ""
    echo -e "${GREEN}================================================================${NC}"
    echo -e "${GREEN}  AppImage Build Complete!${NC}"
    echo -e "${GREEN}================================================================${NC}"
    echo ""
    echo -e "  Output files:"
    echo -e "    ${CYAN}releases/V-Sentinel-${VERSION}-x86_64.AppImage${NC}"
    echo -e "    ${CYAN}releases/V-Sentinel-${VERSION}-linux-x86_64.tar.gz${NC}"
    echo ""
    echo -e "  Usage:"
    echo -e "    chmod +x V-Sentinel-${VERSION}-x86_64.AppImage"
    echo -e "    ./V-Sentinel-${VERSION}-x86_64.AppImage"
    echo ""
    echo -e "${GREEN}================================================================${NC}"
}

# Main
main() {
    check_dependencies
    build_binary
    create_appdir
    create_desktop_entry
    create_appstream
    create_icon
    download_linuxdeploy
    build_appimage
    create_package
    print_summary
}

main "$@"