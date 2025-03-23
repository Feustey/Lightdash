#!/bin/bash

# Couleurs pour les messages
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m'

echo -e "${YELLOW}Démarrage des tests du frontend...${NC}"

# Vérification des dépendances
echo -e "${YELLOW}Vérification des dépendances...${NC}"
command -v wasm-pack >/dev/null 2>&1 || { echo -e "${RED}wasm-pack n'est pas installé. Installation...${NC}"; curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh; }
command -v npm >/dev/null 2>&1 || { echo -e "${RED}npm n'est pas installé.${NC}"; exit 1; }

# Installation des dépendances
echo -e "${YELLOW}Installation des dépendances...${NC}"
npm install

# Tests wasm-pack
echo -e "${YELLOW}Exécution des tests wasm-pack...${NC}"
wasm-pack test --headless --chrome

# Tests frontend
echo -e "${YELLOW}Exécution des tests frontend...${NC}"
npm test

# Linting
echo -e "${YELLOW}Exécution du linting...${NC}"
npm run lint

# Vérification de la compilation
echo -e "${YELLOW}Vérification de la compilation...${NC}"
trunk build

echo -e "${GREEN}Tests terminés avec succès!${NC}" 