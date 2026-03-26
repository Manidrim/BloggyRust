import { apiClient } from "./client";
import type { Article } from "../types";

export const articlesApi = {
  getAll: () =>
    apiClient.get<Article[]>("/articles"),

  getBySlug: (slug: string) =>
    apiClient.get<Article>(`/articles/${slug}`),

  create: (data: {
    title: string;
    content: string;
    excerpt?: string;
    published?: boolean;
    tag_ids?: string[];
  }) => apiClient.post<Article>("/articles", data),

  update: (
    slug: string,
    data: {
      title?: string;
      content?: string;
      excerpt?: string;
      published?: boolean;
      tag_ids?: string[];
    }
  ) => apiClient.put<Article>(`/articles/${slug}`, data),

  delete: (slug: string) =>
    apiClient.delete(`/articles/${slug}`),
};
