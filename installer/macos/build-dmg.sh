#!/bin/bash

# V-Sentinel v2.1.1 - macOS DMG Builder
# Creates a macOS disk image installer

set -e

VERSION="2.1.1"
APP_NAME="V-Sentinel"
DMG_NAME="V-Sentinel-${VERSION}.dmg"

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
CYAN='\033[0;36m'
NC='\033[0m'

echo -e "${CYAN}================================================================${NC}"
echo -e "${CYAN}  V-Sentinel macOS DMG Builder v${VERSION}${NC}"
echo -e "${CYAN}================================================================${NC}"
echo ""

# Check for macOS
if [[ "$(uname)" != "Darwin" ]]; then
    echo -e "${RED}[!] This script must be run on macOS${NC}"
    exit 1
fi

# Check dependencies
check_dependencies() {
    echo -e "${CYAN}[i] Checking dependencies...${NC}"
    
    if ! command -v cargo &> /dev/null; then
        echo -e "${RED}[!] Rust/Cargo not found. Please install Rust first.${NC}"
        exit 1
    fi
    
    echo -e "${GREEN}[+] All dependencies satisfied${NC}"
}

# Build release binary
build_binary() {
    echo -e "${CYAN}[i] Building release binary...${NC}"
    cargo build --release
    echo -e "${GREEN}[+] Binary built successfully${NC}"
}

# Create app bundle structure
create_app_bundle() {
    echo -e "${CYAN}[i] Creating app bundle...${NC}"
    
    rm -rf "${APP_NAME}.app"
    mkdir -p "${APP_NAME}.app/Contents/MacOS"
    mkdir -p "${APP_NAME}.app/Contents/Resources"
    mkdir -p "${APP_NAME}.app/Contents/Frameworks"
    mkdir -p "${APP_NAME}.app/Contents/config"
    mkdir -p "${APP_NAME}.app/Contents/docs"
    
    # Copy binary
    cp target/release/sentinel "${APP_NAME}.app/Contents/MacOS/v-sentinel"
    chmod +x "${APP_NAME}.app/Contents/MacOS/v-sentinel"
    
    # Copy configuration
    cp config/default.toml "${APP_NAME}.app/Contents/config/config.toml"
    
    # Copy documentation
    cp README.md "${APP_NAME}.app/Contents/docs/"
    cp LICENSE "${APP_NAME}.app/Contents/docs/"
    cp INSTALL.md "${APP_NAME}.app/Contents/docs/"
    
    echo -e "${GREEN}[+] App bundle created${NC}"
}

# Create Info.plist
create_plist() {
    echo -e "${CYAN}[i] Creating Info.plist...${NC}"
    
    cat > "${APP_NAME}.app/Contents/Info.plist" << EOF
<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
    <key>CFBundleDevelopmentRegion</key>
    <string>en</string>
    <key>CFBundleExecutable</key>
    <string>v-sentinel</string>
    <key>CFBundleIconFile</key>
    <string>AppIcon</string>
    <key>CFBundleIdentifier</key>
    <string>com.vantiscorp.v-sentinel</string>
    <key>CFBundleInfoDictionaryVersion</key>
    <string>6.0</string>
    <key>CFBundleName</key>
    <string>V-Sentinel</string>
    <key>CFBundleDisplayName</key>
    <string>V-Sentinel</string>
    <key>CFBundlePackageType</key>
    <string>APPL</string>
    <key>CFBundleShortVersionString</key>
    <string>${VERSION}</string>
    <key>CFBundleVersion</key>
    <string>${VERSION}</string>
    <key>CFBundleURLTypes</key>
    <array>
        <dict>
            <key>CFBundleURLSchemes</key>
            <array>
                <string>v-sentinel</string>
            </array>
        </dict>
    </array>
    <key>LSMinimumSystemVersion</key>
    <string>10.15</string>
    <key>NSHighResolutionCapable</key>
    <true/>
    <key>NSHumanReadableCopyright</key>
    <string>Copyright © 2026 VantisCorp. All rights reserved.</string>
    <key>NSPrincipalClass</key>
    <string>NSApplication</string>
    <key>NSAppleEventsUsageDescription</key>
    <string>V-Sentinel requires permission to monitor system events for security analysis.</string>
    <key>NSCameraUsageDescription</key>
    <string>V-Sentinel uses the camera for behavioral biometrics verification.</string>
    <key>NSMicrophoneUsageDescription</key>
    <string>V-Sentinel uses the microphone for audio deepfake detection.</string>
</dict>
</plist>
EOF
    
    echo -e "${GREEN}[+] Info.plist created${NC}"
}

# Create entitlements
create_entitlements() {
    echo -e "${CYAN}[i] Creating entitlements...${NC}"
    
    cat > "${APP_NAME}.entitlements" << EOF
<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
    <key>com.apple.security.cs.allow-jit</key>
    <true/>
    <key>com.apple.security.cs.allow-unsigned-executable-memory</key>
    <true/>
    <key>com.apple.security.cs.disable-library-validation</key>
    <true/>
    <key>com.apple.security.device.camera</key>
    <true/>
    <key>com.apple.security.device.microphone</key>
    <true/>
    <key>com.apple.security.network.client</key>
    <true/>
    <key>com.apple.security.network.server</key>
    <true/>
</dict>
</plist>
EOF
    
    echo -e "${GREEN}[+] Entitlements created${NC}"
}

# Create DMG
create_dmg() {
    echo -e "${CYAN}[i] Creating DMG...${NC}"
    
    rm -f "${DMG_NAME}"
    
    # Create temporary DMG
    hdiutil create -srcfolder "${APP_NAME}.app" -volname "${APP_NAME}" -fs HFS+ \
        -fsargs "-c c=64,a=16,e=16" -format UDRW -size 200m temp.dmg
    
    # Convert to compressed DMG
    hdiutil convert temp.dmg -format UDZO -imagekey zlib-level=9 -o "${DMG_NAME}"
    
    # Cleanup
    rm -f temp.dmg
    
    # Move to releases
    mkdir -p releases
    mv "${DMG_NAME}" releases/
    
    echo -e "${GREEN}[+] DMG created${NC}"
}

# Create ZIP archive
create_zip() {
    echo -e "${CYAN}[i] Creating ZIP archive...${NC}"
    
    zip -r "releases/V-Sentinel-${VERSION}-macos.zip" "${APP_NAME}.app"
    
    echo -e "${GREEN}[+] ZIP archive created${NC}"
}

# Print summary
print_summary() {
    echo ""
    echo -e "${GREEN}================================================================${NC}"
    echo -e "${GREEN}  macOS Build Complete!${NC}"
    echo -e "${GREEN}================================================================${NC}"
    echo ""
    echo -e "  Output files:"
    echo -e "    ${CYAN}releases/V-Sentinel-${VERSION}.dmg${NC}"
    echo -e "    ${CYAN}releases/V-Sentinel-${VERSION}-macos.zip${NC}"
    echo ""
    echo -e "  Installation:"
    echo -e "    1. Open the DMG file"
    echo -e "    2. Drag V-Sentinel to Applications folder"
    echo -e "    3. Run from Applications or Spotlight"
    echo ""
    echo -e "${GREEN}================================================================${NC}"
}

# Main
main() {
    check_dependencies
    build_binary
    create_app_bundle
    create_plist
    create_entitlements
    create_dmg
    create_zip
    print_summary
}

main "$@"