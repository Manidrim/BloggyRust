# Conventions de code

Ces conventions s'appuient sur les principes de *Clean Code* (Robert C. Martin).

---

## Principes généraux

### Noms qui révèlent l'intention

Les noms doivent répondre à : pourquoi ça existe, ce que ça fait, comment l'utiliser.

```rust
// ✅ Bon : le nom dit ce qu'il fait
async fn find_all_published(&self) -> AppResult<Vec<ArticleView>>

// ❌ Mauvais : vague
async fn get_data(&self) -> AppResult<Vec<ArticleView>>
```

### Fonctions courtes et à responsabilité unique (SRP)

Une fonction = une chose. Si une fonction a besoin d'un commentaire pour expliquer ce qu'elle fait, elle fait trop de choses.

### Pas de duplication (DRY)

`require_admin()` est actuellement dupliqué dans `article_handler.rs` et `tag_handler.rs`. À extraire dans un module commun.

---

## Conventions Backend (Rust)

### Nommage

| Élément | Convention | Exemple |
|---------|------------|---------|
| Modules/fichiers | `snake_case` | `article_handler.rs` |
| Types/Structs/Enums | `PascalCase` | `ArticleView`, `AppError` |
| Fonctions/méthodes | `snake_case` | `find_by_slug()` |
| Variables | `snake_case` | `article_id` |
| Constantes | `SCREAMING_SNAKE_CASE` | `MAX_CONTENT_LENGTH` |
| Lifetime parameters | courtes, minuscules | `'a` |

### Structure des handlers

```rust
// Ordre standard des extracteurs Axum
pub async fn handler_name(
    State(state): State<AppState>,         // 1. État partagé
    user: AuthenticatedUser,               // 2. Utilisateur authentifié (si requis)
    Path(param): Path<String>,             // 3. Paramètres de route
    Json(payload): Json<RequestType>,      // 4. Corps JSON
) -> AppResult<impl IntoResponse> {
    // Validation → logique → réponse
}
```

### Gestion des erreurs

```rust
// Toujours utiliser AppResult<T> = Result<T, AppError>
// Convertir les erreurs SQLx explicitement
.map_err(AppError::Database)?

// Retourner des erreurs sémantiques
return Err(AppError::NotFound("Article non trouvé".to_string()));
return Err(AppError::Forbidden("Accès réservé aux administrateurs".to_string()));
```

### Requêtes SQL

```rust
// ✅ Paramètres liés — jamais de concaténation de chaînes
sqlx::query_as!(Article, "SELECT * FROM articles WHERE slug = $1", slug)

// ✅ Utiliser le bon type de fetch selon la cardinalité attendue
.fetch_one(pool)       // Exactement 1 résultat (erreur si 0 ou plusieurs)
.fetch_optional(pool)  // 0 ou 1 résultat
.fetch_all(pool)       // 0 à N résultats
.execute(pool)         // Pas de retour de données (INSERT/UPDATE/DELETE)
```

### Structs de modèles

Chaque entité suit le pattern :
- `Entity` — struct base avec tous les champs DB
- `EntityView` — struct de réponse (peut inclure des données jointes comme `author_username`)
- `CreateEntityRequest` / `UpdateEntityRequest` — DTOs d'entrée

---

## Conventions Frontend (TypeScript/React)

### Nommage

| Élément | Convention | Exemple |
|---------|------------|---------|
| Composants | `PascalCase` | `ArticleCard.tsx` |
| Pages | `PascalCase` + suffix `Page` | `ArticlePage.tsx` |
| Hooks | `use` + `PascalCase` | `useAuth()` |
| Fonctions API | `camelCase` | `getBySlug()` |
| Interfaces/Types | `PascalCase` | `Article`, `AuthResponse` |
| Fichiers non-composants | `camelCase` | `client.ts` |

### Structure des composants

```tsx
// Ordre standard dans un composant React
interface Props {
  // 1. Props typées
}

export function ComponentName({ prop1, prop2 }: Props) {
  // 2. Hooks (useState, useContext, useEffect)
  const [state, setState] = useState<Type>(initial);

  // 3. Fonctions locales
  const handleAction = async () => { ... };

  // 4. JSX return
  return <div>...</div>;
}
```

### Gestion des types

Toutes les interfaces partagées sont dans `src/types/index.ts`.
Ne pas redéclarer des types déjà définis dans ce fichier.

```typescript
// ✅ Importer depuis le fichier central
import { Article, User } from '../types';

// ❌ Éviter de redéfinir localement
interface Article { ... }  // déjà dans types/index.ts
```

### Appels API

```typescript
// ✅ Toujours passer par les fonctions du dossier src/api/
const articles = await articlesApi.getAll();

// ❌ Jamais appeler Axios directement depuis un composant
const res = await axios.get('/articles');
```

### Gestion des états de chargement

```tsx
// Pattern standard dans les pages/composants
const [data, setData] = useState<Type[]>([]);
const [loading, setLoading] = useState(true);
const [error, setError] = useState<string | null>(null);

useEffect(() => {
  const load = async () => {
    try {
      const result = await api.getAll();
      setData(result);
    } catch (err) {
      setError('Message d\'erreur utilisateur');
    } finally {
      setLoading(false);
    }
  };
  load();
}, []);
```

---

## Organisation des fichiers

### Règle de proximité

Placer le code près de là où il est utilisé. Un composant utilisé dans une seule page peut rester dans le dossier de cette page. Les composants partagés entre plusieurs pages vont dans `src/components/`.

### Un fichier = une responsabilité

- `src/api/articles.ts` → uniquement les appels API liés aux articles
- `src/types/index.ts` → uniquement les types partagés
- `src/contexts/AuthContext.tsx` → uniquement la gestion de l'authentification

---

## Ce qu'il faut éviter (dette technique existante)

| Problème | Localisation | Clean Code principle |
|----------|-------------|---------------------|
| `dangerouslySetInnerHTML` sans sanitisation | `ArticlePage.tsx` | Sécurité avant tout |
| `require_admin()` dupliqué | `article_handler.rs`, `tag_handler.rs` | DRY |
| Logique métier dans les handlers | tous les handlers | SRP |
| `AdminPage.tsx` monolithique | `pages/admin/AdminPage.tsx` | Fonctions courtes |
| Gestion d'erreur inconsistante | composants frontend | Cohérence |
| Absence de tests | tout le projet | Code vérifiable |
