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
  // Verify the correct number of cards
  const cards = screen.getAllByRole("article");
  expect(cards).toHaveLength(3);
  // Verify each article title is rendered
  expect(screen.getByText("Article A")).toBeInTheDocument();
  expect(screen.getByText("Article B")).toBeInTheDocument();
  expect(screen.getByText("Article C")).toBeInTheDocument();
});

test("n'affiche pas le message vide quand il y a des articles", () => {
  renderList([makeArticle("1", "Article A")]);
  expect(screen.queryByText(/aucun article publié/i)).not.toBeInTheDocument();
});
