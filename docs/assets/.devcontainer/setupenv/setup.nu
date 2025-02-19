#!/usr/bin/env nu

# Get the directory of the current script
let SCRIPT_DIR = (dirname $nu.current-file)

# Make all .nu files executable recursively
ls $"($SCRIPT_DIR)/**/*.nu" | each { |file| chmod +x $file.name }

# Function to run setup scripts relative to the script directory
def run_setup_scripts [] {
    ls $"($SCRIPT_DIR)/setup/*.nu" | each { |script|
        if ($script | path exists) {
            echo $"Running setup script: ($script.name)"
            nu $script.name
        }
    }
}

# Check command line arguments
if ($nu.args | length) == 0 {
    echo "Usage: setup.nu [install|update]"
    exit 1
}

match $nu.args.0 {
    "install" => {
        echo "Running installation..."
        run_setup_scripts
    },
    "update" => {
        echo "Running update..."
        run_setup_scripts
    },
    _ => {
        echo "Invalid argument. Usage: setup.nu [install|update]"
        exit 1
    }
}

echo "Environment setup complete."
