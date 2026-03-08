@echo off
REM V-Sentinel Windows Installer Builder
REM Requires: NSIS 3.08+, Rust (cargo), Visual Studio Build Tools
REM 
REM Usage: build-installer.bat [VERSION]
REM Example: build-installer.bat 2.1.1

setlocal EnableDelayedExpansion

REM Configuration
set "PRODUCT_NAME=V-Sentinel"
if "%~1"=="" (
    set "VERSION=2.1.1"
) else (
    set "VERSION=%~1"
)
set "INSTALLER_NAME=%PRODUCT_NAME%-!VERSION!-Setup.exe"

echo.
echo ========================================
echo  %PRODUCT_NAME% Windows Installer Builder
echo ========================================
echo.
echo Version: !VERSION!
echo Output: !INSTALLER_NAME!
echo.

REM Check for NSIS
echo [1/5] Checking for NSIS...
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

REM Check for Rust
echo [2/5] Checking for Rust...
where cargo >nul 2>&1
if %ERRORLEVEL% neq 0 (
    echo ERROR: Rust/Cargo not found in PATH!
    echo.
    echo Please install Rust from: https://rustup.rs/
    echo.
    pause
    exit /b 1
)
echo       Rust found.

REM Build release binary
echo [3/5] Building release binary...
echo       This may take several minutes...
cargo build --release
if %ERRORLEVEL% neq 0 (
    echo ERROR: Build failed!
    pause
    exit /b 1
)
echo       Build successful.

REM Copy necessary files
echo [4/5] Preparing installer files...

REM Create asset directory if not exists
if not exist "assets" mkdir "assets"

REM Check for icon
if not exist "assets\icon.ico" (
    echo WARNING: assets\icon.ico not found. Using default icon.
    echo       For best results, provide a custom icon (256x256 .ico recommended^).
)

REM Check for wizard bitmap
if not exist "assets\wizard.bmp" (
    echo WARNING: assets\wizard.bmp not found. Using default wizard image.
    echo       For best results, provide a custom wizard bitmap (164x314 .bmp recommended^).
)

REM Check for license file
if not exist "LICENSE" (
    echo ERROR: LICENSE file not found!
    echo       Please ensure LICENSE file exists in the project root.
    pause
    exit /b 1
)

REM Check for README
if not exist "README.md" (
    echo WARNING: README.md not found.
)

REM Check for CHANGELOG
if not exist "CHANGELOG.md" (
    echo WARNING: CHANGELOG.md not found.
)

REM Check for config
if not exist "config\default.toml" (
    echo WARNING: config\default.toml not found. Creating default...
    if not exist "config" mkdir "config"
)

echo       Files prepared.

REM Build installer with NSIS
echo [5/5] Building NSIS installer...
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

REM Offer to create codesign command
echo Would you like to sign the installer? (Y/N^)
choice /c YN /n
if errorlevel 2 goto :skip_sign
if errorlevel 1 goto :do_sign

:do_sign
echo.
echo Signing the installer...
echo Note: This requires a valid code signing certificate.
set /p CERT_PATH="Enter path to certificate (.pfx): "
set /p CERT_PASS="Enter certificate password: "
signtool sign /f "%CERT_PATH%" /p "%CERT_PASS%" /tr http://timestamp.digicert.com /td sha256 /fd sha256 "installer\windows\!INSTALLER_NAME!"
if %ERRORLEVEL% equ 0 (
    echo Signature applied successfully.
) else (
    echo WARNING: Signing failed. Continuing with unsigned installer.
)

:skip_sign
echo.
echo Build process completed.
pause