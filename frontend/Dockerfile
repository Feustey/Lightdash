FROM rust:1.76-slim as builder

# Installation des dépendances système
RUN apt-get update && apt-get install -y \
    curl \
    build-essential \
    pkg-config \
    nodejs \
    npm \
    && rm -rf /var/lib/apt/lists/*

# Installation de wasm-pack et trunk
RUN rustup target add wasm32-unknown-unknown \
    && cargo install trunk --version 0.17.5

# Configuration du répertoire de travail
WORKDIR /app

# Copie des fichiers du projet
COPY . .

# Installation des dépendances npm
RUN cd frontend && npm install

# Build du projet
RUN cd frontend && ./build.sh

# Image finale
FROM nginx:alpine
COPY --from=builder /app/frontend/dist /usr/share/nginx/html 