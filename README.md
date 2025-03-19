# Lightdash - Interface de Gestion de Nœud Lightning

[![CI](https://github.com/Feustey/Lightdash/actions/workflows/ci.yml/badge.svg)](https://github.com/Feustey/Lightdash/actions/workflows/ci.yml)
[![Deploy](https://github.com/Feustey/Lightdash/actions/workflows/deploy.yml/badge.svg)](https://github.com/Feustey/Lightdash/actions/workflows/deploy.yml)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Rust](https://img.shields.io/badge/Rust-1.70%2B-orange.svg)](https://www.rust-lang.org)
[![Vercel](https://img.shields.io/badge/Vercel-Deployed-black.svg)](https://vercel.com)

Lightdash est une application web moderne développée en Rust qui permet de gérer et surveiller votre nœud Lightning Network. L'application utilise le framework Actix-web pour offrir une interface rapide et fiable.

## Fonctionnalités

- 🔍 Consultation des informations du nœud
- 💰 Gestion des canaux Lightning
- 📊 Suivi des transactions
- 📈 Statistiques du réseau Lightning
- 🌐 Intégration avec Sparkseer et 1ML pour des données enrichies

## Prérequis

- Rust 1.70 ou supérieur
- Un nœud Lightning Network (compatible avec l'API REST LND)
- Accès à Internet pour les API externes (Sparkseer, 1ML)

## Installation

1. Clonez le dépôt :
```bash
git clone https://github.com/votre-username/lightdash.git
cd lightdash
```

2. Créez un fichier `.env` à la racine du projet :
```env
PORT=3000
HOST=127.0.0.1
LIGHTNING_URL=http://votre-noeud:port
NEXT_PUBLIC_API_URL=https://api.sparkseer.space
NEXT_PUBLIC_1ML_URL=https://1ml.com
```

3. Compilez et lancez l'application :
```bash
cargo run --bin lightdash_rust
```

## Architecture

L'application est structurée en plusieurs modules :
- `handlers/` : Gestionnaires de routes HTTP
- `models/` : Structures de données
- `services/` : Logique métier et intégrations externes

## API Endpoints

- `GET /api/node/info` : Informations sur le nœud
- `GET /api/channels` : Liste des canaux
- `GET /api/transactions` : Historique des transactions
- `GET /api/network/stats` : Statistiques du réseau

## Recommandations d'Optimisations (IA)

1. **Performance**
   - Implémenter un système de cache pour les requêtes externes (Sparkseer, 1ML)
   - Utiliser des connexions persistantes avec le nœud Lightning
   - Ajouter des métriques de performance avec Prometheus

2. **Sécurité**
   - Ajouter une authentification JWT
   - Implémenter une limitation de taux (rate limiting)
   - Valider toutes les entrées utilisateur

3. **Architecture**
   - Migrer vers une architecture microservices
   - Utiliser gRPC pour les communications internes
   - Implémenter un système de file d'attente pour les opérations asynchrones

4. **Tests**
   - Ajouter des tests unitaires
   - Implémenter des tests d'intégration
   - Mettre en place des tests de charge

5. **Monitoring**
   - Intégrer OpenTelemetry pour le tracing
   - Ajouter des alertes sur les métriques clés
   - Implémenter un système de logging structuré

6. **Interface Utilisateur**
   - Développer une interface utilisateur en WebAssembly
   - Ajouter des graphiques interactifs
   - Implémenter des notifications en temps réel

7. **Déploiement**
   - Conteneuriser l'application avec Docker
   - Mettre en place un pipeline CI/CD
   - Automatiser les déploiements avec Kubernetes

## Contribution

Les contributions sont les bienvenues ! N'hésitez pas à :
1. Fork le projet
2. Créer une branche pour votre fonctionnalité
3. Soumettre une Pull Request

## Licence

MIT License - Voir le fichier LICENSE pour plus de détails.

## 🤝 Contribution

Les contributions sont les bienvenues ! N'hésitez pas à :

1. Fork le projet
2. Créer une branche (`git checkout -b feature/AmazingFeature`)
3. Commit vos changements (`git commit -m 'Add some AmazingFeature'`)
4. Push vers la branche (`git push origin feature/AmazingFeature`)
5. Ouvrir une Pull Request

## 📝 Licence

Ce projet est sous licence MIT - voir le fichier [LICENSE](LICENSE) pour plus de détails.

## 🙏 Remerciements

- [Lightning Network](https://lightning.network/)
- [Vercel](https://vercel.com/)
- [Rust](https://www.rust-lang.org/)
- [Chart.js](https://www.chartjs.org/) 