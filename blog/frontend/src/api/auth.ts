import { apiClient } from "./client";
import type { AuthResponse } from "../types";

export const authApi = {
  register: (username: string, email: string, password: string) =>
    apiClient.post<AuthResponse>("/auth/register", { username, email, password }),

  login: (email: string, password: string) =>
    apiClient.post<AuthResponse>("/auth/login", { email, password }),
};
