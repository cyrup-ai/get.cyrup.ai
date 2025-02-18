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

# Create cyrup directory
mkdir -p "$HOME/cyrup"
cd "$HOME/cyrup"

# Clone repositories
git clone https://github.com/cyrup-ai/secretrust.git secret
git clone https://github.com/cyrup-ai/cyrup.git .
git clone https://github.com/cyrup-ai/get.cyrup.ai.git sys

# Build and install
cd sys
cargo install --path .
cyrup-sys
