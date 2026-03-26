export function Footer() {
  return (
    <footer style={styles.footer}>
      <p style={styles.text}>© {new Date().getFullYear()} Mon Blog — Propulsé par Rust &amp; React</p>
    </footer>
  );
}

const styles: Record<string, React.CSSProperties> = {
  footer: { padding: "2rem", textAlign: "center", borderTop: "1px solid #e2e8f0", marginTop: "auto" },
  text:   { color: "#a0aec0", fontSize: "0.875rem" },
};
