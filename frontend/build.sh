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
}

# Fonction de build
build() {
    # Install trunk if not already installed
    if ! command -v trunk &> /dev/null; then
        cargo install trunk
    fi

    # Build the project
    $HOME/.cargo/bin/trunk build --release

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