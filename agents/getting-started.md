# Lancer le projet

## Prérequis

- Docker et Docker Compose v2 installés
- (Optionnel pour le développement local) Rust 1.88+, Node.js 20+, npm

---

## Démarrage rapide avec Docker Compose

```bash
cd blog/

# 1. Créer le fichier d'environnement
cp .env.example .env

# 2. Lancer tous les services
docker-compose up --build

# Services disponibles :
# - Frontend : http://localhost:3000
# - Backend  : http://localhost:8080
# - DB       : localhost:5432
```

Le premier démarrage prend quelques minutes (compilation Rust).
Les migrations SQL sont appliquées automatiquement.

### Commandes utiles Docker

```bash
# Démarrer en arrière-plan
docker-compose up -d

# Voir les logs en temps réel
docker-compose logs -f

# Voir les logs d'un service
docker-compose logs -f backend

# Arrêter les services
docker-compose down

# Arrêter et supprimer le volume (reset de la BDD)
docker-compose down -v

# Reconstruire un service après modification du code
docker-compose up --build backend
```

---

## Développement local (sans Docker)

### Démarrer uniquement la base de données

```bash
cd blog/
docker-compose up db
```

### Backend (Rust)

```bash
cd blog/backend/

# Variables d'environnement
export DATABASE_URL="postgres://blog_user:blog_password@localhost:5432/blog_db"
export JWT_SECRET="dev_secret_change_in_production"
export RUST_LOG="debug"

# Lancer en mode développement
cargo run

# ou avec rechargement automatique (nécessite cargo-watch)
cargo install cargo-watch
cargo watch -x run
```

**Attention SQLx** : SQLx valide les requêtes SQL à la compilation contre la base de données réelle.
Si la DB n'est pas accessible, utiliser le mode offline :

```bash
# Générer le cache (à faire une fois avec la DB accessible)
cargo install sqlx-cli --no-default-features --features postgres
cargo sqlx prepare

# Compiler sans accès à la DB
SQLX_OFFLINE=true cargo build
```

### Frontend (React/Vite)

```bash
cd blog/frontend/

# Installer les dépendances
npm install

# Lancer le serveur de développement
npm run dev
# Accessible sur http://localhost:5173
# Le proxy /api redirige vers http://localhost:8080
```

---

## Créer un compte administrateur

### Via la CLI du backend

```bash
# Avec Docker
docker-compose exec backend /app/blog-backend create-admin \
  --username "admin" \
  --email "admin@example.com" \
  --password "MotDePasseSécurisé123!"

# En développement local
cargo run -- create-admin \
  --username "admin" \
  --email "admin@example.com" \
  --password "MotDePasseSécurisé123!"
```

### Via SQL directement

```bash
# Se connecter à la base de données
docker-compose exec db psql -U blog_user -d blog_db

# Promouvoir un utilisateur existant
UPDATE users SET is_admin = TRUE WHERE email = 'user@example.com';
```

---

## Build de production

### Backend

```bash
cd blog/backend/
cargo build --release
# Binaire : target/release/blog-backend
```

### Frontend

```bash
cd blog/frontend/
VITE_API_URL=https://api.mondomaine.com npm run build
# Sortie : dist/
```

---

## Variables d'environnement pour la production

Modifier `blog/.env` (ou injecter via le système d'orchestration) :

```env
POSTGRES_USER=<user_production>
POSTGRES_PASSWORD=<mot_de_passe_fort>
POSTGRES_DB=blog_db
JWT_SECRET=<secret_aléatoire_min_32_caractères>
RUST_LOG=warn
VITE_API_URL=https://api.mondomaine.com
```

**Générer un JWT_SECRET sécurisé** :
```bash
openssl rand -base64 32
```

---

## Tests

**État actuel : aucun test n'existe dans le projet.**

Pour ajouter des tests Rust :
```bash
cd blog/backend/
cargo test
```

Pour ajouter des tests frontend :
```bash
cd blog/frontend/
npm test  # nécessite d'abord d'ajouter Vitest ou Jest
```
