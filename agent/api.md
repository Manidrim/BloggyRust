# API REST

## Endpoints

### Authentification

| Méthode | Route | Auth | Description | Réponse |
|---------|-------|------|-------------|---------|
| POST | `/auth/register` | — | Créer un compte | `200 AuthResponse` |
| POST | `/auth/login` | — | Se connecter | `200 AuthResponse` |

**AuthResponse** :
```json
{
  "token": "eyJ...",
  "user": {
    "id": "uuid",
    "username": "john",
    "email": "john@example.com",
    "is_admin": false
  }
}
```

### Articles

| Méthode | Route | Auth | Description | Réponse |
|---------|-------|------|-------------|---------|
| GET | `/articles` | — | Lister les articles publiés | `200 ArticleView[]` |
| GET | `/articles/:slug` | — | Récupérer un article | `200 ArticleView` |
| POST | `/articles` | Admin | Créer un article | `201 ArticleView` |
| PUT | `/articles/:slug` | Admin | Modifier un article | `200 ArticleView` |
| DELETE | `/articles/:slug` | Admin | Supprimer un article | `204` |

**ArticleView** :
```json
{
  "id": "uuid",
  "title": "Titre",
  "slug": "titre",
  "content": "...",
  "excerpt": "...",
  "author_id": "uuid",
  "author_username": "john",
  "published": true,
  "tags": [{ "id": "uuid", "name": "Rust", "slug": "rust" }],
  "created_at": "2024-01-01T00:00:00Z",
  "updated_at": "2024-01-01T00:00:00Z"
}
```

### Tags

| Méthode | Route | Auth | Description | Réponse |
|---------|-------|------|-------------|---------|
| GET | `/tags` | — | Lister tous les tags | `200 Tag[]` |
| POST | `/tags` | Admin | Créer un tag | `201 Tag` |
| DELETE | `/tags/:id` | Admin | Supprimer un tag | `204` |

### Commentaires

| Méthode | Route | Auth | Description | Réponse |
|---------|-------|------|-------------|---------|
| GET | `/articles/:id/comments` | — | Lister les commentaires | `200 CommentView[]` |
| POST | `/articles/:id/comments` | Authentifié | Ajouter un commentaire | `201 CommentView` |
| DELETE | `/comments/:id` | Auteur ou Admin | Supprimer un commentaire | `204` |

---

## Authentification JWT

### Structure du token

```json
{
  "sub": "user-uuid",
  "username": "john_doe",
  "is_admin": false,
  "exp": 1711000000
}
```

- Algorithme : HS256
- Durée de validité : **7 jours**
- Clé : `JWT_SECRET` (variable d'environnement)

### Envoi du token

```http
Authorization: Bearer eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9...
```

### Niveaux d'accès

| Niveau | Condition | Routes concernées |
|--------|-----------|------------------|
| Public | Aucune | GET articles, GET tags, GET comments |
| Authentifié | JWT valide | POST comments |
| Admin | JWT valide + `is_admin: true` | POST/PUT/DELETE articles, POST/DELETE tags |

---

## Format des erreurs

Toutes les erreurs suivent le même format JSON :

```json
{ "error": "Message d'erreur lisible" }
```

| Status | AppError variant | Cas d'usage |
|--------|-----------------|-------------|
| 400 | `BadRequest` | Données invalides, champ manquant |
| 401 | `Unauthorized` | Token absent ou invalide |
| 403 | `Forbidden` | Authentifié mais pas autorisé |
| 404 | `NotFound` | Ressource inexistante |
| 409 | `Conflict` | Email/username/slug déjà pris |
| 500 | `Internal` / `Database` | Erreur serveur |

---

## Client HTTP Frontend (`src/api/client.ts`)

Instance Axios partagée avec deux intercepteurs :

**Request interceptor** — ajoute le JWT automatiquement :
```typescript
config.headers.Authorization = `Bearer ${token}`;
```

**Response interceptor** — gestion globale des 401 :
```typescript
// Sur erreur 401 → effacer le token + rediriger vers /login
localStorage.removeItem('token');
window.location.href = '/login';
```

### Fonctions API disponibles

```typescript
// articles.ts
articlesApi.getAll()                    // GET /articles
articlesApi.getBySlug(slug)             // GET /articles/:slug
articlesApi.create(data)                // POST /articles
articlesApi.update(slug, data)          // PUT /articles/:slug
articlesApi.delete(slug)                // DELETE /articles/:slug

// auth.ts
authApi.register(data)                  // POST /auth/register
authApi.login(data)                     // POST /auth/login

// comments.ts
commentsApi.list(articleId)             // GET /articles/:id/comments
commentsApi.create(articleId, data)     // POST /articles/:id/comments
commentsApi.delete(commentId)           // DELETE /comments/:id

// tags.ts
tagsApi.getAll()                        // GET /tags
```
