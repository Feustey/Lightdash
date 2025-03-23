#!/bin/bash

# Couleurs pour les messages
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m'

echo -e "${YELLOW}Installation des dépendances...${NC}"

# Installation de Rust
if ! command -v rustc &> /dev/null; then
    echo -e "${YELLOW}Installation de Rust...${NC}"
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
    source $HOME/.cargo/env
else
    echo -e "${GREEN}Rust est déjà installé${NC}"
fi

# Installation de wasm-pack
if ! command -v wasm-pack &> /dev/null; then
    echo -e "${YELLOW}Installation de wasm-pack...${NC}"
    curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh
else
    echo -e "${GREEN}wasm-pack est déjà installé${NC}"
fi

# Installation de trunk
if ! command -v trunk &> /dev/null; then
    echo -e "${YELLOW}Installation de trunk...${NC}"
    cargo install trunk
else
    echo -e "${GREEN}trunk est déjà installé${NC}"
fi

# Installation des dépendances système
if [[ "$OSTYPE" == "darwin"* ]]; then
    if ! command -v cmake &> /dev/null; then
        echo -e "${YELLOW}Installation de cmake...${NC}"
        brew install cmake
    else
        echo -e "${GREEN}cmake est déjà installé${NC}"
    fi
elif [[ "$OSTYPE" == "linux-gnu"* ]]; then
    if ! command -v cmake &> /dev/null; then
        echo -e "${YELLOW}Installation de cmake...${NC}"
        sudo apt-get update
        sudo apt-get install -y cmake
    else
        echo -e "${GREEN}cmake est déjà installé${NC}"
    fi
fi

# Installation des dépendances WebAssembly
echo -e "${YELLOW}Ajout de la cible WebAssembly...${NC}"
rustup target add wasm32-unknown-unknown

echo -e "${GREEN}Installation terminée avec succès !${NC}" 