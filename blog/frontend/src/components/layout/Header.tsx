import { Link, useNavigate } from "react-router-dom";
import { useAuth } from "../../contexts/AuthContext";

export function Header() {
  const { user, logout } = useAuth();
  const navigate = useNavigate();

  function handleLogout() {
    logout();
    navigate("/");
  }

  return (
    <header style={styles.header}>
      <Link to="/" style={styles.brand}>Mon Blog</Link>
      <nav style={styles.nav}>
        <Link to="/" style={styles.link}>Articles</Link>
        {user?.is_admin && (
          <Link to="/admin" style={styles.link}>Admin</Link>
        )}
        {user ? (
          <>
            <span style={styles.username}>{user.username}</span>
            <button onClick={handleLogout} style={styles.button}>Déconnexion</button>
          </>
        ) : (
          <Link to="/login" style={styles.link}>Connexion</Link>
        )}
      </nav>
    </header>
  );
}

const styles: Record<string, React.CSSProperties> = {
  header:   { display: "flex", justifyContent: "space-between", alignItems: "center", padding: "1rem 2rem", borderBottom: "1px solid #e2e8f0", background: "#fff" },
  brand:    { fontSize: "1.5rem", fontWeight: 700, color: "#1a202c", textDecoration: "none" },
  nav:      { display: "flex", gap: "1.5rem", alignItems: "center" },
  link:     { color: "#4a5568", textDecoration: "none", fontWeight: 500 },
  username: { color: "#718096", fontSize: "0.9rem" },
  button:   { background: "none", border: "1px solid #e2e8f0", borderRadius: "6px", padding: "0.4rem 0.9rem", cursor: "pointer", color: "#4a5568" },
};
