FROM rust:1.81 as builder

# Installation des dépendances système
RUN apt-get update && apt-get install -y \
    git \
    pkg-config \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

# Installation de trunk
RUN cargo install trunk

# Installation des dépendances wasm
RUN rustup target add wasm32-unknown-unknown

WORKDIR /app

# Copie du Cargo.toml du workspace root
COPY ../Cargo.toml /app/Cargo.toml

# Copie du Cargo.toml du frontend
COPY Cargo.toml /app/frontend/Cargo.toml

# Création du dossier frontend
RUN mkdir -p /app/frontend

# Copie des fichiers du projet frontend
COPY . /app/frontend/

# Compilation de l'application
RUN cd frontend && trunk build --release

# Image finale
FROM nginx:alpine

# Copie des fichiers compilés
COPY --from=builder /app/frontend/dist /usr/share/nginx/html

# Configuration nginx
COPY frontend/nginx.conf /etc/nginx/conf.d/default.conf

EXPOSE 80

CMD ["nginx", "-g", "daemon off;"] 