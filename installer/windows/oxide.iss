; Oxide — Windows installer (InnoSetup)
;
; Invoked from CI via:
;   ISCC.exe installer\windows\oxide.iss /DVersion=0.6.3 /DArch=x64
;
; /DVersion  — version string without the leading "v" (tag minus "v")
; /DArch     — "x64" or "arm64" — picks the right binary source path
; /DBinary   — absolute path to the compiled oxide.exe; overrides the
;              target-default when testing locally.

#ifndef Version
  #define Version "0.0.0"
#endif

#ifndef Arch
  #define Arch "x64"
#endif

#if Arch == "x64"
  #define TargetTriple "x86_64-pc-windows-msvc"
  #define ArchSuffix "x86_64"
  #define ArchAllowed "x64compatible"
  #define ArchInstallIn64Bit "x64compatible"
#elif Arch == "arm64"
  #define TargetTriple "aarch64-pc-windows-msvc"
  #define ArchSuffix "aarch64"
  #define ArchAllowed "arm64"
  #define ArchInstallIn64Bit "arm64"
#endif

#ifndef Binary
  #define Binary "..\..\target\" + TargetTriple + "\release\oxide.exe"
#endif

[Setup]
AppId={{C4E84F2F-1D41-4FA9-9D8F-67D37CA0F7FC}
AppName=Oxide
AppVersion={#Version}
AppVerName=Oxide {#Version}
AppPublisher=alpCaner
AppPublisherURL=https://github.com/alplabai/oxide
AppSupportURL=https://github.com/alplabai/oxide/issues
AppUpdatesURL=https://github.com/alplabai/oxide/releases
DefaultDirName={autopf}\Oxide
DefaultGroupName=Oxide
AllowNoIcons=yes
LicenseFile=
OutputDir=.
OutputBaseFilename=oxide-setup-{#ArchSuffix}-{#Version}
Compression=lzma2/ultra64
SolidCompression=yes
WizardStyle=modern
; The .ico is produced by installer/build-icons.sh and lives next to this script.
; Silently skipped if the file is absent (e.g. a first clone before running the script).
#if FileExists("oxide.ico")
  SetupIconFile=oxide.ico
#endif
PrivilegesRequired=lowest
PrivilegesRequiredOverridesAllowed=dialog
ArchitecturesAllowed={#ArchAllowed}
ArchitecturesInstallIn64BitMode={#ArchInstallIn64Bit}
UninstallDisplayIcon={app}\oxide.exe
UninstallDisplayName=Oxide {#Version}
DisableProgramGroupPage=auto

[Languages]
Name: "english"; MessagesFile: "compiler:Default.isl"

[Tasks]
Name: "desktopicon"; Description: "{cm:CreateDesktopIcon}"; GroupDescription: "{cm:AdditionalIcons}"; Flags: unchecked

[Files]
Source: "{#Binary}"; DestDir: "{app}"; Flags: ignoreversion
#if FileExists("oxide.ico")
Source: "oxide.ico"; DestDir: "{app}"; Flags: ignoreversion
#endif
; Per-file-type .ico files for Oxide's native .snx*** extensions.
; Produced by installer/build-file-icons.sh into installer/windows/files/.
; Each `#if FileExists` branch is guarded so a fresh clone still builds
; even before `build-file-icons.sh` has been run.
#if FileExists("files\snxprj.ico")
Source: "files\snxprj.ico"; DestDir: "{app}\files"; Flags: ignoreversion
#endif
#if FileExists("files\snxsch.ico")
Source: "files\snxsch.ico"; DestDir: "{app}\files"; Flags: ignoreversion
#endif
#if FileExists("files\snxpcb.ico")
Source: "files\snxpcb.ico"; DestDir: "{app}\files"; Flags: ignoreversion
#endif
#if FileExists("files\snxfpt.ico")
Source: "files\snxfpt.ico"; DestDir: "{app}\files"; Flags: ignoreversion
#endif
#if FileExists("files\snxsim.ico")
Source: "files\snxsim.ico"; DestDir: "{app}\files"; Flags: ignoreversion
#endif
#if FileExists("files\snxlib.ico")
Source: "files\snxlib.ico"; DestDir: "{app}\files"; Flags: ignoreversion
#endif
#if FileExists("files\snxsym.ico")
Source: "files\snxsym.ico"; DestDir: "{app}\files"; Flags: ignoreversion
#endif
#if FileExists("files\snxpkg.ico")
Source: "files\snxpkg.ico"; DestDir: "{app}\files"; Flags: ignoreversion
#endif
#if FileExists("files\snxmat.ico")
Source: "files\snxmat.ico"; DestDir: "{app}\files"; Flags: ignoreversion
#endif
#if FileExists("files\snxcfg.ico")
Source: "files\snxcfg.ico"; DestDir: "{app}\files"; Flags: ignoreversion
#endif
#if FileExists("files\snxmod.ico")
Source: "files\snxmod.ico"; DestDir: "{app}\files"; Flags: ignoreversion
#endif

[Icons]
#if FileExists("oxide.ico")
  #define IconOpt "; IconFilename: ""{app}\oxide.ico"""
#else
  #define IconOpt ""
#endif
Name: "{group}\Oxide"; Filename: "{app}\oxide.exe"{#IconOpt}
Name: "{group}\{cm:UninstallProgram,Oxide}"; Filename: "{uninstallexe}"
Name: "{autodesktop}\Oxide"; Filename: "{app}\oxide.exe"; Tasks: desktopicon{#IconOpt}

[Registry]
; File associations for Oxide's native .snx*** extensions.
; Each extension: user-scope HKCU root (matches PrivilegesRequired=lowest),
; a ProgId keyed under `Oxide.<ext>`, a DefaultIcon pointing at the
; per-type .ico shipped in {app}\files\, and an `open` verb that passes
; the clicked file path to oxide.exe via "%1".
; Covers: snxprj, snxsch, snxpcb, snxfpt, snxsim, snxlib, snxsym.
Root: HKCU; Subkey: "Software\Classes\.snxprj"; ValueType: string; ValueData: "Oxide.snxprj"; Flags: uninsdeletevalue
Root: HKCU; Subkey: "Software\Classes\Oxide.snxprj"; ValueType: string; ValueData: "Oxide Project"; Flags: uninsdeletekey
Root: HKCU; Subkey: "Software\Classes\Oxide.snxprj\DefaultIcon"; ValueType: string; ValueData: "{app}\files\snxprj.ico"
Root: HKCU; Subkey: "Software\Classes\Oxide.snxprj\shell\open\command"; ValueType: string; ValueData: """{app}\oxide.exe"" ""%1"""

Root: HKCU; Subkey: "Software\Classes\.snxsch"; ValueType: string; ValueData: "Oxide.snxsch"; Flags: uninsdeletevalue
Root: HKCU; Subkey: "Software\Classes\Oxide.snxsch"; ValueType: string; ValueData: "Oxide Schematic"; Flags: uninsdeletekey
Root: HKCU; Subkey: "Software\Classes\Oxide.snxsch\DefaultIcon"; ValueType: string; ValueData: "{app}\files\snxsch.ico"
Root: HKCU; Subkey: "Software\Classes\Oxide.snxsch\shell\open\command"; ValueType: string; ValueData: """{app}\oxide.exe"" ""%1"""

Root: HKCU; Subkey: "Software\Classes\.snxpcb"; ValueType: string; ValueData: "Oxide.snxpcb"; Flags: uninsdeletevalue
Root: HKCU; Subkey: "Software\Classes\Oxide.snxpcb"; ValueType: string; ValueData: "Oxide PCB"; Flags: uninsdeletekey
Root: HKCU; Subkey: "Software\Classes\Oxide.snxpcb\DefaultIcon"; ValueType: string; ValueData: "{app}\files\snxpcb.ico"
Root: HKCU; Subkey: "Software\Classes\Oxide.snxpcb\shell\open\command"; ValueType: string; ValueData: """{app}\oxide.exe"" ""%1"""

Root: HKCU; Subkey: "Software\Classes\.snxfpt"; ValueType: string; ValueData: "Oxide.snxfpt"; Flags: uninsdeletevalue
Root: HKCU; Subkey: "Software\Classes\Oxide.snxfpt"; ValueType: string; ValueData: "Oxide Footprint"; Flags: uninsdeletekey
Root: HKCU; Subkey: "Software\Classes\Oxide.snxfpt\DefaultIcon"; ValueType: string; ValueData: "{app}\files\snxfpt.ico"
Root: HKCU; Subkey: "Software\Classes\Oxide.snxfpt\shell\open\command"; ValueType: string; ValueData: """{app}\oxide.exe"" ""%1"""

Root: HKCU; Subkey: "Software\Classes\.snxsim"; ValueType: string; ValueData: "Oxide.snxsim"; Flags: uninsdeletevalue
Root: HKCU; Subkey: "Software\Classes\Oxide.snxsim"; ValueType: string; ValueData: "Oxide Simulation"; Flags: uninsdeletekey
Root: HKCU; Subkey: "Software\Classes\Oxide.snxsim\DefaultIcon"; ValueType: string; ValueData: "{app}\files\snxsim.ico"
Root: HKCU; Subkey: "Software\Classes\Oxide.snxsim\shell\open\command"; ValueType: string; ValueData: """{app}\oxide.exe"" ""%1"""

Root: HKCU; Subkey: "Software\Classes\.snxlib"; ValueType: string; ValueData: "Oxide.snxlib"; Flags: uninsdeletevalue
Root: HKCU; Subkey: "Software\Classes\Oxide.snxlib"; ValueType: string; ValueData: "Oxide Library"; Flags: uninsdeletekey
Root: HKCU; Subkey: "Software\Classes\Oxide.snxlib\DefaultIcon"; ValueType: string; ValueData: "{app}\files\snxlib.ico"
Root: HKCU; Subkey: "Software\Classes\Oxide.snxlib\shell\open\command"; ValueType: string; ValueData: """{app}\oxide.exe"" ""%1"""

Root: HKCU; Subkey: "Software\Classes\.snxsym"; ValueType: string; ValueData: "Oxide.snxsym"; Flags: uninsdeletevalue
Root: HKCU; Subkey: "Software\Classes\Oxide.snxsym"; ValueType: string; ValueData: "Oxide Symbol"; Flags: uninsdeletekey
Root: HKCU; Subkey: "Software\Classes\Oxide.snxsym\DefaultIcon"; ValueType: string; ValueData: "{app}\files\snxsym.ico"
Root: HKCU; Subkey: "Software\Classes\Oxide.snxsym\shell\open\command"; ValueType: string; ValueData: """{app}\oxide.exe"" ""%1"""

Root: HKCU; Subkey: "Software\Classes\.snxpkg"; ValueType: string; ValueData: "Oxide.snxpkg"; Flags: uninsdeletevalue
Root: HKCU; Subkey: "Software\Classes\Oxide.snxpkg"; ValueType: string; ValueData: "Oxide Package"; Flags: uninsdeletekey
Root: HKCU; Subkey: "Software\Classes\Oxide.snxpkg\DefaultIcon"; ValueType: string; ValueData: "{app}\files\snxpkg.ico"
Root: HKCU; Subkey: "Software\Classes\Oxide.snxpkg\shell\open\command"; ValueType: string; ValueData: """{app}\oxide.exe"" ""%1"""

Root: HKCU; Subkey: "Software\Classes\.snxmat"; ValueType: string; ValueData: "Oxide.snxmat"; Flags: uninsdeletevalue
Root: HKCU; Subkey: "Software\Classes\Oxide.snxmat"; ValueType: string; ValueData: "Oxide PCB Material"; Flags: uninsdeletekey
Root: HKCU; Subkey: "Software\Classes\Oxide.snxmat\DefaultIcon"; ValueType: string; ValueData: "{app}\files\snxmat.ico"
Root: HKCU; Subkey: "Software\Classes\Oxide.snxmat\shell\open\command"; ValueType: string; ValueData: """{app}\oxide.exe"" ""%1"""

Root: HKCU; Subkey: "Software\Classes\.snxcfg"; ValueType: string; ValueData: "Oxide.snxcfg"; Flags: uninsdeletevalue
Root: HKCU; Subkey: "Software\Classes\Oxide.snxcfg"; ValueType: string; ValueData: "Oxide Config"; Flags: uninsdeletekey
Root: HKCU; Subkey: "Software\Classes\Oxide.snxcfg\DefaultIcon"; ValueType: string; ValueData: "{app}\files\snxcfg.ico"
Root: HKCU; Subkey: "Software\Classes\Oxide.snxcfg\shell\open\command"; ValueType: string; ValueData: """{app}\oxide.exe"" ""%1"""

Root: HKCU; Subkey: "Software\Classes\.snxmod"; ValueType: string; ValueData: "Oxide.snxmod"; Flags: uninsdeletevalue
Root: HKCU; Subkey: "Software\Classes\Oxide.snxmod"; ValueType: string; ValueData: "Oxide SPICE Model"; Flags: uninsdeletekey
Root: HKCU; Subkey: "Software\Classes\Oxide.snxmod\DefaultIcon"; ValueType: string; ValueData: "{app}\files\snxmod.ico"
Root: HKCU; Subkey: "Software\Classes\Oxide.snxmod\shell\open\command"; ValueType: string; ValueData: """{app}\oxide.exe"" ""%1"""

[Run]
Filename: "{app}\oxide.exe"; Description: "{cm:LaunchProgram,Oxide}"; Flags: nowait postinstall skipifsilent
