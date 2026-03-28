# Technologies utilisées

## Backend — Rust

| Technologie | Version | Rôle |
|-------------|---------|------|
| **Rust** | 1.88 | Langage principal |
| **Axum** | 0.7 | Framework web HTTP asynchrone |
| **Tokio** | 1 (full) | Runtime async |
| **SQLx** | 0.7 | ORM/query builder avec validation compile-time |
| **PostgreSQL** | 16 | Base de données relationnelle |
| **jsonwebtoken** | 9 | Génération et validation JWT |
| **Argon2** | 0.5 | Hachage des mots de passe |
| **Tower** | 0.4 | Middleware HTTP |
| **Tower-HTTP** | 0.5 | CORS, tracing HTTP |
| **Serde / Serde JSON** | 1 | Sérialisation/désérialisation |
| **UUID** | 1 (v4) | Génération d'identifiants |
| **time** | 0.3 | Manipulation des dates |
| **thiserror** | 1 | Macros de dérivation d'erreurs |
| **dotenvy** | 0.15 | Chargement des fichiers `.env` |
| **tracing** | 0.1 | Logs structurés |
| **tracing-subscriber** | 0.3 | Sortie des logs |

### Particularités SQLx

SQLx valide les requêtes SQL **au moment de la compilation** contre la base de données réelle.
En mode offline (`SQLX_OFFLINE=true`), les requêtes sont validées contre un fichier `.sqlx/` généré par `cargo sqlx prepare`.

## Frontend — React/TypeScript

| Technologie | Version | Rôle |
|-------------|---------|------|
| **React** | 18.2 | Framework UI |
| **TypeScript** | 5.3 | Typage statique |
| **Vite** | 5.1 | Bundler + serveur de développement |
| **React Router DOM** | 6.22 | Routing côté client |
| **Axios** | 1.6.7 | Client HTTP avec intercepteurs |

### Ce qui N'est PAS utilisé

- Pas de CSS framework (Bootstrap, Tailwind, etc.) — styles inline uniquement
- Pas de state manager global (Redux, Zustand) — Context API uniquement
- Pas de bibliothèque de composants UI
- Pas de librairie de formulaires
- Pas de librairie de tests (Jest, Vitest, Testing Library)

## Infrastructure

| Technologie | Version | Rôle |
|-------------|---------|------|
| **Docker** | v2+ | Containerisation |
| **Docker Compose** | v2 | Orchestration locale |
| **Nginx** | Alpine | Serveur web (frontend en production) |
| **PostgreSQL** | 16 Alpine | Base de données |

### Ports par défaut

| Service | Port | URL locale |
|---------|------|-----------|
| Frontend (Nginx) | 3000 | http://localhost:3000 |
| Backend (Axum) | 8080 | http://localhost:8080 |
| PostgreSQL | 5432 | localhost:5432 |
| Frontend (dev Vite) | 5173 | http://localhost:5173 |
