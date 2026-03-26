import { useEffect, useState } from "react";
import { useNavigate, useParams } from "react-router-dom";
import { articlesApi } from "../../api/articles";
import { tagsApi } from "../../api/tags";
import type { Tag } from "../../types";

export function ArticleEditorPage() {
  const { slug } = useParams<{ slug?: string }>();
  const navigate = useNavigate();
  const isEditing = Boolean(slug);

  const [form, setForm] = useState({ title: "", content: "", excerpt: "", published: false });
  const [availableTags, setAvailableTags] = useState<Tag[]>([]);
  const [selectedTagIds, setSelectedTagIds] = useState<string[]>([]);
  const [error, setError] = useState<string | null>(null);
  const [isSaving, setIsSaving] = useState(false);

  useEffect(() => {
    tagsApi.getAll().then(({ data }) => setAvailableTags(data));
    if (slug) loadArticleForEditing(slug);
  }, [slug]);

  async function loadArticleForEditing(articleSlug: string) {
    const { data } = await articlesApi.getBySlug(articleSlug);
    setForm({ title: data.title, content: data.content, excerpt: data.excerpt ?? "", published: data.published });
    setSelectedTagIds(data.tags.map((t) => t.id));
  }

  function handleChange(e: React.ChangeEvent<HTMLInputElement | HTMLTextAreaElement>) {
    const { name, value, type } = e.target;
    const checked = type === "checkbox" ? (e.target as HTMLInputElement).checked : undefined;
    setForm((prev) => ({ ...prev, [name]: checked !== undefined ? checked : value }));
  }

  function toggleTag(tagId: string) {
    setSelectedTagIds((prev) =>
      prev.includes(tagId) ? prev.filter((id) => id !== tagId) : [...prev, tagId]
    );
  }

  async function handleSubmit(e: React.FormEvent) {
    e.preventDefault();
    setError(null);
    setIsSaving(true);

    try {
      const payload = { ...form, tag_ids: selectedTagIds, excerpt: form.excerpt || undefined };
      if (isEditing && slug) {
        await articlesApi.update(slug, payload);
      } else {
        await articlesApi.create(payload);
      }
      navigate("/admin");
    } catch {
      setError("Erreur lors de la sauvegarde.");
    } finally {
      setIsSaving(false);
    }
  }

  return (
    <main style={styles.main}>
      <h1 style={styles.heading}>{isEditing ? "Modifier l'article" : "Nouvel article"}</h1>

      {error && <p style={styles.error}>{error}</p>}

      <form onSubmit={handleSubmit} style={styles.form}>
        <label style={styles.label}>Titre</label>
        <input name="title" value={form.title} onChange={handleChange} required style={styles.input} />

        <label style={styles.label}>Extrait</label>
        <input name="excerpt" value={form.excerpt} onChange={handleChange} style={styles.input} placeholder="Résumé affiché dans la liste…" />

        <label style={styles.label}>Contenu (HTML ou texte)</label>
        <textarea name="content" value={form.content} onChange={handleChange} required style={styles.textarea} rows={16} />

        <label style={styles.label}>Tags</label>
        <div style={styles.tags}>
          {availableTags.map((tag) => (
            <button
              key={tag.id}
              type="button"
              onClick={() => toggleTag(tag.id)}
              style={{ ...styles.tagBtn, ...(selectedTagIds.includes(tag.id) ? styles.tagBtnActive : {}) }}
            >
              {tag.name}
            </button>
          ))}
        </div>

        <label style={styles.checkboxLabel}>
          <input type="checkbox" name="published" checked={form.published} onChange={handleChange} />
          Publié
        </label>

        <div style={styles.actions}>
          <button type="button" onClick={() => navigate("/admin")} style={styles.cancelBtn}>Annuler</button>
          <button type="submit" disabled={isSaving} style={styles.saveBtn}>
            {isSaving ? "Sauvegarde…" : "Sauvegarder"}
          </button>
        </div>
      </form>
    </main>
  );
}

const styles: Record<string, React.CSSProperties> = {
  main:          { maxWidth: "800px", margin: "0 auto", padding: "2rem 1rem" },
  heading:       { fontSize: "1.75rem", fontWeight: 700, marginBottom: "1.5rem", color: "#1a202c" },
  error:         { background: "#fff5f5", color: "#c53030", padding: "0.75rem", borderRadius: "6px", marginBottom: "1rem" },
  form:          { display: "flex", flexDirection: "column", gap: "0.75rem" },
  label:         { fontWeight: 500, color: "#4a5568", fontSize: "0.9rem" },
  input:         { padding: "0.65rem", border: "1px solid #e2e8f0", borderRadius: "6px", fontSize: "1rem" },
  textarea:      { padding: "0.65rem", border: "1px solid #e2e8f0", borderRadius: "6px", fontSize: "1rem", fontFamily: "monospace", resize: "vertical" },
  tags:          { display: "flex", gap: "0.5rem", flexWrap: "wrap" },
  tagBtn:        { background: "#edf2f7", color: "#4a5568", border: "none", borderRadius: "999px", padding: "0.3rem 0.9rem", cursor: "pointer", fontSize: "0.85rem" },
  tagBtnActive:  { background: "#4299e1", color: "#fff" },
  checkboxLabel: { display: "flex", alignItems: "center", gap: "0.5rem", fontWeight: 500 },
  actions:       { display: "flex", gap: "1rem", justifyContent: "flex-end", marginTop: "1rem" },
  cancelBtn:     { background: "none", border: "1px solid #e2e8f0", borderRadius: "6px", padding: "0.6rem 1.5rem", cursor: "pointer" },
  saveBtn:       { background: "#4299e1", color: "#fff", border: "none", borderRadius: "6px", padding: "0.6rem 1.5rem", cursor: "pointer", fontWeight: 600 },
};
