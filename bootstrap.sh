#!/bin/bash
set -e

echo "🚀 Welcome to Cyrup AI"

# Install Rust nightly
if ! command -v rustup &> /dev/null; then
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y --default-toolchain nightly
    source "$HOME/.cargo/env"
fi

# Ensure nightly
rustup default nightly
rustup update

# Run installer
cargo install --git https://github.com/cyrup-ai/get.cyrup.ai.git
cyrup-install
