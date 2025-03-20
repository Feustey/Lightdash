#!/bin/bash

# Fonction d'installation
install() {
    # Uninstall existing Rust installation
    rustup self uninstall -y

    # Install Rust and required tools
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
    source $HOME/.cargo/env
    export PATH=$HOME/.cargo/bin:$PATH

    # Install specific Rust version and add wasm target
    rustup install 1.81.0
    rustup default 1.81.0
    rustup target add wasm32-unknown-unknown

    # Verify Rust version
    echo "Verifying Rust version..."
    rustc --version

    # Install and verify trunk
    echo "Installing trunk..."
    cargo install trunk
    echo "Verifying trunk installation..."
    which trunk
    trunk --version
}

# Fonction de build
build() {
    # Ensure PATH includes cargo bin
    export PATH=$HOME/.cargo/bin:$PATH

    # Verify trunk is available
    if ! which trunk &> /dev/null; then
        echo "Trunk not found in PATH. Installing..."
        cargo install trunk
    fi

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
}

# Gestion des arguments
case "$1" in
    "install")
        install
        ;;
    *)
        install
        build
        ;;
esac 