; Script generated by the Inno Setup Script Wizard.
; SEE THE DOCUMENTATION FOR DETAILS ON CREATING INNO SETUP SCRIPT FILES!

#define MyAppName "cli-todo"
#define MyAppVersion "0.2.0-alpha"
#define MyAppPublisher "mackeper"
#define MyAppURL "https://github.com/mackeper/cli-todo"
#define MyAppExeName "todo.exe"

[Setup]
; NOTE: The value of AppId uniquely identifies this application. Do not use the same AppId value in installers for other applications.
; (To generate a new GUID, click Tools | Generate GUID inside the IDE.)
AppId={{881E4C8C-C5AB-4F74-9A5E-8F67FB05ACE3}
AppName={#MyAppName}
AppVersion={#MyAppVersion}
;AppVerName={#MyAppName} {#MyAppVersion}
AppPublisher={#MyAppPublisher}
AppPublisherURL={#MyAppURL}
AppSupportURL={#MyAppURL}
AppUpdatesURL={#MyAppURL}
DefaultDirName={autopf}\{#MyAppName}
DefaultGroupName={#MyAppName}
DisableProgramGroupPage=yes
LicenseFile=..\LICENSE
; Uncomment the following line to run in non administrative install mode (install for current user only.)
;PrivilegesRequired=lowest
PrivilegesRequiredOverridesAllowed=dialog
OutputDir=..\target\release\setup
OutputBaseFilename=cli-todo-setup
Compression=lzma
SolidCompression=yes
WizardStyle=modern

[Languages]
Name: "english"; MessagesFile: "compiler:Default.isl"

[Files]
Source: "..\target\release\{#MyAppExeName}"; DestDir: "{app}"; Flags: ignoreversion
; NOTE: Don't use "Flags: ignoreversion" on any shared system files

[Icons]
Name: "{group}\{#MyAppName}"; Filename: "{app}\{#MyAppExeName}"

[Registry]
; For system-wide installation
Root: HKLM; \
    Subkey: "SYSTEM\CurrentControlSet\Control\Session Manager\Environment"; \
    ValueType: expandsz; \
    ValueName: "Path"; \
    ValueData: "{olddata};{app}"; \
    Check: IsPathAbsentLocalMachine() and IsAdminInstallMode();

; For user-specific installation
Root: HKCU; \
    Subkey: "Environment"; \
    ValueType: expandsz; \
    ValueName: "Path"; \
    ValueData: "{olddata};{app}"; \
    Check: IsPathAbsentUser() and not IsAdminInstallMode();

[Code]
// Write code in pascal

// Check if the path is already present in the system-wide PATH environment variable
function IsPathAbsentLocalMachine(): boolean;
var
  NewPathSegment: string;
  ExistingPath: string;
begin
  if not RegQueryStringValue(
    HKEY_LOCAL_MACHINE,
    'SYSTEM\CurrentControlSet\Control\Session Manager\Environment',
    'Path',
    ExistingPath)
  then
  begin
    Result := True;
    exit;
  end;

  ExistingPath := ';' + LowerCase(ExistingPath) + ';';
  NewPathSegment := LowerCase(ExpandConstant('{app}'));
  Result := Pos(';' + NewPathSegment + ';', ExistingPath) = 0;
end;

// Check if the path is already present in the user-specific PATH environment variable
function IsPathAbsentUser(): boolean;
var
  NewPathSegment: string;
  ExistingPath: string;
begin
  if not RegQueryStringValue(
    HKEY_CURRENT_USER,
    'Environment',
    'Path',
    ExistingPath)
  then
  begin
    Result := True;
    exit;
  end;

  ExistingPath := ';' + LowerCase(ExistingPath) + ';';
  NewPathSegment := LowerCase(ExpandConstant('{app}'));
  Result := Pos(';' + NewPathSegment + ';', ExistingPath) = 0;
end;

[Run]
;Filename: "{app}\{#MyAppExeName}"; Description: "{cm:LaunchProgram,{#StringChange(MyAppName, '&', '&&')}}"; Flags: nowait postinstall skipifsilent

