@echo off
REM V-Sentinel v2.1.0 - Windows Batch Installer
REM Next-generation AI-native security system with quantum-ready cryptography

setlocal enabledelayedexpansion
set VERSION=2.1.0
set INSTALL_DIR=C:\Program Files\V-Sentinel

echo.
echo ================================================================
echo.
echo      V-SENTINEL v%VERSION%
echo.
echo      Next-Generation AI-Native Security System
echo      with Quantum-Ready Cryptography
echo.
echo ================================================================
echo.

REM Check for administrator privileges
net session >nul 2>&1
if %errorLevel% neq 0 (
    echo [!] This script requires Administrator privileges
    echo [i] Right-click and select "Run as administrator"
    pause
    exit /b 1
)

echo [i] Installing V-Sentinel v%VERSION%...
echo.

REM Create directories
echo [i] Creating directories...
if not exist "%INSTALL_DIR%" mkdir "%INSTALL_DIR%"
if not exist "%INSTALL_DIR%\config" mkdir "%INSTALL_DIR%\config"
if not exist "%INSTALL_DIR%\logs" mkdir "%INSTALL_DIR%\logs"
if not exist "%INSTALL_DIR%\modules" mkdir "%INSTALL_DIR%\modules%"
echo [+] Directories created
echo.

REM Check for Rust
echo [i] Checking for Rust installation...
where cargo >nul 2>&1
if %errorLevel% neq 0 (
    echo [!] Rust is not installed
    echo [i] Please install Rust from https://rustup.rs/
    echo [i] Or run: winget install Rustlang.Rust.MSVC
    pause
    exit /b 1
)
echo [+] Rust is installed
echo.

REM Build the project
echo [i] Building V-Sentinel...
cargo build --release
if %errorLevel% neq 0 (
    echo [!] Build failed
    pause
    exit /b 1
)
echo [+] Build completed
echo.

REM Install binary
echo [i] Installing binary...
if exist "target\release\sentinel.exe" (
    copy /Y "target\release\sentinel.exe" "%INSTALL_DIR%\v-sentinel.exe" >nul
    echo [+] Binary installed
) else (
    echo [!] Binary not found. Build may have failed.
    pause
    exit /b 1
)
echo.

REM Create default configuration
echo [i] Creating configuration...
if not exist "%INSTALL_DIR%\config\config.toml" (
    (
        echo # V-Sentinel Configuration File
        echo # Version: %VERSION%
        echo.
        echo [general]
        echo app_name = "V-Sentinel"
        echo log_level = "info"
        echo environment = "production"
        echo.
        echo [security]
        echo zero_trust_enabled = true
        echo shadow_ai_enabled = true
        echo deepfake_enabled = true
        echo ai_security_enabled = true
        echo.
        echo [api]
        echo host = "0.0.0.0"
        echo port = 8080
        echo tls_enabled = true
    ) > "%INSTALL_DIR%\config\config.toml"
)
echo [+] Configuration created
echo.

REM Add to PATH
echo [i] Adding to PATH...
setx /M PATH "%PATH%;%INSTALL_DIR%" >nul 2>&1
echo [+] Added to PATH
echo.

echo ================================================================
echo.
echo                   Installation Complete!
echo.
echo   V-Sentinel v%VERSION% has been installed successfully!
echo.
echo   Installation Directory:
echo     %INSTALL_DIR%
echo.
echo   Configuration:
echo     %INSTALL_DIR%\config\config.toml
echo.
echo   Logs:
echo     %INSTALL_DIR%\logs\
echo.
echo   To start V-Sentinel:
echo     v-sentinel.exe --config "%INSTALL_DIR%\config\config.toml"
echo.
echo   Documentation:
echo     https://github.com/vantisCorp/V-Sentinel
echo.
echo ================================================================
echo.

pause