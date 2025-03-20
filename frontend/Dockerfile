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

# Création d'un utilisateur non-root
RUN addgroup -S nginx && adduser -S -D -H -u 101 -h /usr/share/nginx/html -s /sbin/nologin -G nginx -g nginx nginx

# Copie des fichiers compilés avec les bonnes permissions
COPY --from=builder /app/frontend/dist /usr/share/nginx/html
RUN chown -R nginx:nginx /usr/share/nginx/html && \
    chmod -R 755 /usr/share/nginx/html

# Configuration nginx
COPY frontend/nginx.conf /etc/nginx/conf.d/default.conf

# Configuration des en-têtes de sécurité
RUN echo "add_header X-Frame-Options 'SAMEORIGIN';" >> /etc/nginx/conf.d/default.conf && \
    echo "add_header X-XSS-Protection '1; mode=block';" >> /etc/nginx/conf.d/default.conf && \
    echo "add_header X-Content-Type-Options 'nosniff';" >> /etc/nginx/conf.d/default.conf && \
    echo "add_header Content-Security-Policy \"default-src 'self'; script-src 'self' 'unsafe-inline' 'unsafe-eval' https://cdn.jsdelivr.net; style-src 'self' 'unsafe-inline'; img-src 'self' data: https:; font-src 'self' data: https:;\";" >> /etc/nginx/conf.d/default.conf

# Changement vers l'utilisateur non-root
USER nginx

EXPOSE 80

HEALTHCHECK --interval=30s --timeout=3s \
    CMD wget --no-verbose --tries=1 --spider http://localhost:80/ || exit 1

CMD ["nginx", "-g", "daemon off;"] 