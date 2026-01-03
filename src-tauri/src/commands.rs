use crate::api::FanqieApi;
use crate::downloader::Downloader;
use crate::types::*;
use tauri::AppHandle;

/// 搜索书籍
#[tauri::command]
pub async fn search_books(keyword: String, offset: i32) -> Result<SearchResult, String> {
    let api = FanqieApi::new();
    api.search_books(&keyword, offset)
        .await
        .map_err(|e| e.to_string())
}

/// 获取书籍详情
#[tauri::command]
pub async fn get_book_detail(book_id: String) -> Result<BookInfo, String> {
    let api = FanqieApi::new();
    api.get_book_detail(&book_id)
        .await
        .map_err(|e| e.to_string())
}

/// 获取章节列表
#[tauri::command]
pub async fn get_chapters(book_id: String) -> Result<Vec<Chapter>, String> {
    let api = FanqieApi::new();
    api.get_directory(&book_id)
        .await
        .map_err(|e| e.to_string())
}

/// 下载书籍
#[tauri::command]
pub async fn download_book(
    options: DownloadOptions,
    app_handle: AppHandle,
) -> Result<DownloadResult, String> {
    let downloader = Downloader::new();
    downloader
        .download(options, app_handle)
        .await
        .map_err(|e| e.to_string())
}

/// 获取可用的 API 节点列表
#[tauri::command]
pub fn get_api_sources() -> Vec<ApiSource> {
    vec![
        ApiSource {
            name: "中国|浙江省|宁波市|电信".to_string(),
            base_url: "http://qkfqapi.vv9v.cn".to_string(),
        },
        ApiSource {
            name: "中国|北京市|腾讯云".to_string(),
            base_url: "http://49.232.137.12".to_string(),
        },
        ApiSource {
            name: "日本|东京".to_string(),
            base_url: "https://fq.shusan.cn".to_string(),
        },
    ]
}

/// API 节点信息
#[derive(serde::Serialize)]
pub struct ApiSource {
    pub name: String,
    pub base_url: String,
}
