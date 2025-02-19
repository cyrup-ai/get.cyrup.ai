#!/bin/bash

# Create fonts directory
mkdir -p ~/.local/share/fonts

# Download and install FiraCode Nerd Font
wget -q --show-progress \
  https://github.com/ryanoasis/nerd-fonts/releases/download/v3.1.1/FiraCode.zip
unzip -o FiraCode.zip -d ~/.local/share/fonts/FiraCode
rm FiraCode.zip

# Download and install Ubuntu Nerd Font
wget -q --show-progress \
  https://github.com/ryanoasis/nerd-fonts/releases/download/v3.1.1/Ubuntu.zip
unzip -o Ubuntu.zip -d ~/.local/share/fonts/Ubuntu
rm Ubuntu.zip

# Update font cache
fc-cache -fv

echo "Nerd Fonts installed successfully!"
