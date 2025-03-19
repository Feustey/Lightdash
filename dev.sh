#!/bin/bash

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
NEXT_PUBLIC_SPARKSEER_API_KEY=votre_cle_api
NEXT_PUBLIC_NODE_PUBKEY=votre_pubkey
EOL
    echo "✅ Fichier .env créé. Veuillez le modifier avec vos valeurs."
fi

# Lancement de l'environnement de développement
echo "🚀 Lancement de l'environnement de développement..."
docker-compose -f docker-compose.dev.yml up --build 