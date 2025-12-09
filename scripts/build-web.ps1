# Build script for WebAssembly version of Complex Systems Visualizer
# Usage: .\scripts\build-web.ps1

$ErrorActionPreference = "Stop"

Write-Host "Building Complex Systems Visualizer for Web..." -ForegroundColor Cyan

# Check if trunk is installed
if (-not (Get-Command trunk -ErrorAction SilentlyContinue)) {
    Write-Host "Installing trunk..." -ForegroundColor Yellow
    cargo install trunk
}

# Check if wasm32 target is installed
$targets = rustup target list --installed
if ($targets -notcontains "wasm32-unknown-unknown") {
    Write-Host "Adding wasm32-unknown-unknown target..." -ForegroundColor Yellow
    rustup target add wasm32-unknown-unknown
}

# Build with trunk
Write-Host "Building with trunk..." -ForegroundColor Cyan
Push-Location sim-web
trunk build --release
Pop-Location

Write-Host ""
Write-Host "Build complete!" -ForegroundColor Green
Write-Host "Output files are in: sim-web/dist/" -ForegroundColor Cyan
Write-Host ""
Write-Host "To serve locally, run:" -ForegroundColor Yellow
Write-Host "  cd sim-web && trunk serve" -ForegroundColor White
