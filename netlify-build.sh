#!/bin/bash
set -e

# Restore cargo and target cache (Netlify plugin handles this if enabled)
echo "Restoring cached Rust dependencies if available..."

# Ensure Rust is installed
if ! command -v rustup &>/dev/null; then
  curl https://sh.rustup.rs -sSf | sh -s -- -y
  source $HOME/.cargo/env
fi

# Set toolchain and target
rustup default stable
rustup target add wasm32-unknown-unknown

# Install Trunk if missing
if ! command -v trunk &>/dev/null; then
  cargo install trunk
fi

# Build
trunk build --release

# Copy static assets into the dist folder
cp -r static/styles dist/
cp -r static/img dist/
cp static/_redirects dist/
