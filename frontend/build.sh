#!/bin/bash
set -e

echo "🚀 Démarrage du build..."

# Vérification de l'environnement Rust
echo "📋 Vérification de l'environnement..."
if [ ! -f "$HOME/.cargo/env" ]; then
    echo "❌ Erreur: L'environnement Rust n'est pas configuré"
    exit 1
fi

# Chargement de l'environnement Rust
. "$HOME/.cargo/env"

# Vérification des commandes requises
command -v rustc >/dev/null 2>&1 || { echo "❌ Erreur: rustc n'est pas installé"; exit 1; }
command -v cargo >/dev/null 2>&1 || { echo "❌ Erreur: cargo n'est pas installé"; exit 1; }
command -v trunk >/dev/null 2>&1 || { echo "❌ Erreur: trunk n'est pas installé"; exit 1; }

echo "✅ Version des outils:"
rustc --version
cargo --version
trunk --version

# Création du dossier dist s'il n'existe pas
mkdir -p dist

# Installation des dépendances npm et build du CSS
echo "📦 Installation des dépendances npm..."
npm install --force
echo "🎨 Build du CSS..."
npx tailwindcss -i ./styles/main.css -o ./dist/main.css

# Build du projet avec trunk
echo "🛠️ Build du projet..."
RUSTFLAGS='-C target-feature=+atomics,+bulk-memory,+mutable-globals' \
trunk build --release --public-url /

# Copie des fichiers statiques
echo "📂 Copie des fichiers statiques..."
if [ -d "static" ]; then
    cp -r static/* dist/ 2>/dev/null || :
fi

# Vérification de la présence des fichiers essentiels
echo "✅ Vérification des fichiers générés..."
if [ ! -f "dist/index.html" ]; then
    echo "❌ Erreur: index.html non trouvé"
    exit 1
fi

if [ ! -f "dist/lightdash-frontend.js" ]; then
    echo "❌ Erreur: lightdash-frontend.js non trouvé"
    exit 1
fi

if [ ! -f "dist/lightdash-frontend_bg.wasm" ]; then
    echo "❌ Erreur: lightdash-frontend_bg.wasm non trouvé"
    exit 1
fi

# Vérification de la taille du build
echo "📊 Taille du build :"
du -sh dist/

echo "✅ Build terminé avec succès!" 