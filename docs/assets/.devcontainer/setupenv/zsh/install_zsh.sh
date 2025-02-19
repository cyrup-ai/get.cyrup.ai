#!/bin/zsh

set -euo pipefail

# Color variables
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Helper functions
log_info() { echo -e "${GREEN}INFO:${NC} $1"; }
log_warn() { echo -e "${YELLOW}WARN:${NC} $1" >&2; }
log_error() { echo -e "${RED}ERROR:${NC} $1" >&2; }

# Create history file if it doesn't exist
log_info "Setting up zsh history..."
touch "$HOME/.zsh_history"
chmod 600 "$HOME/.zsh_history"

# Create zsh completions directory
log_info "Creating completions directory..."
mkdir -p "$HOME/.config/zsh/completions"

# Backup existing configs
if [ -f "$HOME/.zshrc" ]; then
    log_info "Backing up existing .zshrc..."
    cp "$HOME/.zshrc" "$HOME/.zshrc.bak-$(date +%Y%m%d-%H%M%S)"
fi

if [ -f "$HOME/.zshenv" ]; then
    log_info "Backing up existing .zshenv..."
    cp "$HOME/.zshenv" "$HOME/.zshenv.bak-$(date +%Y%m%d-%H%M%S)"
fi

# Install antidote if not present
ANTIDOTE_DIR="${ZDOTDIR:-$HOME}/.antidote"
if [[ ! -d "$ANTIDOTE_DIR" ]]; then
    log_info "Installing antidote plugin manager..."
    git clone --depth=1 https://github.com/mattmc3/antidote.git "$ANTIDOTE_DIR"
fi

# Install starship
log_info "Installing starship..."
curl -sS https://starship.rs/install.sh | sh -s -- --yes

# Install starship pure preset
log_info "Installing starship pure preset..."
mkdir -p "$HOME/.config"
starship preset pure-preset > "$HOME/.config/starship.toml"

# Copy configuration files
log_info "Installing zsh configuration files..."
cp "$HOME/workspace/config/zsh/.zshrc" "$HOME/.zshrc"
cp "$HOME/workspace/config/zsh/.zshenv" "$HOME/.zshenv"

# Test configuration
log_info "Testing zsh configuration..."
if ! zsh -c 'source "$HOME/.zshrc"' 2>/dev/null; then
    log_warn "New zshrc might have syntax errors. Please check manually."
fi

log_info "Zsh configuration files installed successfully"
log_info "Please restart your shell or run: source ~/.zshrc"
