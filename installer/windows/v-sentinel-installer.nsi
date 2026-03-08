; V-Sentinel v2.1.1 - Windows Installer Script (NSIS)
; Next-generation AI-native security system with quantum-ready cryptography
;
; To build: makensis v-sentinel-installer.nsi
; Requires: NSIS 3.08+ (https://nsis.sourceforge.io/)

!define PRODUCT_NAME "V-Sentinel"
!define PRODUCT_VERSION "2.1.1"
!define PRODUCT_PUBLISHER "VantisCorp"
!define PRODUCT_WEB_SITE "https://github.com/vantisCorp/V-Sentinel"
!define PRODUCT_DIR_REGKEY "Software\Microsoft\Windows\CurrentVersion\App Paths\v-sentinel.exe"
!define PRODUCT_UNINST_KEY "Software\Microsoft\Windows\CurrentVersion\Uninstall\${PRODUCT_NAME}"
!define PRODUCT_UNINST_ROOT_KEY "HKLM"

!include "MUI2.nsh"
!include "FileFunc.nsh"
!include "LogicLib.nsh"

; MUI Settings
!define MUI_ABORTWARNING
!define MUI_ICON "assets\icon.ico"
!define MUI_UNICON "assets\icon.ico"
!define MUI_WELCOMEFINISHPAGE_BITMAP "assets\welcome.bmp"
!define MUI_HEADERIMAGE
!define MUI_HEADERIMAGE_BITMAP "assets\header.bmp"

; Welcome page
!insertmacro MUI_PAGE_WELCOME

; License page
!insertmacro MUI_PAGE_LICENSE "LICENSE"

; Directory page
!insertmacro MUI_PAGE_DIRECTORY

; Components page
!insertmacro MUI_PAGE_COMPONENTS

; Instfiles page
!insertmacro MUI_PAGE_INSTFILES

; Finish page
!define MUI_FINISHPAGE_RUN "$INSTDIR\v-sentinel.exe"
!define MUI_FINISHPAGE_RUN_PARAMETERS "--setup"
!insertmacro MUI_PAGE_FINISH

; Uninstaller pages
!insertmacro MUI_UNPAGE_INSTFILES

; Language files
!insertmacro MUI_LANGUAGE "English"
!insertmacro MUI_LANGUAGE "Polish"
!insertmacro MUI_LANGUAGE "German"
!insertmacro MUI_LANGUAGE "French"
!insertmacro MUI_LANGUAGE "Spanish"
!insertmacro MUI_LANGUAGE "Russian"
!insertmacro MUI_LANGUAGE "ChineseSimplified"
!insertmacro MUI_LANGUAGE "Japanese"
!insertmacro MUI_LANGUAGE "Korean"

; MUI end ------

Name "${PRODUCT_NAME} ${PRODUCT_VERSION}"
OutFile "V-Sentinel-${PRODUCT_VERSION}-Setup.exe"
InstallDir "$PROGRAMFILES64\${PRODUCT_NAME}"
InstallDirRegKey HKLM "${PRODUCT_DIR_REGKEY}" ""
ShowInstDetails show
ShowUnInstDetails show

; Installation types
InstType "Full Installation"
InstType "Standard Installation"
InstType "Minimal Installation"

; Components
Section "!Core Engine (Required)" SEC01
    SectionIn 1 2 3 RO
    SetOutPath "$INSTDIR"
    SetOverwrite ifnewer
    
    ; Main executable
    File "target\release\v-sentinel.exe"
    
    ; Configuration
    SetOutPath "$INSTDIR\config"
    File "config\default.toml"
    
    ; Documentation
    SetOutPath "$INSTDIR\docs"
    File /r "docs\*.*"
    
    ; Create directories
    CreateDirectory "$INSTDIR\logs"
    CreateDirectory "$INSTDIR\data"
SectionEnd

Section "Security Modules" SEC02
    SectionIn 1 2
    SetOutPath "$INSTDIR\modules"
    
    ; Module files would go here
    ; File "modules\*.*"
    
    ; Enable modules in config
    WriteINIStr "$INSTDIR\config\config.toml" "security" "zero_trust_enabled" "true"
    WriteINIStr "$INSTDIR\config\config.toml" "security" "shadow_ai_enabled" "true"
    WriteINIStr "$INSTDIR\config\config.toml" "security" "deepfake_enabled" "true"
    WriteINIStr "$INSTDIR\config\config.toml" "security" "ai_security_enabled" "true"
SectionEnd

Section "Post-Quantum Cryptography" SEC03
    SectionIn 1
    SetOutPath "$INSTDIR\crypto"
    
    ; PQC libraries
    ; File "crypto\*.*"
    
    WriteINIStr "$INSTDIR\config\config.toml" "cryptography" "kem_algorithm" "kyber768"
    WriteINIStr "$INSTDIR\config\config.toml" "cryptography" "signature_algorithm" "dilithium3"
    WriteINIStr "$INSTDIR\config\config.toml" "cryptography" "hybrid_encryption" "true"
SectionEnd

Section "SDKs and Development Tools" SEC04
    SectionIn 1
    SetOutPath "$INSTDIR\sdk"
    
    ; Python SDK
    SetOutPath "$INSTDIR\sdk\python"
    File /r "sdks\python\*.*"
    
    ; TypeScript SDK
    SetOutPath "$INSTDIR\sdk\typescript"
    File /r "sdks\typescript\*.*"
    
    ; Go SDK
    SetOutPath "$INSTDIR\sdk\go"
    File /r "sdks\go\*.*"
    
    ; CLI Tool
    SetOutPath "$INSTDIR"
    File "target\release\v-sentinel-cli.exe"
SectionEnd

Section "Windows Service" SEC05
    SectionIn 1 2
    ; Install Windows Service
    nsExec::ExecToLog '"$INSTDIR\v-sentinel.exe" --install-service'
    
    ; Configure service
    nsExec::ExecToLog 'sc config V-Sentinel start= auto'
SectionEnd

Section -AdditionalIcons
    SetOutPath $INSTDIR
    CreateDirectory "$SMPROGRAMS\${PRODUCT_NAME}"
    CreateShortCut "$SMPROGRAMS\${PRODUCT_NAME}\V-Sentinel.lnk" "$INSTDIR\v-sentinel.exe"
    CreateShortCut "$SMPROGRAMS\${PRODUCT_NAME}\Configuration.lnk" "notepad.exe" "$INSTDIR\config\config.toml"
    CreateShortCut "$SMPROGRAMS\${PRODUCT_NAME}\Documentation.lnk" "$INSTDIR\docs\README.html"
    CreateShortCut "$SMPROGRAMS\${PRODUCT_NAME}\Uninstall.lnk" "$INSTDIR\uninst.exe"
    CreateShortCut "$DESKTOP\V-Sentinel.lnk" "$INSTDIR\v-sentinel.exe"
SectionEnd

Section -Post
    WriteUninstaller "$INSTDIR\uninst.exe"
    WriteRegStr HKLM "${PRODUCT_DIR_REGKEY}" "" "$INSTDIR\v-sentinel.exe"
    WriteRegStr ${PRODUCT_UNINST_ROOT_KEY} "${PRODUCT_UNINST_KEY}" "DisplayName" "$(^Name)"
    WriteRegStr ${PRODUCT_UNINST_ROOT_KEY} "${PRODUCT_UNINST_KEY}" "UninstallString" "$INSTDIR\uninst.exe"
    WriteRegStr ${PRODUCT_UNINST_ROOT_KEY} "${PRODUCT_UNINST_KEY}" "DisplayIcon" "$INSTDIR\v-sentinel.exe"
    WriteRegStr ${PRODUCT_UNINST_ROOT_KEY} "${PRODUCT_UNINST_KEY}" "DisplayVersion" "${PRODUCT_VERSION}"
    WriteRegStr ${PRODUCT_UNINST_ROOT_KEY} "${PRODUCT_UNINST_KEY}" "URLInfoAbout" "${PRODUCT_WEB_SITE}"
    WriteRegStr ${PRODUCT_UNINST_ROOT_KEY} "${PRODUCT_UNINST_KEY}" "Publisher" "${PRODUCT_PUBLISHER}"
    
    ; Add to PATH
    EnVar::AddValue "PATH" "$INSTDIR"
SectionEnd

; Component descriptions
!insertmacro MUI_FUNCTION_DESCRIPTION_BEGIN
    !insertmacro MUI_DESCRIPTION_TEXT ${SEC01} "Core V-Sentinel engine - required for all functionality"
    !insertmacro MUI_DESCRIPTION_TEXT ${SEC02} "Security modules: Zero Trust, Shadow AI, Deepfake Detection, AI Security"
    !insertmacro MUI_DESCRIPTION_TEXT ${SEC03} "Post-quantum cryptography: Kyber, Dilithium, Falcon"
    !insertmacro MUI_DESCRIPTION_TEXT ${SEC04} "SDKs for Python, TypeScript, Go and CLI tools"
    !insertmacro MUI_DESCRIPTION_TEXT ${SEC05} "Install as Windows Service for automatic startup"
!insertmacro MUI_FUNCTION_DESCRIPTION_END

; Uninstaller
Section Uninstall
    ; Stop service if running
    nsExec::ExecToLog 'net stop V-Sentinel'
    nsExec::ExecToLog '"$INSTDIR\v-sentinel.exe" --uninstall-service'
    
    ; Remove from PATH
    EnVar::DeleteValue "PATH" "$INSTDIR"
    
    ; Delete files
    Delete "$INSTDIR\v-sentinel.exe"
    Delete "$INSTDIR\v-sentinel-cli.exe"
    Delete "$INSTDIR\uninst.exe"
    Delete "$INSTDIR\config\*.*"
    Delete "$INSTDIR\docs\*.*"
    Delete "$INSTDIR\logs\*.*"
    Delete "$INSTDIR\modules\*.*"
    Delete "$INSTDIR\sdk\*.*"
    
    ; Delete directories
    RMDir "$INSTDIR\config"
    RMDir "$INSTDIR\docs"
    RMDir "$INSTDIR\logs"
    RMDir "$INSTDIR\data"
    RMDir "$INSTDIR\modules"
    RMDir "$INSTDIR\sdk\python"
    RMDir "$INSTDIR\sdk\typescript"
    RMDir "$INSTDIR\sdk\go"
    RMDir "$INSTDIR\sdk"
    RMDir "$INSTDIR"
    
    ; Delete shortcuts
    Delete "$SMPROGRAMS\${PRODUCT_NAME}\*.*"
    Delete "$DESKTOP\V-Sentinel.lnk"
    RMDir "$SMPROGRAMS\${PRODUCT_NAME}"
    
    ; Delete registry keys
    DeleteRegKey ${PRODUCT_UNINST_ROOT_KEY} "${PRODUCT_UNINST_KEY}"
    DeleteRegKey HKLM "${PRODUCT_DIR_REGKEY}"
    
    SetAutoClose true
SectionEnd

; Functions
Function .onInit
    !insertmacro MUI_LANGDLL_DISPLAY
    
    ; Check for admin rights
    UserInfo::GetAccountType
    Pop $0
    ${If} $0 != "Admin"
        MessageBox MB_OK|MB_ICONSTOP "This installer requires Administrator privileges."
        Abort
    ${EndIf}
    
    ; Check if already installed
    ReadRegStr $0 HKLM "${PRODUCT_DIR_REGKEY}" ""
    ${If} $0 != ""
        MessageBox MB_YESNO|MB_ICONQUESTION "V-Sentinel is already installed. Would you like to upgrade?" IDYES upgrade
        Abort
    upgrade:
        ; Run uninstaller
        ExecWait '"$0\uninst.exe" /S _?=$0'
    ${EndIf}
FunctionEnd

Function un.onInit
    !insertmacro MUI_UNGETLANGUAGE
    MessageBox MB_ICONQUESTION|MB_YESNO|MB_DEFBUTTON2 "Are you sure you want to completely remove $(^Name) and all of its components?" IDYES +2
    Abort
FunctionEnd