# Windows build script for the Declart spec site.
# Run from repo root: pwsh docs/build.ps1
Set-StrictMode -Version Latest
$ErrorActionPreference = 'Stop'

Copy-Item spec/principles.md docs/src/principles.md
Copy-Item spec/schema.md     docs/src/schema.md
New-Item -ItemType Directory -Force docs/src/kinds | Out-Null
Get-ChildItem spec/kinds/*.md | ForEach-Object {
    Copy-Item $_.FullName "docs/src/kinds/$($_.Name)"
}

mdbook build docs/
