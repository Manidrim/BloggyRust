export interface User {
  id: string;
  username: string;
  is_admin: boolean;
}

export interface Tag {
  id: string;
  name: string;
  slug: string;
  created_at: string;
}

export interface Article {
  id: string;
  title: string;
  slug: string;
  content: string;
  excerpt: string | null;
  author_id: string;
  author_username: string;
  published: boolean;
  tags: Tag[];
  created_at: string;
  updated_at: string;
}

export interface Comment {
  id: string;
  article_id: string;
  author_id: string;
  author_username: string;
  content: string;
  created_at: string;
  updated_at: string;
}

export interface AuthResponse {
  token: string;
  user: User;
}
