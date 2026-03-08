#!/bin/bash
#
# V-Sentinel macOS DMG Builder
# Requires: Xcode Command Line Tools, cargo (Rust)
#
# Usage: ./build-dmg.sh [VERSION]
# Example: ./build-dmg.sh 2.1.1

set -e

# Configuration
PRODUCT_NAME="V-Sentinel"
PRODUCT_EXEC="v-sentinel"
PRODUCT_ID="io.github.v-sentinel.V-Sentinel"
BUNDLE_ID="io.github.v-sentinel.v-sentinel"
PRODUCT_COMMENT="AI-Powered Security Monitoring System"

# Version handling
if [ -z "$1" ]; then
    VERSION="2.1.1"
else
    VERSION="$1"
fi

DMG_NAME="${PRODUCT_NAME}-${VERSION}.dmg"
ZIP_NAME="${PRODUCT_NAME}-${VERSION}-macos.zip"
APP_NAME="${PRODUCT_NAME}.app"

echo ""
echo "========================================"
echo " ${PRODUCT_NAME} macOS DMG Builder"
echo "========================================"
echo ""
echo "Version: ${VERSION}"
echo "Output: ${DMG_NAME}"
echo ""

# Get script directory
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(cd "${SCRIPT_DIR}/../.." && pwd)"
BUILD_DIR="${SCRIPT_DIR}/build"
APP_DIR="${BUILD_DIR}/${APP_NAME}"

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

log_info() {
    echo -e "${GREEN}[INFO]${NC} $1"
}

log_warn() {
    echo -e "${YELLOW}[WARN]${NC} $1"
}

log_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

# Check for macOS
check_platform() {
    if [[ "$(uname)" != "Darwin" ]]; then
        log_error "This script must be run on macOS!"
        exit 1
    fi
    log_info "Platform check passed."
}

# Check for required tools
check_dependencies() {
    log_info "Checking dependencies..."
    
    local missing=()
    
    # Check for Xcode tools
    if ! command -v xcodebuild &> /dev/null; then
        missing+=("Xcode Command Line Tools")
    fi
    
    # Check for cargo (Rust)
    if ! command -v cargo &> /dev/null; then
        missing+=("cargo (Rust)")
    fi
    
    # Check for create-dmg (optional but recommended)
    if ! command -v create-dmg &> /dev/null; then
        log_warn "create-dmg not found. Will use basic hdiutil method."
    fi
    
    if [ ${#missing[@]} -ne 0 ]; then
        log_error "Missing dependencies: ${missing[*]}"
        echo ""
        echo "Please install missing dependencies and try again."
        echo "  Xcode: xcode-select --install"
        echo "  Rust: curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh"
        exit 1
    fi
    
    log_info "All dependencies found."
}

# Build release binary
build_binary() {
    log_info "Building release binary..."
    log_info "This may take several minutes..."
    
    cd "${PROJECT_ROOT}"
    
    # Build for current architecture
    cargo build --release
    
    if [ ! -f "target/release/${PRODUCT_EXEC}" ]; then
        log_error "Build failed! Binary not found."
        exit 1
    fi
    
    log_info "Build successful."
}

# Create app bundle structure
create_app_bundle() {
    log_info "Creating app bundle structure..."
    
    rm -rf "${BUILD_DIR}"
    mkdir -p "${APP_DIR}/Contents/MacOS"
    mkdir -p "${APP_DIR}/Contents/Resources"
    mkdir -p "${APP_DIR}/Contents/Frameworks"
    mkdir -p "${APP_DIR}/Contents/share/doc/${PRODUCT_NAME}"
    mkdir -p "${APP_DIR}/Contents/etc/${PRODUCT_NAME}"
    mkdir -p "${APP_DIR}/Contents/opt/${PRODUCT_NAME}/modules"
    mkdir -p "${APP_DIR}/Contents/opt/${PRODUCT_NAME}/pqc"
    mkdir -p "${APP_DIR}/Contents/opt/${PRODUCT_NAME}/sdk"
    
    log_info "App bundle structure created."
}

# Create Info.plist
create_info_plist() {
    log_info "Creating Info.plist..."
    
    # Detect architecture
    ARCH=$(uname -m)
    
    cat > "${APP_DIR}/Contents/Info.plist" << EOF
<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
    <key>CFBundleDevelopmentRegion</key>
    <string>en</string>
    <key>CFBundleExecutable</key>
    <string>${PRODUCT_EXEC}</string>
    <key>CFBundleIconFile</key>
    <string>AppIcon</string>
    <key>CFBundleIdentifier</key>
    <string>${BUNDLE_ID}</string>
    <key>CFBundleInfoDictionaryVersion</key>
    <string>6.0</string>
    <key>CFBundleName</key>
    <string>${PRODUCT_NAME}</string>
    <key>CFBundlePackageType</key>
    <string>APPL</string>
    <key>CFBundleShortVersionString</key>
    <string>${VERSION}</string>
    <key>CFBundleVersion</key>
    <string>${VERSION}</string>
    <key>CFBundleSupportedPlatforms</key>
    <array>
        <string>MacOSX</string>
    </array>
    <key>LSApplicationCategoryType</key>
    <string>public.app-category.utilities</string>
    <key>LSMinimumSystemVersion</key>
    <string>10.15</string>
    <key>NSHighResolutionCapable</key>
    <true/>
    <key>NSHumanReadableCopyright</key>
    <string>Copyright © 2024 V-Sentinel Security Team. Licensed under AGPL-3.0.</string>
    <key>NSPrincipalClass</key>
    <string>NSApplication</string>
    <key>NSSupportsAutomaticGraphicsSwitching</key>
    <true/>
    <key>LSUIElement</key>
    <false/>
    <key>NSAppleEventsUsageDescription</key>
    <string>V-Sentinel needs to send Apple Events for system monitoring.</string>
    <key>NSCameraUsageDescription</key>
    <string>V-Sentinel may use the camera for deepfake detection analysis.</string>
    <key>NSMicrophoneUsageDescription</key>
    <string>V-Sentinel may use the microphone for audio deepfake detection.</string>
</dict>
</plist>
EOF
    
    log_info "Info.plist created."
}

# Install binary and files
install_files() {
    log_info "Installing files..."
    
    # Copy main binary
    cp "${PROJECT_ROOT}/target/release/${PRODUCT_EXEC}" "${APP_DIR}/Contents/MacOS/"
    chmod +x "${APP_DIR}/Contents/MacOS/${PRODUCT_EXEC}"
    
    # Copy configuration
    if [ -f "${PROJECT_ROOT}/config/default.toml" ]; then
        cp "${PROJECT_ROOT}/config/default.toml" "${APP_DIR}/Contents/etc/${PRODUCT_NAME}/"
    fi
    
    # Copy documentation
    cp "${PROJECT_ROOT}/README.md" "${APP_DIR}/Contents/share/doc/${PRODUCT_NAME}/" 2>/dev/null || true
    cp "${PROJECT_ROOT}/LICENSE" "${APP_DIR}/Contents/share/doc/${PRODUCT_NAME}/" 2>/dev/null || true
    cp "${PROJECT_ROOT}/CHANGELOG.md" "${APP_DIR}/Contents/share/doc/${PRODUCT_NAME}/" 2>/dev/null || true
    
    # Copy modules if exist
    if [ -d "${PROJECT_ROOT}/modules" ]; then
        cp -r "${PROJECT_ROOT}/modules/"* "${APP_DIR}/Contents/opt/${PRODUCT_NAME}/modules/" 2>/dev/null || true
    fi
    
    # Copy PQC if exist
    if [ -d "${PROJECT_ROOT}/pqc" ]; then
        cp -r "${PROJECT_ROOT}/pqc/"* "${APP_DIR}/Contents/opt/${PRODUCT_NAME}/pqc/" 2>/dev/null || true
    fi
    
    # Copy SDK if exist
    if [ -d "${PROJECT_ROOT}/sdk" ]; then
        cp -r "${PROJECT_ROOT}/sdk/"* "${APP_DIR}/Contents/opt/${PRODUCT_NAME}/sdk/" 2>/dev/null || true
    fi
    
    log_info "Files installed."
}

# Create application icon
create_icon() {
    log_info "Creating application icon..."
    
    ICON_PNG="${PROJECT_ROOT}/assets/icon.png"
    ICON_ICNS="${APP_DIR}/Contents/Resources/AppIcon.icns"
    
    if [ -f "${ICON_PNG}" ]; then
        # Create iconset directory
        ICONSET_DIR="${BUILD_DIR}/AppIcon.iconset"
        mkdir -p "${ICONSET_DIR}"
        
        # Generate all required sizes
        sips -z 16 16 "${ICON_PNG}" --out "${ICONSET_DIR}/icon_16x16.png"
        sips -z 32 32 "${ICON_PNG}" --out "${ICONSET_DIR}/icon_16x16@2x.png"
        sips -z 32 32 "${ICON_PNG}" --out "${ICONSET_DIR}/icon_32x32.png"
        sips -z 64 64 "${ICON_PNG}" --out "${ICONSET_DIR}/icon_32x32@2x.png"
        sips -z 128 128 "${ICON_PNG}" --out "${ICONSET_DIR}/icon_128x128.png"
        sips -z 256 256 "${ICON_PNG}" --out "${ICONSET_DIR}/icon_128x128@2x.png"
        sips -z 256 256 "${ICON_PNG}" --out "${ICONSET_DIR}/icon_256x256.png"
        sips -z 512 512 "${ICON_PNG}" --out "${ICONSET_DIR}/icon_256x256@2x.png"
        sips -z 512 512 "${ICON_PNG}" --out "${ICONSET_DIR}/icon_512x512.png"
        sips -z 1024 1024 "${ICON_PNG}" --out "${ICONSET_DIR}/icon_512x512@2x.png"
        
        # Create icns
        iconutil -c icns "${ICONSET_DIR}" -o "${ICON_ICNS}"
        
        log_info "Application icon created."
    else
        log_warn "No icon found at ${ICON_PNG}. Skipping icon creation."
    fi
}

# Create entitlements file
create_entitlements() {
    log_info "Creating entitlements file..."
    
    ENTITLEMENTS="${BUILD_DIR}/entitlements.plist"
    
    cat > "${ENTITLEMENTS}" << EOF
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
    <key>com.apple.security.automation.apple-events</key>
    <true/>
    <key>com.apple.security.network.client</key>
    <true/>
    <key>com.apple.security.network.server</key>
    <true/>
    <key>com.apple.security.files.user-selected.read-write</key>
    <true/>
</dict>
</plist>
EOF
    
    log_info "Entitlements file created."
}

# Sign the application
sign_app() {
    log_info "Signing application..."
    
    ENTITLEMENTS="${BUILD_DIR}/entitlements.plist"
    
    # Get signing identity (use ad-hoc signing if no developer certificate)
    SIGN_IDENTITY="-"
    
    # Check for developer certificate
    if security find-identity -v -p codesigning | grep -q "Developer ID"; then
        SIGN_IDENTITY=$(security find-identity -v -p codesigning | head -1 | awk -F'"' '{print $2}')
        log_info "Using signing identity: ${SIGN_IDENTITY}"
    else
        log_warn "No Developer ID certificate found. Using ad-hoc signing."
        log_warn "The app will not be notarized. For distribution, use proper signing."
    fi
    
    # Sign the app
    codesign --force --deep --sign "${SIGN_IDENTITY}" \
        --entitlements "${ENTITLEMENTS}" \
        --options runtime \
        "${APP_DIR}"
    
    if [ $? -eq 0 ]; then
        log_info "Application signed successfully."
    else
        log_warn "Signing failed. Continuing with unsigned app."
    fi
}

# Create DMG
create_dmg() {
    log_info "Creating DMG..."
    
    DMG_PATH="${SCRIPT_DIR}/${DMG_NAME}"
    
    # Remove existing DMG
    rm -f "${DMG_PATH}"
    
    if command -v create-dmg &> /dev/null; then
        # Use create-dmg for better DMG creation
        create-dmg \
            --volname "${PRODUCT_NAME}" \
            --volicon "${PROJECT_ROOT}/assets/icon.icns" \
            --background "${PROJECT_ROOT}/assets/dmg-background.png" \
            --window-pos 200 120 \
            --window-size 660 400 \
            --icon-size 100 \
            --icon "${APP_NAME}" 180 170 \
            --hide-extension "${APP_NAME}" \
            --app-drop-link 480 170 \
            "${DMG_PATH}" \
            "${APP_DIR}"
    else
        # Fallback to hdiutil
        log_info "Using hdiutil for DMG creation..."
        
        DMG_TEMP="${BUILD_DIR}/temp.dmg"
        
        # Create temporary DMG
        hdiutil create -volname "${PRODUCT_NAME}" \
            -srcfolder "${APP_DIR}" \
            -fs HFS+ \
            -fsargs "-c c=64,a=16,e=16" \
            -format UDRW \
            "${DMG_TEMP}"
        
        # Convert to compressed DMG
        hdiutil convert "${DMG_TEMP}" \
            -format UDZO \
            -imagekey zlib-level=9 \
            -o "${DMG_PATH}"
        
        rm -f "${DMG_TEMP}"
    fi
    
    if [ -f "${DMG_PATH}" ]; then
        log_info "DMG created: ${DMG_PATH}"
        ls -lh "${DMG_PATH}"
    else
        log_error "DMG creation failed!"
        exit 1
    fi
}

# Create ZIP archive (alternative distribution)
create_zip() {
    log_info "Creating ZIP archive..."
    
    ZIP_PATH="${SCRIPT_DIR}/${ZIP_NAME}"
    
    cd "${BUILD_DIR}"
    zip -r "${ZIP_PATH}" "${APP_NAME}"
    
    if [ -f "${ZIP_PATH}" ]; then
        log_info "ZIP created: ${ZIP_PATH}"
        ls -lh "${ZIP_PATH}"
    else
        log_warn "ZIP creation failed."
    fi
}

# Cleanup
cleanup() {
    log_info "Cleaning up..."
    # Keep the app bundle for potential debugging
    # rm -rf "${BUILD_DIR}"
}

# Main function
main() {
    echo ""
    
    check_platform
    check_dependencies
    build_binary
    create_app_bundle
    create_info_plist
    install_files
    create_icon
    create_entitlements
    sign_app
    create_dmg
    create_zip
    cleanup
    
    echo ""
    echo "========================================"
    echo " BUILD SUCCESSFUL!"
    echo "========================================"
    echo ""
    echo "DMG created: ${SCRIPT_DIR}/${DMG_NAME}"
    echo "ZIP created: ${SCRIPT_DIR}/${ZIP_NAME}"
    echo ""
    echo "Next steps:"
    echo "  1. Test the DMG: open ${SCRIPT_DIR}/${DMG_NAME}"
    echo "  2. For notarization: xcrun notarytool submit ${DMG_NAME} --wait"
    echo "  3. Upload to GitHub Releases"
    echo ""
}

# Run main
main "$@"