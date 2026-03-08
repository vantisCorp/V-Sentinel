@echo off
REM V-Sentinel Windows Installer Builder
REM Downloads pre-built binary from GitHub Releases
REM Requires: NSIS 3.08+, PowerShell (for downloading)
REM 
REM Usage: build-installer.bat [VERSION]
REM Example: build-installer.bat 2.1.2

setlocal EnableDelayedExpansion

REM Configuration
set "PRODUCT_NAME=V-Sentinel"
if "%~1"=="" (
    set "VERSION=2.1.2"
) else (
    set "VERSION=%~1"
)
set "INSTALLER_NAME=%PRODUCT_NAME%-!VERSION!-Setup.exe"
set "GITHUB_REPO=vantisCorp/V-Sentinel"
set "BINARY_NAME=v-sentinel-!VERSION!-x86_64-pc-windows-msvc.zip"

echo.
echo ========================================
echo  %PRODUCT_NAME% Windows Installer Builder
echo ========================================
echo.
echo Version: !VERSION!
echo Output: !INSTALLER_NAME!
echo.

REM Check for NSIS
echo [1/4] Checking for NSIS...
where makensis >nul 2>&1
if %ERRORLEVEL% neq 0 (
    echo ERROR: NSIS not found in PATH!
    echo.
    echo Please install NSIS from: https://nsis.sourceforge.io/Download
    echo After installation, add NSIS to your system PATH.
    echo.
    pause
    exit /b 1
)
echo       NSIS found.

REM Create bin directory
echo [2/4] Preparing files...
if not exist "bin" mkdir "bin"

REM Download pre-built binary from GitHub Releases
echo [3/4] Downloading pre-built binary from GitHub Releases...
echo       URL: https://github.com/!GITHUB_REPO!/releases/download/v!VERSION!/!BINARY_NAME!

powershell -Command "& { ^
    $url = 'https://github.com/!GITHUB_REPO!/releases/download/v!VERSION!/!BINARY_NAME!'; ^
    $output = 'bin\!BINARY_NAME!'; ^
    try { ^
        Write-Host 'Downloading...'; ^
        [Net.ServicePointManager]::SecurityProtocol = [Net.SecurityProtocolType]::Tls12; ^
        Invoke-WebRequest -Uri $url -OutFile $output -UseBasicParsing; ^
        Write-Host 'Download completed.'; ^
    } catch { ^
        Write-Host 'ERROR: Download failed!' -ForegroundColor Red; ^
        Write-Host 'The Windows binary may not be available yet.'; ^
        Write-Host 'Please check: https://github.com/!GITHUB_REPO!/releases'; ^
        exit 1; ^
    } ^
}"

if %ERRORLEVEL% neq 0 (
    echo.
    echo ERROR: Failed to download pre-built binary!
    echo.
    echo The Windows binary for version !VERSION! may not be available yet.
    echo Please check: https://github.com/!GITHUB_REPO!/releases
    echo.
    pause
    exit /b 1
)

REM Extract the binary
echo       Extracting binary...
powershell -Command "& { ^
    Expand-Archive -Path 'bin\!BINARY_NAME!' -DestinationPath 'bin' -Force; ^
}"

if not exist "bin\v-sentinel.exe" (
    echo ERROR: v-sentinel.exe not found after extraction!
    pause
    exit /b 1
)

REM Copy necessary files
echo       Files prepared.

REM Build installer with NSIS
echo [4/4] Building NSIS installer...
cd installer\windows
makensis /DPRODUCT_VERSION=!VERSION! v-sentinel-installer.nsi
if %ERRORLEVEL% neq 0 (
    echo ERROR: NSIS compilation failed!
    cd ..\..
    pause
    exit /b 1
)
cd ..\..

if exist "installer\windows\!INSTALLER_NAME!" (
    echo.
    echo ========================================
    echo  BUILD SUCCESSFUL!
    echo ========================================
    echo.
    echo Installer created: installer\windows\!INSTALLER_NAME!
    echo.
    echo File size:
    for %%I in ("installer\windows\!INSTALLER_NAME!") do echo       %%~zI bytes
    echo.
    echo Next steps:
    echo  1. Test the installer on a Windows machine
    echo  2. Sign the executable with your code signing certificate
    echo  3. Upload to GitHub Releases
    echo.
) else (
    echo ERROR: Installer was not created!
    echo Check the NSIS output for errors.
    pause
    exit /b 1
)

echo Build process completed.
pause