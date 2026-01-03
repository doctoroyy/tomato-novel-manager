import { useEffect, useState } from 'react';
import './App.css';

interface Asset {
  name: string;
  browser_download_url: string;
}

interface Release {
  tag_name: string;
  assets: Asset[];
}

const REPO_OWNER = 'doctoroyy';
const REPO_NAME = 'tomato-novel-manager';

function App() {
  const [release, setRelease] = useState<Release | null>(null);
  const [lang, setLang] = useState<'zh' | 'en'>('zh');

  useEffect(() => {
    // Fetch list of releases instead of just latest to be safer
    fetch(`https://api.github.com/repos/${REPO_OWNER}/${REPO_NAME}/releases`)
      .then(res => {
        if (!res.ok) throw new Error('Network response was not ok');
        return res.json();
      })
      .then(data => {
        // Data is an array of releases, take the first one (latest)
        if (Array.isArray(data) && data.length > 0) {
          setRelease(data[0]);
        }
      })
      .catch(console.error);
  }, []);

  const getDownloadLink = (keywords: string[]) => {
    if (!release || !release.assets) return '#';
    
    // Find asset that matches ALL keywords
    const url = release.assets.find(a => {
      const name = a.name.toLowerCase();
      // Special case for macOS universality: if we ask for x64 but there's a universe bin, we might accept it (optional logic)
      // For now, strict matching
      return keywords.every(k => name.includes(k.toLowerCase()));
    })?.browser_download_url || '#';
    
    // Use ghproxy.net mirror which is more stable and has valid SSL
    if (url !== '#' && lang === 'zh') {
      return `https://ghproxy.net/${url}`;
    }
    return url;
  };

  const t = {
    zh: {
      title: '番茄小说下载器',
      subtitle: '更现代、更快速、支持多平台的番茄小说下载工具',
      download: '下载',
      features: {
        fast: '极速下载',
        fastDesc: '基于 Rust 构建，多线程并发，体验飞一般的下载速度。',
        cross: '跨平台',
        crossDesc: '完美支持 Windows (x64), macOS (Intel/Silicon) 和 Linux。',
        format: '多格式',
        formatDesc: '支持导出 TXT 文本或 EPUB 电子书格式，适配各种阅读器。',
      },
      platforms: {
        win: 'Windows 安装包',
        mac: 'macOS (Intel/Silicon)',
        linux: 'Linux (Deb/AppImage)',
      },
      footer: '基于 Tauri v2 构建 • MIT 开源协议'
    },
    en: {
      title: 'Tomato Novel Manager',
      subtitle: 'A modern, fast, and cross-platform downloader for Fanqie Novels.',
      download: 'Download',
      features: {
        fast: 'Blazing Fast',
        fastDesc: 'Built with Rust for high performance and low memory usage.',
        cross: 'Cross Platform',
        crossDesc: 'First-class support for Windows, macOS, and Linux.',
        format: 'Multiple Formats',
        formatDesc: 'Export to TXT or EPUB for your favorite e-reader.',
      },
      platforms: {
        win: 'Windows Installer',
        mac: 'macOS (Universal)',
        linux: 'Linux',
      },
      footer: 'Built with Tauri v2 • MIT License'
    }
  }[lang];

  return (
    <div className="app">
      <div className="navbar container">
        <div className="logo">
          <img src="/logo.svg" alt="Logo" />
          <span>Tomato Manager</span>
        </div>
        <button className="lang-switch" onClick={() => setLang(l => l === 'zh' ? 'en' : 'zh')}>
          {lang === 'zh' ? 'English' : '中文'}
        </button>
      </div>

      <main className="container">
        <section className="hero">
          <div className="hero-gradient" />
          <h1>{t.title}</h1>
          <p>{t.subtitle}</p>
          
          <div className="download-grid">
            {/* Windows */}
            <div className="card">
              <span className="card-icon">🪟</span>
              <h3>Windows</h3>
              <p>{t.platforms.win}</p>
              <div className="btn-group">
                <a href={getDownloadLink(['.msi', 'x64'])} className="btn">
                   x64 .msi
                </a>
                <a href={getDownloadLink(['.exe', 'x64'])} className="btn secondary">
                   x64 .exe
                </a>
              </div>
              <div style={{marginTop: '0.5rem'}}>
                 {/* Fallback or other archs could go here */}
              </div>
            </div>

            {/* macOS */}
            <div className="card">
              <span className="card-icon">🍎</span>
              <h3>macOS</h3>
              <p>{t.platforms.mac}</p>
              <div className="btn-group">
                <a href={getDownloadLink(['.dmg', 'aarch64'])} className="btn">
                   Apple Silicon
                </a>
                <a href={getDownloadLink(['.dmg', 'x64'])} className="btn secondary">
                   Intel (x64)
                </a>
              </div>
              <div style={{marginTop: '1rem', fontSize: '0.8rem', opacity: 0.8, textAlign: 'left', background: 'rgba(0,0,0,0.2)', padding: '8px', borderRadius: '4px'}}>
                <div style={{fontWeight: 'bold', marginBottom: '4px'}}>⚠️ App is damaged?</div>
                <div>Run in Terminal:</div>
                <code style={{display: 'block', background: '#000', padding: '4px', borderRadius: '2px', marginTop: '4px', userSelect: 'all'}}>
                  sudo xattr -cr /Applications/"Tomato Novel Manager.app"
                </code>
              </div>
            </div>

            {/* Linux */}
            <div className="card">
              <span className="card-icon">🐧</span>
              <h3>Linux</h3>
              <p>{t.platforms.linux}</p>
               <div className="btn-group">
                <a href={getDownloadLink(['.deb', 'amd64'])} className="btn">
                   .deb
                </a>
                <a href={getDownloadLink(['.AppImage', 'amd64'])} className="btn secondary">
                   .AppImage
                </a>
              </div>
            </div>
          </div>
          
          <p style={{marginTop: '2rem', fontSize: '0.9rem', opacity: 0.7}}>
            Latest Version: {release?.tag_name || 'Loading...'}
          </p>
        </section>

        <section className="features">
          <div className="feature-grid">
            <div className="feature-item">
              <h3>⚡️ {t.features.fast}</h3>
              <p>{t.features.fastDesc}</p>
            </div>
            <div className="feature-item">
              <h3>🖥️ {t.features.cross}</h3>
              <p>{t.features.crossDesc}</p>
            </div>
            <div className="feature-item">
              <h3>📚 {t.features.format}</h3>
              <p>{t.features.formatDesc}</p>
            </div>
          </div>
        </section>
      </main>

      <footer>
        <p>{t.footer}</p>
        <p>
          <a href={`https://github.com/${REPO_OWNER}/${REPO_NAME}`} target="_blank">
            View on GitHub
          </a>
        </p>
      </footer>
    </div>
  )
}

export default App
