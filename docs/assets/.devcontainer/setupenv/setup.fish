#!/usr/bin/env fish

# Get the directory of the current script
set SCRIPT_DIR (dirname (status -f))

# Make all .fish files executable recursively
find "$SCRIPT_DIR" -type f -name "*.fish" -exec chmod +x {} +

# Function to run setup scripts relative to the script directory
function run_setup_scripts
    for script in "$SCRIPT_DIR"/setup/*.fish
        if test -f "$script"
            echo "Running setup script: $script"
            fish "$script"
        end
    end
end

# Check command line arguments
if test (count $argv) -eq 0
    echo "Usage: setup.fish [install|update]"
    exit 1
end

switch $argv[1]
    case install
        echo "Running installation..."
        run_setup_scripts
    case update
        echo "Running update..."
        run_setup_scripts
    case '*'
        echo "Invalid argument. Usage: setup.fish [install|update]"
        exit 1
end

echo "Environment setup complete."
