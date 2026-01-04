import { useState, useEffect } from "react";
import { searchBooks, BookInfo, SearchResult } from "../lib/api";
import "./SearchView.css";

interface SearchViewProps {
  onSelectBook: (book: BookInfo) => void;
}

const LAST_KEYWORD_KEY = "lastSearchKeyword";

export function SearchView({ onSelectBook }: SearchViewProps) {
  const [keyword, setKeyword] = useState("");
  const [results, setResults] = useState<SearchResult | null>(null);
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);

  // 加载上次搜索的关键词
  useEffect(() => {
    const lastKeyword = localStorage.getItem(LAST_KEYWORD_KEY);
    if (lastKeyword) {
      setKeyword(lastKeyword);
    }
  }, []);

  const handleSearch = async () => {
    if (!keyword.trim()) return;

    setLoading(true);
    setError(null);

    try {
      const result = await searchBooks(keyword.trim());
      setResults(result);
      // 保存搜索关键词到缓存
      localStorage.setItem(LAST_KEYWORD_KEY, keyword.trim());
    } catch (err) {
      setError(err instanceof Error ? err.message : "搜索失败");
    } finally {
      setLoading(false);
    }
  };

  const handleKeyPress = (e: React.KeyboardEvent) => {
    if (e.key === "Enter") {
      handleSearch();
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
    <div className="search-view">
      <div className="search-header">
        <h2>🍅 番茄小说下载器</h2>
        <p className="subtitle">搜索并下载你喜欢的小说</p>
      </div>

      <div className="search-box">
        <input
          type="text"
          placeholder="输入书名或作者搜索..."
          value={keyword}
          onChange={(e) => setKeyword(e.target.value)}
          onKeyPress={handleKeyPress}
          disabled={loading}
        />
        <button onClick={handleSearch} disabled={loading || !keyword.trim()}>
          {loading ? "搜索中..." : "搜索"}
        </button>
      </div>

      {error && <div className="error-message">{error}</div>}

      {results && (
        <div className="search-results">
          <div className="results-header">
            找到 {results.total} 本相关书籍
          </div>
          <div className="book-list">
            {results.books.map((book) => (
              <div
                key={book.book_id}
                className="book-card"
                onClick={() => onSelectBook(book)}
              >
                <div className="book-cover">
                  {book.cover_url ? (
                    <img src={book.cover_url} alt={book.book_name} />
                  ) : (
                    <div className="no-cover">📖</div>
                  )}
                </div>
                <div className="book-info">
                  <h3 className="book-title">{book.book_name}</h3>
                  <p className="book-author">作者: {book.author}</p>
                  <div className="book-meta">
                    {book.category && (
                      <span className="category">{book.category}</span>
                    )}
                    {book.word_count && (
                      <span className="word-count">
                        {formatWordCount(book.word_count)}
                      </span>
                    )}
                    {book.chapter_count && (
                      <span className="chapter-count">
                        {book.chapter_count}章
                      </span>
                    )}
                  </div>
                  <p className="book-desc">
                    {book.description?.slice(0, 100)}
                    {book.description?.length > 100 ? "..." : ""}
                  </p>
                </div>
              </div>
            ))}
          </div>
          {results.has_more && (
            <div className="load-more">
              <button onClick={() => {}}>加载更多</button>
            </div>
          )}
        </div>
      )}

      {!results && !loading && (
        <div className="empty-state">
          <div className="empty-icon">🔍</div>
          <p>输入关键词开始搜索</p>
        </div>
      )}
    </div>
  );
}
