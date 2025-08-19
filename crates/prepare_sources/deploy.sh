#!/bin/bash

# Stop any existing instance of the application
# This assumes you have a way to identify and stop the process, e.g., by PID or process name
# For simplicity, this example just kills any process named 'prepare_sources'
# In a production environment, you'd use a more robust process management system (e.g., systemd, supervisor)

# Find and kill existing processes
PID=$(pgrep prepare_sources)
if [ -n "$PID" ]; then
  echo "Stopping existing prepare_sources process (PID: $PID)..."
  kill $PID
  sleep 2 # Give it a moment to shut down
fi

# Build the latest release version
echo "Building new release version..."
cargo build --release

# Check if build was successful
if [ $? -ne 0 ]; then
  echo "Build failed. Aborting deployment."
  exit 1
fi

# Start the new version in the background
echo "Starting new prepare_sources instance..."
# Replace <command> and <path> with your desired default command and project path
# Example: ./target/release/prepare_sources analyze-rust /path/to/your/project &
./target/release/prepare_sources analyze-rust /mnt/c/Users/gentd/OneDrive/Documents/GitHub/solfunmeme-dioxus &

echo "Deployment complete."
