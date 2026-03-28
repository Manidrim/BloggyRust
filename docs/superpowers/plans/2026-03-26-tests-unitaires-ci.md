# Tests unitaires & CI Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Ajouter des tests unitaires au backend Rust et au frontend React, puis configurer deux workflows GitHub Actions qui les exécutent sur chaque PR.

**Architecture:** Tests Rust dans des modules `#[cfg(test)]` en bas des fichiers existants (fonctions pures uniquement, sans DB). Tests frontend avec Vitest + React Testing Library. Deux workflows CI indépendants avec path filters.

**Tech Stack:** Rust (built-in test runner), Vitest 1.x, @testing-library/react 14.x, @testing-library/jest-dom 6.x, jsdom, GitHub Actions.

---

## Fichiers créés / modifiés

| Action | Fichier |
|---|---|
| Modify | `blog/backend/src/handlers/auth_handler.rs` |
| Modify | `blog/backend/src/handlers/article_handler.rs` |
| Modify | `blog/backend/src/middleware/auth.rs` |
| Modify | `blog/frontend/package.json` |
| Modify | `blog/frontend/vite.config.ts` |
| Create | `blog/frontend/src/setupTests.ts` |
| Create | `blog/frontend/src/__tests__/components/ArticleCard.test.tsx` |
| Create | `blog/frontend/src/__tests__/components/Header.test.tsx` |
| Create | `blog/frontend/src/__tests__/components/ArticleList.test.tsx` |
| Create | `blog/frontend/src/__tests__/contexts/AuthContext.test.tsx` |
| Create | `.github/workflows/ci-backend.yml` |
| Create | `.github/workflows/ci-frontend.yml` |

---

### Task 1 : Tests unitaires — `auth_handler.rs` (validate + password)

**Files:**
- Modify: `blog/backend/src/handlers/auth_handler.rs`

- [ ] **Step 1 : Ajouter le module de test à la fin du fichier**

Ajouter à la fin de `blog/backend/src/handlers/auth_handler.rs` :

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::user::RegisterRequest;

    fn valid_payload() -> RegisterRequest {
        RegisterRequest {
            username: "alice".to_string(),
            email: "alice@example.com".to_string(),
            password: "securepassword".to_string(),
        }
    }

    // --- validate_register_payload ---

    #[test]
    fn validate_register_rejects_empty_username() {
        let payload = RegisterRequest {
            username: "   ".to_string(),
            ..valid_payload()
        };
        let result = validate_register_payload(&payload);
        assert!(matches!(result, Err(AppError::BadRequest(_))));
    }

    #[test]
    fn validate_register_rejects_short_password() {
        let payload = RegisterRequest {
            password: "short".to_string(),
            ..valid_payload()
        };
        let result = validate_register_payload(&payload);
        assert!(matches!(result, Err(AppError::BadRequest(_))));
    }

    #[test]
    fn validate_register_rejects_invalid_email() {
        let payload = RegisterRequest {
            email: "not-an-email".to_string(),
            ..valid_payload()
        };
        let result = validate_register_payload(&payload);
        assert!(matches!(result, Err(AppError::BadRequest(_))));
    }

    #[test]
    fn validate_register_accepts_valid_payload() {
        let result = validate_register_payload(&valid_payload());
        assert!(result.is_ok());
    }

    // --- hash_password / verify_password ---

    #[test]
    fn password_round_trip_succeeds() {
        let password = "my_secure_password";
        let hash = hash_password(password).expect("hashing should succeed");
        let result = verify_password(password, &hash);
        assert!(result.is_ok());
    }

    #[test]
    fn wrong_password_returns_unauthorized() {
        let hash = hash_password("correct_password").expect("hashing should succeed");
        let result = verify_password("wrong_password", &hash);
        assert!(matches!(result, Err(AppError::Unauthorized)));
    }
}
```

- [ ] **Step 2 : Lancer les tests et vérifier qu'ils passent**

```bash
cd blog/backend && cargo test auth_handler
```

Sortie attendue : `6 tests` passants, aucun échec.

- [ ] **Step 3 : Commit**

```bash
git add blog/backend/src/handlers/auth_handler.rs
git commit -m "test(backend): add unit tests for auth_handler"
```

---

### Task 2 : Tests unitaires — `article_handler.rs` (slugify + validate + require_admin)

**Files:**
- Modify: `blog/backend/src/handlers/article_handler.rs`

- [ ] **Step 1 : Ajouter le module de test à la fin du fichier**

Ajouter à la fin de `blog/backend/src/handlers/article_handler.rs` :

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        middleware::auth::AuthenticatedUser,
        models::article::CreateArticleRequest,
    };
    use uuid::Uuid;

    fn admin_user() -> AuthenticatedUser {
        AuthenticatedUser {
            id: Uuid::new_v4(),
            username: "admin".to_string(),
            is_admin: true,
        }
    }

    fn regular_user() -> AuthenticatedUser {
        AuthenticatedUser {
            id: Uuid::new_v4(),
            username: "user".to_string(),
            is_admin: false,
        }
    }

    // --- slugify ---

    #[test]
    fn slugify_lowercases_and_replaces_spaces() {
        assert_eq!(slugify("Hello World"), "hello-world");
    }

    #[test]
    fn slugify_collapses_multiple_separators() {
        assert_eq!(slugify("Hello  --  World"), "hello-world");
    }

    #[test]
    fn slugify_removes_special_characters() {
        // é est alphanumeric en Rust (Unicode), donc conservé tel quel
        assert_eq!(slugify("C'est l'été !"), "c-est-l-été");
    }

    #[test]
    fn slugify_preserves_numbers() {
        assert_eq!(slugify("Article 42"), "article-42");
    }

    #[test]
    fn slugify_empty_string_returns_empty() {
        assert_eq!(slugify(""), "");
    }

    // --- validate_create_payload ---

    #[test]
    fn validate_create_rejects_empty_title() {
        let payload = CreateArticleRequest {
            title: "   ".to_string(),
            content: "some content".to_string(),
            excerpt: None,
            published: None,
            tag_ids: None,
        };
        let result = validate_create_payload(&payload);
        assert!(matches!(result, Err(AppError::BadRequest(_))));
    }

    #[test]
    fn validate_create_rejects_empty_content() {
        let payload = CreateArticleRequest {
            title: "My Title".to_string(),
            content: "   ".to_string(),
            excerpt: None,
            published: None,
            tag_ids: None,
        };
        let result = validate_create_payload(&payload);
        assert!(matches!(result, Err(AppError::BadRequest(_))));
    }

    #[test]
    fn validate_create_accepts_valid_payload() {
        let payload = CreateArticleRequest {
            title: "My Title".to_string(),
            content: "Some content".to_string(),
            excerpt: None,
            published: None,
            tag_ids: None,
        };
        let result = validate_create_payload(&payload);
        assert!(result.is_ok());
    }

    // --- require_admin ---

    #[test]
    fn require_admin_allows_admin_user() {
        assert!(require_admin(&admin_user()).is_ok());
    }

    #[test]
    fn require_admin_rejects_regular_user() {
        let result = require_admin(&regular_user());
        assert!(matches!(result, Err(AppError::Forbidden)));
    }
}
```

- [ ] **Step 2 : Lancer les tests et vérifier qu'ils passent**

```bash
cd blog/backend && cargo test article_handler
```

Sortie attendue : `8 tests` passants, aucun échec.

- [ ] **Step 3 : Commit**

```bash
git add blog/backend/src/handlers/article_handler.rs
git commit -m "test(backend): add unit tests for article_handler"
```

---

### Task 3 : Tests unitaires — `middleware/auth.rs` (create_jwt round-trip)

**Files:**
- Modify: `blog/backend/src/middleware/auth.rs`

- [ ] **Step 1 : Ajouter le module de test à la fin du fichier**

Ajouter à la fin de `blog/backend/src/middleware/auth.rs` :

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use jsonwebtoken::{decode, DecodingKey, Validation};

    #[test]
    fn create_jwt_produces_decodable_token_with_correct_claims() {
        let user_id = Uuid::new_v4();
        let secret = "test_secret_key";

        let token = create_jwt(user_id, "alice", true, secret)
            .expect("JWT creation should succeed");

        let key = DecodingKey::from_secret(secret.as_bytes());
        let decoded = decode::<JwtClaims>(&token, &key, &Validation::default())
            .expect("JWT decoding should succeed");

        assert_eq!(decoded.claims.sub, user_id);
        assert_eq!(decoded.claims.username, "alice");
        assert!(decoded.claims.is_admin);
    }

    #[test]
    fn create_jwt_with_different_secret_fails_to_decode() {
        let user_id = Uuid::new_v4();
        let token = create_jwt(user_id, "bob", false, "secret_a")
            .expect("JWT creation should succeed");

        let key = DecodingKey::from_secret("secret_b".as_bytes());
        let result = decode::<JwtClaims>(&token, &key, &Validation::default());

        assert!(result.is_err());
    }

    #[test]
    fn create_jwt_non_admin_claim_is_preserved() {
        let user_id = Uuid::new_v4();
        let secret = "test_secret";
        let token = create_jwt(user_id, "bob", false, secret)
            .expect("JWT creation should succeed");

        let key = DecodingKey::from_secret(secret.as_bytes());
        let decoded = decode::<JwtClaims>(&token, &key, &Validation::default())
            .expect("JWT decoding should succeed");

        assert!(!decoded.claims.is_admin);
        assert_eq!(decoded.claims.username, "bob");
    }
}
```

- [ ] **Step 2 : Lancer les tests et vérifier qu'ils passent**

```bash
cd blog/backend && cargo test middleware
```

Sortie attendue : `3 tests` passants, aucun échec.

- [ ] **Step 3 : Lancer tous les tests backend**

```bash
cd blog/backend && cargo test
```

Sortie attendue : `17 tests` passants, aucun échec.

- [ ] **Step 4 : Commit**

```bash
git add blog/backend/src/middleware/auth.rs
git commit -m "test(backend): add unit tests for JWT middleware"
```

---

### Task 4 : Setup environnement de test Frontend

**Files:**
- Modify: `blog/frontend/package.json`
- Modify: `blog/frontend/vite.config.ts`
- Create: `blog/frontend/src/setupTests.ts`

- [ ] **Step 1 : Installer les dépendances de test**

```bash
cd blog/frontend && npm install --save-dev vitest jsdom @testing-library/react @testing-library/jest-dom @testing-library/user-event
```

- [ ] **Step 2 : Ajouter le script test dans `package.json`**

Dans `blog/frontend/package.json`, ajouter `"test": "vitest"` dans `scripts` :

```json
{
  "name": "blog-frontend",
  "version": "0.1.0",
  "private": true,
  "scripts": {
    "dev": "vite",
    "build": "tsc && vite build",
    "preview": "vite preview",
    "test": "vitest"
  },
  "dependencies": {
    "react": "^18.2.0",
    "react-dom": "^18.2.0",
    "react-router-dom": "^6.22.0",
    "axios": "^1.6.7"
  },
  "devDependencies": {
    "@testing-library/jest-dom": "^6.4.2",
    "@testing-library/react": "^14.2.1",
    "@testing-library/user-event": "^14.5.2",
    "@types/react": "^18.2.55",
    "@types/react-dom": "^18.2.19",
    "@vitejs/plugin-react": "^4.2.1",
    "jsdom": "^24.0.0",
    "typescript": "^5.3.3",
    "vite": "^5.1.1",
    "vitest": "^1.3.1"
  }
}
```

- [ ] **Step 3 : Mettre à jour `vite.config.ts` pour inclure la config de test**

Remplacer le contenu de `blog/frontend/vite.config.ts` par :

```ts
import { defineConfig } from "vite";
import react from "@vitejs/plugin-react";

export default defineConfig({
  plugins: [react()],
  server: {
    proxy: {
      "/api": {
        target: process.env.VITE_API_URL ?? "http://localhost:8080",
        changeOrigin: true,
        rewrite: (path) => path.replace(/^\/api/, ""),
      },
    },
  },
  test: {
    environment: "jsdom",
    setupFiles: ["./src/setupTests.ts"],
    globals: true,
  },
});
```

- [ ] **Step 4 : Créer le fichier de setup des matchers**

Créer `blog/frontend/src/setupTests.ts` :

```ts
import "@testing-library/jest-dom";
```

- [ ] **Step 5 : Vérifier que Vitest démarre sans erreur**

```bash
cd blog/frontend && npm test -- --run
```

Sortie attendue : `No test files found` (normal, les tests n'existent pas encore).

- [ ] **Step 6 : Commit**

```bash
git add blog/frontend/package.json blog/frontend/vite.config.ts blog/frontend/src/setupTests.ts blog/frontend/package-lock.json
git commit -m "test(frontend): setup Vitest + React Testing Library"
```

---

### Task 5 : Tests `ArticleCard`

**Files:**
- Create: `blog/frontend/src/__tests__/components/ArticleCard.test.tsx`

- [ ] **Step 1 : Créer le fichier de test**

Créer `blog/frontend/src/__tests__/components/ArticleCard.test.tsx` :

```tsx
import { render, screen } from "@testing-library/react";
import { MemoryRouter } from "react-router-dom";
import { ArticleCard } from "../../components/articles/ArticleCard";
import type { Article } from "../../types";

const mockArticle: Article = {
  id: "1",
  title: "Mon premier article",
  slug: "mon-premier-article",
  content: "Contenu complet",
  excerpt: "Un bref résumé de l'article",
  author_id: "42",
  author_username: "alice",
  published: true,
  tags: [{ id: "t1", name: "Rust", slug: "rust", created_at: "2024-01-01T00:00:00Z" }],
  created_at: "2024-01-15T00:00:00Z",
  updated_at: "2024-01-15T00:00:00Z",
};

function renderCard(article: Article = mockArticle) {
  return render(
    <MemoryRouter>
      <ArticleCard article={article} />
    </MemoryRouter>
  );
}

test("affiche le titre de l'article", () => {
  renderCard();
  expect(screen.getByText("Mon premier article")).toBeInTheDocument();
});

test("affiche l'extrait quand il est présent", () => {
  renderCard();
  expect(screen.getByText("Un bref résumé de l'article")).toBeInTheDocument();
});

test("n'affiche pas d'extrait quand il est null", () => {
  renderCard({ ...mockArticle, excerpt: null });
  expect(screen.queryByText("Un bref résumé de l'article")).not.toBeInTheDocument();
});

test("affiche le tag de l'article", () => {
  renderCard();
  expect(screen.getByText("Rust")).toBeInTheDocument();
});

test("le lien du titre pointe vers le bon slug", () => {
  renderCard();
  const links = screen.getAllByRole("link");
  const titleLink = links.find((l) => l.getAttribute("href") === "/articles/mon-premier-article");
  expect(titleLink).toBeInTheDocument();
});

test("n'affiche pas de tags quand la liste est vide", () => {
  renderCard({ ...mockArticle, tags: [] });
  expect(screen.queryByText("Rust")).not.toBeInTheDocument();
});
```

- [ ] **Step 2 : Lancer les tests**

```bash
cd blog/frontend && npm test -- --run
```

Sortie attendue : `6 tests` passants.

- [ ] **Step 3 : Commit**

```bash
git add blog/frontend/src/__tests__/components/ArticleCard.test.tsx
git commit -m "test(frontend): add ArticleCard unit tests"
```

---

### Task 6 : Tests `ArticleList`

**Files:**
- Create: `blog/frontend/src/__tests__/components/ArticleList.test.tsx`

- [ ] **Step 1 : Créer le fichier de test**

Créer `blog/frontend/src/__tests__/components/ArticleList.test.tsx` :

```tsx
import { render, screen } from "@testing-library/react";
import { MemoryRouter } from "react-router-dom";
import { ArticleList } from "../../components/articles/ArticleList";
import type { Article } from "../../types";

function makeArticle(id: string, title: string): Article {
  return {
    id,
    title,
    slug: `slug-${id}`,
    content: "content",
    excerpt: null,
    author_id: "a1",
    author_username: "alice",
    published: true,
    tags: [],
    created_at: "2024-01-01T00:00:00Z",
    updated_at: "2024-01-01T00:00:00Z",
  };
}

function renderList(articles: Article[]) {
  return render(
    <MemoryRouter>
      <ArticleList articles={articles} />
    </MemoryRouter>
  );
}

test("affiche un message quand la liste est vide", () => {
  renderList([]);
  expect(screen.getByText(/aucun article publié/i)).toBeInTheDocument();
});

test("affiche autant de cartes qu'il y a d'articles", () => {
  const articles = [
    makeArticle("1", "Article A"),
    makeArticle("2", "Article B"),
    makeArticle("3", "Article C"),
  ];
  renderList(articles);
  expect(screen.getByText("Article A")).toBeInTheDocument();
  expect(screen.getByText("Article B")).toBeInTheDocument();
  expect(screen.getByText("Article C")).toBeInTheDocument();
});

test("n'affiche pas le message vide quand il y a des articles", () => {
  renderList([makeArticle("1", "Article A")]);
  expect(screen.queryByText(/aucun article publié/i)).not.toBeInTheDocument();
});
```

- [ ] **Step 2 : Lancer les tests**

```bash
cd blog/frontend && npm test -- --run
```

Sortie attendue : `9 tests` passants.

- [ ] **Step 3 : Commit**

```bash
git add blog/frontend/src/__tests__/components/ArticleList.test.tsx
git commit -m "test(frontend): add ArticleList unit tests"
```

---

### Task 7 : Tests `Header`

**Files:**
- Create: `blog/frontend/src/__tests__/components/Header.test.tsx`

- [ ] **Step 1 : Créer le fichier de test**

Créer `blog/frontend/src/__tests__/components/Header.test.tsx` :

```tsx
import { render, screen } from "@testing-library/react";
import { MemoryRouter } from "react-router-dom";
import { AuthProvider } from "../../contexts/AuthContext";
import { Header } from "../../components/layout/Header";

function renderHeader() {
  return render(
    <MemoryRouter>
      <AuthProvider>
        <Header />
      </AuthProvider>
    </MemoryRouter>
  );
}

beforeEach(() => {
  localStorage.clear();
});

test("affiche le logo du blog", () => {
  renderHeader();
  expect(screen.getByText("Mon Blog")).toBeInTheDocument();
});

test("affiche le lien Articles", () => {
  renderHeader();
  expect(screen.getByRole("link", { name: "Articles" })).toBeInTheDocument();
});

test("affiche le lien Connexion quand non authentifié", () => {
  renderHeader();
  expect(screen.getByRole("link", { name: "Connexion" })).toBeInTheDocument();
});

test("n'affiche pas le bouton Déconnexion quand non authentifié", () => {
  renderHeader();
  expect(screen.queryByRole("button", { name: "Déconnexion" })).not.toBeInTheDocument();
});

test("affiche le bouton Déconnexion quand authentifié", () => {
  const user = { id: "1", username: "alice", is_admin: false };
  localStorage.setItem("user", JSON.stringify(user));
  renderHeader();
  expect(screen.getByRole("button", { name: "Déconnexion" })).toBeInTheDocument();
});

test("affiche le nom d'utilisateur quand authentifié", () => {
  const user = { id: "1", username: "alice", is_admin: false };
  localStorage.setItem("user", JSON.stringify(user));
  renderHeader();
  expect(screen.getByText("alice")).toBeInTheDocument();
});

test("n'affiche pas le lien Admin pour un utilisateur non-admin", () => {
  const user = { id: "1", username: "alice", is_admin: false };
  localStorage.setItem("user", JSON.stringify(user));
  renderHeader();
  expect(screen.queryByRole("link", { name: "Admin" })).not.toBeInTheDocument();
});

test("affiche le lien Admin pour un admin", () => {
  const user = { id: "1", username: "admin", is_admin: true };
  localStorage.setItem("user", JSON.stringify(user));
  renderHeader();
  expect(screen.getByRole("link", { name: "Admin" })).toBeInTheDocument();
});
```

- [ ] **Step 2 : Lancer les tests**

```bash
cd blog/frontend && npm test -- --run
```

Sortie attendue : `17 tests` passants.

- [ ] **Step 3 : Commit**

```bash
git add blog/frontend/src/__tests__/components/Header.test.tsx
git commit -m "test(frontend): add Header unit tests"
```

---

### Task 8 : Tests `AuthContext`

**Files:**
- Create: `blog/frontend/src/__tests__/contexts/AuthContext.test.tsx`

- [ ] **Step 1 : Créer le fichier de test**

Créer `blog/frontend/src/__tests__/contexts/AuthContext.test.tsx` :

```tsx
import { render, screen, act } from "@testing-library/react";
import { vi } from "vitest";
import { AuthProvider, useAuth } from "../../contexts/AuthContext";

// Mock de l'API auth pour éviter les appels réseau
vi.mock("../../api/auth", () => ({
  authApi: {
    login: vi.fn(),
    register: vi.fn(),
  },
}));

import { authApi } from "../../api/auth";

// Composant de test minimal pour exposer le contexte
function TestConsumer() {
  const { user, isLoading } = useAuth();
  if (isLoading) return <div>Loading</div>;
  return <div>{user ? `Logged in as ${user.username}` : "Not logged in"}</div>;
}

function renderWithProvider() {
  return render(
    <AuthProvider>
      <TestConsumer />
    </AuthProvider>
  );
}

beforeEach(() => {
  localStorage.clear();
  vi.clearAllMocks();
});

test("état initial : non authentifié", async () => {
  renderWithProvider();
  expect(await screen.findByText("Not logged in")).toBeInTheDocument();
});

test("restaure la session depuis localStorage au montage", async () => {
  const user = { id: "1", username: "alice", is_admin: false };
  localStorage.setItem("user", JSON.stringify(user));
  renderWithProvider();
  expect(await screen.findByText("Logged in as alice")).toBeInTheDocument();
});

test("login stocke le token et l'utilisateur", async () => {
  const mockUser = { id: "2", username: "bob", is_admin: false };
  vi.mocked(authApi.login).mockResolvedValue({
    data: { token: "fake-token", user: mockUser },
  } as any);

  function TestLogin() {
    const { user, login } = useAuth();
    return (
      <div>
        <span>{user ? `Logged in as ${user.username}` : "Not logged in"}</span>
        <button onClick={() => login("bob@example.com", "password123")}>Login</button>
      </div>
    );
  }

  const { getByRole } = render(
    <AuthProvider>
      <TestLogin />
    </AuthProvider>
  );

  await act(async () => {
    getByRole("button", { name: "Login" }).click();
  });

  expect(screen.getByText("Logged in as bob")).toBeInTheDocument();
  expect(localStorage.getItem("token")).toBe("fake-token");
});

test("logout vide l'état et localStorage", async () => {
  const user = { id: "1", username: "alice", is_admin: false };
  localStorage.setItem("user", JSON.stringify(user));
  localStorage.setItem("token", "some-token");

  function TestLogout() {
    const { user, logout } = useAuth();
    return (
      <div>
        <span>{user ? `Logged in as ${user.username}` : "Not logged in"}</span>
        <button onClick={logout}>Logout</button>
      </div>
    );
  }

  const { getByRole } = render(
    <AuthProvider>
      <TestLogout />
    </AuthProvider>
  );

  expect(await screen.findByText("Logged in as alice")).toBeInTheDocument();

  await act(async () => {
    getByRole("button", { name: "Logout" }).click();
  });

  expect(screen.getByText("Not logged in")).toBeInTheDocument();
  expect(localStorage.getItem("token")).toBeNull();
  expect(localStorage.getItem("user")).toBeNull();
});
```

- [ ] **Step 2 : Lancer tous les tests frontend**

```bash
cd blog/frontend && npm test -- --run
```

Sortie attendue : `21 tests` passants, aucun échec.

- [ ] **Step 3 : Commit**

```bash
git add blog/frontend/src/__tests__/contexts/AuthContext.test.tsx
git commit -m "test(frontend): add AuthContext unit tests"
```

---

### Task 9 : Workflow CI Backend

**Files:**
- Create: `.github/workflows/ci-backend.yml`

- [ ] **Step 1 : Créer le répertoire workflows**

```bash
mkdir -p .github/workflows
```

- [ ] **Step 2 : Créer le fichier workflow**

Créer `.github/workflows/ci-backend.yml` :

```yaml
name: CI — Backend

on:
  pull_request:
    paths:
      - "blog/backend/**"

jobs:
  test:
    name: Rust unit tests
    runs-on: ubuntu-latest

    steps:
      - name: Checkout
        uses: actions/checkout@v4

      - name: Install Rust stable
        uses: dtolnay/rust-toolchain@stable

      - name: Cache Cargo registry
        uses: actions/cache@v4
        with:
          path: |
            ~/.cargo/registry
            ~/.cargo/git
            blog/backend/target
          key: ${{ runner.os }}-cargo-${{ hashFiles('blog/backend/Cargo.lock') }}
          restore-keys: ${{ runner.os }}-cargo-

      - name: Run tests
        working-directory: blog/backend
        env:
          SQLX_OFFLINE: "true"
        run: cargo test
```

- [ ] **Step 3 : Commit**

```bash
git add .github/workflows/ci-backend.yml
git commit -m "ci: add backend unit test workflow"
```

---

### Task 10 : Workflow CI Frontend

**Files:**
- Create: `.github/workflows/ci-frontend.yml`

- [ ] **Step 1 : Créer le fichier workflow**

Créer `.github/workflows/ci-frontend.yml` :

```yaml
name: CI — Frontend

on:
  pull_request:
    paths:
      - "blog/frontend/**"

jobs:
  test:
    name: Vitest unit tests
    runs-on: ubuntu-latest

    steps:
      - name: Checkout
        uses: actions/checkout@v4

      - name: Setup Node.js
        uses: actions/setup-node@v4
        with:
          node-version: "20"
          cache: "npm"
          cache-dependency-path: blog/frontend/package-lock.json

      - name: Install dependencies
        working-directory: blog/frontend
        run: npm ci

      - name: Run tests
        working-directory: blog/frontend
        run: npm test -- --run
```

- [ ] **Step 2 : Commit**

```bash
git add .github/workflows/ci-frontend.yml
git commit -m "ci: add frontend unit test workflow"
```

---

## Vérification finale

- [ ] `cd blog/backend && cargo test` → tous les tests passent
- [ ] `cd blog/frontend && npm test -- --run` → tous les tests passent
- [ ] Les deux fichiers `.github/workflows/*.yml` existent et sont valides YAML
