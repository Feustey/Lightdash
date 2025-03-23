#!/bin/bash

# Couleurs pour les messages
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m'

echo -e "${YELLOW}Démarrage du build du backend...${NC}"

# Vérification de Rust
echo -e "${YELLOW}Vérification de Rust...${NC}"
command -v cargo >/dev/null 2>&1 || { echo -e "${RED}Rust n'est pas installé. Veuillez installer Rust pour continuer.${NC}"; exit 1; }

# Build du backend
echo -e "${YELLOW}Build du backend...${NC}"
cargo build --release

# Vérification du build
if [ -f "target/release/lightdash" ]; then
    echo -e "${GREEN}Build réussi !${NC}"
    echo -e "L'exécutable est disponible dans target/release/lightdash"
else
    echo -e "${RED}Erreur lors du build${NC}"
    exit 1
fi 