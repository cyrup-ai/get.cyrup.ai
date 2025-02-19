#!/bin/bash

# Get the directory of the current script
SCRIPT_DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )"

# Make all .sh files executable recursively
find "$SCRIPT_DIR" -type f -name "*.sh" -exec chmod +x {} +

# Function to run setup scripts relative to the script directory
run_setup_scripts() {
    for script in "$SCRIPT_DIR"/setup/*.sh; do
        if [ -f "$script" ]; then
            echo "Running setup script: $script"
            bash "$script"
        fi
    done
}

# Check command line arguments
if [ $# -eq 0 ]; then
    echo "Usage: $0 [install|update]"
    exit 1
fi

case "$1" in
    install)
        echo "Running installation..."
        run_setup_scripts
        ;;
    update)
        echo "Running update..."
        run_setup_scripts
        ;;
    *)
        echo "Invalid argument. Usage: $0 [install|update]"
        exit 1
        ;;
esac

echo "Environment setup complete."
