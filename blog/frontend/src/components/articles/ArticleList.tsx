import type { Article } from "../../types";
import { ArticleCard } from "./ArticleCard";

interface ArticleListProps {
  articles: Article[];
}

export function ArticleList({ articles }: ArticleListProps) {
  if (articles.length === 0) {
    return <p style={{ color: "#718096", textAlign: "center", padding: "3rem" }}>Aucun article publié pour l'instant.</p>;
  }

  return (
    <div style={styles.list}>
      {articles.map((article) => (
        <ArticleCard key={article.id} article={article} />
      ))}
    </div>
  );
}

const styles: Record<string, React.CSSProperties> = {
  list: { display: "flex", flexDirection: "column", gap: "1.5rem" },
};
