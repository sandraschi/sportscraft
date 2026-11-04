; SportsCraft current-user installer.
; Build from the repo root:  makensis /NOCD installer\installer.nsi
; (/NOCD keeps File paths relative to the repo root, not to installer\.)
Unicode true
!include "MUI2.nsh"

!define APP_NAME "SportsCraft"
!define APP_VERSION "0.1.1"
!define APP_PUBLISHER "Antigravity Craft Collective"
!define APP_KEY "Software\Microsoft\Windows\CurrentVersion\Uninstall\SportsCraft"

Name "${APP_NAME} ${APP_VERSION}"
; makensis will not create the output folder itself.
!system 'if not exist "dist" mkdir "dist"'
OutFile "dist\SportsCraft-Setup-${APP_VERSION}.exe"
InstallDir "$LOCALAPPDATA\SportsCraft"
InstallDirRegKey HKCU "Software\SportsCraft" "InstallDir"
RequestExecutionLevel user
SetCompressor /SOLID lzma
ShowInstDetails show
ShowUninstDetails show

!define MUI_ABORTWARNING
!insertmacro MUI_PAGE_WELCOME
!insertmacro MUI_PAGE_DIRECTORY
!insertmacro MUI_PAGE_INSTFILES
!insertmacro MUI_UNPAGE_CONFIRM
!insertmacro MUI_UNPAGE_INSTFILES
!insertmacro MUI_LANGUAGE "English"

Section "SportsCraft" SecMain
  SectionIn RO
  SetOutPath "$INSTDIR"
  File "target\release\sportscraft.exe"
  File "target\release\sportscraft-cli.exe"
  File "target\release\sportscraft-mcp.exe"
  File "README.md"
  File "CHANGELOG.md"
  File "AGENTS.md"
  File "LICENSE-MIT"
  File "LICENSE-APACHE"

  CreateDirectory "$SMPROGRAMS\SportsCraft"
  CreateShortcut "$SMPROGRAMS\SportsCraft\SportsCraft.lnk" "$INSTDIR\sportscraft.exe"
  CreateShortcut "$SMPROGRAMS\SportsCraft\SportsCraft CLI.lnk" "$INSTDIR\sportscraft-cli.exe" "--help"
  CreateShortcut "$DESKTOP\SportsCraft.lnk" "$INSTDIR\sportscraft.exe"

  WriteUninstaller "$INSTDIR\Uninstall.exe"

  WriteRegStr HKCU "Software\SportsCraft" "InstallDir" "$INSTDIR"
  WriteRegStr HKCU "${APP_KEY}" "DisplayName" "${APP_NAME}"
  WriteRegStr HKCU "${APP_KEY}" "DisplayVersion" "${APP_VERSION}"
  WriteRegStr HKCU "${APP_KEY}" "Publisher" "${APP_PUBLISHER}"
  WriteRegStr HKCU "${APP_KEY}" "DisplayIcon" "$INSTDIR\sportscraft.exe"
  WriteRegStr HKCU "${APP_KEY}" "UninstallString" '"$INSTDIR\Uninstall.exe"'
  WriteRegDWORD HKCU "${APP_KEY}" "NoModify" 1
  WriteRegDWORD HKCU "${APP_KEY}" "NoRepair" 1
SectionEnd

Section "Uninstall"
  Delete "$INSTDIR\sportscraft.exe"
  Delete "$INSTDIR\sportscraft-cli.exe"
  Delete "$INSTDIR\sportscraft-mcp.exe"
  Delete "$INSTDIR\README.md"
  Delete "$INSTDIR\CHANGELOG.md"
  Delete "$INSTDIR\AGENTS.md"
  Delete "$INSTDIR\LICENSE-MIT"
  Delete "$INSTDIR\LICENSE-APACHE"
  Delete "$INSTDIR\Uninstall.exe"

  Delete "$DESKTOP\SportsCraft.lnk"
  RMDir /r "$SMPROGRAMS\SportsCraft"
  RMDir "$INSTDIR"

  DeleteRegKey HKCU "${APP_KEY}"
  DeleteRegKey HKCU "Software\SportsCraft"
SectionEnd
