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

Write-Host "Building mdbook-declart preprocessor..."
cargo build --package mdbook-declart --release

Write-Host "Building WASM for playground..."
New-Item -ItemType Directory -Force docs/src/playground/wasm | Out-Null
$wasmOutDir = (Resolve-Path "docs/src/playground/wasm").Path
wasm-pack build crates/declart-wasm --target web --out-dir $wasmOutDir --release

$env:PATH = "$((Resolve-Path 'target/release').Path)$([IO.Path]::PathSeparator)$env:PATH"
mdbook build docs/
