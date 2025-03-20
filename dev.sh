#!/bin/bash

# Variables d'environnement
export SPARKSEER_API_KEY=votre_cle_api
export NODE_PUBKEY=votre_pubkey
export API_URL=https://api.sparkseer.space
export ML_URL=https://1ml.com

# Vérification de la présence de Docker
if ! command -v docker &> /dev/null; then
    echo "❌ Docker n'est pas installé. Veuillez l'installer d'abord."
    exit 1
fi

# Vérification de la présence de Docker Compose
if ! command -v docker-compose &> /dev/null; then
    echo "❌ Docker Compose n'est pas installé. Veuillez l'installer d'abord."
    exit 1
fi

# Création du fichier .env s'il n'existe pas
if [ ! -f .env ]; then
    echo "📝 Création du fichier .env..."
    cat > .env << EOL
SPARKSEER_API_KEY=votre_cle_api
NODE_PUBKEY=votre_pubkey
API_URL=https://api.sparkseer.space
ML_URL=https://1ml.com
EOL
    echo "✅ Fichier .env créé. Veuillez le modifier avec vos valeurs."
fi

# Lancement de l'environnement de développement
echo "🚀 Lancement de l'environnement de développement..."
docker-compose -f docker-compose.dev.yml up --build 