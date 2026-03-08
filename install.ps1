# V-Sentinel v2.1.0 - Windows Installation Script
# Next-generation AI-native security system with quantum-ready cryptography

param(
    [switch]$Install,
    [switch]$Uninstall,
    [switch]$Start,
    [switch]$Stop,
    [string]$ConfigPath = "",
    [string]$InstallDir = "C:\Program Files\V-Sentinel"
)

$VERSION = "2.1.0"
$ErrorActionPreference = "Stop"

# Colors
function Write-ColorOutput {
    param([string]$Message, [string]$Color = "White")
    Write-Host $Message -ForegroundColor $Color
}

# Print banner
function Print-Banner {
    Write-ColorOutput @"
================================================================================

     V-SENTINEL v$VERSION
     
     Next-Generation AI-Native Security System
     with Quantum-Ready Cryptography

================================================================================
"@ -Color Cyan
}

# Check administrator
function Check-Administrator {
    $currentUser = [Security.Principal.WindowsIdentity]::GetCurrent()
    $principal = New-Object Security.Principal.WindowsPrincipal($currentUser)
    $isAdmin = $principal.IsInRole([Security.Principal.WindowsBuiltInRole]::Administrator)
    
    if (-not $isAdmin) {
        Write-ColorOutput "[!] This script must be run as Administrator" -Color Yellow
        Write-ColorOutput "[i] Right-click PowerShell and select 'Run as Administrator'" -Color Cyan
        exit 1
    }
}

# Install dependencies
function Install-Dependencies {
    Write-ColorOutput "[i] Checking dependencies..." -Color Cyan
    
    # Check Rust
    $rustInstalled = Get-Command rustc -ErrorAction SilentlyContinue
    if (-not $rustInstalled) {
        Write-ColorOutput "[i] Installing Rust..." -Color Cyan
        winget install Rustlang.Rust.MSVC --silent --accept-source-agreements --accept-package-agreements
    }
    
    # Check Visual Studio Build Tools
    $vsBuildTools = Get-ItemProperty "HKLM:\SOFTWARE\Microsoft\VisualStudio\*" -ErrorAction SilentlyContinue
    if (-not $vsBuildTools) {
        Write-ColorOutput "[i] Installing Visual Studio Build Tools..." -Color Cyan
        winget install Microsoft.VisualStudio.2022.BuildTools --silent --accept-source-agreements --accept-package-agreements --override "--wait --quiet --add Microsoft.VisualStudio.Workload.VCTools --includeRecommended"
    }
    
    Write-ColorOutput "[+] Dependencies installed" -Color Green
}

# Create directories
function Create-Directories {
    Write-ColorOutput "[i] Creating directories..." -Color Cyan
    
    $directories = @(
        $InstallDir,
        "$InstallDir\config",
        "$InstallDir\logs",
        "$InstallDir\modules"
    )
    
    foreach ($dir in $directories) {
        if (-not (Test-Path $dir)) {
            New-Item -ItemType Directory -Path $dir -Force | Out-Null
        }
    }
    
    Write-ColorOutput "[+] Directories created" -Color Green
}

# Build project
function Build-Project {
    Write-ColorOutput "[i] Building V-Sentinel v$VERSION..." -Color Cyan
    
    $cargoPath = Get-Command cargo -ErrorAction SilentlyContinue
    if (-not $cargoPath) {
        Write-ColorOutput "[!] Cargo not found. Please install Rust first." -Color Red
        exit 1
    }
    
    cargo build --release
    
    Write-ColorOutput "[+] Build completed" -Color Green
}

# Install binaries
function Install-Binaries {
    Write-ColorOutput "[i] Installing binaries..." -Color Cyan
    
    if (Test-Path "target\release\sentinel.exe") {
        Copy-Item "target\release\sentinel.exe" "$InstallDir\v-sentinel.exe" -Force
    }
    
    # Add to PATH
    $currentPath = [Environment]::GetEnvironmentVariable("PATH", "Machine")
    if ($currentPath -notlike "*$InstallDir*") {
        [Environment]::SetEnvironmentVariable("PATH", "$currentPath;$InstallDir", "Machine")
    }
    
    Write-ColorOutput "[+] Binaries installed" -Color Green
}

# Install configuration
function Install-Config {
    Write-ColorOutput "[i] Installing configuration..." -Color Cyan
    
    $configFile = "$InstallDir\config\config.toml"
    
    if (-not (Test-Path $configFile)) {
        @"
# V-Sentinel Configuration File
# Version: $VERSION

[general]
app_name = "V-Sentinel"
log_level = "info"
environment = "production"

[security]
zero_trust_enabled = true
shadow_ai_enabled = true
deepfake_enabled = true
ai_security_enabled = true

[cryptography]
pq_algorithm = "kyber768"
sig_algorithm = "dilithium3"
hybrid_encryption = true

[api]
host = "0.0.0.0"
port = 8080
tls_enabled = true

[monitoring]
metrics_enabled = true
metrics_port = 9090
health_checks_enabled = true

[logging]
log_file = "C:\\Program Files\\V-Sentinel\\logs\\v-sentinel.log"
max_log_size = 100
backup_count = 5
"@ | Out-File -FilePath $configFile -Encoding utf8
    }
    
    Write-ColorOutput "[+] Configuration installed" -Color Green
}

# Create Windows Service
function Create-Service {
    Write-ColorOutput "[i] Creating Windows Service..." -Color Cyan
    
    $serviceName = "V-Sentinel"
    $serviceExists = Get-Service -Name $serviceName -ErrorAction SilentlyContinue
    
    if ($serviceExists) {
        Remove-Service -Name $serviceName -Force
        Start-Sleep -Seconds 2
    }
    
    New-Service -Name $serviceName `
                -BinaryPathName "$InstallDir\v-sentinel.exe --config $InstallDir\config\config.toml" `
                -DisplayName "V-Sentinel Security System" `
                -Description "Next-generation AI-native security system with quantum-ready cryptography" `
                -StartupType Automatic
    
    Write-ColorOutput "[+] Windows Service created" -Color Green
}

# Print post-install
function Print-PostInstall {
    Write-ColorOutput @"

================================================================================
                      Installation Complete!
================================================================================

  V-Sentinel v$VERSION has been installed successfully!

  Quick Start:
    Start-Service V-Sentinel
    Get-Service V-Sentinel

  Configuration:
    $InstallDir\config\config.toml

  Logs:
    $InstallDir\logs\

  Documentation:
    https://github.com/vantisCorp/V-Sentinel

================================================================================
"@ -Color Green
}

# Uninstall
function Uninstall-VSentinel {
    Write-ColorOutput "[i] Uninstalling V-Sentinel..." -Color Cyan
    
    # Stop and remove service
    $service = Get-Service -Name "V-Sentinel" -ErrorAction SilentlyContinue
    if ($service) {
        Stop-Service -Name "V-Sentinel" -Force
        Remove-Service -Name "V-Sentinel" -Force
    }
    
    # Remove files
    if (Test-Path $InstallDir) {
        Remove-Item -Path $InstallDir -Recurse -Force
    }
    
    # Remove from PATH
    $currentPath = [Environment]::GetEnvironmentVariable("PATH", "Machine")
    $newPath = ($currentPath -split ';' | Where-Object { $_ -ne $InstallDir }) -join ';'
    [Environment]::SetEnvironmentVariable("PATH", $newPath, "Machine")
    
    Write-ColorOutput "[+] V-Sentinel uninstalled" -Color Green
}

# Main
Print-Banner
Check-Administrator

if ($Uninstall) {
    Uninstall-VSentinel
} elseif ($Start) {
    Start-Service -Name "V-Sentinel"
    Write-ColorOutput "[+] V-Sentinel started" -Color Green
} elseif ($Stop) {
    Stop-Service -Name "V-Sentinel"
    Write-ColorOutput "[+] V-Sentinel stopped" -Color Green
} else {
    Install-Dependencies
    Create-Directories
    Build-Project
    Install-Binaries
    Install-Config
    Create-Service
    Print-PostInstall
}