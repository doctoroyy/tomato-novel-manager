# 🍅 Tomato Novel Manager (Fanqie Novel Downloader)

<div align="center">
  <img src="app-icon.svg" width="128" height="128" alt="Tomato Novel Manager Icon" />
  <h1>Tomato Novel Manager</h1>
  <p>
    <strong>English</strong> | <a href="#中文说明">中文说明</a>
  </p>
</div>

A modern, cross-platform desktop application for downloading novels from Fanqie Novel, built with Tauri v2 + React + Rust.

![License](https://img.shields.io/badge/license-MIT-blue)
![Tauri](https://img.shields.io/badge/Tauri-v2-orange)
![Build](https://img.shields.io/github/actions/workflow/status/POf-L/Fanqie-novel-Downloader/release.yml)

## ✨ Features

- **Blazing Fast**: Built with Rust for high performance and low memory usage.
- **Cross-Platform**: Windows, macOS, and Linux support.
- **Smart Search**: Search books directly within the app.
- **Multiple Formats**: Export novels to **TXT** or **EPUB**.
- **Resilient**: Auto-fallback mechanism ensures downloads work even when some API nodes are down.
- **Modern UI**: Clean, responsive interface built with React.

## 🚀 Getting Started

### Prerequisites

- [Node.js](https://nodejs.org/) (v16+)
- [Rust](https://www.rust-lang.org/) (stable)
- [pnpm](https://pnpm.io/) (recommended)

### Installation

1. Clone the repository:
   ```bash
   git clone https://github.com/doctoroyy/tomato-novel-manager.git
   cd tomato-novel-manager
   ```

2. Install dependencies:
   ```bash
   pnpm install
   ```

3. Run in development mode:
   ```bash
   pnpm tauri dev
   ```

### Building for Production

To build the application for your operating system:

```bash
pnpm tauri build
```

The output will be in `src-tauri/target/release/bundle`.

## 🛠 Tech Stack

- **Frontend**: React, TypeScript, Vite, CSS
- **Backend (Core)**: Rust, Tauri, Reqwest, Tokio
- **Packaging**: GitHub Actions

---

<a name="中文说明"></a>

# 🍅 番茄小说管理器 (Tomato Novel Manager)

<div align="center">
  <img src="app-icon.svg" width="128" height="128" alt="Logo" />
</div>

一个主要基于 Tauri v2 + React + Rust 构建的现代化跨平台番茄小说下载器。

## ✨ 功能特性

- **极速体验**: 基于 Rust 构建，高性能且低内存占用。
- **跨平台支持**: 支持 Windows, macOS 和 Linux。
- **智能搜索**: 内置书籍搜索功能。
- **多格式导出**: 支持导出为 **TXT** 或 **EPUB** 格式。
- **高可用性**: 内置 API 自动故障转移机制，确保下载稳定。
- **现代界面**: 简洁、响应式的用户界面。

## 🚀 快速上手

### 环境要求

- [Node.js](https://nodejs.org/) (v16+)
- [Rust](https://www.rust-lang.org/) (stable)
- [pnpm](https://pnpm.io/) (推荐)

### 安装步骤

1. 克隆仓库：
   ```bash
   git clone https://github.com/doctoroyy/tomato-novel-manager.git
   cd tomato-novel-manager
   ```

2. 安装依赖：
   ```bash
   pnpm install
   ```

3. 运行开发模式：
   ```bash
   pnpm tauri dev
   ```

### 打包发布

构建适用于您当前系统的安装包：

```bash
pnpm tauri build
```

构建产物将位于 `src-tauri/target/release/bundle` 目录下。

## 📄 许可证

本项目基于 MIT 许可证开源。仅供学习交流使用，请勿用于商业用途。
