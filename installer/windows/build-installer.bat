@echo off
REM V-Sentinel Windows Installer Builder
REM Requires: NSIS 3.08+ (https://nsis.sourceforge.io/)

setlocal enabledelayedexpansion
set VERSION=2.1.1
set PRODUCT_NAME=V-Sentinel

echo.
echo ================================================================
echo   V-Sentinel Windows Installer Builder v%VERSION%
echo ================================================================
echo.

REM Check for NSIS
where makensis >nul 2>&1
if %errorLevel% neq 0 (
    echo [!] NSIS not found. Please install NSIS 3.08+
    echo [i] Download from: https://nsis.sourceforge.io/
    echo [i] Or run: winget install NSIS.NSIS
    pause
    exit /b 1
)

REM Check for Rust
where cargo >nul 2>&1
if %errorLevel% neq 0 (
    echo [!] Rust not found. Please install Rust.
    echo [i] Run: winget install Rustlang.Rust.MSVC
    pause
    exit /b 1
)

echo [i] Building V-Sentinel release binary...
cargo build --release
if %errorLevel% neq 0 (
    echo [!] Build failed!
    pause
    exit /b 1
)
echo [+] Build completed

REM Create asset directories if they don't exist
if not exist "assets" mkdir "assets"
if not exist "installer\windows\assets" mkdir "installer\windows\assets"

echo.
echo [i] Creating installer assets...

REM Create a simple icon (placeholder)
if not exist "assets\icon.ico" (
    echo [!] Warning: assets\icon.ico not found
    echo [i] Please provide an icon file or the installer will use default icon
)

echo [i] Building NSIS installer...
cd installer\windows

makensis /DPRODUCT_VERSION=%VERSION% v-sentinel-installer.nsi

if %errorLevel% neq 0 (
    echo [!] NSIS compilation failed!
    cd ..\..
    pause
    exit /b 1
)

cd ..\..

REM Move installer to releases folder
if not exist "releases" mkdir "releases"
move /Y "installer\windows\V-Sentinel-%VERSION%-Setup.exe" "releases&quot; >nul 2>&1

echo.
echo ================================================================
echo   Installer created successfully!
echo
echo   Output: releases\V-Sentinel-%VERSION%-Setup.exe
echo   Size:
for %%A in (releases\V-Sentinel-%VERSION%-Setup.exe) do echo     %%~zA bytes
echo ================================================================
echo.

pause