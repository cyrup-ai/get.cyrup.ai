#!/bin/bash

# Install system dependencies
apt-get update && apt-get install -y \
    pkg-config \
    libx11-dev \
    libxcb1-dev \
    libxcb-render0-dev \
    libxcb-shape0-dev \
    libxcb-xfixes0-dev \
    libxkbcommon-dev \
    libgtk-3-dev \
    libsoup2.4-dev \
    libwebkit2gtk-4.0-dev

# Clone and build Zed
git clone https://github.com/zed-industries/zed.git /tmp/zed
cd /tmp/zed

# Build with release profile
cargo build --release

# Install binary
mv target/release/zed /usr/local/bin/

# Clean up
cd /
rm -rf /tmp/zed

# Create config directory
mkdir -p $HOME/.config/zed/themes
