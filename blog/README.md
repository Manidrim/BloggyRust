# Blog — Rust + React + PostgreSQL

Application blog complète, orchestrée via Docker Compose.

## Stack

| Couche      | Technologie               |
|-------------|---------------------------|
| Backend     | Rust · Axum · SQLx        |
| Frontend    | React 18 · Vite · TypeScript |
| Base de données | PostgreSQL 16         |
| Auth        | JWT (jsonwebtoken · argon2) |
| Infra       | Docker Compose            |

---

## Démarrage rapide

### 1. Prérequis

- [Docker](https://docs.docker.com/get-docker/) + Docker Compose v2

### 2. Configuration

```bash
cp .env.example .env
# Éditez .env si nécessaire (secrets, ports…)
```

### 3. Lancer le projet

```bash
docker-compose up --build
```

| Service   | URL                      |
|-----------|--------------------------|
| Frontend  | http://localhost:3000    |
| Backend   | http://localhost:8080    |
| PostgreSQL| localhost:5432           |

---

## Architecture du backend

```
backend/src/
├── main.rs                  # Point d'entrée, AppState, serveur
├── config.rs                # Variables d'environnement
├── error.rs                 # AppError → réponses HTTP uniformes
├── models/                  # Structs de données (User, Article, Tag, Comment)
├── repositories/            # Accès base de données (une responsabilité par repo)
├── handlers/                # Handlers HTTP (une responsabilité par domaine)
├── middleware/              # Auth JWT (Bearer token)
└── routes/                  # Assemblage du routeur
```

## Architecture du frontend

```
frontend/src/
├── api/           # Fonctions d'appel HTTP (axios)
├── components/    # Composants réutilisables (ArticleCard, CommentSection…)
├── contexts/      # AuthContext (état utilisateur global)
├── pages/         # Pages (Home, Article, Login, Admin)
└── types/         # Types TypeScript partagés
```

---

## Routes API

### Auth
| Méthode | Route            | Description        |
|---------|------------------|--------------------|
| POST    | /auth/register   | Créer un compte    |
| POST    | /auth/login      | Se connecter → JWT |

### Articles
| Méthode | Route              | Auth   | Description               |
|---------|--------------------|--------|---------------------------|
| GET     | /articles          | —      | Liste des articles publiés |
| GET     | /articles/:slug    | —      | Détail d'un article       |
| POST    | /articles          | Admin  | Créer un article          |
| PUT     | /articles/:slug    | Admin  | Modifier un article       |
| DELETE  | /articles/:slug    | Admin  | Supprimer un article      |

### Commentaires
| Méthode | Route                       | Auth        | Description           |
|---------|-----------------------------|-------------|-----------------------|
| GET     | /articles/:id/comments      | —           | Lister les commentaires |
| POST    | /articles/:id/comments      | Connecté    | Poster un commentaire |
| DELETE  | /comments/:id               | Auteur/Admin| Supprimer             |

### Tags
| Méthode | Route       | Auth   | Description   |
|---------|-------------|--------|---------------|
| GET     | /tags       | —      | Lister les tags |
| POST    | /tags       | Admin  | Créer un tag  |
| DELETE  | /tags/:id   | Admin  | Supprimer     |

---

## Développement local (sans Docker)

### Backend

```bash
cd backend
# Assurez-vous que PostgreSQL tourne (ex: docker-compose up db)
export DATABASE_URL=postgres://blog_user:blog_password@localhost:5432/blog_db
export JWT_SECRET=dev_secret
cargo run
```

> **Note SQLx** : les macros `query!` et `query_as!` nécessitent une base
> accessible à la compilation. Pour compiler sans base active, générez le
> cache offline :
> ```bash
> cargo install sqlx-cli
> cargo sqlx prepare
> ```
> puis exportez `SQLX_OFFLINE=true`.

### Frontend

```bash
cd frontend
npm install
npm run dev   # http://localhost:5173
```

---

## Premier administrateur

Après inscription via `/auth/register`, passez `is_admin = true` directement en base :

```sql
UPDATE users SET is_admin = TRUE WHERE email = 'votre@email.com';
```

