param(
    [Parameter(Mandatory = $true)][ValidatePattern('^\d+\.\d+\.\d+$')][string]$Version,
    [Parameter(Mandatory = $true)][ValidatePattern('^[0-9A-Fa-f]{64}$')][string]$Sha256
)
$ErrorActionPreference = "Stop"
$directory = Join-Path $PSScriptRoot "..\packaging\winget\Weslley08.CMR\$Version"
New-Item -Path $directory -ItemType Directory -Force | Out-Null
$identifier = "Weslley08.CMR"
$asset = "cmr-x86_64-pc-windows-msvc.zip"
$url = "https://github.com/Weslley08/cmr/releases/download/v$Version/$asset"
@"
PackageIdentifier: $identifier
PackageVersion: $Version
DefaultLocale: en-US
ManifestType: version
ManifestVersion: 1.9.0
"@ | Set-Content -Path (Join-Path $directory "$identifier.yaml") -Encoding UTF8
@"
PackageIdentifier: $identifier
PackageVersion: $Version
PackageLocale: en-US
Publisher: Weslley08
PackageName: Claude Model Router
License: MIT
ShortDescription: Companion CLI for Claude Code model routing
PackageUrl: https://github.com/Weslley08/cmr
ManifestType: defaultLocale
ManifestVersion: 1.9.0
"@ | Set-Content -Path (Join-Path $directory "$identifier.locale.en-US.yaml") -Encoding UTF8
@"
PackageIdentifier: $identifier
PackageVersion: $Version
InstallerType: zip
NestedInstallerType: portable
NestedInstallerFiles:
  - RelativeFilePath: cmr.exe
    PortableCommandAlias: cmr
Installers:
  - Architecture: x64
    InstallerUrl: $url
    InstallerSha256: $($Sha256.ToUpperInvariant())
ManifestType: installer
ManifestVersion: 1.9.0
"@ | Set-Content -Path (Join-Path $directory "$identifier.installer.yaml") -Encoding UTF8
Write-Host "WinGet manifest generated in: $directory"
Write-Host "Validate using: winget validate --manifest '$directory'"
Write-Host "Then submit to microsoft/winget-pkgs; publication requires external review."
