# Regenerates the README images (assets/hero.png, assets/gallery.png) from the declarations in this folder.
# Needs: the declart CLI (cargo build --release -p declart-cli) and Chrome or Edge.
$ErrorActionPreference = 'Stop'
$here = $PSScriptRoot
$root = Split-Path $here -Parent | Split-Path -Parent
$cli = Join-Path $root 'target/release/declart.exe'
if (-not (Test-Path $cli)) { $cli = Join-Path $root 'target/release/declart' }
$browser = @(
  "$env:ProgramFiles\Google\Chrome\Application\chrome.exe",
  "${env:ProgramFiles(x86)}\Microsoft\Edge\Application\msedge.exe"
) | Where-Object { Test-Path $_ } | Select-Object -First 1
if (-not $browser) { throw 'Chrome or Edge is required to capture the images.' }

$tmp = Join-Path $here '.tmp'
New-Item -ItemType Directory -Force $tmp | Out-Null
& $cli render (Join-Path $here 'hero.toml') --format png --width 900 -o (Join-Path $tmp 'hero.png')
Get-ChildItem (Join-Path $here 'gallery/*.toml') | ForEach-Object {
  & $cli render $_.FullName --format png --width 1000 -o (Join-Path $tmp "$($_.BaseName).png")
}

# Headless Chrome reserves part of the window for its own UI, so capture a taller window and crop to size.
Add-Type -AssemblyName System.Drawing
function Capture($page, $out, $w, $h) {
  $url = ([System.Uri](Join-Path $here $page)).AbsoluteUri
  $raw = Join-Path $tmp 'capture.png'
  Remove-Item $raw -ErrorAction SilentlyContinue
  Start-Process $browser -Wait -NoNewWindow -ArgumentList @('--headless=new', '--disable-gpu', '--hide-scrollbars',
    '--force-device-scale-factor=2', '--virtual-time-budget=5000', "--window-size=$w,$($h + 200)", "--screenshot=`"$raw`"", $url)
  $ms = [System.IO.MemoryStream]::new([System.IO.File]::ReadAllBytes($raw))
  $src = [System.Drawing.Bitmap]::new($ms)
  $dst = $src.Clone([System.Drawing.Rectangle]::new(0, 0, $w * 2, $h * 2), $src.PixelFormat)
  $src.Dispose()
  $dst.Save($out, [System.Drawing.Imaging.ImageFormat]::Png)
  $dst.Dispose()
}
Capture 'hero.html' (Join-Path $root 'assets/hero.png') 1600 900
Capture 'gallery.html' (Join-Path $root 'assets/gallery.png') 1500 1420
Remove-Item -Recurse -Force $tmp
