# V-Sentinel Installation Guide

## 📦 Installation Methods

V-Sentinel v2.1.0 can be installed using one of the following methods:

---

## 🐧 Linux Installation

### Quick Install (Recommended)

```bash
# Download and run the installer
curl -fsSL https://raw.githubusercontent.com/vantisCorp/V-Sentinel/main/install.sh | sudo bash
```

### Manual Install

```bash
# Clone the repository
git clone https://github.com/vantisCorp/V-Sentinel.git
cd V-Sentinel

# Make the installer executable
chmod +x install.sh

# Run the installer
sudo ./install.sh
```

### Post-Installation (Linux)

```bash
# Start the service
sudo systemctl start v-sentinel

# Check status
sudo systemctl status v-sentinel

# Enable auto-start on boot
sudo systemctl enable v-sentinel
```

---

## 🪟 Windows Installation

### Method 1: PowerShell (Recommended)

```powershell
# Run as Administrator
Set-ExecutionPolicy Bypass -Scope Process -Force

# Download and run
Invoke-WebRequest -Uri "https://raw.githubusercontent.com/vantisCorp/V-Sentinel/main/install.ps1" -OutFile "install.ps1"
.\install.ps1 -Install
```

### Method 2: Batch File

1. Download `install.bat`
2. Right-click and select "Run as Administrator"

### Post-Installation (Windows)

```powershell
# Start the service
Start-Service V-Sentinel

# Check status
Get-Service V-Sentinel
```

---

## 🍎 macOS Installation

```bash
# Install via Homebrew (coming soon)
brew install vantisCorp/v-sentinel/v-sentinel

# Or build from source
git clone https://github.com/vantisCorp/V-Sentinel.git
cd V-Sentinel
cargo build --release
sudo cp target/release/sentinel /usr/local/bin/v-sentinel
```

---

## 🐳 Docker Installation

```bash
# Pull the image
docker pull vantiscorp/v-sentinel:2.1.0

# Run container
docker run -d \
  --name v-sentinel \
  -p 8080:8080 \
  -p 9090:9090 \
  -v /etc/v-sentinel:/etc/v-sentinel \
  vantiscorp/v-sentinel:2.1.0
```

### Docker Compose

```yaml
version: '3.8'
services:
  v-sentinel:
    image: vantiscorp/v-sentinel:2.1.0
    container_name: v-sentinel
    restart: unless-stopped
    ports:
      - "8080:8080"
      - "9090:9090"
    volumes:
      - ./config:/etc/v-sentinel
      - ./logs:/var/log/v-sentinel
```

---

## 📋 Requirements

### Linux
- Ubuntu 20.04+, Debian 11+, RHEL 8+, CentOS 8+, or compatible
- 2GB RAM minimum (4GB recommended)
- 1GB disk space
- Root/sudo access

### Windows
- Windows 10/11 or Windows Server 2019+
- 2GB RAM minimum (4GB recommended)
- 1GB disk space
- Administrator access
- Visual Studio Build Tools (for compilation)

### macOS
- macOS 11 (Big Sur) or later
- 2GB RAM minimum (4GB recommended)
- 1GB disk space

---

## ⚙️ Configuration

The default configuration file is located at:

| Platform | Config Path |
|----------|-------------|
| Linux | `/etc/v-sentinel/config.toml` |
| Windows | `C:\Program Files\V-Sentinel\config\config.toml` |
| macOS | `/usr/local/etc/v-sentinel/config.toml` |

### Basic Configuration

```toml
[general]
app_name = "V-Sentinel"
log_level = "info"
environment = "production"

[security]
zero_trust_enabled = true
shadow_ai_enabled = true
deepfake_enabled = true
ai_security_enabled = true

[api]
host = "0.0.0.0"
port = 8080
tls_enabled = true

[monitoring]
metrics_enabled = true
metrics_port = 9090
```

---

## ✅ Verification

After installation, verify that V-Sentinel is working:

```bash
# Check version
v-sentinel --version

# Check health
curl http://localhost:8080/health

# View metrics
curl http://localhost:9090/metrics
```

---

## 🔄 Upgrading

### Linux
```bash
sudo systemctl stop v-sentinel
curl -fsSL https://raw.githubusercontent.com/vantisCorp/V-Sentinel/main/install.sh | sudo bash
sudo systemctl start v-sentinel
```

### Windows
```powershell
Stop-Service V-Sentinel
.\install.ps1 -Install
Start-Service V-Sentinel
```

---

## 🗑️ Uninstallation

### Linux
```bash
sudo systemctl stop v-sentinel
sudo systemctl disable v-sentinel
sudo rm -rf /opt/v-sentinel /etc/v-sentinel /var/log/v-sentinel
sudo rm /usr/local/bin/v-sentinel
sudo rm /etc/systemd/system/v-sentinel.service
sudo systemctl daemon-reload
```

### Windows
```powershell
.\install.ps1 -Uninstall
```

---

## 🆘 Troubleshooting

### Linux

**Service won't start:**
```bash
# Check logs
journalctl -u v-sentinel -f

# Check configuration
v-sentinel --config /etc/v-sentinel/config.toml --validate
```

**Permission denied:**
```bash
# Ensure correct permissions
sudo chown -R root:root /opt/v-sentinel
sudo chmod 755 /usr/local/bin/v-sentinel
```

### Windows

**Service won't start:**
```powershell
# Check Event Viewer
Get-EventLog -LogName Application -Source V-Sentinel -Newest 10

# Run manually for debugging
& "C:\Program Files\V-Sentinel\v-sentinel.exe" --config "C:\Program Files\V-Sentinel\config\config.toml"
```

---

## 📞 Support

- **Documentation:** https://github.com/vantisCorp/V-Sentinel
- **Issues:** https://github.com/vantisCorp/V-Sentinel/issues
- **Email:** support@vantiscorp.com

---

© 2026 VantisCorp. All rights reserved.