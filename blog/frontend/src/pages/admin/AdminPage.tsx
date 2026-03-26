import { useEffect, useState } from "react";
import { Link, useNavigate } from "react-router-dom";
import { articlesApi } from "../../api/articles";
import { tagsApi } from "../../api/tags";
import { useAuth } from "../../contexts/AuthContext";
import type { Article, Tag } from "../../types";

export function AdminPage() {
  const { user } = useAuth();
  const navigate = useNavigate();
  const [articles, setArticles] = useState<Article[]>([]);
  const [tags, setTags] = useState<Tag[]>([]);
  const [newTag, setNewTag] = useState({ name: "", slug: "" });

  useEffect(() => {
    if (!user?.is_admin) {
      navigate("/");
      return;
    }
    loadData();
  }, [user]);

  async function loadData() {
    const [articlesRes, tagsRes] = await Promise.all([
      articlesApi.getAll(),
      tagsApi.getAll(),
    ]);
    setArticles(articlesRes.data);
    setTags(tagsRes.data);
  }

  async function handleDeleteArticle(slug: string) {
    if (!confirm("Supprimer cet article ?")) return;
    await articlesApi.delete(slug);
    setArticles((prev) => prev.filter((a) => a.slug !== slug));
  }

  async function handleCreateTag(e: React.FormEvent) {
    e.preventDefault();
    const { data } = await tagsApi.create(newTag.name, newTag.slug);
    setTags((prev) => [...prev, data]);
    setNewTag({ name: "", slug: "" });
  }

  async function handleDeleteTag(id: string) {
    await tagsApi.delete(id);
    setTags((prev) => prev.filter((t) => t.id !== id));
  }

  return (
    <main style={styles.main}>
      <div style={styles.headerRow}>
        <h1 style={styles.heading}>Administration</h1>
        <Link to="/admin/articles/new" style={styles.newBtn}>+ Nouvel article</Link>
      </div>

      <section style={styles.section}>
        <h2 style={styles.subheading}>Articles</h2>
        <table style={styles.table}>
          <thead>
            <tr>
              <th style={styles.th}>Titre</th>
              <th style={styles.th}>Statut</th>
              <th style={styles.th}>Date</th>
              <th style={styles.th}>Actions</th>
            </tr>
          </thead>
          <tbody>
            {articles.map((article) => (
              <tr key={article.id}>
                <td style={styles.td}>{article.title}</td>
                <td style={styles.td}>
                  <span style={{ ...styles.badge, ...(article.published ? styles.badgePublished : styles.badgeDraft) }}>
                    {article.published ? "Publié" : "Brouillon"}
                  </span>
                </td>
                <td style={styles.td}>{new Date(article.created_at).toLocaleDateString("fr-FR")}</td>
                <td style={{ ...styles.td, display: "flex", gap: "0.5rem" }}>
                  <Link to={`/admin/articles/${article.slug}/edit`} style={styles.editBtn}>Modifier</Link>
                  <button onClick={() => handleDeleteArticle(article.slug)} style={styles.deleteBtn}>Supprimer</button>
                </td>
              </tr>
            ))}
          </tbody>
        </table>
      </section>

      <section style={styles.section}>
        <h2 style={styles.subheading}>Tags</h2>
        <form onSubmit={handleCreateTag} style={styles.tagForm}>
          <input
            placeholder="Nom du tag"
            value={newTag.name}
            onChange={(e) => setNewTag((p) => ({ ...p, name: e.target.value }))}
            required
            style={styles.tagInput}
          />
          <input
            placeholder="slug"
            value={newTag.slug}
            onChange={(e) => setNewTag((p) => ({ ...p, slug: e.target.value }))}
            required
            style={styles.tagInput}
          />
          <button type="submit" style={styles.addTagBtn}>Ajouter</button>
        </form>
        <div style={styles.tagList}>
          {tags.map((tag) => (
            <span key={tag.id} style={styles.tagChip}>
              {tag.name}
              <button onClick={() => handleDeleteTag(tag.id)} style={styles.removeTagBtn}>×</button>
            </span>
          ))}
        </div>
      </section>
    </main>
  );
}

const styles: Record<string, React.CSSProperties> = {
  main:           { maxWidth: "960px", margin: "0 auto", padding: "2rem 1rem" },
  headerRow:      { display: "flex", justifyContent: "space-between", alignItems: "center", marginBottom: "2rem" },
  heading:        { fontSize: "1.75rem", fontWeight: 700, color: "#1a202c", margin: 0 },
  newBtn:         { background: "#4299e1", color: "#fff", padding: "0.6rem 1.25rem", borderRadius: "6px", textDecoration: "none", fontWeight: 600 },
  section:        { marginBottom: "3rem" },
  subheading:     { fontSize: "1.25rem", fontWeight: 600, marginBottom: "1rem", color: "#2d3748" },
  table:          { width: "100%", borderCollapse: "collapse" },
  th:             { textAlign: "left", padding: "0.75rem 1rem", background: "#f7fafc", borderBottom: "2px solid #e2e8f0", color: "#4a5568", fontSize: "0.85rem", textTransform: "uppercase" },
  td:             { padding: "0.75rem 1rem", borderBottom: "1px solid #e2e8f0", color: "#2d3748" },
  badge:          { padding: "0.2rem 0.6rem", borderRadius: "999px", fontSize: "0.75rem", fontWeight: 600 },
  badgePublished: { background: "#c6f6d5", color: "#276749" },
  badgeDraft:     { background: "#fed7d7", color: "#9b2c2c" },
  editBtn:        { color: "#4299e1", textDecoration: "none", fontSize: "0.875rem" },
  deleteBtn:      { background: "none", border: "none", color: "#fc8181", cursor: "pointer", fontSize: "0.875rem" },
  tagForm:        { display: "flex", gap: "0.75rem", marginBottom: "1rem" },
  tagInput:       { padding: "0.5rem 0.75rem", border: "1px solid #e2e8f0", borderRadius: "6px", fontSize: "0.9rem" },
  addTagBtn:      { background: "#48bb78", color: "#fff", border: "none", borderRadius: "6px", padding: "0.5rem 1rem", cursor: "pointer", fontWeight: 600 },
  tagList:        { display: "flex", gap: "0.5rem", flexWrap: "wrap" },
  tagChip:        { display: "flex", alignItems: "center", gap: "0.4rem", background: "#edf2f7", padding: "0.3rem 0.75rem", borderRadius: "999px", fontSize: "0.85rem" },
  removeTagBtn:   { background: "none", border: "none", cursor: "pointer", color: "#a0aec0", fontWeight: 700, lineHeight: 1 },
};
