use crate::types::*;
use anyhow::{anyhow, Result};
use reqwest::Client;
use std::collections::HashMap;
use std::time::Duration;

/// API 端点配置
const API_SOURCES: &[&str] = &[
    "http://qkfqapi.vv9v.cn",
    "http://49.232.137.12",
    "http://43.248.77.205:22222",
    "https://fq.shusan.cn",
];

/// 番茄小说 API 客户端
pub struct FanqieApi {
    client: Client,
    base_url: String,
}

impl FanqieApi {
    pub fn new() -> Self {
        let mut headers = reqwest::header::HeaderMap::new();
        headers.insert("Referer", "https://fanqienovel.com/".parse().unwrap());
        headers.insert("X-Requested-With", "XMLHttpRequest".parse().unwrap());
        headers.insert("Accept-Language", "zh-CN,zh;q=0.9,en-US;q=0.8,en;q=0.7".parse().unwrap());
        headers.insert("Accept", "application/json, text/javascript, */*; q=0.01".parse().unwrap());

        let client = Client::builder()
            .timeout(Duration::from_secs(30))
            .connect_timeout(Duration::from_secs(10))
            .user_agent("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36")
            .default_headers(headers)
            .build()
            .expect("Failed to create HTTP client");

        Self {
            client,
            base_url: API_SOURCES[0].to_string(),
        }
    }
    
    // 省略 with_base_url ...

    /// 搜索书籍
    pub async fn search_books(&self, keyword: &str, offset: i32) -> Result<SearchResult> {
        let keyword = keyword.to_string();
        let client = self.client.clone();
        
        self.try_with_fallback(move |base_url| {
            let keyword = keyword.clone();
            let client = client.clone();
            let offset_str = offset.to_string();
            async move {
                let url = format!("{}/api/search", base_url);
                
                let resp = client
                    .get(&url)
                    .query(&[
                        ("key", keyword.as_str()),
                        ("tab_type", "3"),
                        ("offset", offset_str.as_str()),
                    ])
                    .send()
                    .await?;

                let data: serde_json::Value = resp.json().await?;
                
                if data["code"].as_i64() != Some(200) {
                    return Err(anyhow!("API 返回错误: {:?}", data["message"]));
                }

                // 从 search_tabs 中提取书籍数据
                let mut books = Vec::new();
                let mut has_more = false;
                
                if let Some(search_tabs) = data["data"]["search_tabs"].as_array() {
                    for tab in search_tabs {
                        if let Some(tab_data) = tab["data"].as_array() {
                            if !tab_data.is_empty() {
                                has_more = tab["has_more"].as_bool().unwrap_or(false);
                                
                                for item in tab_data {
                                    // book_data 是一个数组，取第一个元素
                                    let book_info = if let Some(book_data_arr) = item["book_data"].as_array() {
                                        book_data_arr.get(0)
                                    } else {
                                        None
                                    };
                                    
                                    let b = book_info.unwrap_or(item);
                                    let book_id = item["book_id"].as_str()
                                        .or_else(|| b["book_id"].as_str());
                                    
                                    if let Some(id) = book_id {
                                        books.push(BookInfo {
                                            book_id: id.to_string(),
                                            book_name: b["book_name"].as_str().unwrap_or("未知").to_string(),
                                            author: b["author"].as_str().unwrap_or("未知").to_string(),
                                            cover_url: b["thumb_url"].as_str()
                                                .or_else(|| b["cover_url"].as_str())
                                                .unwrap_or("")
                                                .to_string(),
                                            description: b["abstract"].as_str().unwrap_or("").to_string(),
                                            word_count: b["word_number"].as_i64()
                                                .or_else(|| b["word_count"].as_i64()),
                                            chapter_count: b["serial_count"].as_i64()
                                                .or_else(|| b["chapter_number"].as_i64()),
                                            category: b["category"].as_str().map(|s| s.to_string()),
                                            status: b["creation_status"].as_str().map(|s| s.to_string()),
                                        });
                                    }
                                }
                                break; // 找到有数据的 tab 就停止
                            }
                        }
                    }
                }

                let total = books.len() as i64;

                Ok(SearchResult {
                    books,
                    total,
                    has_more,
                })
            }
        }).await
    }

    /// 获取书籍详情
    pub async fn get_book_detail(&self, book_id: &str) -> Result<BookInfo> {
        let book_id = book_id.to_string();
        let client = self.client.clone();

        self.try_with_fallback(move |base_url| {
            let book_id = book_id.clone();
            let client = client.clone();
            async move {
                let url = format!("{}/api/detail", base_url);
                
                let resp = client
                    .get(&url)
                    .query(&[("book_id", &book_id)])
                    .send()
                    .await?;

                let data: serde_json::Value = resp.json().await?;
                
                if data["code"].as_i64() != Some(200) {
                    return Err(anyhow!("API 返回错误: {:?}", data["message"]));
                }

                // 处理嵌套的 data 结构
                let book_data = if data["data"]["data"].is_object() {
                    &data["data"]["data"]
                } else {
                    &data["data"]
                };

                // 检查书籍是否下架
                if book_data["message"].as_str() == Some("BOOK_REMOVE") {
                    return Err(anyhow!("书籍已下架"));
                }

                Ok(BookInfo {
                    book_id: book_id.to_string(),
                    book_name: book_data["book_name"].as_str().unwrap_or("未知").to_string(),
                    author: book_data["author"].as_str().unwrap_or("未知").to_string(),
                    cover_url: book_data["thumb_url"].as_str()
                        .or_else(|| book_data["cover_url"].as_str())
                        .unwrap_or("")
                        .to_string(),
                    description: book_data["abstract"].as_str().unwrap_or("").to_string(),
                    word_count: book_data["word_count"].as_i64(),
                    chapter_count: book_data["serial_count"].as_i64()
                        .or_else(|| book_data["chapter_count"].as_i64()),
                    category: book_data["category"].as_str().map(|s| s.to_string()),
                    status: book_data["creation_status"].as_str().map(|s| s.to_string()),
                })
            }
        }).await
    }

    /// 获取章节目录
    pub async fn get_directory(&self, book_id: &str) -> Result<Vec<Chapter>> {
        // 先尝试 /api/directory
        if let Ok(chapters) = self.try_directory_api(book_id).await {
            if !chapters.is_empty() {
                return Ok(chapters);
            }
        }
        
        // 回退到 /api/book
        self.try_book_api(book_id).await
    }
    
    async fn try_directory_api(&self, book_id: &str) -> Result<Vec<Chapter>> {
        let book_id = book_id.to_string();
        let client = self.client.clone();

        self.try_with_fallback(move |base_url| {
            let book_id = book_id.clone();
            let client = client.clone();
            async move {
                let url = format!("{}/api/directory", base_url);
                
                let resp = client
                    .get(&url)
                    .query(&[("book_id", &book_id)])
                    .send()
                    .await?;

                let data: serde_json::Value = resp.json().await?;
                
                if data["code"].as_i64() != Some(200) {
                    return Err(anyhow!("directory API 返回错误"));
                }

                let lists = data["data"]["lists"].as_array()
                    .ok_or_else(|| anyhow!("无法获取章节列表"))?;

                let chapters: Vec<Chapter> = lists
                    .iter()
                    .enumerate()
                    .filter_map(|(idx, ch)| {
                        Some(Chapter {
                            id: ch["item_id"].as_str()?.to_string(),
                            title: ch["title"].as_str().unwrap_or("未知章节").to_string(),
                            index: idx,
                        })
                    })
                    .collect();

                Ok(chapters)
            }
        }).await
    }
    
    async fn try_book_api(&self, book_id: &str) -> Result<Vec<Chapter>> {
        let book_id = book_id.to_string();
        let client = self.client.clone();

        self.try_with_fallback(move |base_url| {
            let book_id = book_id.clone();
            let client = client.clone();
            async move {
                let url = format!("{}/api/book", base_url);
                
                let resp = client
                    .get(&url)
                    .query(&[("book_id", &book_id)])
                    .send()
                    .await?;

                let data: serde_json::Value = resp.json().await?;
                
                if data["code"].as_i64() != Some(200) {
                    return Err(anyhow!("book API 返回错误: {:?}", data["message"]));
                }
                
                let inner = &data["data"]["data"];
                let mut chapters = Vec::new();
                
                // 尝试从 chapterListWithVolume 获取
                if let Some(volumes) = inner["chapterListWithVolume"].as_array() {
                    let mut idx = 0;
                    for volume in volumes {
                        if let Some(vol_chapters) = volume.as_array() {
                            for ch in vol_chapters {
                                let item_id = ch["itemId"].as_str()
                                    .or_else(|| ch["item_id"].as_str());
                                if let Some(id) = item_id {
                                    chapters.push(Chapter {
                                        id: id.to_string(),
                                        title: ch["title"].as_str().unwrap_or("未知章节").to_string(),
                                        index: idx,
                                    });
                                    idx += 1;
                                }
                            }
                        }
                    }
                }
                
                // 如果还是空的，尝试从 allItemIds 生成
                if chapters.is_empty() {
                    if let Some(ids) = inner["allItemIds"].as_array() {
                        for (idx, id) in ids.iter().enumerate() {
                            if let Some(id_str) = id.as_str() {
                                chapters.push(Chapter {
                                    id: id_str.to_string(),
                                    title: format!("第{}章", idx + 1),
                                    index: idx,
                                });
                            }
                        }
                    }
                }
                
                if chapters.is_empty() {
                    return Err(anyhow!("无法从任何 API 获取章节列表"));
                }

                Ok(chapters)
            }
        }).await
    }

    /// 获取单个章节内容
    pub async fn get_chapter_content(&self, item_id: &str) -> Result<String> {
        let item_id = item_id.to_string(); // Clone for closure
        let client = self.client.clone();
        
        self.try_with_fallback(move |base_url| {
            let item_id = item_id.clone();
            let client = client.clone();
            async move {
                let url = format!("{}/api/content", base_url);
                let resp = client
                    .get(&url)
                    .query(&[
                        ("item_id", item_id.as_str()),
                        ("tab", "小说"),
                    ])
                    .send()
                    .await?;

                let data: serde_json::Value = resp.json().await?;
                
                if data["code"].as_i64() != Some(200) {
                    return Err(anyhow!("API 返回错误: {:?}", data["message"]));
                }

                let content = data["data"]["content"].as_str()
                    .or_else(|| data["data"].as_str())
                    .unwrap_or("");
                    
                if content.trim().is_empty() {
                    return Err(anyhow!("内容为空"));
                }

                Ok(process_content(content))
            }
        }).await
    }

    /// 极速模式 - 获取整本书内容
    pub async fn get_full_content(&self, book_id: &str) -> Result<HashMap<String, String>> {
        let book_id = book_id.to_string();
        let client = self.client.clone();

        self.try_with_fallback(move |base_url| {
            let book_id = book_id.clone();
            let client = client.clone();
            async move {
                // 尝试批量模式
                let url = format!("{}/api/content", base_url);
                
                let resp = client
                    .get(&url)
                    .query(&[
                        ("book_id", book_id.as_str()),
                        ("tab", "批量"),
                    ])
                    .send()
                    .await?;

                let data: serde_json::Value = resp.json().await?;
                
                if data["code"].as_i64() != Some(200) {
                    return Err(anyhow!("极速模式不可用"));
                }

                let mut content_map = HashMap::new();

                // 尝试解析批量内容
                if let Some(lists) = data["data"]["lists"].as_array() {
                    for item in lists {
                        if let (Some(id), Some(content)) = (
                            item["item_id"].as_str(),
                            item["content"].as_str()
                        ) {
                            content_map.insert(id.to_string(), process_content(content));
                        }
                    }
                }

                if content_map.is_empty() {
                    return Err(anyhow!("批量模式返回空内容"));
                }

                Ok(content_map)
            }
        }).await
    }

    /// 尝试多个 API 节点
    pub async fn try_with_fallback<F, T, Fut>(&self, operation: F) -> Result<T>
    where
        F: Fn(String) -> Fut,
        Fut: std::future::Future<Output = Result<T>>,
    {
        for base_url in API_SOURCES {
            match operation(base_url.to_string()).await {
                Ok(result) => return Ok(result),
                Err(_) => continue,
            }
        }
        Err(anyhow!("所有 API 节点均不可用"))
    }
}

impl Default for FanqieApi {
    fn default() -> Self {
        Self::new()
    }
}

/// 处理章节内容，清理 HTML 标签等
fn process_content(content: &str) -> String {
    use regex::Regex;
    
    let mut result = content.to_string();
    
    // 替换 br 和 p 标签为换行
    let br_re = Regex::new(r"<br\s*/?>").unwrap();
    result = br_re.replace_all(&result, "\n").to_string();
    
    let p_open_re = Regex::new(r"<p[^>]*>").unwrap();
    result = p_open_re.replace_all(&result, "\n").to_string();
    
    let p_close_re = Regex::new(r"</p>").unwrap();
    result = p_close_re.replace_all(&result, "\n").to_string();
    
    // 移除其他 HTML 标签
    let html_re = Regex::new(r"<[^>]+>").unwrap();
    result = html_re.replace_all(&result, "").to_string();
    
    // 清理多余空白
    let space_re = Regex::new(r"[ \t]+").unwrap();
    result = space_re.replace_all(&result, " ").to_string();
    
    let leading_space_re = Regex::new(r"\n[ \t]+").unwrap();
    result = leading_space_re.replace_all(&result, "\n").to_string();
    
    let trailing_space_re = Regex::new(r"[ \t]+\n").unwrap();
    result = trailing_space_re.replace_all(&result, "\n").to_string();
    
    // 规范化多个换行
    let multi_newline_re = Regex::new(r"\n{3,}").unwrap();
    result = multi_newline_re.replace_all(&result, "\n\n").to_string();
    
    // 处理段落
    let lines: Vec<&str> = result.lines().collect();
    let paragraphs: Vec<&str> = lines.iter()
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .collect();
    
    paragraphs.join("\n\n")
}
