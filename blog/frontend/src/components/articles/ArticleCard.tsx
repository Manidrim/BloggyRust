import { Link } from "react-router-dom";
import type { Article } from "../../types";

interface ArticleCardProps {
  article: Article;
}

export function ArticleCard({ article }: ArticleCardProps) {
  const formattedDate = new Date(article.created_at).toLocaleDateString("fr-FR", {
    year: "numeric", month: "long", day: "numeric",
  });

  return (
    <article style={styles.card}>
      <Link to={`/articles/${article.slug}`} style={styles.titleLink}>
        <h2 style={styles.title}>{article.title}</h2>
      </Link>

      <div style={styles.meta}>
        <span>{article.author_username}</span>
        <span style={styles.dot}>·</span>
        <time>{formattedDate}</time>
      </div>

      {article.excerpt && <p style={styles.excerpt}>{article.excerpt}</p>}

      {article.tags.length > 0 && (
        <div style={styles.tags}>
          {article.tags.map((tag) => (
            <span key={tag.id} style={styles.tag}>{tag.name}</span>
          ))}
        </div>
      )}

      <Link to={`/articles/${article.slug}`} style={styles.readMore}>
        Lire l'article →
      </Link>
    </article>
  );
}

const styles: Record<string, React.CSSProperties> = {
  card:      { padding: "1.5rem", border: "1px solid #e2e8f0", borderRadius: "8px", background: "#fff" },
  titleLink: { textDecoration: "none", color: "inherit" },
  title:     { fontSize: "1.4rem", fontWeight: 700, margin: "0 0 0.5rem", color: "#1a202c" },
  meta:      { display: "flex", gap: "0.5rem", color: "#718096", fontSize: "0.875rem", marginBottom: "0.75rem" },
  dot:       { color: "#cbd5e0" },
  excerpt:   { color: "#4a5568", lineHeight: 1.6, marginBottom: "1rem" },
  tags:      { display: "flex", gap: "0.5rem", flexWrap: "wrap", marginBottom: "1rem" },
  tag:       { background: "#edf2f7", color: "#4a5568", padding: "0.2rem 0.6rem", borderRadius: "999px", fontSize: "0.75rem" },
  readMore:  { color: "#4299e1", textDecoration: "none", fontSize: "0.9rem", fontWeight: 500 },
};
