#!/usr/bin/env bash

set -euo pipefail

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Log functions
log_info() { echo -e "${GREEN}[INFO]${NC} $1"; }
log_warn() { echo -e "${YELLOW}[WARN]${NC} $1"; }
log_error() { echo -e "${RED}[ERROR]${NC} $1"; }

# Check if command exists
command_exists() {
    command -v "$1" >/dev/null 2>&1
}

# Check and install rust
install_rust() {
    if ! command_exists rustup; then
        log_info "Installing Rust via rustup..."
        curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y --default-toolchain nightly
        source "$HOME/.cargo/env"
    else
        log_info "Rust is already installed, updating to nightly..."
        rustup default nightly
        rustup update
    fi
}

# Install cargo extensions
install_cargo_extensions() {
    log_info "Installing cargo extensions..."
    
    # Install cargo-binstall if not present
    if ! command_exists cargo-binstall; then
        log_info "Installing cargo-binstall..."
        curl -L --proto '=https' --tlsv1.2 -sSf https://raw.githubusercontent.com/cargo-bins/cargo-binstall/main/install-from-release.sh | bash
    fi

    # Use cargo-binstall for faster installation of other tools
    cargo binstall -y cargo-edit rust-script || {
        log_warn "cargo-binstall failed, falling back to cargo install..."
        cargo install cargo-edit rust-script
    }
}

# Install setcyrup binary
install_setcyrup() {
    log_info "Installing setcyrup..."
    
    # Create temp directory
    local tmp_dir
    tmp_dir=$(mktemp -d)
    trap 'rm -rf "$tmp_dir"' EXIT
    
    # Download and verify setcyrup binary
    curl -sSL "https://get.cyrup.ai/assets/setcyrup" -o "$tmp_dir/setcyrup"
    chmod +x "$tmp_dir/setcyrup"
    
    # Test binary
    "$tmp_dir/setcyrup" --version || {
        log_error "Failed to verify setcyrup binary"
        exit 1
    }
    
    # Install to ~/.local/bin
    mkdir -p "$HOME/.local/bin"
    mv "$tmp_dir/setcyrup" "$HOME/.local/bin/setcyrup"
    
    # Add to PATH if not already there
    if [[ ":$PATH:" != *":$HOME/.local/bin:"* ]]; then
        log_warn "Adding ~/.local/bin to PATH in shell config..."
        
        # Detect shell and update rc file
        local shell_rc
        case "$SHELL" in
            */zsh) shell_rc="$HOME/.zshrc" ;;
            */bash) shell_rc="$HOME/.bashrc" ;;
            *) shell_rc="" ;;
        esac
        
        if [ -n "$shell_rc" ]; then
            echo 'export PATH="$HOME/.local/bin:$PATH"' >> "$shell_rc"
            log_info "Added ~/.local/bin to PATH in $shell_rc"
            log_warn "Please restart your shell or run: source $shell_rc"
        else
            log_warn "Could not detect shell config file. Please add ~/.local/bin to your PATH manually"
        fi
    fi
}

main() {
    log_info "Starting Cyrup AI installer..."
    
    # Install dependencies
    install_rust
    install_cargo_extensions
    install_setcyrup
    
    log_info "Installation complete! Run 'setcyrup --help' to get started"
    log_info "If setcyrup command is not found, please restart your shell or add ~/.local/bin to your PATH"
}

# Run main function
main
