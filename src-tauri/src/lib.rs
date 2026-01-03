// 模块定义
mod api;
mod commands;
mod downloader;
mod types;

use commands::{download_book, get_api_sources, get_book_detail, get_chapters, search_books};

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_fs::init())
        .invoke_handler(tauri::generate_handler![
            search_books,
            get_book_detail,
            get_chapters,
            download_book,
            get_api_sources,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
