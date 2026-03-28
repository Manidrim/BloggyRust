# Fichiers de configuration

## Variables d'environnement

### Fichier de référence : `blog/.env.example`

```env
# Base de données PostgreSQL
POSTGRES_USER=blog_user
POSTGRES_PASSWORD=blog_password
POSTGRES_DB=blog_db

# Sécurité — à changer impérativement en production
JWT_SECRET=change_me_in_production_please

# Logging backend (trace | debug | info | warn | error)
RUST_LOG=info

# URL de l'API pour le build frontend
VITE_API_URL=http://localhost:8080
```

### Variables backend (lues par `backend/src/config.rs`)

| Variable | Obligatoire | Défaut | Description |
|----------|-------------|--------|-------------|
| `DATABASE_URL` | Oui | — | `postgres://user:pass@host:5432/db` |
| `JWT_SECRET` | Oui | — | Clé de signature JWT (≥32 chars en prod) |
| `HOST` | Non | `0.0.0.0` | Adresse d'écoute du serveur |
| `PORT` | Non | `8080` | Port du serveur |
| `RUST_LOG` | Non | `info` | Niveau de log Tokio/Tracing |

### Variable frontend (build-time Vite)

| Variable | Description |
|----------|-------------|
| `VITE_API_URL` | URL de base de l'API (injectée dans le bundle à la compilation) |

**Important** : `VITE_API_URL` est une variable **build-time**. Elle est injectée dans le JavaScript au moment du `npm run build`. Changer cette valeur après le build n'a aucun effet.

---

## Docker Compose — `blog/docker-compose.yml`

Trois services orchestrés :

### Service `db` (PostgreSQL)
```yaml
image: postgres:16-alpine
volumes:
  - postgres_data:/var/lib/postgresql/data  # persistance
  - ./migrations:/docker-entrypoint-initdb.d  # migrations auto au 1er démarrage
healthcheck: pg_isready  # les autres services attendent ce check
```

### Service `backend` (Rust/Axum)
```yaml
depends_on:
  db:
    condition: service_healthy  # attend que PostgreSQL soit prêt
environment:
  DATABASE_URL: postgres://...
  JWT_SECRET: ${JWT_SECRET}  # lu depuis .env
  RUST_LOG: ${RUST_LOG:-info}
```

### Service `frontend` (React/Nginx)
```yaml
depends_on:
  - backend
build:
  args:
    VITE_API_URL: ${VITE_API_URL:-http://localhost:8080}
```

---

## Dockerfile Backend — `blog/backend/Dockerfile`

Build **multi-stage** pour optimiser la taille de l'image :

```
Stage 1 (builder): rust:1.88-slim
  ├── Installe les dépendances système (libssl-dev, pkg-config, libpq-dev)
  ├── Cache les dépendances Cargo séparément du code source
  └── cargo build --release

Stage 2 (runtime): debian:bookworm-slim
  ├── Installe uniquement les runtime deps (ca-certificates, libssl3, libpq5)
  └── Copie uniquement le binaire compilé
```

---

## Dockerfile Frontend — `blog/frontend/Dockerfile`

Build **multi-stage** :

```
Stage 1 (builder): node:20-alpine
  ├── npm ci (installation déterministe)
  ├── ARG VITE_API_URL (injecté comme ENV pour Vite)
  └── npm run build → dist/

Stage 2 (runtime): nginx:alpine
  ├── Copie dist/ dans /usr/share/nginx/html
  └── Utilise nginx.conf personnalisé
```

---

## Nginx Frontend — `blog/frontend/nginx.conf`

```nginx
# SPA routing — toute route inconnue retourne index.html
location / {
    try_files $uri $uri/ /index.html;
}

# Cache 1 an (immutable) pour les assets avec hash dans le nom
location ~* \.(js|css|png|jpg|svg|woff2)$ {
    expires 1y;
    add_header Cache-Control "public, immutable";
}

# Gzip activé pour js, css, json, svg, html
```

---

## TypeScript — `blog/frontend/tsconfig.json`

```json
{
  "compilerOptions": {
    "target": "ES2020",
    "module": "ESNext",
    "moduleResolution": "bundler",
    "strict": true,           // Typage strict activé
    "jsx": "react-jsx"
  }
}
```

---

## Vite — `blog/frontend/vite.config.ts`

```typescript
export default defineConfig({
  plugins: [react()],
  server: {
    proxy: {
      '/api': process.env.VITE_API_URL  // Proxy dev vers le backend
    }
  }
})
```

---

## Cargo — `blog/backend/Cargo.toml`

```toml
[package]
name = "blog-backend"
version = "0.1.0"
edition = "2021"

[[bin]]
name = "blog-backend"
```

Dépendances clés avec leurs features activées :
- `sqlx` : features `["runtime-tokio-rustls", "postgres", "uuid", "time", "macros"]`
- `tokio` : features `["full"]`
- `tower-http` : features `["cors", "trace"]`
- `uuid` : features `["v4", "serde"]`

---

## Migrations — `blog/migrations/`

Les migrations sont appliquées **automatiquement** au premier démarrage du container Docker (via le point de montage dans `docker-entrypoint-initdb.d`).

L'ordre est déterminé par le préfixe numérique :

| Fichier | Crée |
|---------|------|
| `001_create_users.sql` | Table `users` |
| `002_create_articles.sql` | Table `articles` |
| `003_create_tags.sql` | Tables `tags` + `article_tags` |
| `004_create_comments.sql` | Table `comments` |

**Note** : `docker-entrypoint-initdb.d` n'exécute les scripts qu'à l'**initialisation** du volume. Pour ré-appliquer, il faut supprimer le volume : `docker-compose down -v`.
