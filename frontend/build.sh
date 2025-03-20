#!/bin/bash

# Installation des outils nécessaires
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
source "$HOME/.cargo/env"
rustup target add wasm32-unknown-unknown
cargo install --locked trunk
cargo install --locked wasm-bindgen-cli

# Build du projet
cd frontend
trunk build --release

# Copie des fichiers dans le répertoire de sortie
mkdir -p /vercel/output/static
cp -r dist/* /vercel/output/static/ 