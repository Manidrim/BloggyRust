# Base de données

## Technologie

- **PostgreSQL 16 Alpine** (via Docker)
- **SQLx 0.7** côté Rust (validation compile-time des requêtes)
- Pas d'ORM complet — requêtes SQL brutes avec `sqlx::query_as!`

## Schéma

### Table `users`

```sql
CREATE TABLE users (
    id          UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    username    VARCHAR(50) UNIQUE NOT NULL,
    email       VARCHAR(255) UNIQUE NOT NULL,
    password_hash TEXT NOT NULL,            -- Argon2 hash
    is_admin    BOOLEAN NOT NULL DEFAULT FALSE,
    created_at  TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at  TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_users_email ON users(email);
```

### Table `articles`

```sql
CREATE TABLE articles (
    id          UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    title       VARCHAR(500) NOT NULL,
    slug        VARCHAR(500) UNIQUE NOT NULL,  -- généré à partir du titre
    content     TEXT NOT NULL,
    excerpt     VARCHAR(500),
    author_id   UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    published   BOOLEAN NOT NULL DEFAULT FALSE,
    created_at  TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at  TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_articles_slug      ON articles(slug);
CREATE INDEX idx_articles_author_id ON articles(author_id);
CREATE INDEX idx_articles_published ON articles(published);
```

### Tables `tags` et `article_tags`

```sql
CREATE TABLE tags (
    id         UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    name       VARCHAR(100) UNIQUE NOT NULL,
    slug       VARCHAR(100) UNIQUE NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TABLE article_tags (
    article_id UUID NOT NULL REFERENCES articles(id) ON DELETE CASCADE,
    tag_id     UUID NOT NULL REFERENCES tags(id) ON DELETE CASCADE,
    PRIMARY KEY (article_id, tag_id)        -- relation many-to-many
);

CREATE INDEX idx_article_tags_article ON article_tags(article_id);
CREATE INDEX idx_article_tags_tag     ON article_tags(tag_id);
```

### Table `comments`

```sql
CREATE TABLE comments (
    id         UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    article_id UUID NOT NULL REFERENCES articles(id) ON DELETE CASCADE,
    author_id  UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    content    TEXT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_comments_article_id ON comments(article_id);
CREATE INDEX idx_comments_author_id  ON comments(author_id);
```

---

## Repositories (couche d'accès données)

### `UserRepository`

Fichier : `backend/src/repositories/user_repository.rs`

| Méthode | Description |
|---------|-------------|
| `find_by_id(id)` | Récupère un utilisateur par UUID |
| `find_by_email(email)` | Récupère un utilisateur par email (pour login) |
| `email_exists(email)` | Vérifie si un email est déjà utilisé |
| `create(data)` | Crée un utilisateur standard |
| `create_admin(data)` | Crée un utilisateur avec `is_admin = true` |

### `ArticleRepository`

Fichier : `backend/src/repositories/article_repository.rs`

| Méthode | Description |
|---------|-------------|
| `find_all_published()` | Articles publiés avec auteur (⚠️ N+1 pour les tags) |
| `find_by_slug(slug)` | Article par slug avec auteur et tags |
| `find_by_id(id)` | Article par UUID |
| `create(data, author_id)` | Crée un article (slug auto-généré) |
| `update(slug, data)` | Met à jour titre/content/excerpt/published |
| `delete(slug)` | Supprime l'article (cascade sur commentaires) |
| `set_tags(article_id, tag_ids)` | Remplace tous les tags d'un article |
| `find_tags_for_article(id)` | Tags d'un article (appelé N fois) |

### `CommentRepository`

Fichier : `backend/src/repositories/comment_repository.rs`

| Méthode | Description |
|---------|-------------|
| `find_by_article(article_id)` | Tous les commentaires avec auteur (JOIN) |
| `create(article_id, author_id, content)` | Crée un commentaire (CTE avec JOIN) |
| `find_by_id(id)` | Commentaire par UUID (pour autorisation) |
| `delete(id)` | Supprime un commentaire |

### `TagRepository`

Fichier : `backend/src/repositories/tag_repository.rs`

| Méthode | Description |
|---------|-------------|
| `find_all()` | Tous les tags |
| `find_by_id(id)` | Tag par UUID |
| `create(name)` | Crée un tag (slug auto-généré) |
| `delete(id)` | Supprime un tag |

---

## Patterns de requêtes

### Requête avec JOIN (pattern standard)

```rust
// Récupère commentaire + username de l'auteur en une requête
sqlx::query_as!(CommentView,
    r#"SELECT c.*, u.username as author_username
       FROM comments c
       JOIN users u ON c.author_id = u.id
       WHERE c.article_id = $1
       ORDER BY c.created_at ASC"#,
    article_id
)
.fetch_all(pool)
.await
```

### Pattern CTE (INSERT + JOIN atomique)

```rust
// Insère et retourne avec données jointes en une seule requête
sqlx::query_as!(CommentView,
    r#"WITH inserted AS (
         INSERT INTO comments (article_id, author_id, content)
         VALUES ($1, $2, $3)
         RETURNING *
       )
       SELECT i.*, u.username as author_username
       FROM inserted i
       JOIN users u ON i.author_id = u.id"#,
    ...
)
```

### Problème N+1 connu (articles + tags)

```rust
// ⚠️ PROBLÈME : 1 requête articles + N requêtes tags
let rows = sqlx::query!("SELECT * FROM articles WHERE published = true").fetch_all(pool).await?;
for row in rows {
    let tags = self.find_tags_for_article(row.id).await?;  // N requêtes supplémentaires
}
```

**Correction recommandée** : utiliser `array_agg` avec un JOIN ou une requête `WHERE article_id IN (...)`.

---

## Gestion des connexions

Le `PgPool` SQLx est créé une seule fois dans `main.rs` et partagé via `AppState` :

```rust
let pool = PgPool::connect(&config.database_url).await?;
let state = AppState { pool: Arc::new(pool), config };
```

Les repositories reçoivent `&PgPool` en référence (lifetime `'a`) — pas de clone.

---

## Migrations

Les migrations sont dans `blog/migrations/` et appliquées automatiquement via Docker.
Pour les appliquer manuellement avec `sqlx-cli` :

```bash
cargo install sqlx-cli --no-default-features --features postgres
DATABASE_URL="postgres://..." sqlx migrate run
```
