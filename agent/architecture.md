# Architecture

## Vue d'ensemble

BloggY est une application **full-stack monolithique** déployée en containers Docker.
Les trois services (frontend, backend, database) communiquent via un réseau Docker interne.

```
┌─────────────────────────────────────────┐
│              Docker Network             │
│                                         │
│  ┌──────────┐    ┌──────────────────┐   │
│  │ Frontend │───▶│    Backend API   │   │
│  │  React   │    │    Rust/Axum     │───┼──▶ PostgreSQL
│  │  :3000   │    │     :8080        │   │       :5432
│  └──────────┘    └──────────────────┘   │
│                                         │
└─────────────────────────────────────────┘
```

## Architecture Backend

### Pattern : Clean Architecture (partiel) avec Repository Pattern

```
main.rs
  └── AppState { pool: PgPool, config: AppConfig }
        │
        ├── Routes (routes/mod.rs)
        │     └── Handler functions (handlers/)
        │           ├── AuthenticatedUser extractor (middleware/auth.rs)
        │           ├── Path / Json extractors (Axum)
        │           └── Repositories (repositories/)
        │                 └── PgPool queries
        │
        └── AppError (error.rs) → HTTP responses
```

### Couches Backend

| Couche | Dossier | Responsabilité |
|--------|---------|----------------|
| **Handlers** | `src/handlers/` | Recevoir la requête HTTP, valider, appeler les repositories, retourner la réponse |
| **Middleware** | `src/middleware/` | Extraction et validation JWT (`AuthenticatedUser`) |
| **Repositories** | `src/repositories/` | Toutes les requêtes SQL, retournent des types de modèles |
| **Models** | `src/models/` | Structs de données : entités DB, vues (DTOs), requêtes entrantes |
| **Routes** | `src/routes/` | Composition du routeur Axum |
| **Config** | `src/config.rs` | Lecture des variables d'environnement |
| **Errors** | `src/error.rs` | Enum unifié → status codes HTTP |

**Ce qui est absent** : pas de couche Service entre Handler et Repository. La logique métier est dans les handlers (dette technique à résorber).

### Flux d'une requête authentifiée

```
HTTP Request
    │
    ▼
Axum Router (routes/mod.rs)
    │
    ▼
AuthenticatedUser::from_request_parts() (middleware/auth.rs)
    │  Extrait le JWT du header Authorization
    │  Valide la signature et l'expiration
    │  Retourne AuthenticatedUser { id, username, is_admin }
    │
    ▼
Handler (ex: create_article)
    │  Vérifie is_admin si nécessaire
    │  Valide les données entrantes
    │
    ▼
Repository (ex: ArticleRepository::create)
    │  Exécute la requête SQL via SQLx
    │  Retourne AppResult<ArticleView>
    │
    ▼
Handler retourne (StatusCode, Json<ArticleView>)
```

### Gestion des erreurs

```rust
// error.rs — toutes les erreurs passent par AppError
pub enum AppError {
    NotFound(String),
    Unauthorized(String),
    Forbidden(String),
    BadRequest(String),
    Conflict(String),
    Database(sqlx::Error),
    Internal(String),
}

// Impl IntoResponse : chaque variant → status HTTP + JSON
// { "error": "message" }
```

## Architecture Frontend

### Pattern : Modular Component Architecture

```
App.tsx (Router + Layout)
  └── AuthProvider (AuthContext.tsx)
        │
        ├── Pages (src/pages/)
        │     ├── HomePage
        │     ├── ArticlePage
        │     ├── LoginPage
        │     └── admin/
        │           ├── AdminPage
        │           └── ArticleEditorPage
        │
        ├── Composants (src/components/)
        │     ├── layout/ (Header, Footer)
        │     ├── articles/ (ArticleCard, ArticleList)
        │     └── comments/ (CommentSection)
        │
        └── API Layer (src/api/)
              ├── client.ts (Axios + intercepteurs)
              ├── articles.ts
              ├── auth.ts
              ├── comments.ts
              └── tags.ts
```

### Gestion de l'état

| État | Mécanisme | Persistance |
|------|-----------|-------------|
| Authentification | `AuthContext` (Context API) | `localStorage` |
| Token JWT | `AuthContext` | `localStorage["token"]` |
| Données utilisateur | `AuthContext` | `localStorage["user"]` |
| État local des composants | `useState` | Mémoire (session) |

### Flux d'authentification

```
Login Form
    │  POST /auth/login
    ▼
authApi.login()
    │  Axios → backend
    ▼
AuthResponse { token, user }
    │
    ▼
AuthContext.login()
    │  Stocke token + user dans localStorage
    │  Met à jour l'état React
    ▼
Redirect vers /
```

### Protection des routes admin

```tsx
// App.tsx — protection manuelle dans les composants
// Pas de ProtectedRoute générique — chaque page vérifie useAuth()
const { user } = useAuth();
if (!user?.is_admin) return <Navigate to="/" />;
```

## Schema de base de données

```
users
  id (UUID PK)
  username (UNIQUE)
  email (UNIQUE)
  password_hash
  is_admin (BOOL)
  created_at, updated_at

articles
  id (UUID PK)
  title
  slug (UNIQUE)
  content
  excerpt
  author_id → users.id (CASCADE)
  published (BOOL)
  created_at, updated_at

tags
  id (UUID PK)
  name (UNIQUE)
  slug (UNIQUE)
  created_at

article_tags
  article_id → articles.id (CASCADE)
  tag_id → tags.id (CASCADE)
  PRIMARY KEY (article_id, tag_id)

comments
  id (UUID PK)
  article_id → articles.id (CASCADE)
  author_id → users.id (CASCADE)
  content
  created_at, updated_at
```

## Points d'attention architecturaux

1. **Problème N+1** — `ArticleRepository::find_all()` fait 1 requête pour les articles + N requêtes pour les tags de chaque article. À corriger avec un JOIN ou une requête `IN`.

2. **Pas de pagination** — `GET /articles` retourne tous les articles. À ajouter avant mise en production avec trafic.

3. **CORS permissif** — `Any` origin accepté. À restreindre au domaine de production.

4. **XSS potentiel** — `ArticlePage.tsx` utilise `dangerouslySetInnerHTML` pour le contenu des articles. À sanitiser côté backend ou frontend.

5. **Code dupliqué** — `require_admin()` est défini dans `article_handler.rs` et `tag_handler.rs`. À extraire dans un module partagé.
