#!/bin/bash

set -e  # Stopper le script en cas d'erreur

echo "🔧 Correction du problème de HOME..."

# Définir HOME et PATH proprement
export HOME=/vercel
export USER=vercel
export PATH="$HOME/.cargo/bin:$PATH"

echo "✅ HOME = $HOME"
echo "✅ USER = $USER"
echo "✅ PATH = $PATH"

# Vérifier si Rust est déjà installé
if ! command -v rustc &> /dev/null; then
    echo "🔧 Installation de Rust..."
    rm -rf $HOME/.cargo $HOME/.rustup  # Supprimer d'anciennes versions
    
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y --default-toolchain 1.81.0
    source "$HOME/.cargo/env"
    rustc --version  # Vérification
fi

# Installer cargo-binstall proprement
if ! command -v cargo-binstall &> /dev/null; then
    echo "🔧 Installation de cargo-binstall..."
    curl -fsSL https://raw.githubusercontent.com/cargo-bins/cargo-binstall/main/install-from-binstall-release.sh | bash
    export PATH="$HOME/.cargo/bin:$PATH"
fi

# Vérifier et installer Trunk
if ! command -v trunk &> /dev/null; then
    echo "🔧 Installation de Trunk..."
    cargo binstall -y trunk
    export PATH="$HOME/.cargo/bin:$PATH"
    which trunk
    trunk --version  # Vérification
fi

echo "✅ Rust et Trunk installés avec succès."

# Construire le projet
cd frontend
trunk build --release
ls -la dist  # Vérifier si "dist" est bien généré

echo "✅ Build terminé avec succès."
