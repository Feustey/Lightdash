#!/bin/bash

# Couleurs pour les messages
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m'

echo -e "${YELLOW}Démarrage du build du projet...${NC}"

# Vérification des dépendances
command -v trunk >/dev/null 2>&1 || { echo -e "${RED}Trunk n'est pas installé. Exécutez d'abord install.sh${NC}"; exit 1; }
command -v wasm-pack >/dev/null 2>&1 || { echo -e "${RED}wasm-pack n'est pas installé. Exécutez d'abord install.sh${NC}"; exit 1; }

# Nettoyage
echo -e "${YELLOW}Nettoyage des fichiers de build...${NC}"
rm -rf frontend/dist
rm -rf frontend/target

# Build du frontend
echo -e "${YELLOW}Construction du frontend...${NC}"
cd frontend
trunk build --release

# Vérification
if [ -d "dist" ]; then
    echo -e "${GREEN}Build réussi ! Les fichiers sont dans frontend/dist${NC}"
else
    echo -e "${RED}Erreur lors du build${NC}"
    exit 1
fi

cd ..

echo -e "${GREEN}Build du projet terminé avec succès !${NC}"
echo -e "Pour démarrer l'application :"
echo -e "1. Frontend : cd frontend && trunk serve"
echo -e "2. Backend : cd backend && cargo run"
echo -e "3. Docker : docker-compose up" 