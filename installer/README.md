# V-Sentinel GUI Installers

This directory contains platform-specific GUI installers for V-Sentinel. These installers provide a user-friendly installation experience with visual wizards, component selection, and automatic integration with the operating system.

## Available Installers

| Platform | Format | File Name | Description |
|----------|--------|-----------|-------------|
| Windows | `.exe` | `V-Sentinel-2.1.1-Setup.exe` | NSIS-based installer with GUI wizard |
| Linux | `.AppImage` | `V-Sentinel-2.1.1-x86_64.AppImage` | Portable AppImage (no installation required) |
| macOS | `.dmg` | `V-Sentinel-2.1.1.dmg` | DMG disk image with drag-and-drop installation |

---

## Windows Installer

### Requirements

- Windows 10/11 (64-bit)
- Administrator privileges for installation
- ~150 MB disk space

### Building the Installer

**Prerequisites:**
- [NSIS (Nullsoft Scriptable Install System)](https://nsis.sourceforge.io/Download) 3.08 or later
- [Rust](https://rustup.rs/) with cargo
- Visual Studio Build Tools (for compiling Rust)

**Build Steps:**

```cmd
cd installer\windows
build-installer.bat
```

Or with a specific version:

```cmd
build-installer.bat 2.1.2
```

**Output:** `V-Sentinel-VERSION-Setup.exe`

### Features

- Multi-language support (English, Polish, German, French, Spanish, Russian, Chinese, Japanese, Korean)
- Component selection:
  - Core Engine (required)
  - Security Modules (Zero Trust, Shadow AI Detection, Deepfake Detection)
  - Post-Quantum Cryptography module
  - SDK files for integrations
  - Windows Service installation
- Start Menu shortcuts
- Desktop shortcut
- Automatic uninstaller
- Windows Service integration

### Code Signing (Optional)

For production distribution, sign the installer with your code signing certificate:

```cmd
signtool sign /f "path\to\certificate.pfx" /p "password" /tr http://timestamp.digicert.com /td sha256 /fd sha256 "V-Sentinel-VERSION-Setup.exe"
```

---

## Linux AppImage

### Requirements

- Linux x86_64 (any modern distribution)
- FUSE (for running AppImage)
- ~200 MB disk space

### Building the AppImage

**Prerequisites:**
- curl, wget, tar
- Rust with cargo
- AppStream utilities (optional, for metadata)

**Build Steps:**

```bash
cd installer/linux
chmod +x build-appimage.sh
./build-appimage.sh
```

Or with a specific version:

```bash
./build-appimage.sh 2.1.2
```

**Output:** `V-Sentinel-VERSION-x86_64.AppImage`

### Usage

The AppImage is a portable executable - no installation required:

```bash
# Make executable
chmod +x V-Sentinel-2.1.1-x86_64.AppImage

# Run directly
./V-Sentinel-2.1.1-x86_64.AppImage

# Extract to inspect contents (optional)
./V-Sentinel-2.1.1-x86_64.AppImage --appimage-extract
```

### System Integration

To integrate with your desktop environment:

```bash
# Install desktop entry
./V-Sentinel-2.1.1-x86_64.AppImage --appimage-install
```

---

## macOS DMG

### Requirements

- macOS 10.15 (Catalina) or later
- Intel or Apple Silicon Mac
- ~200 MB disk space

### Building the DMG

**Prerequisites:**
- Xcode Command Line Tools: `xcode-select --install`
- Rust with cargo: `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`
- Optional: [create-dmg](https://github.com/create-dmg/create-dmg) for better DMG appearance

**Build Steps:**

```bash
cd installer/macos
chmod +x build-dmg.sh
./build-dmg.sh
```

Or with a specific version:

```bash
./build-dmg.sh 2.1.2
```

**Output:**
- `V-Sentinel-VERSION.dmg` - DMG disk image
- `V-Sentinel-VERSION-macos.zip` - ZIP archive (alternative)

### Installation

1. Open the DMG file
2. Drag V-Sentinel.app to the Applications folder
3. Launch from Applications or Spotlight

### Notarization (For Distribution)

For distribution outside the Mac App Store, notarize the app:

```bash
# Submit for notarization
xcrun notarytool submit V-Sentinel-2.1.1.dmg --apple-id "your@email.com" --password "app-specific-password" --team-id "TEAMID" --wait

# Staple the ticket
xcrun stapler staple V-Sentinel-2.1.1.dmg
```

---

## CI/CD Integration

### GitHub Actions Example

```yaml
name: Build Installers

on:
  push:
    tags:
      - 'v*'

jobs:
  build-windows:
    runs-on: windows-latest
    steps:
      - uses: actions/checkout@v4
      - uses: ilammy/msvc-dev-cmd@v1
      - uses: NuGet/setup-nuget@v1
      - run: |
          choco install nsis -y
          cargo build --release
          cd installer/windows
          makensis v-sentinel-installer.nsi
      - uses: actions/upload-artifact@v4
        with:
          name: windows-installer
          path: installer/windows/*.exe

  build-linux:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - run: |
          sudo apt-get install -y libfuse2
          cargo build --release
          cd installer/linux
          chmod +x build-appimage.sh
          ./build-appimage.sh
      - uses: actions/upload-artifact@v4
        with:
          name: linux-appimage
          path: installer/linux/*.AppImage

  build-macos:
    runs-on: macos-latest
    steps:
      - uses: actions/checkout@v4
      - run: |
          cargo build --release
          cd installer/macos
          chmod +x build-dmg.sh
          ./build-dmg.sh
      - uses: actions/upload-artifact@v4
        with:
          name: macos-dmg
          path: installer/macos/*.dmg

  release:
    needs: [build-windows, build-linux, build-macos]
    runs-on: ubuntu-latest
    steps:
      - uses: actions/download-artifact@v4
      - uses: softprops/action-gh-release@v1
        with:
          files: |
            windows-installer/*.exe
            linux-appimage/*.AppImage
            macos-dmg/*.dmg
```

---

## Directory Structure

```
installer/
├── README.md                   # This file
├── windows/
│   ├── v-sentinel-installer.nsi    # NSIS installer script
│   └── build-installer.bat         # Build script for Windows
├── linux/
│   └── build-appimage.sh           # AppImage builder script
└── macos/
    └── build-dmg.sh                # DMG builder script
```

---

## Customization

### Adding Assets

**Windows:**
- `assets/icon.ico` - Application icon (256x256 recommended)
- `assets/wizard.bmp` - Installer wizard image (164x314 pixels)

**Linux:**
- `assets/icon.svg` - Application icon (SVG preferred)
- `assets/icon.png` - Application icon (256x256 PNG)

**macOS:**
- `assets/icon.png` - Application icon (1024x1024 PNG)
- `assets/dmg-background.png` - DMG background image

### Changing Version

All installer scripts accept a version parameter:

```bash
# Windows
build-installer.bat 2.2.0

# Linux
./build-appimage.sh 2.2.0

# macOS
./build-dmg.sh 2.2.0
```

Or modify the `VERSION` variable at the top of each build script.

---

## Support

For issues or questions about the installers:

- GitHub Issues: https://github.com/V-Sentinel/V-Sentinel/issues
- Documentation: https://github.com/V-Sentinel/V-Sentinel/wiki

---

## License

V-Sentinel is dual-licensed:
- **Open Source**: AGPL-3.0 (for open-source projects)
- **Commercial**: Available for commercial use (contact for licensing)