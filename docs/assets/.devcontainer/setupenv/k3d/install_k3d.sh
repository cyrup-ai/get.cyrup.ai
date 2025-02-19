#!/bin/bash

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

# Check for required tools
check_requirements() {
    log_info "Checking development requirements..."

    # Check for Docker
    if ! command -v docker &> /dev/null; then
        log_error "Docker is required. Please install it first."
        exit 1
    fi

    # Install k3d if not present
    if ! command -v k3d &> /dev/null; then
        log_info "Installing k3d..."
        brew install k3d
    fi

    # Install kubectl if not present
    if ! command -v kubectl &> /dev/null; then
        log_info "Installing kubectl..."
        brew install kubectl
    fi
}

# Start local Kubernetes cluster
setup_k3d() {
    log_info "Setting up k3d cluster..."

    # Create cluster if it doesn't exist
    if ! k3d cluster list | grep -q "dev-cluster"; then
        k3d cluster create dev-cluster \
            --api-port 6443 \
            --servers 1 \
            --agents 1 \
            --no-lb \
            --k3s-arg "--disable=traefik@server:0" \
            --wait
    else
        log_info "Cluster already exists"
    fi

    # Set kubectl context
    kubectl config use-context k3d-dev-cluster
}

# Main setup process
main() {
    check_requirements
    setup_k3d

    log_info "K3d development environment setup complete!"
    log_info "K3d cluster 'dev-cluster' is running"
    log_info "Kubernetes API available at localhost:6443"
    log_info "Current kubectl context: $(kubectl config current-context)"
}

main
