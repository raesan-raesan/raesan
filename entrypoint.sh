#!/bin/bash

# Exit immediately if a command exits with a non-zero status
set -e

# Print all commands before executing (useful for debugging)
set -x 

# Function to handle graceful shutdown
cleanup() {
    echo "Received shutdown signal. Cleaning up..."
    # Add any specific cleanup for your Rust web app if needed
    exit 0
}

# Trap signals to run cleanup function
trap cleanup SIGINT SIGTERM

# Optional: Log start of application
echo "Starting Raesan....."

# Execute the application
exec /app/raesan serve --database "./database/raesan_base.db"
