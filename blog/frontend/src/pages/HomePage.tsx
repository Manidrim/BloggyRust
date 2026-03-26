import { useEffect, useState } from "react";
import { articlesApi } from "../api/articles";
import { ArticleList } from "../components/articles/ArticleList";
import type { Article } from "../types";

export function HomePage() {
  const [articles, setArticles] = useState<Article[]>([]);
  const [isLoading, setIsLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);

  useEffect(() => {
    loadArticles();
  }, []);

  async function loadArticles() {
    try {
      const { data } = await articlesApi.getAll();
      setArticles(data);
    } catch {
      setError("Impossible de charger les articles.");
    } finally {
      setIsLoading(false);
    }
  }

  if (isLoading) return <LoadingMessage />;
  if (error)     return <ErrorMessage message={error} />;

  return (
    <main style={styles.main}>
      <h1 style={styles.heading}>Articles</h1>
      <ArticleList articles={articles} />
    </main>
  );
}

function LoadingMessage() {
  return <p style={{ textAlign: "center", padding: "4rem", color: "#718096" }}>Chargement…</p>;
}

function ErrorMessage({ message }: { message: string }) {
  return <p style={{ textAlign: "center", padding: "4rem", color: "#fc8181" }}>{message}</p>;
}

const styles: Record<string, React.CSSProperties> = {
  main:    { maxWidth: "800px", margin: "0 auto", padding: "2rem 1rem" },
  heading: { fontSize: "2rem", fontWeight: 700, marginBottom: "2rem", color: "#1a202c" },
};
