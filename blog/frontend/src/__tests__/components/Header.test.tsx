import { render, screen } from "@testing-library/react";
import { MemoryRouter } from "react-router-dom";
import { AuthProvider } from "../../contexts/AuthContext";
import { Header } from "../../components/layout/Header";

function renderHeader() {
  return render(
    <MemoryRouter>
      <AuthProvider>
        <Header />
      </AuthProvider>
    </MemoryRouter>
  );
}

beforeEach(() => {
  localStorage.clear();
});

test("affiche le logo du blog", () => {
  renderHeader();
  expect(screen.getByText("Mon Blog")).toBeInTheDocument();
});

test("affiche le lien Articles", () => {
  renderHeader();
  expect(screen.getByRole("link", { name: "Articles" })).toBeInTheDocument();
});

test("affiche le lien Connexion quand non authentifié", () => {
  renderHeader();
  expect(screen.getByRole("link", { name: "Connexion" })).toBeInTheDocument();
});

test("n'affiche pas le bouton Déconnexion quand non authentifié", () => {
  renderHeader();
  expect(screen.queryByRole("button", { name: "Déconnexion" })).not.toBeInTheDocument();
});

test("affiche le bouton Déconnexion quand authentifié", () => {
  const user = { id: "1", username: "alice", is_admin: false };
  localStorage.setItem("user", JSON.stringify(user));
  renderHeader();
  expect(screen.getByRole("button", { name: "Déconnexion" })).toBeInTheDocument();
});

test("affiche le nom d'utilisateur quand authentifié", () => {
  const user = { id: "1", username: "alice", is_admin: false };
  localStorage.setItem("user", JSON.stringify(user));
  renderHeader();
  expect(screen.getByText("alice")).toBeInTheDocument();
});

test("n'affiche pas le lien Admin pour un utilisateur non-admin", () => {
  const user = { id: "1", username: "alice", is_admin: false };
  localStorage.setItem("user", JSON.stringify(user));
  renderHeader();
  expect(screen.queryByRole("link", { name: "Admin" })).not.toBeInTheDocument();
});

test("affiche le lien Admin pour un admin", () => {
  const user = { id: "1", username: "admin", is_admin: true };
  localStorage.setItem("user", JSON.stringify(user));
  renderHeader();
  expect(screen.getByRole("link", { name: "Admin" })).toBeInTheDocument();
});
