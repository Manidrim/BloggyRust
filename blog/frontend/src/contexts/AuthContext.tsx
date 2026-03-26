import { createContext, useContext, useEffect, useState } from "react";
import type { ReactNode } from "react";
import { authApi } from "../api/auth";
import type { User } from "../types";

interface AuthContextValue {
  user: User | null;
  isLoading: boolean;
  login: (email: string, password: string) => Promise<void>;
  register: (username: string, email: string, password: string) => Promise<void>;
  logout: () => void;
}

const AuthContext = createContext<AuthContextValue | null>(null);

export function AuthProvider({ children }: { children: ReactNode }) {
  const [user, setUser] = useState<User | null>(null);
  const [isLoading, setIsLoading] = useState(true);

  useEffect(() => {
    restoreSessionFromStorage();
  }, []);

  function restoreSessionFromStorage() {
    const storedUser = localStorage.getItem("user");
    if (storedUser) {
      setUser(JSON.parse(storedUser));
    }
    setIsLoading(false);
  }

  async function login(email: string, password: string) {
    const { data } = await authApi.login(email, password);
    persistSession(data.token, data.user);
  }

  async function register(username: string, email: string, password: string) {
    const { data } = await authApi.register(username, email, password);
    persistSession(data.token, data.user);
  }

  function logout() {
    localStorage.removeItem("token");
    localStorage.removeItem("user");
    setUser(null);
  }

  function persistSession(token: string, authenticatedUser: User) {
    localStorage.setItem("token", token);
    localStorage.setItem("user", JSON.stringify(authenticatedUser));
    setUser(authenticatedUser);
  }

  return (
    <AuthContext.Provider value={{ user, isLoading, login, register, logout }}>
      {children}
    </AuthContext.Provider>
  );
}

export function useAuth() {
  const ctx = useContext(AuthContext);
  if (!ctx) throw new Error("useAuth must be used within AuthProvider");
  return ctx;
}
