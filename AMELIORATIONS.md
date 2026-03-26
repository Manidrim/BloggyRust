# Propositions d'Améliorations - BloggY

*Analyse du 26 mars 2026*

---

## Priorité CRITIQUE (Sécurité)

### 1. Corriger la configuration CORS trop permissive
**Fichier** : `backend/src/main.rs`
Le backend accepte toutes les origines (`.allow_origin(Any)`), toutes les méthodes et tous les headers. C'est une faille de sécurité majeure en production. Il faut configurer une liste blanche d'origines autorisées depuis les variables d'environnement.

### 2. Sécuriser les secrets Docker
**Fichier** : `docker-compose.yml`
Les secrets (JWT_SECRET, POSTGRES_PASSWORD) sont en dur dans le fichier. Il faut utiliser un `.env` avec un `.env.example` versionné (sans valeurs réelles) et retirer l'exposition publique du port PostgreSQL (5432).

### 3. Sanitiser le HTML des articles (XSS)
**Fichier** : `frontend/src/pages/ArticlePage.tsx`
L'utilisation de `dangerouslySetInnerHTML` sans sanitisation expose le site aux attaques XSS. Il faut intégrer `DOMPurify` pour nettoyer le contenu avant affichage.

### 4. Ajouter les security headers Nginx
**Fichiers** : `frontend/nginx.conf`, `nginx/nginx.conf`
Il manque les headers de sécurité essentiels : X-Content-Type-Options, X-Frame-Options, Content-Security-Policy, Referrer-Policy.

---

## Priorité HAUTE (Clean Code)

### 5. Éliminer la duplication de `require_admin()`
**Fichiers** : `backend/src/handlers/article_handler.rs`, `backend/src/handlers/tag_handler.rs`
La fonction `require_admin()` est dupliquée dans deux fichiers. Elle doit être déplacée dans `middleware/auth.rs` (violation du principe DRY).

### 6. Ajouter une couche Service entre Handlers et Repositories
**Backend** : Le code mélange validation, orchestration et accès BD dans les handlers. Il faut créer un dossier `services/` avec `ArticleService`, `AuthService`, etc., pour respecter le principe de responsabilité unique (SRP).

### 7. Corriger le problème N+1 queries sur les articles
**Fichier** : `backend/src/repositories/article_repository.rs`
`find_all_published()` fait une requête par article pour récupérer les tags. Il faut utiliser un JOIN ou une requête batch.

### 8. Ajouter la gestion d'erreur systématique côté frontend
**Fichiers** : `AdminPage.tsx`, `CommentSection.tsx`, `ArticleEditorPage.tsx`
Plusieurs appels API n'ont aucun try-catch. Les suppressions de commentaires et d'articles échouent silencieusement. Il faut un pattern cohérent de gestion d'erreur.

### 9. Créer un fichier `.gitignore`
**Racine du projet** : Absent. Les dossiers `target/`, `node_modules/`, fichiers `.env`, logs et secrets risquent d'être versionnés.

---

## Priorité MOYENNE (Architecture & Qualité)

### 10. Ajouter la pagination des articles
**Backend + Frontend** : `find_all_published()` retourne TOUS les articles sans limite. Avec 1000+ articles, les performances seront catastrophiques. Il faut ajouter des paramètres `page` et `limit` côté API et un composant de pagination côté frontend.

### 11. Ajouter des index à la base de données
**Fichiers** : `migrations/*.sql`
Il manque des index sur `users.email`, `users.username`, `articles.slug`, et un index composite sur `articles(published, created_at DESC)`.

### 12. Extraire les styles répétés dans un thème partagé
**Frontend** : Les mêmes couleurs, paddings et patterns flexbox sont copiés dans chaque composant. Il faut créer `constants/theme.ts` et un utilitaire `formatDate()` (dupliqué dans 3 fichiers).

### 13. Ajouter des tests (aucun test n'existe)
**Backend + Frontend** : Zéro test dans tout le projet. C'est la violation Clean Code la plus critique. Commencer par les tests unitaires des repositories (Rust) et les tests de composants (Vitest + React Testing Library).

### 14. Décomposer `AdminPage.tsx` en sous-composants
**Fichier** : `frontend/src/pages/admin/AdminPage.tsx`
Ce composant gère articles ET tags dans un seul fichier. Il faut extraire `ArticlesTable` et `TagManager`.

### 15. Exécuter le backend avec un utilisateur non-root dans Docker
**Fichiers** : `backend/Dockerfile`, `frontend/Dockerfile`
Les deux Dockerfiles exécutent les processus en tant que root, ce qui est un risque d'escalade de privilèges.

---

## Priorité BASSE (Fonctionnalités)

### 16. Ajouter la recherche d'articles
Pas d'endpoint de recherche par titre ou contenu. Implémenter une recherche full-text PostgreSQL.

### 17. Ajouter le filtrage par tags
Les tags sont affichés mais ne servent à rien côté navigation. Ajouter un filtre cliquable.

### 18. Implémenter un flux RSS/Atom
Indispensable pour un blog. Permet l'abonnement aux nouveaux articles.

### 19. Ajouter les commentaires imbriqués (threading)
Les commentaires sont linéaires, sans possibilité de répondre à un commentaire spécifique.

### 20. Ajouter un système de brouillons et publication programmée
Les articles sont soit publiés soit non publiés, sans notion de date de publication future.

---

## Suggestion du jour

**Tâche recommandée : #5 - Éliminer la duplication de `require_admin()`**

C'est un refactoring Clean Code précis, rapide et impactant. Il touche au principe DRY (chapitre fondamental du livre Clean Code) et améliore la maintenabilité sans risque de régression. Une fois fait, cela ouvre naturellement la porte à la tâche #6 (couche Service).
