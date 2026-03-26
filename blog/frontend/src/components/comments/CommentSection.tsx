import { useEffect, useState } from "react";
import { commentsApi } from "../../api/comments";
import { useAuth } from "../../contexts/AuthContext";
import type { Comment } from "../../types";

interface CommentSectionProps {
  articleId: string;
}

export function CommentSection({ articleId }: CommentSectionProps) {
  const { user } = useAuth();
  const [comments, setComments] = useState<Comment[]>([]);
  const [newComment, setNewComment] = useState("");
  const [isSubmitting, setIsSubmitting] = useState(false);

  useEffect(() => {
    loadComments();
  }, [articleId]);

  async function loadComments() {
    const { data } = await commentsApi.getByArticle(articleId);
    setComments(data);
  }

  async function handleSubmit(e: React.FormEvent) {
    e.preventDefault();
    if (!newComment.trim()) return;

    setIsSubmitting(true);
    try {
      const { data } = await commentsApi.create(articleId, newComment.trim());
      setComments((prev) => [...prev, data]);
      setNewComment("");
    } finally {
      setIsSubmitting(false);
    }
  }

  async function handleDelete(commentId: string) {
    await commentsApi.delete(commentId);
    setComments((prev) => prev.filter((c) => c.id !== commentId));
  }

  return (
    <section style={styles.section}>
      <h3 style={styles.heading}>Commentaires ({comments.length})</h3>

      <div style={styles.list}>
        {comments.map((comment) => (
          <div key={comment.id} style={styles.comment}>
            <div style={styles.commentHeader}>
              <strong>{comment.author_username}</strong>
              <time style={styles.time}>
                {new Date(comment.created_at).toLocaleDateString("fr-FR")}
              </time>
              {(user?.id === comment.author_id || user?.is_admin) && (
                <button onClick={() => handleDelete(comment.id)} style={styles.deleteBtn}>
                  Supprimer
                </button>
              )}
            </div>
            <p style={styles.commentContent}>{comment.content}</p>
          </div>
        ))}
      </div>

      {user ? (
        <form onSubmit={handleSubmit} style={styles.form}>
          <textarea
            value={newComment}
            onChange={(e) => setNewComment(e.target.value)}
            placeholder="Votre commentaire…"
            style={styles.textarea}
            rows={4}
          />
          <button type="submit" disabled={isSubmitting} style={styles.submitBtn}>
            {isSubmitting ? "Envoi…" : "Publier"}
          </button>
        </form>
      ) : (
        <p style={styles.loginHint}>
          <a href="/login">Connectez-vous</a> pour laisser un commentaire.
        </p>
      )}
    </section>
  );
}

const styles: Record<string, React.CSSProperties> = {
  section:        { marginTop: "3rem" },
  heading:        { fontSize: "1.25rem", fontWeight: 700, marginBottom: "1.5rem", color: "#1a202c" },
  list:           { display: "flex", flexDirection: "column", gap: "1rem", marginBottom: "2rem" },
  comment:        { background: "#f7fafc", borderRadius: "8px", padding: "1rem" },
  commentHeader:  { display: "flex", gap: "0.75rem", alignItems: "center", marginBottom: "0.5rem" },
  time:           { color: "#a0aec0", fontSize: "0.8rem" },
  deleteBtn:      { background: "none", border: "none", color: "#fc8181", cursor: "pointer", fontSize: "0.8rem", marginLeft: "auto" },
  commentContent: { margin: 0, color: "#4a5568", lineHeight: 1.6 },
  form:           { display: "flex", flexDirection: "column", gap: "0.75rem" },
  textarea:       { padding: "0.75rem", border: "1px solid #e2e8f0", borderRadius: "6px", resize: "vertical", fontFamily: "inherit", fontSize: "1rem" },
  submitBtn:      { alignSelf: "flex-end", background: "#4299e1", color: "#fff", border: "none", borderRadius: "6px", padding: "0.6rem 1.5rem", cursor: "pointer", fontWeight: 600 },
  loginHint:      { color: "#718096", fontStyle: "italic" },
};
