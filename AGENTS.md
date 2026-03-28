# Agent Guide — BloggY

Ce fichier est le **point d'entrée** pour tout agent AI travaillant sur ce projet.
Il référence la documentation détaillée dans le dossier `agents/`.

---

## Qu'est-ce que BloggY ?

BloggY est un **blog full-stack** avec :
- Backend **Rust/Axum** — API REST avec authentification JWT
- Frontend **React/TypeScript** — SPA avec React Router
- Base de données **PostgreSQL 16**
- Déploiement **Docker Compose** (3 services : frontend, backend, db)

Chemin du code source : `blog/`

---

## Documentation disponible

| Fichier | Contenu |
|---------|---------|
| [`agents/structure.md`](agents/structure.md) | Arborescence complète du projet, règles de navigation |
| [`agents/technologies.md`](agents/technologies.md) | Stack technique, versions, ports |
| [`agents/architecture.md`](agents/architecture.md) | Patterns architecturaux, flux de données, schéma DB |
| [`agents/configuration.md`](agents/configuration.md) | Variables d'environnement, Docker, Nginx, Vite, Cargo |
| [`agents/api.md`](agents/api.md) | Tous les endpoints REST, auth JWT, format des erreurs |
| [`agents/database.md`](agents/database.md) | Schéma SQL, repositories, patterns de requêtes |
| [`agents/conventions.md`](agents/conventions.md) | Conventions de nommage et de code (inspirées Clean Code) |
| [`agents/getting-started.md`](agents/getting-started.md) | Comment lancer, builder, créer un admin |
| [`agents/known-issues.md`](agents/known-issues.md) | Problèmes connus, dette technique, améliorations prioritaires |

---

## Guide de navigation rapide

### "Je veux modifier un endpoint API"
→ Handler : `blog/backend/src/handlers/<entité>_handler.rs`
→ Route : `blog/backend/src/routes/mod.rs`
→ Documentation : [`agents/api.md`](agents/api.md)

### "Je veux modifier une requête SQL"
→ `blog/backend/src/repositories/<entité>_repository.rs`
→ Documentation : [`agents/database.md`](agents/database.md)

### "Je veux modifier l'interface utilisateur"
→ Pages : `blog/frontend/src/pages/`
→ Composants : `blog/frontend/src/components/`
→ Appels API : `blog/frontend/src/api/`

### "Je veux modifier la configuration"
→ Variables d'env : `blog/.env` (ou `.env.example`)
→ Docker : `blog/docker-compose.yml`
→ Documentation : [`agents/configuration.md`](agents/configuration.md)

### "Je veux ajouter une nouvelle entité"
Ordre de création :
1. Migration SQL dans `blog/migrations/`
2. Model Rust dans `blog/backend/src/models/`
3. Repository Rust dans `blog/backend/src/repositories/`
4. Handler Rust dans `blog/backend/src/handlers/`
5. Route dans `blog/backend/src/routes/mod.rs`
6. Type TypeScript dans `blog/frontend/src/types/index.ts`
7. Fonctions API dans `blog/frontend/src/api/`
8. Composants/pages React

---

## ⚠️ Règle impérative : mise à jour de la documentation

> **OBLIGATOIRE** — À chaque fois qu'un agent effectue une modification dans le projet (ajout, suppression, déplacement ou renommage de fichiers, changement d'architecture, modification d'un endpoint, ajout d'une entité, etc.), il **DOIT** mettre à jour la documentation correspondante. Cela inclut :
>
> - **`AGENTS.md`** (ce fichier) si la modification impacte la structure générale, les conventions ou les points d'attention.
> - **Les fichiers du dossier `agents/`** (`structure.md`, `api.md`, `database.md`, `architecture.md`, etc.) si la modification concerne leur périmètre respectif.
>
> La documentation n'est utile que si elle reflète l'état réel du code. Une modification sans mise à jour de la documentation est considérée comme **incomplète**. L'agent doit traiter la mise à jour documentaire comme une étape à part entière de chaque tâche, au même titre que le code lui-même.

---

## Points d'attention

> **Sécurité** — `ArticlePage.tsx` utilise `dangerouslySetInnerHTML` sans sanitisation. Tout contenu HTML dans les articles est un risque XSS potentiel.

> **Performance** — `ArticleRepository::find_all_published()` a un problème N+1 : une requête supplémentaire est faite par article pour récupérer ses tags. À corriger avant mise à l'échelle.

> **Tests** — Le projet n'a **aucun test**. Toute modification doit être vérifiée manuellement.

> **SQLx offline** — Si la base de données n'est pas accessible lors du build Rust, utiliser `SQLX_OFFLINE=true` et s'assurer que le cache `.sqlx/` est à jour.

---

## Conventions essentielles

- **Rust** : `snake_case` pour tout sauf types (`PascalCase`) et constantes (`SCREAMING_SNAKE_CASE`)
- **TypeScript/React** : composants en `PascalCase`, fonctions en `camelCase`
- **Erreurs Rust** : toujours utiliser `AppError` et `AppResult<T>`
- **Appels API frontend** : toujours passer par `src/api/`, jamais Axios directement
- **Requêtes SQL** : toujours utiliser des paramètres liés (`$1`, `$2`), jamais de concaténation

Pour les conventions détaillées : [`agents/conventions.md`](agents/conventions.md)
