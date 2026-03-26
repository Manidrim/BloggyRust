import { BrowserRouter, Route, Routes } from "react-router-dom";
import { Footer } from "./components/layout/Footer";
import { Header } from "./components/layout/Header";
import { AuthProvider } from "./contexts/AuthContext";
import { AdminPage } from "./pages/admin/AdminPage";
import { ArticleEditorPage } from "./pages/admin/ArticleEditorPage";
import { ArticlePage } from "./pages/ArticlePage";
import { HomePage } from "./pages/HomePage";
import { LoginPage } from "./pages/LoginPage";

export default function App() {
  return (
    <AuthProvider>
      <BrowserRouter>
        <div style={styles.layout}>
          <Header />
          <div style={styles.content}>
            <Routes>
              <Route path="/"                            element={<HomePage />} />
              <Route path="/articles/:slug"             element={<ArticlePage />} />
              <Route path="/login"                      element={<LoginPage />} />
              <Route path="/admin"                      element={<AdminPage />} />
              <Route path="/admin/articles/new"         element={<ArticleEditorPage />} />
              <Route path="/admin/articles/:slug/edit"  element={<ArticleEditorPage />} />
            </Routes>
          </div>
          <Footer />
        </div>
      </BrowserRouter>
    </AuthProvider>
  );
}

const styles: Record<string, React.CSSProperties> = {
  layout:  { display: "flex", flexDirection: "column", minHeight: "100vh", background: "#f7fafc" },
  content: { flex: 1 },
};
