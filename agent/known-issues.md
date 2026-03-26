# Problèmes connus et améliorations

Ce fichier recense les problèmes identifiés dans le code actuel, classés par priorité.
Source : `AMELIORATIONS.md` à la racine du projet.

---

## Critique (sécurité / données)

### XSS potentiel dans ArticlePage
- **Fichier** : `frontend/src/pages/ArticlePage.tsx`
- **Problème** : `dangerouslySetInnerHTML` utilisé pour afficher le contenu des articles sans sanitisation
- **Risque** : Injection de scripts si du HTML malveillant est stocké en base
- **Correction** : Utiliser DOMPurify côté frontend, ou sanitiser côté backend avant stockage

### CORS permissif
- **Fichier** : `backend/src/main.rs`
- **Problème** : `AllowOrigin::any()` accepte toutes les origines
- **Correction** : Restreindre au domaine de production

### Secrets dans les fichiers de config
- **Fichiers** : `blog/docker-compose.yml`, `blog/.env.example`
- **Problème** : Pas de `.gitignore`, risque de commit du `.env`
- **Correction** : Ajouter `.gitignore` incluant `.env`

---

## Performance

### Problème N+1 sur les articles
- **Fichier** : `backend/src/repositories/article_repository.rs`
- **Problème** : `find_all_published()` exécute 1 requête pour les articles + N requêtes pour les tags de chaque article
- **Correction** : Utiliser `array_agg` ou un JOIN avec agrégation

```sql
-- Correction suggérée
SELECT a.*,
       json_agg(t.*) FILTER (WHERE t.id IS NOT NULL) as tags,
       u.username as author_username
FROM articles a
LEFT JOIN article_tags at ON a.id = at.article_id
LEFT JOIN tags t ON at.tag_id = t.id
JOIN users u ON a.author_id = u.id
WHERE a.published = true
GROUP BY a.id, u.username
```

### Pas de pagination
- **Fichier** : `backend/src/handlers/article_handler.rs`
- **Problème** : `GET /articles` retourne tous les articles sans limite
- **Correction** : Ajouter paramètres `?page=1&per_page=20` avec LIMIT/OFFSET en SQL

---

## Qualité du code

### Absence de couche Service
- **Problème** : La logique métier est directement dans les handlers — violation du SRP
- **Correction** : Ajouter une couche `services/` entre handlers et repositories

### Duplication de `require_admin()`
- **Fichiers** : `article_handler.rs`, `tag_handler.rs`
- **Correction** : Extraire dans `handlers/mod.rs` ou créer un helper partagé

### `AdminPage.tsx` monolithique
- **Fichier** : `frontend/src/pages/admin/AdminPage.tsx`
- **Problème** : Page trop grande, gère articles et tags dans le même composant
- **Correction** : Séparer en `ArticlesAdminPanel.tsx` et `TagsAdminPanel.tsx`

### Gestion d'erreur inconsistante côté frontend
- **Problème** : Certains composants ont try/catch, d'autres non
- **Correction** : Standardiser le pattern de gestion d'erreur dans tous les composants

---

## Tests

### Aucun test dans le projet
- **Problème** : Zero couverture de tests (unitaires, intégration, e2e)
- **Priorité** : Haute — toute modification est risquée sans filet
- **À implémenter** :
  - Tests unitaires Rust pour les repositories (`#[cfg(test)]`)
  - Tests d'intégration Rust pour les handlers avec base de test
  - Tests React avec Vitest + Testing Library
  - Tests e2e avec Playwright ou Cypress

---

## Fonctionnalités manquantes

| Fonctionnalité | Complexité | Impact |
|----------------|------------|--------|
| Pagination des articles | Faible | Haute |
| Commentaires imbriqués | Moyenne | Moyenne |
| Recherche full-text | Moyenne | Haute |
| Flux RSS/Atom | Faible | Moyenne |
| Confirmation email à l'inscription | Moyenne | Moyenne |
| Upload d'images | Haute | Moyenne |
| Rate limiting | Moyenne | Haute (sécurité) |
| Refresh token JWT | Moyenne | Haute (UX) |

---

## Dette technique résumée

```
Priorité 1 (sécurité)    : XSS, CORS, gitignore
Priorité 2 (performance) : N+1 queries, pagination
Priorité 3 (qualité)     : Tests, couche service, duplication
Priorité 4 (features)    : Pagination UI, recherche, RSS
```
