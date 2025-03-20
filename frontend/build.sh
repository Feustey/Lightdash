#!/bin/bash

set -e  # Arrêter le script en cas d'erreur

echo "🔧 Installation de Rust et Trunk..."

# Assurer que le répertoire Cargo existe
export HOME="/vercel"
export PATH="$HOME/.cargo/bin:$PATH"

# Vérification du bon HOME (éviter les erreurs `$HOME differs from euid-obtained home directory`)
echo "✅ HOME = $HOME"
echo "✅ PATH = $PATH"

# Supprimer les anciennes installations pour éviter des conflits
rm -rf $HOME/.cargo $HOME/.rustup

# Installer Rust proprement
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y --default-toolchain 1.81.0
export PATH="$HOME/.cargo/bin:$PATH"
source "$HOME/.cargo/env"
rustc --version  # Vérification

# Installer cargo-binstall proprement
curl -fsSL https://raw.githubusercontent.com/cargo-bins/cargo-binstall/main/install-from-binstall-release.sh | bash
export PATH="$HOME/.cargo/bin:$PATH"

# Installer Trunk
cargo binstall -y trunk
which trunk
trunk --version  # Vérification

echo "✅ Rust et Trunk installés avec succès."

# Construire le projet
cd frontend
trunk build --release
ls -la dist  # Vérifier si le dossier "dist" est bien généré

echo "✅ Build terminé avec succès."
