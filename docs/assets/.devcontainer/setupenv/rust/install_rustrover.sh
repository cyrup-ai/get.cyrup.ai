#!/bin/bash

# Download and extract RustRover
RUSTROVER_VERSION="2023.3"
DOWNLOAD_URL="https://download.jetbrains.com/rustrover/RustRover-${RUSTROVER_VERSION}.tar.gz"

# Create installation directory
mkdir -p /opt/rustrover

# Download and extract
curl -L $DOWNLOAD_URL | tar xz -C /opt/rustrover --strip-components=1

# Create symlink to the binary
ln -s /opt/rustrover/bin/rustrover.sh /usr/local/bin/rustrover

# Install RustRover server
cargo install --git https://github.com/rust-lang/rust-analyzer.git rust-analyzer
cargo install --locked bacon

# Create config directory
mkdir -p $HOME/.config/JetBrains/RustRover${RUSTROVER_VERSION}

# Set up environment variables
echo 'export PATH="/opt/rustrover/bin:$PATH"' >> $HOME/.zshrc
