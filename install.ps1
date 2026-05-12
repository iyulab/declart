#Requires -Version 5.1
[CmdletBinding()]
param(
    [string]$Version = "",
    [string]$InstallDir = "$env:LOCALAPPDATA\Programs\declart"
)

$ErrorActionPreference = "Stop"
$Repo = "iyulab/declart"
$Binary = "declart.exe"
$Target = "x86_64-pc-windows-msvc"

function Get-LatestVersion {
    $release = Invoke-RestMethod "https://api.github.com/repos/$Repo/releases/latest"
    return $release.tag_name
}

if (-not $Version) {
    Write-Host "Fetching latest release..."
    $Version = Get-LatestVersion
}

$Filename = "declart-$Version-$Target.zip"
$Url = "https://github.com/$Repo/releases/download/$Version/$Filename"
$TempZip = Join-Path $env:TEMP $Filename

Write-Host "Installing declart $Version for $Target..."
Write-Host "Downloading: $Url"

New-Item -ItemType Directory -Force -Path $InstallDir | Out-Null
Invoke-WebRequest -Uri $Url -OutFile $TempZip -UseBasicParsing
Expand-Archive -Path $TempZip -DestinationPath $InstallDir -Force
Remove-Item $TempZip

$BinaryPath = Join-Path $InstallDir $Binary
Write-Host "Installed to $BinaryPath"

$CurrentPath = [Environment]::GetEnvironmentVariable("PATH", "User")
if ($CurrentPath -notlike "*$InstallDir*") {
    [Environment]::SetEnvironmentVariable(
        "PATH",
        "$InstallDir;$CurrentPath",
        "User"
    )
    Write-Host "Added $InstallDir to user PATH (restart shell to apply)"
}

& $BinaryPath --version
