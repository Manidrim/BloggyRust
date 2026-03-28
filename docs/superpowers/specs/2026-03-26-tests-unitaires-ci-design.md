# Design : Tests unitaires & CI — BloggY

**Date :** 2026-03-26
**Scope :** Tests unitaires backend (Rust) + frontend (React/TS) + workflows GitHub Actions

---

## Contexte

Le projet BloggY est un blog full-stack sans aucun test ni workflow CI. Ce design couvre la mise en place des tests unitaires et de l'automatisation CI sur GitHub.

**Hors scope :** tests d'intégration (nécessitent PostgreSQL), tests e2e, tests des repositories SQL. À traiter dans un second temps.

---

## 1. Tests unitaires Backend (Rust)

### Principe
Les tests Rust s'écrivent dans un module `#[cfg(test)]` en bas de chaque fichier source. Aucune dépendance supplémentaire requise.

### Fichiers modifiés et cas testés

**`blog/backend/src/handlers/auth_handler.rs`**

| Fonction | Cas testés |
|---|---|
| `validate_register_payload` | username vide → BadRequest, password < 8 chars → BadRequest, email sans `@` → BadRequest, payload valide → Ok |
| `hash_password` + `verify_password` | round-trip valide (hash puis vérifie), mauvais mot de passe → Unauthorized |

**`blog/backend/src/handlers/article_handler.rs`**

| Fonction | Cas testés |
|---|---|
| `slugify` | titre normal, espaces multiples, caractères spéciaux remplacés par `-`, doubles tirets fusionnés, chiffres conservés |
| `validate_create_payload` | titre vide → BadRequest, content vide → BadRequest, payload valide → Ok |
| `require_admin` | user admin → Ok, user non-admin → Forbidden |

**`blog/backend/src/middleware/auth.rs`**

| Fonction | Cas testés |
|---|---|
| `create_jwt` | encode puis décode → claims correspondent (sub, username, is_admin) |

### Accès aux fonctions privées
Les fonctions testées sont `fn` (privées). Elles sont accessibles dans `#[cfg(test)]` via `use super::*`.

---

## 2. Tests unitaires Frontend (React/TypeScript)

### Nouvelles dépendances

```json
"devDependencies" à ajouter:
  "vitest": "^1.x",
  "jsdom": "^24.x",
  "@testing-library/react": "^14.x",
  "@testing-library/jest-dom": "^6.x",
  "@testing-library/user-event": "^14.x"
```

### Configuration Vite

`vite.config.ts` — ajout d'un bloc `test` :
```ts
test: {
  environment: 'jsdom',
  setupFiles: ['./src/setupTests.ts'],
  globals: true,
}
```

`src/setupTests.ts` (nouveau fichier) :
```ts
import '@testing-library/jest-dom'
```

### Script npm

Ajout dans `package.json` :
```json
"test": "vitest"
```

### Tests à créer dans `blog/frontend/src/__tests__/`

| Fichier de test | Ce qui est testé |
|---|---|
| `components/ArticleCard.test.tsx` | Rendu titre, extrait, lien slug vers l'article |
| `components/Header.test.tsx` | Logo présent, liens nav, bouton "Login" si non-auth, "Logout" si auth |
| `components/ArticleList.test.tsx` | Liste vide → message explicite, N articles → N cards rendues |
| `contexts/AuthContext.test.tsx` | État initial non-authentifié, login stocke token dans context, logout vide l'état |

---

## 3. GitHub Actions — Deux workflows séparés

### `.github/workflows/ci-backend.yml`

- **Trigger :** `pull_request`, path filter `blog/backend/**`
- **Runner :** `ubuntu-latest`
- **Steps :** checkout → Rust stable (via `dtolnay/rust-toolchain`) → `cargo test` dans `blog/backend/`
- **Pas de service DB** (tests unitaires uniquement)

### `.github/workflows/ci-frontend.yml`

- **Trigger :** `pull_request`, path filter `blog/frontend/**`
- **Runner :** `ubuntu-latest`
- **Steps :** checkout → Node 20 (via `actions/setup-node`) → `npm ci` → `npm test -- --run`
- `--run` force Vitest en mode non-interactif (pas de watch)

### Comportement
Les deux workflows sont indépendants et s'exécutent en parallèle. Un workflow ne se déclenche que si des fichiers de son domaine sont modifiés dans la PR.

---

## Fichiers créés / modifiés

### Créés
- `.github/workflows/ci-backend.yml`
- `.github/workflows/ci-frontend.yml`
- `blog/frontend/src/setupTests.ts`
- `blog/frontend/src/__tests__/components/ArticleCard.test.tsx`
- `blog/frontend/src/__tests__/components/Header.test.tsx`
- `blog/frontend/src/__tests__/components/ArticleList.test.tsx`
- `blog/frontend/src/__tests__/contexts/AuthContext.test.tsx`

### Modifiés
- `blog/backend/src/handlers/auth_handler.rs` — ajout `#[cfg(test)]`
- `blog/backend/src/handlers/article_handler.rs` — ajout `#[cfg(test)]`
- `blog/backend/src/middleware/auth.rs` — ajout `#[cfg(test)]`
- `blog/frontend/package.json` — ajout dépendances test + script
- `blog/frontend/vite.config.ts` — ajout bloc `test`
