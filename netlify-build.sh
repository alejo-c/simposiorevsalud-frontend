#!/bin/bash
set -e

# Install Rust (if not already)
if ! command -v rustup &>/dev/null; then
  curl https://sh.rustup.rs -sSf | sh -s -- -y
  source $HOME/.cargo/env
fi

# Install wasm target
rustup target add wasm32-unknown-unknown

# Install trunk (if not already)
if ! command -v trunk &>/dev/null; then
  cargo install trunk
fi

# Build the project
trunk build --release
