import { useEffect, useState } from "react";
import { useParams } from "react-router-dom";
import { articlesApi } from "../api/articles";
import { CommentSection } from "../components/comments/CommentSection";
import type { Article } from "../types";

export function ArticlePage() {
  const { slug } = useParams<{ slug: string }>();
  const [article, setArticle] = useState<Article | null>(null);
  const [isLoading, setIsLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);

  useEffect(() => {
    if (slug) loadArticle(slug);
  }, [slug]);

  async function loadArticle(articleSlug: string) {
    try {
      const { data } = await articlesApi.getBySlug(articleSlug);
      setArticle(data);
    } catch {
      setError("Article introuvable.");
    } finally {
      setIsLoading(false);
    }
  }

  if (isLoading) return <p style={{ textAlign: "center", padding: "4rem" }}>Chargement…</p>;
  if (error || !article) return <p style={{ textAlign: "center", padding: "4rem", color: "#fc8181" }}>{error ?? "Article introuvable."}</p>;

  return (
    <main style={styles.main}>
      <ArticleHeader article={article} />
      <div
        style={styles.content}
        dangerouslySetInnerHTML={{ __html: article.content }}
      />
      <CommentSection articleId={article.id} />
    </main>
  );
}

function ArticleHeader({ article }: { article: Article }) {
  const formattedDate = new Date(article.created_at).toLocaleDateString("fr-FR", {
    year: "numeric", month: "long", day: "numeric",
  });

  return (
    <header style={styles.header}>
      <h1 style={styles.title}>{article.title}</h1>
      <div style={styles.meta}>
        <span>Par {article.author_username}</span>
        <span>·</span>
        <time>{formattedDate}</time>
      </div>
      {article.tags.length > 0 && (
        <div style={styles.tags}>
          {article.tags.map((tag) => (
            <span key={tag.id} style={styles.tag}>{tag.name}</span>
          ))}
        </div>
      )}
    </header>
  );
}

const styles: Record<string, React.CSSProperties> = {
  main:    { maxWidth: "800px", margin: "0 auto", padding: "2rem 1rem" },
  header:  { marginBottom: "2rem", paddingBottom: "1.5rem", borderBottom: "1px solid #e2e8f0" },
  title:   { fontSize: "2.5rem", fontWeight: 700, margin: "0 0 0.75rem", color: "#1a202c", lineHeight: 1.2 },
  meta:    { display: "flex", gap: "0.75rem", color: "#718096", marginBottom: "1rem" },
  tags:    { display: "flex", gap: "0.5rem", flexWrap: "wrap" },
  tag:     { background: "#edf2f7", color: "#4a5568", padding: "0.2rem 0.6rem", borderRadius: "999px", fontSize: "0.8rem" },
  content: { color: "#2d3748", lineHeight: 1.8, fontSize: "1.05rem" },
};
