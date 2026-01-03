import { invoke } from "@tauri-apps/api/core";

// 类型定义
export interface BookInfo {
  book_id: string;
  book_name: string;
  author: string;
  cover_url: string;
  description: string;
  word_count?: number;
  chapter_count?: number;
  category?: string;
  status?: string;
}

export interface Chapter {
  id: string;
  title: string;
  index: number;
}

export interface SearchResult {
  books: BookInfo[];
  total: number;
  has_more: boolean;
}

export interface DownloadOptions {
  book_id: string;
  save_path: string;
  format: string;
  start_chapter?: number;
  end_chapter?: number;
}

export interface DownloadProgress {
  current: number;
  total: number;
  percent: number;
  message: string;
  book_id: string;
}

export interface DownloadResult {
  success: boolean;
  file_path?: string;
  error?: string;
  book_name: string;
}

export interface ApiSource {
  name: string;
  base_url: string;
}

// API 调用封装
export async function searchBooks(keyword: string, offset = 0): Promise<SearchResult> {
  return await invoke("search_books", { keyword, offset });
}

export async function getBookDetail(bookId: string): Promise<BookInfo> {
  return await invoke("get_book_detail", { bookId });
}

export async function getChapters(bookId: string): Promise<Chapter[]> {
  return await invoke("get_chapters", { bookId });
}

export async function downloadBook(options: DownloadOptions): Promise<DownloadResult> {
  return await invoke("download_book", { options });
}

export async function getApiSources(): Promise<ApiSource[]> {
  return await invoke("get_api_sources");
}
