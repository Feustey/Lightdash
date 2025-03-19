#!/bin/bash
set -e

echo "🚀 Démarrage du build..."

# Vérification de l'environnement
if [ ! -f "Cargo.toml" ]; then
    echo "❌ Erreur: Cargo.toml non trouvé"
    exit 1
fi

# Installation de Rust et des outils nécessaires
echo "📦 Installation de Rust..."
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
source $HOME/.cargo/env || source ~/.cargo/env

echo "🎯 Installation de la cible wasm32..."
rustup target add wasm32-unknown-unknown

echo "🔧 Installation de trunk..."
cargo install --locked trunk || true

# Configuration des optimisations
echo "⚡ Configuration des optimisations..."
export RUSTFLAGS="-C opt-level=3 -C codegen-units=1"

# Build du frontend
echo "🏗️ Build du frontend..."
cd frontend
trunk build --release

# Vérification et copie des fichiers
echo "📋 Vérification des fichiers générés..."
if [ ! -d "dist" ]; then
    echo "❌ Erreur: Le dossier dist n'a pas été créé"
    exit 1
fi

echo "📦 Copie des fichiers vers le dossier de sortie Vercel..."
mkdir -p ../.vercel/output/static
cp -r dist/* ../.vercel/output/static/

# Vérification des fichiers copiés
if [ ! -f "../.vercel/output/static/index.html" ]; then
    echo "❌ Erreur: index.html non trouvé dans le dossier de sortie"
    exit 1
fi

echo "✅ Build terminé avec succès!" 