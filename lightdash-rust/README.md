# Lightdash

Lightdash est une interface web moderne pour gérer votre nœud Lightning Bitcoin. Elle offre une expérience utilisateur intuitive pour surveiller et gérer vos canaux de paiement, transactions et informations du nœud.

## 🌟 Fonctionnalités

- **Tableau de bord interactif**
  - Statistiques en temps réel du nœud
  - Visualisation des canaux de paiement
  - Graphiques de transactions
  - Thème sombre/clair

- **Gestion des canaux**
  - Création de nouveaux canaux
  - Fermeture de canaux existants
  - Visualisation des balances
  - État des canaux en temps réel

- **Gestion des transactions**
  - Historique complet des transactions
  - Envoi de paiements
  - Création d'invoices
  - Filtres et recherche

## 🚀 Installation

### Prérequis

- Rust 1.70 ou supérieur
- Un nœud Lightning (LND, c-lightning, etc.)
- Vercel CLI (pour le déploiement)

### Configuration locale

1. Cloner le repository :
```bash
git clone https://github.com/votre-username/lightdash.git
cd lightdash
```

2. Créer un fichier `.env` :
```env
LIGHTNING_URL=http://votre-nœud-lightning:8080
RUST_LOG=info
```

3. Compiler et exécuter :
```bash
cargo build --release
cargo run
```

### Déploiement sur Vercel

1. Installer Vercel CLI :
```bash
npm install -g vercel
```

2. Se connecter à Vercel :
```bash
vercel login
```

3. Configurer les variables d'environnement :
```bash
vercel env add LIGHTNING_URL
vercel env add LIGHTNING_MACAROON
vercel env add LIGHTNING_CERT
vercel env add RUST_LOG
```

4. Déployer :
```bash
vercel --prod
```

## 🔧 Configuration

### Variables d'environnement

| Variable | Description | Requis |
|----------|-------------|---------|
| `LIGHTNING_URL` | URL de votre nœud Lightning | Oui |
| `LIGHTNING_MACAROON` | Macaroon d'authentification | Oui |
| `LIGHTNING_CERT` | Certificat TLS | Oui |
| `RUST_LOG` | Niveau de log (info, debug, etc.) | Non |

### Sécurité

- Ne partagez jamais vos macaroons ou certificats
- Utilisez HTTPS en production
- Limitez les origines CORS selon vos besoins

## 📚 Documentation API

### Endpoints

- `GET /api/node/info` - Informations du nœud
- `GET /api/channels` - Liste des canaux
- `GET /api/transactions` - Historique des transactions
- `POST /api/payments` - Envoyer un paiement
- `POST /api/invoices` - Créer une invoice

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