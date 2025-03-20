# Lightdash

Application de gestion de nœud Lightning Network.

## Prérequis

- Docker et Docker Compose
- Rust 1.81 ou supérieur (pour le développement local)

## Installation

1. Clonez le repository :
```bash
git clone https://github.com/votre-username/lightdash.git
cd lightdash
```

2. Configurez les variables d'environnement :
```bash
cp .env.example .env
```

3. Modifiez le fichier `.env` avec vos clés API :
```env
SPARKSEER_API_KEY=votre_cle_api
NODE_PUBKEY=votre_pubkey
API_URL=https://api.sparkseer.space
ML_URL=https://1ml.com
```

## Démarrage

### En développement local

```bash
./dev.sh
```

L'application sera accessible à l'adresse : http://localhost:8080

### Avec Docker

```bash
docker-compose up --build
```

## Structure du projet

```
lightdash/
├── frontend/           # Application frontend en Rust/Yew
│   ├── src/           # Code source
│   ├── index.html     # Template HTML
│   └── Cargo.toml     # Dépendances Rust
├── Dockerfile         # Configuration Docker
├── docker-compose.yml # Configuration Docker Compose
└── README.md         # Documentation
```

## Technologies utilisées

- Rust
- Yew (framework web)
- Docker
- Nginx 