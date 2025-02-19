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

    # Check for Rust
    if ! command -v cargo &> /dev/null; then
        log_error "Rust is required. Please install it first."
        exit 1
    fi

    # Check for Homebrew
    if ! command -v brew &> /dev/null; then
        log_error "Homebrew is required. Please install it first."
        exit 1
    fi

    # Install colima if not present (for local kubernetes)
    if ! command -v colima &> /dev/null; then
        log_info "Installing Colima..."
        brew install colima
    fi

    # Install kubectl if not present
    if ! command -v kubectl &> /dev/null; then
        log_info "Installing kubectl..."
        brew install kubectl
    fi
}

# Start local Kubernetes cluster
start_kubernetes() {
    log_info "Starting Colima with Kubernetes..."
    if ! colima status &> /dev/null; then
        colima start --kubernetes
    elif ! colima status | grep -q "kubernetes: true"; then
        colima stop
        colima start --kubernetes
    fi
}

# Setup Rust development environment
setup_rust_dev() {
    log_info "Setting up Rust development environment..."

    # Add kube-rs dependencies to Cargo.toml if needed
    if ! cargo add kube --dry-run &> /dev/null; then
        log_info "Adding kube-rs dependencies..."
        cargo add kube
        cargo add k8s-openapi --features=v1_26
        cargo add tokio --features=full
        cargo add anyhow
    fi

    # Create example if it doesn't exist
    if [ ! -f "src/main.rs" ]; then
        log_info "Creating example Rust kube-rs code..."
        mkdir -p src
        cat > src/main.rs << 'EOF'
use anyhow::Result;
use k8s_openapi::api::core::v1::Pod;
use kube::{
    api::{Api, ListParams},
    Client,
};

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize the Kubernetes client
    let client = Client::try_default().await?;

    // Get an API handle to the Pods
    let pods: Api<Pod> = Api::default_namespaced(client);

    // List all pods in the namespace
    let pod_list = pods.list(&ListParams::default()).await?;

    println!("Pods in current namespace:");
    for pod in pod_list {
        if let Some(metadata) = pod.metadata {
            println!(" - {}", metadata.name.unwrap_or_default());
        }
    }

    Ok(())
}
EOF
    fi
}

# Main setup process
main() {
    check_requirements
    start_kubernetes
    setup_rust_dev

    log_info "Development environment setup complete!"
    log_info "You can now use kube-rs to interact with your local Kubernetes cluster"
    log_info "Example code has been created in src/main.rs"
    log_info "Run with: cargo run"
}

main
