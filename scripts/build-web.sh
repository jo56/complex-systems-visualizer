#!/bin/bash
# Build script for WebAssembly version of Complex Systems Visualizer
# Usage: ./scripts/build-web.sh

set -e

echo "Building Complex Systems Visualizer for Web..."

# Check if trunk is installed
if ! command -v trunk &> /dev/null; then
    echo "Installing trunk..."
    cargo install trunk
fi

# Check if wasm32 target is installed
if ! rustup target list --installed | grep -q "wasm32-unknown-unknown"; then
    echo "Adding wasm32-unknown-unknown target..."
    rustup target add wasm32-unknown-unknown
fi

# Build with trunk
echo "Building with trunk..."
cd sim-web
trunk build --release
cd ..

echo ""
echo "Build complete!"
echo "Output files are in: sim-web/dist/"
echo ""
echo "To serve locally, run:"
echo "  cd sim-web && trunk serve"
