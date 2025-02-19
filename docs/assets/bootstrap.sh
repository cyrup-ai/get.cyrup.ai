#!/bin/bash

set -euo pipefail

# 1. Install Rust nightly and add to path
if ! command -v rustc &> /dev/null; then
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- --default-toolchain nightly -y
    source "$HOME/.cargo/env"
    
    # Add to shell config for permanence
    for rc in "${HOME}/.bashrc" "${HOME}/.zshrc"; do
        if [[ -f "$rc" ]]; then
            echo 'export PATH="$HOME/.cargo/bin:$PATH"' >> "$rc"
        fi
    done
fi

# 2. Install cargo-edit if not present
if ! command -v cargo-add &> /dev/null; then
    cargo install cargo-edit
fi

# 3. Install rust-script if not present
if ! command -v rust-script &> /dev/null; then
    cargo install rust-script
fi

# 4. Install cargo-binstall if not present
if ! command -v cargo-binstall &> /dev/null; then
    cargo install cargo-binstall
fi

# 5. Get and run setcyrup binary
curl -L --proto '=https' --tlsv1.2 -sSf https://get.cyrup.ai/assets/setcyrup -o /tmp/setcyrup
chmod +x /tmp/setcyrup
/tmp/setcyrup
