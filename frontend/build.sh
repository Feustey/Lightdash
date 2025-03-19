#!/bin/bash
set -e

echo "🚀 Démarrage du build..."

# Vérification de l'environnement
if [ ! -f "Cargo.toml" ]; then
    echo "❌ Erreur: Cargo.toml non trouvé"
    exit 1
fi

# Création du dossier dist s'il n'existe pas
mkdir -p dist

# Installation des dépendances npm et build du CSS
echo "📦 Installation des dépendances npm..."
npm install
echo "🎨 Build du CSS..."
npx tailwindcss -i ./styles/main.css -o ./dist/main.css

# Build du projet avec trunk
echo "🛠️ Build du projet..."
trunk build --release

# Copie des fichiers statiques
echo "📂 Copie des fichiers statiques..."
if [ -d "static" ]; then
    cp -r static/* dist/ 2>/dev/null || :
fi

# Vérification de la taille du build
echo "📊 Taille du build :"
du -sh dist/

# Vérification des fichiers générés
echo "✅ Build terminé avec succès!" 