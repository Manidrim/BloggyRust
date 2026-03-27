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
