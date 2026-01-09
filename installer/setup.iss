; Inno Setup Script for procsnipe
; https://jrsoftware.org/isinfo.php

#define MyAppName "procsnipe"
#define MyAppVersion "1.0"
#define MyAppPublisher "berochitiri"
#define MyAppURL "https://github.com/berochitiri/procsnipe"
#define MyAppExeName "procsnipe.exe"

[Setup]
; NOTE: The value of AppId uniquely identifies this application.
AppId={{8F9A4B2C-1D3E-4F5A-9B7C-2E6D8A3F5B1C}
AppName={#MyAppName}
AppVersion={#MyAppVersion}
AppPublisher={#MyAppPublisher}
AppPublisherURL={#MyAppURL}
DefaultDirName={autopf}\{#MyAppName}
DefaultGroupName={#MyAppName}
DisableProgramGroupPage=yes
LicenseFile=..\installer\LICENSE.txt
InfoAfterFile=..\installer\disclaimer.txt
OutputDir=..\releases
OutputBaseFilename=procsnipe-setup-v{#MyAppVersion}
Compression=lzma
SolidCompression=yes
WizardStyle=modern
PrivilegesRequired=admin
ArchitecturesInstallIn64BitMode=x64

[Languages]
Name: "english"; MessagesFile: "compiler:Default.isl"

[Tasks]
Name: "desktopicon"; Description: "{cm:CreateDesktopIcon}"; GroupDescription: "{cm:AdditionalIcons}"; Flags: unchecked
Name: "startmenu"; Description: "Create Start Menu shortcut"; GroupDescription: "{cm:AdditionalIcons}"; Flags: checkedonce
Name: "autostart"; Description: "Run at Windows startup (system tray mode)"; GroupDescription: "Startup:"; Flags: unchecked

[Files]
Source: "..\target\release\{#MyAppExeName}"; DestDir: "{app}"; Flags: ignoreversion
Source: "..\assets\*"; DestDir: "{app}\assets"; Flags: ignoreversion recursesubdirs createallsubdirs
; NOTE: Don't use "Flags: ignoreversion" on any shared system files

[Icons]
Name: "{group}\{#MyAppName}"; Filename: "{app}\{#MyAppExeName}"
Name: "{group}\{cm:UninstallProgram,{#MyAppName}}"; Filename: "{uninstallexe}"
Name: "{autodesktop}\{#MyAppName}"; Filename: "{app}\{#MyAppExeName}"; Tasks: desktopicon

[Registry]
; Add to startup if option is selected (run in tray mode)
Root: HKCU; Subkey: "Software\Microsoft\Windows\CurrentVersion\Run"; ValueType: string; ValueName: "procsnipe"; ValueData: """{app}\{#MyAppExeName}"" --tray"; Flags: uninsdeletevalue; Tasks: autostart

[Run]
Filename: "{app}\{#MyAppExeName}"; Description: "{cm:LaunchProgram,{#StringChange(MyAppName, '&', '&&')}}"; Flags: nowait postinstall skipifsilent

[Code]
procedure InitializeWizard;
var
  Page: TOutputMsgWizardPage;
begin
  Page := CreateOutputMsgPage(wpWelcome,
    'Important Disclaimer', 'Please read carefully before continuing',
    '⚠️ WARNING:' + #13#10 + #13#10 +
    'procsnipe is a system process manager that can:' + #13#10 +
    '  • Monitor all running processes' + #13#10 +
    '  • Terminate any process including system processes' + #13#10 + #13#10 +
    'RISKS:' + #13#10 +
    '  • Killing system processes can cause system instability' + #13#10 +
    '  • Terminating protected processes may require admin rights' + #13#10 +
    '  • Unexpected crashes may result from improper use' + #13#10 + #13#10 +
    'USE AT YOUR OWN RISK' + #13#10 + #13#10 +
    'By continuing, you accept responsibility for any consequences' + #13#10 +
    'of using this software.');
end;
