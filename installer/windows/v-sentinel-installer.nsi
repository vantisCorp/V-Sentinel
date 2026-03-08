; V-Sentinel Windows Installer Script
; Requires NSIS (Nullsoft Scriptable Install System) 3.08+
; Build with: makensis v-sentinel-installer.nsi

!include "MUI2.nsh"
!include "FileFunc.nsh"
!include "LogicLib.nsh"
!include "x64.nsh"

; Application information
!define PRODUCT_NAME "V-Sentinel"
!define PRODUCT_VERSION "2.1.1"
!define PRODUCT_PUBLISHER "V-Sentinel Security Team"
!define PRODUCT_WEB_SITE "https://github.com/V-Sentinel/V-Sentinel"
!define PRODUCT_DIR_REGKEY "Software\Microsoft\Windows\CurrentVersion\App Paths\v-sentinel.exe"
!define PRODUCT_UNINST_KEY "Software\Microsoft\Windows\CurrentVersion\Uninstall\${PRODUCT_NAME}"
!define PRODUCT_UNINST_ROOT_KEY "HKLM"

; Installer settings
!define INSTALL_MODE "allUsers"
!define INSTALL_SIZE 157286400 ; ~150MB

; Multi-language support
!insertmacro MUI_LANGUAGE "English"
!insertmacro MUI_LANGUAGE "Polish"
!insertmacro MUI_LANGUAGE "German"
!insertmacro MUI_LANGUAGE "French"
!insertmacro MUI_LANGUAGE "Spanish"
!insertmacro MUI_LANGUAGE "Russian"
!insertmacro MUI_LANGUAGE "SimpChinese"
!insertmacro MUI_LANGUAGE "Japanese"
!insertmacro MUI_LANGUAGE "Korean"

; Interface settings
!define MUI_ABORTWARNING
!define MUI_ICON "assets\icon.ico"
!define MUI_UNICON "assets\icon.ico"
!define MUI_WELCOMEFINISHPAGE_BITMAP "assets\wizard.bmp"
!define MUI_UNWELCOMEFINISHPAGE_BITMAP "assets\wizard.bmp"

; Welcome page
!insertmacro MUI_PAGE_WELCOME

; License page
!insertmacro MUI_PAGE_LICENSE "LICENSE"

; Components page
!insertmacro MUI_PAGE_COMPONENTS

; Directory page
!insertmacro MUI_PAGE_DIRECTORY

; Install progress page
!insertmacro MUI_PAGE_INSTFILES

; Finish page
!define MUI_FINISHPAGE_RUN "$INSTDIR\v-sentinel.exe"
!define MUI_FINISHPAGE_RUN_TEXT "$(LaunchProgram)"
!insertmacro MUI_PAGE_FINISH

; Uninstaller pages
!insertmacro MUI_UNPAGE_INSTFILES

; Installer attributes
Name "${PRODUCT_NAME} ${PRODUCT_VERSION}"
OutFile "V-Sentinel-${PRODUCT_VERSION}-Setup.exe"
InstallDir "$PROGRAMFILES64\${PRODUCT_NAME}"
InstallDirRegKey HKLM "${PRODUCT_DIR_REGKEY}" ""
ShowInstDetails show
ShowUnInstDetails show
RequestExecutionLevel admin

; Language strings
LangString LaunchProgram ${LANG_ENGLISH} "Launch V-Sentinel"
LangString LaunchProgram ${LANG_POLISH} "Uruchom V-Sentinel"
LangString LaunchProgram ${LANG_GERMAN} "V-Sentinel starten"
LangString LaunchProgram ${LANG_FRENCH} "Lancer V-Sentinel"
LangString LaunchProgram ${LANG_SPANISH} "Iniciar V-Sentinel"
LangString LaunchProgram ${LANG_RUSSIAN} "Запустить V-Sentinel"
LangString LaunchProgram ${LANG_SIMPCHINESE} "启动 V-Sentinel"
LangString LaunchProgram ${LANG_JAPANESE} "V-Sentinel を起動"
LangString LaunchProgram ${LANG_KOREAN} "V-Sentinel 실행"

LangString DESC_SecCore ${LANG_ENGLISH} "Core V-Sentinel engine and command-line interface (required)"
LangString DESC_SecCore ${LANG_POLISH} "Silnik V-Sentinel i interfejs wiersza poleceń (wymagane)"
LangString DESC_SecCore ${LANG_GERMAN} "V-Sentinel Kernmodul und Kommandozeile (erforderlich)"
LangString DESC_SecCore ${LANG_FRENCH} "Moteur V-Sentinel et interface en ligne de commande (requis)"
LangString DESC_SecCore ${LANG_SPANISH} "Motor principal de V-Sentinel e interfaz de línea de comandos (requerido)"
LangString DESC_SecCore ${LANG_RUSSIAN} "Основной движок V-Sentinel и интерфейс командной строки (обязательно)"
LangString DESC_SecCore ${LANG_SIMPCHINESE} "V-Sentinel 核心引擎和命令行界面（必需）"
LangString DESC_SecCore ${LANG_JAPANESE} "V-Sentinel コアエンジンとコマンドラインインターフェース（必須）"
LangString DESC_SecCore ${LANG_KOREAN} "V-Sentinel 핵심 엔진 및 명령줄 인터페이스 (필수)"

LangString DESC_SecModules ${LANG_ENGLISH} "Security modules: Zero Trust, Shadow AI Detection, Deepfake Detection"
LangString DESC_SecModules ${LANG_POLISH} "Moduły bezpieczeństwa: Zero Trust, Wykrywanie Shadow AI, Wykrywanie Deepfake"
LangString DESC_SecModules ${LANG_GERMAN} "Sicherheitsmodule: Zero Trust, Shadow AI Erkennung, Deepfake Erkennung"
LangString DESC_SecModules ${LANG_FRENCH} "Modules de sécurité: Zero Trust, Détection Shadow AI, Détection Deepfake"
LangString DESC_SecModules ${LANG_SPANISH} "Módulos de seguridad: Zero Trust, Detección Shadow AI, Detección Deepfake"
LangString DESC_SecModules ${LANG_RUSSIAN} "Модули безопасности: Zero Trust, Обнаружение Shadow AI, Обнаружение Deepfake"
LangString DESC_SecModules ${LANG_SIMPCHINESE} "安全模块：零信任、影子AI检测、深度伪造检测"
LangString DESC_SecModules ${LANG_JAPANESE} "セキュリティモジュール: Zero Trust、Shadow AI検出、Deepfake検出"
LangString DESC_SecModules ${LANG_KOREAN} "보안 모듈: Zero Trust, Shadow AI 탐지, Deepfake 탐지"

LangString DESC_SecPQC ${LANG_ENGLISH} "Post-Quantum Cryptography module for future-proof encryption"
LangString DESC_SecPQC ${LANG_POLISH} "Moduł kryptografii post-kwantowej dla szyfrowania odpornego na przyszłość"
LangString DESC_SecPQC ${LANG_GERMAN} "Post-Quanten-Kryptographie Modul für zukunftssichere Verschlüsselung"
LangString DESC_SecPQC ${LANG_FRENCH} "Module de cryptographie post-quantique pour un chiffrement durable"
LangString DESC_SecPQC ${LANG_SPANISH} "Módulo de criptografía post-cuántica para cifrado a prueba de futuro"
LangString DESC_SecPQC ${LANG_RUSSIAN} "Модуль постквантовой криптографии для устойчивого шифрования"
LangString DESC_SecPQC ${LANG_SIMPCHINESE} "后量子密码模块，实现面向未来的加密"
LangString DESC_SecPQC ${LANG_JAPANESE} "ポスト量子暗号モジュール（将来対応の暗号化）"
LangString DESC_SecPQC ${LANG_KOREAN} "양자 내성 암호화 모듈"

LangString DESC_SecSDK ${LANG_ENGLISH} "SDK files for Python, JavaScript, Go, and Rust integrations"
LangString DESC_SecSDK ${LANG_POLISH} "Pliki SDK dla integracji Python, JavaScript, Go i Rust"
LangString DESC_SecSDK ${LANG_GERMAN} "SDK-Dateien für Python, JavaScript, Go und Rust Integrationen"
LangString DESC_SecSDK ${LANG_FRENCH} "Fichiers SDK pour intégrations Python, JavaScript, Go et Rust"
LangString DESC_SecSDK ${LANG_SPANISH} "Archivos SDK para integraciones Python, JavaScript, Go y Rust"
LangString DESC_SecSDK ${LANG_RUSSIAN} "SDK файлы для интеграции Python, JavaScript, Go и Rust"
LangString DESC_SecSDK ${LANG_SIMPCHINESE} "Python、JavaScript、Go 和 Rust 集成的 SDK 文件"
LangString DESC_SecSDK ${LANG_JAPANESE} "Python、JavaScript、Go、Rust統合用SDKファイル"
LangString DESC_SecSDK ${LANG_KOREAN} "Python, JavaScript, Go, Rust 통합용 SDK 파일"

LangString DESC_SecService ${LANG_ENGLISH} "Install V-Sentinel as a Windows Service (runs in background)"
LangString DESC_SecService ${LANG_POLISH} "Zainstaluj V-Sentinel jako usługę Windows (działa w tle)"
LangString DESC_SecService ${LANG_GERMAN} "V-Sentinel als Windows-Dienst installieren (läuft im Hintergrund)"
LangString DESC_SecService ${LANG_FRENCH} "Installer V-Sentinel comme service Windows (fonctionne en arrière-plan)"
LangString DESC_SecService ${LANG_SPANISH} "Instalar V-Sentinel como servicio de Windows (se ejecuta en segundo plano)"
LangString DESC_SecService ${LANG_RUSSIAN} "Установить V-Sentinel как службу Windows (работает в фоновом режиме)"
LangString DESC_SecService ${LANG_SIMPCHINESE} "将 V-Sentinel 安装为 Windows 服务（后台运行）"
LangString DESC_SecService ${LANG_JAPANESE} "V-SentinelをWindowsサービスとしてインストール（バックグラウンドで実行）"
LangString DESC_SecService ${LANG_KOREAN} "V-Sentinel을 Windows 서비스로 설치 (백그라운드 실행)"

; Component sections
Section "!$(DESC_SecCore)" SecCore
  SectionIn RO
  
  SetOutPath "$INSTDIR"
  SetOverwrite ifnewer
  
  ; Main executable
  File "target\release\v-sentinel.exe"
  
  ; Configuration files
  File "config\default.toml"
  
  ; Documentation
  File "README.md"
  File "LICENSE"
  File "CHANGELOG.md"
  
  ; Create directories
  CreateDirectory "$INSTDIR\logs"
  CreateDirectory "$INSTDIR\data"
  CreateDirectory "$INSTDIR\config"
  
  ; Write installation info
  WriteRegStr HKLM "${PRODUCT_DIR_REGKEY}" "" "$INSTDIR\v-sentinel.exe"
  WriteRegStr HKLM "${PRODUCT_DIR_REGKEY}" "Path" "$INSTDIR"
  
SectionEnd

Section "$(DESC_SecModules)" SecModules
  SetOutPath "$INSTDIR\modules"
  SetOverwrite ifnewer
  
  ; Security modules
  File /r "modules\*.*"
  
  ; Module configuration
  SetOutPath "$INSTDIR\config"
  File "config\modules\*.*"
SectionEnd

Section "$(DESC_SecPQC)" SecPQC
  SetOutPath "$INSTDIR\pqc"
  SetOverwrite ifnewer
  
  ; Post-Quantum Cryptography libraries
  File /r "pqc\*.*"
SectionEnd

Section "$(DESC_SecSDK)" SecSDK
  SetOutPath "$INSTDIR\sdk"
  SetOverwrite ifnewer
  
  ; SDK files
  File /r "sdk\*.*"
SectionEnd

Section "$(DESC_SecService)" SecService
  ; Create Windows Service
  nsExec::ExecToLog 'sc create V-Sentinel binPath= "$INSTDIR\v-sentinel.exe --service" start= auto DisplayName= "V-Sentinel Security Service"'
  nsExec::ExecToLog 'sc description V-Sentinel "V-Sentinel AI-Powered Security Monitoring Service"'
SectionEnd

; Additional installation tasks
Section -AdditionalIcons
  CreateDirectory "$SMPROGRAMS\${PRODUCT_NAME}"
  CreateShortCut "$SMPROGRAMS\${PRODUCT_NAME}\V-Sentinel.lnk" "$INSTDIR\v-sentinel.exe"
  CreateShortCut "$SMPROGRAMS\${PRODUCT_NAME}\Documentation.lnk" "$INSTDIR\README.md"
  CreateShortCut "$SMPROGRAMS\${PRODUCT_NAME}\Uninstall.lnk" "$INSTDIR\uninst.exe"
  CreateShortCut "$DESKTOP\V-Sentinel.lnk" "$INSTDIR\v-sentinel.exe"
SectionEnd

Section -Post
  WriteUninstaller "$INSTDIR\uninst.exe"
  WriteRegStr ${PRODUCT_UNINST_ROOT_KEY} "${PRODUCT_UNINST_KEY}" "DisplayName" "$(^Name)"
  WriteRegStr ${PRODUCT_UNINST_ROOT_KEY} "${PRODUCT_UNINST_KEY}" "UninstallString" "$INSTDIR\uninst.exe"
  WriteRegStr ${PRODUCT_UNINST_ROOT_KEY} "${PRODUCT_UNINST_KEY}" "DisplayIcon" "$INSTDIR\v-sentinel.exe"
  WriteRegStr ${PRODUCT_UNINST_ROOT_KEY} "${PRODUCT_UNINST_KEY}" "DisplayVersion" "${PRODUCT_VERSION}"
  WriteRegStr ${PRODUCT_UNINST_ROOT_KEY} "${PRODUCT_UNINST_KEY}" "URLInfoAbout" "${PRODUCT_WEB_SITE}"
  WriteRegStr ${PRODUCT_UNINST_ROOT_KEY} "${PRODUCT_UNINST_KEY}" "Publisher" "${PRODUCT_PUBLISHER}"
  WriteRegDWORD ${PRODUCT_UNINST_ROOT_KEY} "${PRODUCT_UNINST_KEY}" "EstimatedSize" "${INSTALL_SIZE}"
SectionEnd

; Component descriptions
!insertmacro MUI_FUNCTION_DESCRIPTION_BEGIN
  !insertmacro MUI_DESCRIPTION_TEXT ${SecCore} $(DESC_SecCore)
  !insertmacro MUI_DESCRIPTION_TEXT ${SecModules} $(DESC_SecModules)
  !insertmacro MUI_DESCRIPTION_TEXT ${SecPQC} $(DESC_SecPQC)
  !insertmacro MUI_DESCRIPTION_TEXT ${SecSDK} $(DESC_SecSDK)
  !insertmacro MUI_DESCRIPTION_TEXT ${SecService} $(DESC_SecService)
!insertmacro MUI_FUNCTION_DESCRIPTION_END

; Uninstaller section
Section Uninstall
  ; Stop and remove service if installed
  nsExec::ExecToLog 'net stop V-Sentinel'
  nsExec::ExecToLog 'sc delete V-Sentinel'
  
  ; Remove files
  Delete "$INSTDIR\v-sentinel.exe"
  Delete "$INSTDIR\uninst.exe"
  Delete "$INSTDIR\README.md"
  Delete "$INSTDIR\LICENSE"
  Delete "$INSTDIR\CHANGELOG.md"
  Delete "$INSTDIR\config\default.toml"
  
  ; Remove directories
  RMDir /r "$INSTDIR\modules"
  RMDir /r "$INSTDIR\pqc"
  RMDir /r "$INSTDIR\sdk"
  RMDir /r "$INSTDIR\logs"
  RMDir /r "$INSTDIR\data"
  RMDir /r "$INSTDIR\config"
  RMDir "$INSTDIR"
  
  ; Remove shortcuts
  Delete "$SMPROGRAMS\${PRODUCT_NAME}\*.*"
  RMDir "$SMPROGRAMS\${PRODUCT_NAME}"
  Delete "$DESKTOP\V-Sentinel.lnk"
  
  ; Remove registry keys
  DeleteRegKey ${PRODUCT_UNINST_ROOT_KEY} "${PRODUCT_UNINST_KEY}"
  DeleteRegKey HKLM "${PRODUCT_DIR_REGKEY}"
  
  SetAutoClose true
SectionEnd

; Function to check prerequisites
Function .onInit
  !insertmacro MUI_LANGDLL_DISPLAY
  
  ; Check for 64-bit Windows
  ${If} ${RunningX64}
    StrCpy $INSTDIR "$PROGRAMFILES64\${PRODUCT_NAME}"
  ${Else}
    MessageBox MB_OK|MB_ICONSTOP "V-Sentinel requires a 64-bit version of Windows."
    Abort
  ${EndIf}
FunctionEnd

; Function to uninstall previous version
Function UninstallPrevious
  ; Check for previous installation
  ReadRegStr $0 HKLM "${PRODUCT_DIR_REGKEY}" ""
  ${If} $0 != ""
    ; Run uninstaller
    ExecWait '"$0\uninst.exe" /S _?=$0'
  ${EndIf}
FunctionEnd