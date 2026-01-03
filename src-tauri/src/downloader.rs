use crate::api::FanqieApi;
use crate::types::*;
use anyhow::{anyhow, Result};
use epub_builder::{EpubBuilder, EpubContent, ZipLibrary};
use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::Path;
use tauri::{AppHandle, Emitter};

/// 下载器
pub struct Downloader {
    api: FanqieApi,
}

impl Downloader {
    pub fn new() -> Self {
        Self {
            api: FanqieApi::new(),
        }
    }

    pub fn with_api(api: FanqieApi) -> Self {
        Self { api }
    }

    /// 下载书籍
    pub async fn download(
        &self,
        options: DownloadOptions,
        app_handle: AppHandle,
    ) -> Result<DownloadResult> {
        let book_id = &options.book_id;
        let save_path = &options.save_path;
        let format = options.format.to_lowercase();

        // 发送进度
        let emit_progress = |current: usize, total: usize, message: &str| {
            let percent = if total > 0 {
                (current as f64 / total as f64) * 100.0
            } else {
                0.0
            };
            let _ = app_handle.emit(
                "download-progress",
                DownloadProgress {
                    current,
                    total,
                    percent,
                    message: message.to_string(),
                    book_id: book_id.clone(),
                },
            );
        };

        emit_progress(0, 100, "正在获取书籍信息...");

        // 获取书籍详情
        let book_info = self.api.get_book_detail(book_id).await?;
        emit_progress(5, 100, &format!("获取到: {}", book_info.book_name));

        // 获取章节目录
        emit_progress(10, 100, "正在获取章节目录...");
        let chapters = self.api.get_directory(book_id).await?;
        let total_chapters = chapters.len();
        emit_progress(15, 100, &format!("共 {} 章", total_chapters));

        // 筛选章节范围
        let chapters_to_download: Vec<_> = chapters
            .into_iter()
            .filter(|ch| {
                let start = options.start_chapter.unwrap_or(0);
                let end = options.end_chapter.unwrap_or(usize::MAX);
                ch.index >= start && ch.index < end
            })
            .collect();

        if chapters_to_download.is_empty() {
            return Err(anyhow!("没有可下载的章节"));
        }

        // 尝试极速模式
        emit_progress(20, 100, "尝试极速下载模式...");
        let chapter_contents = match self.api.get_full_content(book_id).await {
            Ok(content_map) => {
                emit_progress(50, 100, "极速模式成功，正在处理内容...");
                let mut contents = Vec::new();
                for ch in &chapters_to_download {
                    if let Some(content) = content_map.get(&ch.id) {
                        contents.push(ChapterContent {
                            title: ch.title.clone(),
                            content: content.clone(),
                            index: ch.index,
                        });
                    }
                }
                
                // 如果极速模式没有获取到所有章节，回退到普通模式
                if contents.len() < chapters_to_download.len() {
                    emit_progress(55, 100, "极速模式内容不完整，切换到普通模式...");
                    self.download_chapters_normal(&chapters_to_download, &app_handle, book_id).await?
                } else {
                    contents
                }
            }
            Err(_) => {
                emit_progress(25, 100, "极速模式不可用，使用普通模式...");
                self.download_chapters_normal(&chapters_to_download, &app_handle, book_id).await?
            }
        };

        emit_progress(85, 100, "正在生成文件...");

        // 生成文件
        let file_path = match format.as_str() {
            "epub" => {
                self.create_epub(&book_info, &chapter_contents, save_path)?
            }
            _ => {
                self.create_txt(&book_info, &chapter_contents, save_path)?
            }
        };

        emit_progress(100, 100, "下载完成！");

        Ok(DownloadResult {
            success: true,
            file_path: Some(file_path),
            error: None,
            book_name: book_info.book_name,
        })
    }

    /// 普通模式下载章节
    async fn download_chapters_normal(
        &self,
        chapters: &[Chapter],
        app_handle: &AppHandle,
        book_id: &str,
    ) -> Result<Vec<ChapterContent>> {
        let total = chapters.len();
        let mut contents = Vec::with_capacity(total);

        for (idx, ch) in chapters.iter().enumerate() {
            let progress = 25 + (idx as f64 / total as f64 * 60.0) as usize;
            let _ = app_handle.emit(
                "download-progress",
                DownloadProgress {
                    current: idx + 1,
                    total,
                    percent: progress as f64,
                    message: format!("下载中: {}/{} - {}", idx + 1, total, ch.title),
                    book_id: book_id.to_string(),
                },
            );

            match self.api.get_chapter_content(&ch.id).await {
                Ok(content) => {
                    contents.push(ChapterContent {
                        title: ch.title.clone(),
                        content,
                        index: ch.index,
                    });
                }
                Err(e) => {
                    eprintln!("章节 {} 下载失败: {}", ch.title, e);
                    // 继续下载其他章节
                }
            }

            // 添加小延迟避免请求过快
            tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
        }

        Ok(contents)
    }

    /// 创建 TXT 文件
    fn create_txt(
        &self,
        book_info: &BookInfo,
        chapters: &[ChapterContent],
        save_path: &str,
    ) -> Result<String> {
        let filename = sanitize_filename(&format!(
            "{} 作者：{}.txt",
            book_info.book_name, book_info.author
        ));
        let file_path = Path::new(save_path).join(&filename);

        let file = File::create(&file_path)?;
        let mut writer = BufWriter::new(file);

        // 写入书籍信息
        writeln!(writer, "{}", book_info.book_name)?;
        writeln!(writer, "作者：{}", book_info.author)?;
        if !book_info.description.is_empty() {
            writeln!(writer, "\n简介：\n{}", book_info.description)?;
        }
        writeln!(writer, "\n{}\n", "=".repeat(50))?;

        // 写入章节
        for ch in chapters {
            writeln!(writer, "\n{}\n", ch.title)?;
            writeln!(writer, "{}\n", ch.content)?;
        }

        Ok(file_path.to_string_lossy().to_string())
    }

    /// 创建 EPUB 文件
    fn create_epub(
        &self,
        book_info: &BookInfo,
        chapters: &[ChapterContent],
        save_path: &str,
    ) -> Result<String> {
        let filename = sanitize_filename(&format!(
            "{} 作者：{}.epub",
            book_info.book_name, book_info.author
        ));
        let file_path = Path::new(save_path).join(&filename);

        let file = File::create(&file_path)?;
        let zip = ZipLibrary::new().map_err(|e| anyhow!("创建 ZIP 库失败: {}", e))?;
        let mut epub = EpubBuilder::new(zip).map_err(|e| anyhow!("创建 EPUB 构建器失败: {}", e))?;

        // 设置元数据
        epub.metadata("title", &book_info.book_name)
            .map_err(|e| anyhow!("设置标题失败: {}", e))?;
        epub.metadata("author", &book_info.author)
            .map_err(|e| anyhow!("设置作者失败: {}", e))?;
        epub.metadata("lang", "zh-CN")
            .map_err(|e| anyhow!("设置语言失败: {}", e))?;
        
        if !book_info.description.is_empty() {
            epub.metadata("description", &book_info.description)
                .map_err(|e| anyhow!("设置描述失败: {}", e))?;
        }

        // 创建简介页
        let intro_html = format!(
            r#"<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE html>
<html xmlns="http://www.w3.org/1999/xhtml">
<head><title>书籍信息</title></head>
<body>
<h1>{}</h1>
<p><strong>作者：</strong>{}</p>
<hr/>
<h3>简介</h3>
<p>{}</p>
</body>
</html>"#,
            book_info.book_name,
            book_info.author,
            book_info.description.replace('\n', "<br/>")
        );

        epub.add_content(
            EpubContent::new("intro.xhtml", intro_html.as_bytes())
                .title("书籍信息")
        ).map_err(|e| anyhow!("添加简介页失败: {}", e))?;

        // 添加章节
        for (idx, ch) in chapters.iter().enumerate() {
            let content_html = ch.content
                .split("\n\n")
                .map(|p| format!("<p>{}</p>", p.trim()))
                .collect::<Vec<_>>()
                .join("\n");

            let chapter_html = format!(
                r#"<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE html>
<html xmlns="http://www.w3.org/1999/xhtml">
<head><title>{}</title></head>
<body>
<h1>{}</h1>
<div>{}</div>
</body>
</html>"#,
                ch.title, ch.title, content_html
            );

            epub.add_content(
                EpubContent::new(format!("chapter_{}.xhtml", idx + 1), chapter_html.as_bytes())
                    .title(&ch.title)
            ).map_err(|e| anyhow!("添加章节 {} 失败: {}", ch.title, e))?;
        }

        epub.generate(file).map_err(|e| anyhow!("生成 EPUB 文件失败: {}", e))?;

        Ok(file_path.to_string_lossy().to_string())
    }
}

impl Default for Downloader {
    fn default() -> Self {
        Self::new()
    }
}

/// 清理文件名中的非法字符
fn sanitize_filename(name: &str) -> String {
    let illegal_chars = ['\\', '/', ':', '*', '?', '"', '<', '>', '|'];
    let mut result = name.to_string();
    for ch in illegal_chars {
        result = result.replace(ch, "_");
    }
    result
}
