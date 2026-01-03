use serde::{Deserialize, Serialize};

/// 书籍信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BookInfo {
    pub book_id: String,
    pub book_name: String,
    pub author: String,
    pub cover_url: String,
    #[serde(rename = "abstract")]
    pub description: String,
    pub word_count: Option<i64>,
    pub chapter_count: Option<i64>,
    pub category: Option<String>,
    pub status: Option<String>,
}

/// 章节信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Chapter {
    pub id: String,
    pub title: String,
    pub index: usize,
}

/// 章节内容
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChapterContent {
    pub title: String,
    pub content: String,
    pub index: usize,
}

/// 搜索结果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchResult {
    pub books: Vec<BookInfo>,
    pub total: i64,
    pub has_more: bool,
}

/// 下载进度
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DownloadProgress {
    pub current: usize,
    pub total: usize,
    pub percent: f64,
    pub message: String,
    pub book_id: String,
}

/// 下载结果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DownloadResult {
    pub success: bool,
    pub file_path: Option<String>,
    pub error: Option<String>,
    pub book_name: String,
}

/// 下载选项
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DownloadOptions {
    pub book_id: String,
    pub save_path: String,
    pub format: String, // "txt" or "epub"
    pub start_chapter: Option<usize>,
    pub end_chapter: Option<usize>,
}

/// API 响应包装
#[derive(Debug, Deserialize)]
pub struct ApiResponse<T> {
    pub code: i32,
    pub message: Option<String>,
    pub data: Option<T>,
}

/// 目录项
#[derive(Debug, Clone, Deserialize)]
pub struct DirectoryItem {
    pub item_id: String,
    pub title: String,
}

/// 目录响应数据
#[derive(Debug, Deserialize)]
pub struct DirectoryData {
    pub lists: Vec<DirectoryItem>,
}

/// 书籍详情响应内层数据
#[derive(Debug, Deserialize)]
pub struct BookDetailInner {
    pub code: Option<i32>,
    pub message: Option<String>,
    pub data: Option<BookInfo>,
}

/// 内容响应
#[derive(Debug, Deserialize)]
pub struct ContentResponse {
    pub content: Option<String>,
    pub title: Option<String>,
}
