#!/bin/bash
set -e

# Install Rust if needed
if ! command -v rustup &>/dev/null; then
  curl https://sh.rustup.rs -sSf | sh -s -- -y
  source $HOME/.cargo/env
fi

# ✅ Set default Rust toolchain
rustup default stable

# Install wasm target
rustup target add wasm32-unknown-unknown

# Install trunk if not already
if ! command -v trunk &>/dev/null; then
  cargo install trunk
fi

# Build with Trunk
trunk build --release
