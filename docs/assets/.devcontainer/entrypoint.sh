#!/bin/bash
set -o errexit -o nounset -o pipefail
set -x

# Security: Prevent secrets from being logged
export HISTCONTROL=ignorespace
unset DOCKER_PASSWORD 2>/dev/null || true
unset AWS_ACCESS_KEY_ID 2>/dev/null || true
unset AWS_SECRET_ACCESS_KEY 2>/dev/null || true

# Initialize Python tools
source "$HOME/.rye/env"
export PDM_HOME="$HOME/.local/pdm"
export PATH="$PDM_HOME/bin:$PATH"

# Initialize nvm
export NVM_DIR="$HOME/.nvm"
[ -s "$NVM_DIR/nvm.sh" ] && \. "$NVM_DIR/nvm.sh"
[ -s "$NVM_DIR/bash_completion" ] && \. "$NVM_DIR/bash_completion"

# Initialize Starship prompt for all shells
export STARSHIP_CONFIG="$HOME/.config/starship.toml"
if [ -z "$STARSHIP_SHELL" ]; then
    eval "$(starship init bash)"
fi

# Start Docker daemon if it's not running
if ! pgrep dockerd >/dev/null; then
    dockerd &
    # Wait for Docker daemon to be ready
    while ! docker info >/dev/null 2>&1; do
        sleep 1
    done
fi

# Run the k3d setup script
if [ -f "/usr/local/bin/install_k3d.sh" ]; then
    /usr/local/bin/install_k3d.sh
fi

# Execute the command passed to docker run
exec "$@"
