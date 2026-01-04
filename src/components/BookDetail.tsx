import { useState, useEffect } from "react";
import { listen } from "@tauri-apps/api/event";
import { save } from "@tauri-apps/plugin-dialog";
import {
  BookInfo,
  Chapter,
  DownloadOptions,
  DownloadProgress,
  DownloadResult,
  getChapters,
  downloadBook,
} from "../lib/api";
import "./BookDetail.css";

interface BookDetailProps {
  book: BookInfo;
  onBack: () => void;
}

export function BookDetail({ book, onBack }: BookDetailProps) {
  const [chapters, setChapters] = useState<Chapter[]>([]);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);
  const [format, setFormat] = useState<"txt" | "epub">("txt");
  const [downloading, setDownloading] = useState(false);
  const [progress, setProgress] = useState<DownloadProgress | null>(null);
  const [result, setResult] = useState<DownloadResult | null>(null);

  useEffect(() => {
    loadChapters();
  }, [book.book_id]);

  useEffect(() => {
    const unlisten = listen<DownloadProgress>("download-progress", (event) => {
      if (event.payload.book_id === book.book_id) {
        setProgress(event.payload);
      }
    });

    return () => {
      unlisten.then((fn) => fn());
    };
  }, [book.book_id]);

  const loadChapters = async () => {
    setLoading(true);
    setError(null);
    try {
      const data = await getChapters(book.book_id);
      setChapters(data);
    } catch (err) {
      setError(err instanceof Error ? err.message : "获取章节列表失败");
    } finally {
      setLoading(false);
    }
  };

  const handleDownload = async () => {
    try {
      // 选择保存路径
      const savePath = await save({
        defaultPath: `${book.book_name}.${format}`,
        filters: [
          {
            name: format === "epub" ? "EPUB 文件" : "文本文件",
            extensions: [format],
          },
        ],
      });

      if (!savePath) return;

      // 获取目录路径 (兼容 Windows 和 Unix)
      const lastSlash = Math.max(
        savePath.lastIndexOf("/"),
        savePath.lastIndexOf("\\")
      );
      const dirPath = savePath.substring(0, lastSlash);

      setDownloading(true);
      setProgress(null);
      setResult(null);

      const options: DownloadOptions = {
        book_id: book.book_id,
        save_path: dirPath,
        format,
      };

      const downloadResult = await downloadBook(options);
      setResult(downloadResult);
      
      // 保存下载历史到缓存
      if (downloadResult.success) {
        try {
          const history = JSON.parse(localStorage.getItem("downloadHistory") || "[]");
          const newEntry = {
            book_id: book.book_id,
            book_name: book.book_name,
            author: book.author,
            format,
            file_path: downloadResult.file_path,
            timestamp: new Date().toISOString(),
          };
          history.unshift(newEntry);
          // 只保留最近 50 条记录
          if (history.length > 50) {
            history.pop();
          }
          localStorage.setItem("downloadHistory", JSON.stringify(history));
        } catch (e) {
          // 如果缓存数据损坏，重置为空数组
          console.error("Failed to save download history:", e);
          localStorage.setItem("downloadHistory", "[]");
        }
      }
    } catch (err) {
      setResult({
        success: false,
        error: err instanceof Error ? err.message : String(err),
        book_name: book.book_name,
      });
    } finally {
      setDownloading(false);
    }
  };

  const formatWordCount = (count?: number) => {
    if (!count) return "";
    if (count >= 10000) {
      return `${(count / 10000).toFixed(1)}万字`;
    }
    return `${count}字`;
  };

  return (
    <div className="book-detail">
      <button className="back-button" onClick={onBack}>
        ← 返回搜索
      </button>

      <div className="book-header">
        <div className="cover-section">
          {book.cover_url ? (
            <img
              src={book.cover_url}
              alt={book.book_name}
              className="cover-image"
            />
          ) : (
            <div className="no-cover">📖</div>
          )}
        </div>
        <div className="info-section">
          <h1 className="book-name">{book.book_name}</h1>
          <p className="author">作者: {book.author}</p>
          <div className="meta-info">
            {book.category && <span className="tag">{book.category}</span>}
            {book.word_count && (
              <span className="tag">{formatWordCount(book.word_count)}</span>
            )}
            {book.chapter_count && (
              <span className="tag">{book.chapter_count} 章</span>
            )}
            {book.status && <span className="tag status">{book.status}</span>}
          </div>
          <p className="description">{book.description}</p>
        </div>
      </div>

      <div className="download-section">
        <h3>下载选项</h3>
        <div className="format-selector">
          <label className={`format-option ${format === "txt" ? "active" : ""}`}>
            <input
              type="radio"
              name="format"
              value="txt"
              checked={format === "txt"}
              onChange={() => setFormat("txt")}
              disabled={downloading}
            />
            <span className="format-icon">📄</span>
            <span className="format-name">TXT 格式</span>
            <span className="format-desc">纯文本，兼容性好</span>
          </label>
          <label className={`format-option ${format === "epub" ? "active" : ""}`}>
            <input
              type="radio"
              name="format"
              value="epub"
              checked={format === "epub"}
              onChange={() => setFormat("epub")}
              disabled={downloading}
            />
            <span className="format-icon">📚</span>
            <span className="format-name">EPUB 格式</span>
            <span className="format-desc">适合电子书阅读器</span>
          </label>
        </div>

        <button
          className="download-button"
          onClick={handleDownload}
          disabled={downloading || chapters.length === 0}
        >
          {downloading ? "下载中..." : `开始下载 (${chapters.length} 章)`}
        </button>

        {progress && downloading && (
          <div className="progress-section">
            <div className="progress-bar">
              <div
                className="progress-fill"
                style={{ width: `${progress.percent}%` }}
              />
            </div>
            <p className="progress-text">{progress.message}</p>
          </div>
        )}

        {result && (
          <div className={`result-message ${result.success ? "success" : "error"}`}>
            {result.success ? (
              <>
                <span className="result-icon">✅</span>
                <span>下载完成！文件保存到: {result.file_path}</span>
              </>
            ) : (
              <>
                <span className="result-icon">❌</span>
                <span>下载失败: {result.error}</span>
              </>
            )}
          </div>
        )}
      </div>

      <div className="chapters-section">
        <h3>章节目录 {!loading && `(${chapters.length} 章)`}</h3>
        {loading ? (
          <div className="loading">加载中...</div>
        ) : error ? (
          <div className="error">{error}</div>
        ) : (
          <div className="chapter-list">
            {chapters.slice(0, 50).map((ch) => (
              <div key={ch.id} className="chapter-item">
                <span className="chapter-index">{ch.index + 1}</span>
                <span className="chapter-title">{ch.title}</span>
              </div>
            ))}
            {chapters.length > 50 && (
              <div className="more-chapters">
                还有 {chapters.length - 50} 章未显示...
              </div>
            )}
          </div>
        )}
      </div>
    </div>
  );
}
