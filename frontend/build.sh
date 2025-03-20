#!/bin/bash

# Install Rust and required tools
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
source $HOME/.cargo/env

# Switch to nightly and add wasm target
rustup default nightly
rustup target add wasm32-unknown-unknown

# Install trunk
cargo install trunk

# Build the project
trunk build --release

# Create dist directory if it doesn't exist
mkdir -p dist

# Copy the built files to dist
cp -r dist/* dist/ 