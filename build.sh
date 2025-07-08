#!/bin/bash

# Build script for Oscilloscope project
# Compiles Rust backend to WASM and places it in the frontend

set -e

echo "Building Rust backend to WASM..."

# Build with wasm-pack
RUSTFLAGS='--cfg getrandom_backend="wasm_js" -C target-feature=+simd128' \
wasm-pack build --target web --out-dir ./pkg

echo "Build complete! WASM files generated in pkg/"
echo ""
echo "To run the application:"
echo "  1. Serve the app directory with a static file server"
echo "  2. Open index.html in a web browser"
