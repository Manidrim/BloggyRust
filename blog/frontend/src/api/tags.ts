import { apiClient } from "./client";
import type { Tag } from "../types";

export const tagsApi = {
  getAll: () =>
    apiClient.get<Tag[]>("/tags"),

  create: (name: string, slug: string) =>
    apiClient.post<Tag>("/tags", { name, slug }),

  delete: (id: string) =>
    apiClient.delete(`/tags/${id}`),
};
