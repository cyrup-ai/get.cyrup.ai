#!/bin/bash

# Get the absolute path of the project directory
PROJECT_PATH=$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)

# Create a unique identifier using the last part of the path and timestamp
PATH_HASH=$(echo "$PROJECT_PATH" | shasum | cut -c1-6)
TIMESTAMP=$(date +%s | cut -c-6)

# Combine to create a unique hostname
echo "omni-${PATH_HASH}-${TIMESTAMP}"
