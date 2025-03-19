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
    && cargo install trunk --version 0.17.5 --locked

# Configuration du répertoire de travail
WORKDIR /app

# Copie des fichiers du projet
COPY frontend/ ./

# Installation des dépendances et build
RUN npm install && \
    trunk build --release

# Image finale avec Nginx
FROM nginx:alpine
COPY --from=builder /app/dist /usr/share/nginx/html
COPY frontend/nginx.conf /etc/nginx/conf.d/default.conf 