# V-Sentinel Installers

This directory contains platform-specific installer builders for V-Sentinel v2.1.1.

## Quick Download

| Platform | File | Size |
|----------|------|------|
| Windows | `V-Sentinel-2.1.1-Setup.exe` | ~15 MB |
| Linux | `V-Sentinel-2.1.1-x86_64.AppImage` | ~20 MB |
| macOS | `V-Sentinel-2.1.1.dmg` | ~18 MB |

## Building Installers

### Windows (.exe)

**Requirements:**
- Windows 10/11
- [NSIS 3.08+](https://nsis.sourceforge.io/) (for building)
- Rust with Cargo

**Build:**
```batch
cd installer\windows
build-installer.bat
```

**Output:** `releases\V-Sentinel-2.1.1-Setup.exe`

The installer includes:
- Full GUI installation wizard
- Component selection (Core, Security Modules, PQC, SDKs)
- Windows Service integration
- Uninstaller
- Start Menu and Desktop shortcuts

---

### Linux (.AppImage)

**Requirements:**
- Linux x86_64
- Rust with Cargo
- wget

**Build:**
```bash
cd installer/linux
chmod +x build-appimage.sh
./build-appimage.sh
```

**Output:** `releases/V-Sentinel-2.1.1-x86_64.AppImage`

The AppImage is:
- Portable - no installation required
- Compatible with most Linux distributions
- Self-contained with all dependencies

**Usage:**
```bash
chmod +x V-Sentinel-2.1.1-x86_64.AppImage
./V-Sentinel-2.1.1-x86_64.AppImage
```

---

### macOS (.dmg)

**Requirements:**
- macOS 11+ (Big Sur)
- Xcode Command Line Tools
- Rust with Cargo

**Build:**
```bash
cd installer/macos
chmod +x build-dmg.sh
./build-dmg.sh
```

**Output:** 
- `releases/V-Sentinel-2.1.1.dmg`
- `releases/V-Sentinel-2.1.1-macos.zip`

**Installation:**
1. Open the DMG file
2. Drag V-Sentinel to Applications
3. Run from Applications or Spotlight

---

## Directory Structure

```
installer/
├── windows/
│   ├── v-sentinel-installer.nsi    # NSIS installer script
│   ├── build-installer.bat         # Windows build script
│   └── assets/                     # Installer graphics
│       ├── icon.ico
│       ├── welcome.bmp
│       └── header.bmp
├── linux/
│   ├── build-appimage.sh           # AppImage builder
│   └── assets/
│       └── icon.svg
├── macos/
│   ├── build-dmg.sh                # DMG builder
│   └── assets/
│       └── icon.icns
└── README.md                       # This file
```

---

## Customization

### Adding Custom Icons

**Windows:**
1. Create `installer/windows/assets/icon.ico` (256x256)
2. Create `installer/windows/assets/welcome.bmp` (164x314)
3. Create `installer/windows/assets/header.bmp` (150x57)

**Linux:**
1. Create `installer/linux/assets/icon.svg` or `icon.png`

**macOS:**
1. Create `installer/macos/assets/icon.icns`

### Modifying Installation Components

Edit the NSIS script for Windows:
```nsis
; In v-sentinel-installer.nsi
Section "Your Custom Section" SEC_CUSTOM
    ; Add your files
    File "path\to\your\files\*.*"
SectionEnd
```

---

## Release Process

1. Build all installers:
   ```bash
   # Windows (on Windows)
   cd installer/windows && build-installer.bat
   
   # Linux (on Linux)
   cd installer/linux && ./build-appimage.sh
   
   # macOS (on macOS)
   cd installer/macos && ./build-dmg.sh
   ```

2. Upload to GitHub Release:
   - `V-Sentinel-2.1.1-Setup.exe`
   - `V-Sentinel-2.1.1-x86_64.AppImage`
   - `V-Sentinel-2.1.1.dmg`

---

## Troubleshooting

### Windows
- **"NSIS not found"**: Install NSIS from https://nsis.sourceforge.io/
- **"Access denied"**: Run as Administrator

### Linux
- **"FUSE not found"**: Install fuse (`sudo apt install fuse libfuse2`)
- **"Permission denied"**: Run `chmod +x V-Sentinel-*.AppImage`

### macOS
- **"Cannot verify developer"**: Right-click > Open > Open
- **"App is damaged"**: Run `xattr -cr V-Sentinel.app`

---

© 2026 VantisCorp. All rights reserved.