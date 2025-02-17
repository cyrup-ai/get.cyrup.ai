#!/bin/bash
set -e

echo "Cyrup Bootstrap"
echo "Installing minimal requirements for cyrup-install..."

# Install Rust if not present
if ! command -v rustup &> /dev/null; then
    echo "Installing Rust..."
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y --default-toolchain nightly
    source "$HOME/.cargo/env"
fi

# Ensure we're on nightly
if ! rustup show active-toolchain | grep -q "nightly"; then
    echo "Switching to nightly toolchain..."
    rustup default nightly
    rustup update
fi

echo -e "\nRust installation complete. You can now proceed with cyrup-install."
