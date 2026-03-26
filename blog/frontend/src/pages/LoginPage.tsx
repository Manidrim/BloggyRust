import { useState } from "react";
import { useNavigate } from "react-router-dom";
import { useAuth } from "../contexts/AuthContext";

type Mode = "login" | "register";

export function LoginPage() {
  const { login, register } = useAuth();
  const navigate = useNavigate();
  const [mode, setMode] = useState<Mode>("login");
  const [form, setForm] = useState({ username: "", email: "", password: "" });
  const [error, setError] = useState<string | null>(null);
  const [isSubmitting, setIsSubmitting] = useState(false);

  function handleChange(e: React.ChangeEvent<HTMLInputElement>) {
    setForm((prev) => ({ ...prev, [e.target.name]: e.target.value }));
  }

  async function handleSubmit(e: React.FormEvent) {
    e.preventDefault();
    setError(null);
    setIsSubmitting(true);

    try {
      if (mode === "login") {
        await login(form.email, form.password);
      } else {
        await register(form.username, form.email, form.password);
      }
      navigate("/");
    } catch (err: unknown) {
      const message = extractErrorMessage(err);
      setError(message);
    } finally {
      setIsSubmitting(false);
    }
  }

  return (
    <main style={styles.main}>
      <div style={styles.card}>
        <h1 style={styles.heading}>
          {mode === "login" ? "Connexion" : "Inscription"}
        </h1>

        {error && <p style={styles.error}>{error}</p>}

        <form onSubmit={handleSubmit} style={styles.form}>
          {mode === "register" && (
            <FormField label="Nom d'utilisateur" name="username" value={form.username} onChange={handleChange} />
          )}
          <FormField label="Email" name="email" type="email" value={form.email} onChange={handleChange} />
          <FormField label="Mot de passe" name="password" type="password" value={form.password} onChange={handleChange} />

          <button type="submit" disabled={isSubmitting} style={styles.submitBtn}>
            {isSubmitting ? "…" : mode === "login" ? "Se connecter" : "S'inscrire"}
          </button>
        </form>

        <p style={styles.switchHint}>
          {mode === "login" ? (
            <>Pas encore de compte ? <button onClick={() => setMode("register")} style={styles.switchBtn}>S'inscrire</button></>
          ) : (
            <>Déjà un compte ? <button onClick={() => setMode("login")} style={styles.switchBtn}>Se connecter</button></>
          )}
        </p>
      </div>
    </main>
  );
}

interface FormFieldProps {
  label: string;
  name: string;
  value: string;
  onChange: (e: React.ChangeEvent<HTMLInputElement>) => void;
  type?: string;
}

function FormField({ label, name, value, onChange, type = "text" }: FormFieldProps) {
  return (
    <div style={styles.field}>
      <label htmlFor={name} style={styles.label}>{label}</label>
      <input
        id={name}
        name={name}
        type={type}
        value={value}
        onChange={onChange}
        required
        style={styles.input}
      />
    </div>
  );
}

function extractErrorMessage(err: unknown): string {
  if (err && typeof err === "object" && "response" in err) {
    const response = (err as { response?: { data?: { error?: string } } }).response;
    if (response?.data?.error) return response.data.error;
  }
  return "Une erreur est survenue. Veuillez réessayer.";
}

const styles: Record<string, React.CSSProperties> = {
  main:      { display: "flex", justifyContent: "center", alignItems: "center", minHeight: "60vh", padding: "2rem" },
  card:      { width: "100%", maxWidth: "400px", background: "#fff", border: "1px solid #e2e8f0", borderRadius: "12px", padding: "2rem" },
  heading:   { fontSize: "1.75rem", fontWeight: 700, marginBottom: "1.5rem", textAlign: "center", color: "#1a202c" },
  error:     { background: "#fff5f5", color: "#c53030", padding: "0.75rem 1rem", borderRadius: "6px", marginBottom: "1rem", fontSize: "0.9rem" },
  form:      { display: "flex", flexDirection: "column", gap: "1rem" },
  field:     { display: "flex", flexDirection: "column", gap: "0.4rem" },
  label:     { fontWeight: 500, fontSize: "0.9rem", color: "#4a5568" },
  input:     { padding: "0.65rem 0.9rem", border: "1px solid #e2e8f0", borderRadius: "6px", fontSize: "1rem", outline: "none" },
  submitBtn: { background: "#4299e1", color: "#fff", border: "none", borderRadius: "6px", padding: "0.75rem", fontWeight: 600, fontSize: "1rem", cursor: "pointer", marginTop: "0.5rem" },
  switchHint:{ textAlign: "center", marginTop: "1.25rem", color: "#718096", fontSize: "0.9rem" },
  switchBtn: { background: "none", border: "none", color: "#4299e1", cursor: "pointer", fontWeight: 600, fontSize: "0.9rem" },
};
