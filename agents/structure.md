# Structure du projet

## Arborescence complète

```
BloggY/
├── agents/                         # Documentation agent AI (ce dossier)
├── AGENTS.md                       # Point d'entrée de la documentation agent
├── blog/                           # Application principale
│   ├── .env                        # Variables d'environnement (ne pas committer)
│   ├── .env.example                # Template des variables d'environnement
│   ├── docker-compose.yml          # Orchestration des services
│   ├── migrations/                 # Migrations SQL PostgreSQL (ordre strict)
│   │   ├── 001_create_users.sql
│   │   ├── 002_create_articles.sql
│   │   ├── 003_create_tags.sql
│   │   └── 004_create_comments.sql
│   ├── backend/                    # API REST en Rust
│   │   ├── Cargo.toml              # Dépendances Rust
│   │   ├── Cargo.lock              # Lock des dépendances
│   │   ├── Dockerfile              # Build multi-stage Rust → Debian slim
│   │   └── src/
│   │       ├── main.rs             # Point d'entrée, AppState, CLI create-admin
│   │       ├── config.rs           # Chargement des variables d'environnement
│   │       ├── error.rs            # AppError → réponses HTTP
│   │       ├── handlers/           # Couche HTTP (contrôleurs)
│   │       │   ├── mod.rs
│   │       │   ├── auth_handler.rs
│   │       │   ├── article_handler.rs
│   │       │   ├── comment_handler.rs
│   │       │   └── tag_handler.rs
│   │       ├── middleware/         # Middleware Axum
│   │       │   ├── mod.rs
│   │       │   └── auth.rs         # Extracteur JWT AuthenticatedUser
│   │       ├── models/             # Structs de données + DTOs
│   │       │   ├── mod.rs
│   │       │   ├── user.rs
│   │       │   ├── article.rs
│   │       │   ├── comment.rs
│   │       │   └── tag.rs
│   │       ├── repositories/       # Couche d'accès à la base de données
│   │       │   ├── mod.rs
│   │       │   ├── user_repository.rs
│   │       │   ├── article_repository.rs
│   │       │   ├── comment_repository.rs
│   │       │   └── tag_repository.rs
│   │       └── routes/
│   │           └── mod.rs          # Définition du routeur
│   └── frontend/                   # SPA React/TypeScript
│       ├── package.json            # Dépendances npm
│       ├── tsconfig.json           # Configuration TypeScript
│       ├── vite.config.ts          # Configuration Vite
│       ├── nginx.conf              # Nginx pour servir le build
│       ├── Dockerfile              # Build multi-stage Node → Nginx
│       └── src/
│           ├── main.tsx            # Point d'entrée React
│           ├── App.tsx             # Routeur + layout principal
│           ├── types/
│           │   └── index.ts        # Interfaces TypeScript partagées
│           ├── api/                # Clients HTTP par domaine
│           │   ├── client.ts       # Instance Axios + intercepteurs JWT
│           │   ├── articles.ts
│           │   ├── auth.ts
│           │   ├── comments.ts
│           │   └── tags.ts
│           ├── contexts/
│           │   └── AuthContext.tsx # État auth global + persistance localStorage
│           ├── components/         # Composants réutilisables
│           │   ├── articles/
│           │   │   ├── ArticleCard.tsx
│           │   │   └── ArticleList.tsx
│           │   ├── comments/
│           │   │   └── CommentSection.tsx
│           │   └── layout/
│           │       ├── Header.tsx
│           │       └── Footer.tsx
│           └── pages/              # Pages (une par route)
│               ├── HomePage.tsx
│               ├── ArticlePage.tsx
│               ├── LoginPage.tsx
│               └── admin/
│                   ├── AdminPage.tsx
│                   └── ArticleEditorPage.tsx
└── AMELIORATIONS.md                # Liste de 20 améliorations identifiées
```

## Règles de navigation

- **Chercher un endpoint HTTP** → `backend/src/handlers/` + `backend/src/routes/mod.rs`
- **Chercher une requête SQL** → `backend/src/repositories/`
- **Chercher un type partagé** → `frontend/src/types/index.ts` (frontend) ou `backend/src/models/` (backend)
- **Chercher la config Docker** → `blog/docker-compose.yml` + `Dockerfile` de chaque service
- **Chercher les migrations** → `blog/migrations/` (ordre par préfixe numérique)
- **Chercher les variables d'env** → `blog/.env.example` + `backend/src/config.rs`
