#!/bin/bash
#
# V-Sentinel Linux AppImage Builder
# Requires: curl, wget, tar, appstream
#
# Usage: ./build-appimage.sh [VERSION]
# Example: ./build-appimage.sh 2.1.1

set -e

# Configuration
PRODUCT_NAME="V-Sentinel"
PRODUCT_EXEC="v-sentinel"
PRODUCT_ID="io.github.v-sentinel.V-Sentinel"
PRODUCT_COMMENT="AI-Powered Security Monitoring System"
PRODUCT_CATEGORIES="Security;System;Monitor;"
PRODUCT_KEYWORDS="security;monitoring;ai;threat;detection;"

# Version handling
if [ -z "$1" ]; then
    VERSION="2.1.1"
else
    VERSION="$1"
fi

APPIMAGE_NAME="${PRODUCT_NAME}-${VERSION}-x86_64.AppImage"

echo ""
echo "========================================"
echo " ${PRODUCT_NAME} Linux AppImage Builder"
echo "========================================"
echo ""
echo "Version: ${VERSION}"
echo "Output: ${APPIMAGE_NAME}"
echo ""

# Get script directory
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(cd "${SCRIPT_DIR}/../.." && pwd)"
BUILD_DIR="${SCRIPT_DIR}/build"
APPDIR="${BUILD_DIR}/${PRODUCT_NAME}.AppDir"

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

# Check for required tools
check_dependencies() {
    log_info "Checking dependencies..."
    
    local missing=()
    
    # Check for curl
    if ! command -v curl &> /dev/null; then
        missing+=("curl")
    fi
    
    # Check for wget
    if ! command -v wget &> /dev/null; then
        missing+=("wget")
    fi
    
    # Check for cargo (Rust)
    if ! command -v cargo &> /dev/null; then
        missing+=("cargo (Rust)")
    fi
    
    if [ ${#missing[@]} -ne 0 ]; then
        log_error "Missing dependencies: ${missing[*]}"
        echo ""
        echo "Please install missing dependencies and try again."
        echo "On Debian/Ubuntu: sudo apt install ${missing[*]}"
        echo "On Fedora: sudo dnf install ${missing[*]}"
        exit 1
    fi
    
    log_info "All dependencies found."
}

# Build release binary
build_binary() {
    log_info "Building release binary..."
    log_info "This may take several minutes..."
    
    cd "${PROJECT_ROOT}"
    cargo build --release
    
    if [ ! -f "target/release/${PRODUCT_EXEC}" ]; then
        log_error "Build failed! Binary not found."
        exit 1
    fi
    
    log_info "Build successful."
}

# Create AppDir structure
create_appdir() {
    log_info "Creating AppDir structure..."
    
    rm -rf "${BUILD_DIR}"
    mkdir -p "${APPDIR}"
    
    # Create directories
    mkdir -p "${APPDIR}/usr/bin"
    mkdir -p "${APPDIR}/usr/lib"
    mkdir -p "${APPDIR}/usr/share/applications"
    mkdir -p "${APPDIR}/usr/share/icons/hicolor/256x256/apps"
    mkdir -p "${APPDIR}/usr/share/icons/hicolor/scalable/apps"
    mkdir -p "${APPDIR}/usr/share/metainfo"
    mkdir -p "${APPDIR}/usr/share/doc/${PRODUCT_NAME}"
    mkdir -p "${APPDIR}/etc/${PRODUCT_NAME}"
    mkdir -p "${APPDIR}/opt/${PRODUCT_NAME}/modules"
    mkdir -p "${APPDIR}/opt/${PRODUCT_NAME}/pqc"
    mkdir -p "${APPDIR}/opt/${PRODUCT_NAME}/sdk"
    
    log_info "AppDir structure created."
}

# Install binary and files
install_files() {
    log_info "Installing files..."
    
    # Copy main binary
    cp "${PROJECT_ROOT}/target/release/${PRODUCT_EXEC}" "${APPDIR}/usr/bin/"
    chmod +x "${APPDIR}/usr/bin/${PRODUCT_EXEC}"
    
    # Copy configuration
    if [ -f "${PROJECT_ROOT}/config/default.toml" ]; then
        cp "${PROJECT_ROOT}/config/default.toml" "${APPDIR}/etc/${PRODUCT_NAME}/"
    fi
    
    # Copy documentation
    cp "${PROJECT_ROOT}/README.md" "${APPDIR}/usr/share/doc/${PRODUCT_NAME}/" 2>/dev/null || true
    cp "${PROJECT_ROOT}/LICENSE" "${APPDIR}/usr/share/doc/${PRODUCT_NAME}/" 2>/dev/null || true
    cp "${PROJECT_ROOT}/CHANGELOG.md" "${APPDIR}/usr/share/doc/${PRODUCT_NAME}/" 2>/dev/null || true
    
    # Copy modules if exist
    if [ -d "${PROJECT_ROOT}/modules" ]; then
        cp -r "${PROJECT_ROOT}/modules/"* "${APPDIR}/opt/${PRODUCT_NAME}/modules/" 2>/dev/null || true
    fi
    
    # Copy PQC if exist
    if [ -d "${PROJECT_ROOT}/pqc" ]; then
        cp -r "${PROJECT_ROOT}/pqc/"* "${APPDIR}/opt/${PRODUCT_NAME}/pqc/" 2>/dev/null || true
    fi
    
    # Copy SDK if exist
    if [ -d "${PROJECT_ROOT}/sdk" ]; then
        cp -r "${PROJECT_ROOT}/sdk/"* "${APPDIR}/opt/${PRODUCT_NAME}/sdk/" 2>/dev/null || true
    fi
    
    log_info "Files installed."
}

# Create desktop entry
create_desktop_entry() {
    log_info "Creating desktop entry..."
    
    cat > "${APPDIR}/usr/share/applications/${PRODUCT_ID}.desktop" << EOF
[Desktop Entry]
Type=Application
Name=${PRODUCT_NAME}
Comment=${PRODUCT_COMMENT}
Exec=${PRODUCT_EXEC}
Icon=${PRODUCT_ID}
Terminal=true
Categories=${PRODUCT_CATEGORIES}
Keywords=${PRODUCT_KEYWORDS}
StartupNotify=true
StartupWMClass=${PRODUCT_NAME}
X-AppImage-Version=${VERSION}
EOF
    
    # Also copy to AppDir root
    cp "${APPDIR}/usr/share/applications/${PRODUCT_ID}.desktop" "${APPDIR}/"
    
    log_info "Desktop entry created."
}

# Create AppStream metadata
create_appstream_metadata() {
    log_info "Creating AppStream metadata..."
    
    cat > "${APPDIR}/usr/share/metainfo/${PRODUCT_ID}.metainfo.xml" << EOF
<?xml version="1.0" encoding="UTF-8"?>
<component type="desktop-application">
  <id>${PRODUCT_ID}</id>
  <name>${PRODUCT_NAME}</name>
  <summary>${PRODUCT_COMMENT}</summary>
  <metadata_license>AGPL-3.0-or-later</metadata_license>
  <project_license>AGPL-3.0-or-later</project_license>
  <developer_name>V-Sentinel Security Team</developer_name>
  <url type="homepage">https://github.com/V-Sentinel/V-Sentinel</url>
  <url type="bugtracker">https://github.com/V-Sentinel/V-Sentinel/issues</url>
  <url type="donation">https://github.com/sponsors/V-Sentinel</url>
  
  <description>
    <p>
      V-Sentinel is an AI-powered security monitoring system that provides 
      comprehensive protection against modern cyber threats.
    </p>
    <p>Features:</p>
    <ul>
      <li>Zero Trust Architecture</li>
      <li>Shadow AI Detection</li>
      <li>Deepfake Detection</li>
      <li>Post-Quantum Cryptography</li>
      <li>Real-time Threat Intelligence</li>
      <li>Multi-language Support</li>
    </ul>
  </description>
  
  <launchable type="desktop-id">${PRODUCT_ID}.desktop</launchable>
  
  <categories>
    <category>Security</category>
    <category>System</category>
  </categories>
  
  <requires>
    <display_length compare="ge">medium</display_length>
  </requires>
  
  <content_rating type="oars-1.1"/>
  
  <releases>
    <release version="${VERSION}" date="$(date +%Y-%m-%d)">
      <description>
        <p>Latest release with security improvements and bug fixes.</p>
      </description>
    </release>
  </releases>
</component>
EOF
    
    log_info "AppStream metadata created."
}

# Create AppRun script
create_apprun() {
    log_info "Creating AppRun script..."
    
    cat > "${APPDIR}/AppRun" << 'EOF'
#!/bin/bash
SELF=$(readlink -f "$0")
HERE=${SELF%/*}
export PATH="${HERE}/usr/bin:${PATH}"
export LD_LIBRARY_PATH="${HERE}/usr/lib:${LD_LIBRARY_PATH}"
export XDG_DATA_DIRS="${HERE}/usr/share:${XDG_DATA_DIRS}"
export V_SENTINEL_HOME="${HERE}"
exec "${HERE}/usr/bin/v-sentinel" "$@"
EOF
    
    chmod +x "${APPDIR}/AppRun"
    
    log_info "AppRun created."
}

# Create icon
create_icon() {
    log_info "Creating application icon..."
    
    # Check for existing icon
    ICON_SVG="${PROJECT_ROOT}/assets/icon.svg"
    ICON_PNG="${PROJECT_ROOT}/assets/icon.png"
    
    if [ -f "${ICON_SVG}" ]; then
        cp "${ICON_SVG}" "${APPDIR}/usr/share/icons/hicolor/scalable/apps/${PRODUCT_ID}.svg"
        cp "${ICON_SVG}" "${APPDIR}/${PRODUCT_ID}.svg"
        log_info "SVG icon installed."
    elif [ -f "${ICON_PNG}" ]; then
        cp "${ICON_PNG}" "${APPDIR}/usr/share/icons/hicolor/256x256/apps/${PRODUCT_ID}.png"
        cp "${ICON_PNG}" "${APPDIR}/${PRODUCT_ID}.png"
        log_info "PNG icon installed."
    else
        log_warn "No icon found. Creating placeholder..."
        
        # Create a simple placeholder icon using ImageMagick if available
        if command -v convert &> /dev/null; then
            convert -size 256x256 xc:navy \
                -gravity center \
                -pointsize 72 \
                -fill white \
                -annotate 0 "VS" \
                "${APPDIR}/${PRODUCT_ID}.png"
            cp "${APPDIR}/${PRODUCT_ID}.png" "${APPDIR}/usr/share/icons/hicolor/256x256/apps/"
            log_info "Placeholder icon created."
        else
            log_warn "ImageMagick not found. Skipping icon creation."
        fi
    fi
}

# Download linuxdeploy
download_linuxdeploy() {
    log_info "Downloading linuxdeploy..."
    
    LINUXDEPLOY_URL="https://github.com/linuxdeploy/linuxdeploy/releases/download/continuous/linuxdeploy-x86_64.AppImage"
    LINUXDEPLOY="${BUILD_DIR}/linuxdeploy-x86_64.AppImage"
    
    if [ ! -f "${LINUXDEPLOY}" ]; then
        wget -q "${LINUXDEPLOY_URL}" -O "${LINUXDEPLOY}"
        chmod +x "${LINUXDEPLOY}"
        log_info "linuxdeploy downloaded."
    else
        log_info "linuxdeploy already exists."
    fi
}

# Build AppImage
build_appimage() {
    log_info "Building AppImage..."
    
    cd "${BUILD_DIR}"
    
    export OUTPUT="${APPIMAGE_NAME}"
    export ARCH=x86_64
    
    "${LINUXDEPLOY}" --appdir "${APPDIR}" --output appimage
    
    if [ -f "${BUILD_DIR}/${APPIMAGE_NAME}" ]; then
        mv "${BUILD_DIR}/${APPIMAGE_NAME}" "${SCRIPT_DIR}/"
        log_info "AppImage created: ${SCRIPT_DIR}/${APPIMAGE_NAME}"
    else
        log_error "AppImage not created!"
        exit 1
    fi
}

# Cleanup
cleanup() {
    log_info "Cleaning up..."
    rm -rf "${BUILD_DIR}"
}

# Main function
main() {
    echo ""
    
    check_dependencies
    build_binary
    create_appdir
    install_files
    create_desktop_entry
    create_appstream_metadata
    create_apprun
    create_icon
    download_linuxdeploy
    build_appimage
    cleanup
    
    echo ""
    echo "========================================"
    echo " BUILD SUCCESSFUL!"
    echo "========================================"
    echo ""
    echo "AppImage created: ${SCRIPT_DIR}/${APPIMAGE_NAME}"
    echo ""
    ls -lh "${SCRIPT_DIR}/${APPIMAGE_NAME}"
    echo ""
    echo "Next steps:"
    echo "  1. Test the AppImage: ./${APPIMAGE_NAME}"
    echo "  2. Upload to GitHub Releases"
    echo ""
}

# Run main
main "$@"