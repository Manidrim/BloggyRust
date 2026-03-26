import { apiClient } from "./client";
import type { Comment } from "../types";

export const commentsApi = {
  getByArticle: (articleId: string) =>
    apiClient.get<Comment[]>(`/articles/${articleId}/comments`),

  create: (articleId: string, content: string) =>
    apiClient.post<Comment>(`/articles/${articleId}/comments`, { content }),

  delete: (commentId: string) =>
    apiClient.delete(`/comments/${commentId}`),
};
