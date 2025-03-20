#!/bin/bash

# Install Rust and required tools
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
source $HOME/.cargo/env
export PATH=$HOME/.cargo/bin:$PATH

# Install specific Rust version and add wasm target
rustup install 1.81.0
rustup default 1.81.0
rustup target add wasm32-unknown-unknown

# Install trunk
cargo install trunk

# Build the project
trunk build --release

# Check the output directory
echo "Checking build output directory..."
ls -la dist/

# Ensure the dist directory exists and copy files
mkdir -p dist
cp -r dist/* ./

# Final check of the output
echo "Final output directory contents:"
ls -la 