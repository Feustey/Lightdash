#!/bin/bash

set -e  # Stopper le script en cas d'erreur

echo "🔧 Installation des dépendances..."

# Vérifier si Rust est déjà installé
if ! command -v rustc &> /dev/null; then
    echo "🔧 Installation de Rust..."
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y --default-toolchain 1.81.0
    source "$HOME/.cargo/env"
    rustc --version  # Vérification
fi

# Vérifier et installer Trunk
if ! command -v trunk &> /dev/null; then
    echo "🔧 Installation de Trunk..."
    cargo install trunk
    which trunk
    trunk --version  # Vérification
fi

echo "✅ Rust et Trunk installés avec succès."

# Construire le projet
trunk build --release
ls -la dist  # Vérifier si "dist" est bien généré

echo "✅ Build terminé avec succès."
