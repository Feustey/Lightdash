# Étape de build
FROM rust:1.81.0-slim-bullseye as builder

# Installation des dépendances système
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

# Installation de Trunk et des outils nécessaires
RUN cargo install trunk
RUN cargo install wasm-bindgen-cli

# Installation de wasm32 target
RUN rustup target add wasm32-unknown-unknown

# Création du répertoire de travail
WORKDIR /usr/src/app

# Copie des fichiers de configuration
COPY Cargo.toml .
COPY frontend/Cargo.toml frontend/
COPY frontend/Trunk.toml frontend/
COPY frontend/index.html frontend/
COPY frontend/src frontend/src
COPY frontend/styles.css frontend/
COPY rust-toolchain.toml .

# Build de l'application
WORKDIR /usr/src/app/frontend
RUN trunk build --release

# Étape de production
FROM nginx:alpine

# Copie des fichiers statiques depuis l'étape de build
COPY --from=builder /usr/src/app/frontend/dist /usr/share/nginx/html

# Copie de la configuration nginx
COPY nginx.conf /etc/nginx/conf.d/default.conf

# Exposition du port
EXPOSE 80

# Démarrage de nginx
CMD ["nginx", "-g", "daemon off;"] 