<a id="简体中文"></a>

# 🦙 DeveLlamaGUI Pro

**一个轻量级的 [llama.cpp](https://github.com/ggerganov/llama.cpp) 服务器原生GUI——用 Rust 编写。**

[![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)
[![Platform](https://img.shields.io/badge/platform-Windows-green.svg)](#)
[![Size](https://img.shields.io/badge/size-~3MB-orange.svg)](#)

<p align="center">
  <img src="screenshot_cyberpunk.png" width="480" alt="Cyberpunk 主题">
  <img src="screenshot_sakura_pink.png" width="480" alt="Sakura Pink 主题">
</p>

<p align="center"><em>Cyberpunk（默认）& Sakura Pink 主题</em></p>

## 🌐 Languages / 语言

[简体中文](#简体中文) | [English](#english) | [繁體中文](#繁體中文) | [日本語](#日本語) | [한국어](#한국어)

---

## 为什么选择 DeveLlamaGUI Pro？

用 `llama.cpp` 运行本地大模型很强大，但命令行界面太麻烦了——你得记住几十个参数，输入长命令，而且无法在运行时调整参数。

第三方GUI如 LM Studio 自带引擎，导致**巨大的下载量（几个GB）**和**高内存占用（1GB+）**。

**DeveLlamaGUI Pro** 采用了不同的方式：它只是一个 ~3MB 的外壳，直接调用你自己的 `llama-server.exe`，零捆绑，零冗余。

### 📊 对比一览

| | DeveLlamaGUI Pro | LM Studio | llama.cpp CLI |
|---|---|---|---|
| **容量** | ~3 MB | ~2 GB+ | ~50 MB |
| **GUI 内存占用** | ~35 MB* | ~1 GB+ | N/A |
| **图形界面** | ✅ 原生 | ✅ Electron | ❌ 仅命令行 |
| **运行时调参** | ✅ | ❌ | ❌ |
| **预设管理** | ✅ 7种内置 | ✅ | ❌ |
| **使用自己的llama.cpp** | ✅ | ❌ (内置) | ✅ |

> \* GUI 程序本身仅占 ~35MB 内存（llama-server 进程已运行后）。首次启动约 145MB（含CJK字体加载）。

### 💾 内存占用详情

```
内存占用 (Working Set)
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

首次启动      ████████████████████████████████████░░░░░░░░░░  ~145 MB
              (加载CJK字体 + OpenGL渲染上下文)

模型运行后    ████████████░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░  ~35 MB
              (字体已缓存, repaint频率降低, 系统回收空闲页)

对比 LM Studio  ████████████████████████████████████████████████████████████████████████████████  ~1 GB+
```

### 核心功能

- 🚀 **启动和管理** llama-server，不再输入命令
- ⚡ **运行时调参** — 实时调整 temperature、top-k、top-p、repeat penalty
- 💾 **预设系统** — 7种内置预设 + 自定义保存/加载/删除
- 🔍 **自动验证** — 启动后自动检测 ctx_size 是否匹配，状态栏实时显示
- 🌍 **5种语言** — English, 简体中文, 繁體中文, 日本語, 한국어
- 🎨 **6种主题** — Cyberpunk、Sakura Pink、Minimal Light、Ocean Blue、Midnight、Forest Green
- 📋 **控制台** — 实时日志查看，支持一键复制和导出文件
- 🖥️ **原生性能** — Rust + egui + OpenGL 渲染，极低资源占用
- ⚙️ **全参数支持** — GPU层数、上下文大小、批处理、Flash Attention、KV缓存类型、连续批处理等

### 内置预设

| 预设 | Temperature | Top-K | Top-P | Min-P | Repeat Penalty |
|------|-------------|-------|-------|-------|----------------|
| General Chat | 0.7 | 40 | 0.9 | 0.05 | 1.1 |
| **Code Mode** | **0.6** | **20** | **1.0** | **0.0** | **1.05** |
| Creative Writing | 0.9 | 100 | 0.95 | 0.05 | 1.1 |
| OpenClaw | 0.6 | 40 | 0.9 | 0.05 | 1.1 |
| Roleplay | 0.8 | 100 | 0.95 | 0.05 | 1.1 |
| Math & Logic | 0.3 | 10 | 0.8 | 0.0 | 1.0 |
| Brainstorm | 1.0 | 100 | 0.95 | 0.05 | 1.15 |

## 快速开始

1. **下载** 最新版 [Releases](https://github.com/deveuper/DeveLlamaGUI-Pro/releases)
2. **放置** `DeveLlamaGUI-Pro.exe` 到任意位置
3. **配置** `llama-server.exe` 路径和 GGUF 模型文件路径
4. **点击 Start** — 就这么简单！

### 系统要求

- Windows 10/11 x64
- [llama.cpp](https://github.com/ggerganov/llama.cpp)（只需 `llama-server.exe`）
- GGUF 模型文件
- NVIDIA GPU（可选，用于GPU加速）

## 配置

所有设置自动保存到 `%APPDATA%/DeveLlamaGUI/config.json`，下次启动自动恢复。

### 支持的参数

| 参数 | 命令行标志 | 说明 |
|------|-----------|------|
| 模型路径 | `--model` | GGUF 模型文件路径 |
| 主机 | `--host` | 服务器地址（默认: 127.0.0.1） |
| 端口 | `--port` | 服务器端口（默认: 8080） |
| GPU层数 | `--n-gpu-layers` | 卸载到GPU的层数 |
| 上下文大小 | `--ctx-size` | 提示上下文大小 |
| 线程数 | `--threads` | 线程数量 |
| 批处理大小 | `--batch-size` | 逻辑批大小 |
| 微批大小 | `--ubatch-size` | 物理批大小 |
| Flash Attention | `--flash-attn` | 启用Flash Attention |
| KV缓存类型 | `--cache-type-k/v` | KV缓存量化格式 |
| 并行槽位 | `--parallel` | 并行序列数 |
| 连续批处理 | `--no-cont-batching` | 禁用连续批处理 |

## 从源码构建

```bash
git clone https://github.com/deveuper/DeveLlamaGUI-Pro.git
cd DeveLlamaGUI-Pro
cargo build --release
```

优化后的二进制文件在 `target/release/DeveLlamaGUI-Pro.exe`。

## 技术栈

- **Rust** — 内存安全，极速
- **egui / eframe** — 即时模式GUI框架
- **glow (OpenGL)** — 轻量GPU渲染后端
- **rfd** — 原生文件对话框

## 许可证

MIT License — 自由使用、修改和分发。

---

<a id="english"></a>

## 🇬🇧 English

**A lightweight, native GUI for [llama.cpp](https://github.com/ggerganov/llama.cpp) server — written in Rust.**

Running local LLMs with `llama.cpp` is powerful, but the command-line interface is cumbersome — you have to memorize dozens of flags, type long commands, and there's no way to adjust parameters at runtime.

Third-party GUIs like LM Studio bundle their own engine, resulting in **massive downloads (several GB)** and **high memory usage (1GB+)**.

**DeveLlamaGUI Pro** takes a different approach: it's just a ~3MB shell that calls your own `llama-server.exe`. Zero bundling, zero bloat.

### 📊 Comparison

| | DeveLlamaGUI Pro | LM Studio | llama.cpp CLI |
|---|---|---|---|
| **Size** | ~3 MB | ~2 GB+ | ~50 MB |
| **GUI Memory** | ~35 MB* | ~1 GB+ | N/A |
| **GUI** | ✅ Native | ✅ Electron | ❌ CLI only |
| **Runtime tuning** | ✅ | ❌ | ❌ |
| **Preset management** | ✅ 7 built-in | ✅ | ❌ |
| **Uses your llama.cpp** | ✅ | ❌ (bundled) | ✅ |

> \* The GUI itself uses only ~35MB of memory (after llama-server is running). First launch is ~145MB (CJK font loading + OpenGL context).

### 💾 Memory Usage

```
Memory (Working Set)
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

First launch    ████████████████████████████████████░░░░░░░░░░  ~145 MB
                (CJK fonts + OpenGL render context)

After model     ████████████░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░  ~35 MB
is running      (fonts cached, lower repaint rate, OS reclaims pages)

LM Studio       ████████████████████████████████████████████████████████████████████████████████  ~1 GB+
```

### Key Features

- 🚀 **Launch & manage** llama-server with a clean GUI — no more typing commands
- ⚡ **Runtime parameter tuning** — adjust temperature, top-k, top-p, repeat penalty on the fly
- 💾 **Preset system** — 7 built-in presets + custom save/load/delete
- 🔍 **Auto verification** — ctx_size check after server start, real-time status bar
- 🌍 **5 languages** — English, 简体中文, 繁體中文, 日本語, 한국어
- 🎨 **6 themes** — Cyberpunk, Sakura Pink, Minimal Light, Ocean Blue, Midnight, Forest Green
- 📋 **Console** — real-time log viewer with copy-all and export-to-file
- 🖥️ **Native performance** — Rust + egui + OpenGL rendering, minimal resource usage
- ⚙️ **Full parameter support** — GPU layers, context size, batch size, Flash Attention, KV cache type, continuous batching, and more

### Quick Start

1. **Download** the latest release from [Releases](https://github.com/deveuper/DeveLlamaGUI-Pro/releases)
2. **Place** `DeveLlamaGUI-Pro.exe` anywhere on your system
3. **Configure** the path to your `llama-server.exe` and GGUF model file
4. **Click Start** — that's it!

### Requirements

- Windows 10/11 x64
- [llama.cpp](https://github.com/ggerganov/llama.cpp) (just the `llama-server.exe` binary)
- A GGUF model file
- NVIDIA GPU (optional, for GPU acceleration)

## Build from Source

```bash
git clone https://github.com/deveuper/DeveLlamaGUI-Pro.git
cd DeveLlamaGUI-Pro
cargo build --release
```

The optimized binary will be at `target/release/DeveLlamaGUI-Pro.exe`.

## Tech Stack

- **Rust** — Memory-safe, blazing fast
- **egui / eframe** — Immediate-mode GUI framework
- **glow (OpenGL)** — Lightweight GPU rendering backend
- **rfd** — Native file dialogs

## License

MIT License — feel free to use, modify, and distribute.

---

<a id="繁體中文"></a>

## 🇹🇼 繁體中文

**一個輕量級的 [llama.cpp](https://github.com/ggerganov/llama.cpp) 伺服器原生GUI——用 Rust 編寫。**

用 `llama.cpp` 運行本地大模型很強大，但命令列介面太麻煩了——你得記住幾十個參數，輸入長命令，而且無法在運行時調整參數。

第三方GUI如 LM Studio 自帶引擎，導致**巨大的下載量（幾個GB）**和**高記憶體佔用（1GB+）**。

**DeveLlamaGUI Pro** 採用了不同的方式：它只是一個 ~3MB 的外殼，直接呼叫你自己的 `llama-server.exe`，零捆綁，零冗餘。

### 核心功能

- 🚀 **啟動和管理** llama-server，不再輸入命令
- ⚡ **運行時調參** — 即時調整 temperature、top-k、top-p、repeat penalty
- 💾 **預設系統** — 7種內建預設 + 自訂儲存/載入/刪除
- 🔍 **自動驗證** — 啟動後自動檢測 ctx_size 是否匹配
- 🌍 **5種語言** — English, 簡體中文, 繁體中文, 日本語, 한국어
- 🎨 **6種主題** — 包括暗色、亮色和自訂強調色
- 📋 **控制台** — 即時日誌查看，支援一鍵複製和匯出檔案
- 🖥️ **原生效能** — Rust + egui + OpenGL 建構，極低資源佔用（運行後僅 ~35MB）
- ⚙️ **全參數支援** — GPU層數、上下文大小、批次處理、Flash Attention、KV快取類型等

---

<a id="日本語"></a>

## 🇯🇵 日本語

**[llama.cpp](https://github.com/ggerganov/llama.cpp) サーバーのための軽量ネイティブGUI — Rustで作成。**

`llama.cpp`でローカルLLMを実行するのは強力ですが、コマンドラインインターフェースは面倒です。

LM StudioなどのサードパーティGUIは独自のエンジンを同梱しており、**巨大なダウンロード（数GB）**と**高いメモリ使用量（1GB+）**をもたらします。

**DeveLlamaGUI Pro**は別のアプローチを取ります：~3MBのシェルだけで、自分の`llama-server.exe`を呼び出します。

### 主な機能

- 🚀 **起動と管理** — llama-serverをGUIで操作
- ⚡ **ランタイムパラメータ調整** — temperature、top-k、top-p等をリアルタイムで変更
- 💾 **プリセットシステム** — 7種類の内蔵プリセット + カスタム保存/読み込み/削除
- 🔍 **自動検証** — サーバー起動後にctx_sizeの一致を自動確認
- 🌍 **5言語対応** — English, 简体中文, 繁體中文, 日本語, 한국어
- 🎨 **6テーマ** — ダーク、ライト、カスタムアクセントカラーなど
- 📋 **コンソール** — リアルタイムログ表示、全コピー＆ファイルエクスポート対応
- 🖥️ **ネイティブパフォーマンス** — Rust + egui + OpenGL、最小リソース使用（実行後～35MB）
- ⚙️ **全パラメータ対応** — GPUレイヤー、コンテキストサイズ、Flash Attention等

---

<a id="한국어"></a>

## 🇰🇷 한국어

**[llama.cpp](https://github.com/ggerganov/llama.cpp) 서버를 위한 가벼운 네이티브 GUI — Rust로 작성됨.**

`llama.cpp`로 로컬 LLM을 실행하는 것은 강력하지만, 명령줄 인터페이스는 번거롭습니다.

LM Studio 같은 서드파티 GUI는 자체 엔진을 번들로 제공하여 **거대한 다운로드(수 GB)**와 **높은 메모리 사용량(1GB+)**을 초래합니다.

**DeveLlamaGUI Pro**는 다른 접근 방식을 취합니다: ~3MB 셸로 자신의 `llama-server.exe`를 호출합니다.

### 주요 기능

- 🚀 **시작 및 관리** — GUI로 llama-server 조작
- ⚡ **런타임 매개변수 조정** — temperature, top-k, top-p 등 실시간 변경
- 💾 **프리셋 시스템** — 7개 내장 프리셋 + 커스텀 저장/로드/삭제
- 🔍 **자동 검증** — 서버 시작 후 ctx_size 일치 여부 자동 확인
- 🌍 **5개 언어** — English, 简体中文, 繁體中文, 日本語, 한국어
- 🎨 **6개 테마** — 다크, 라이트, 커스텀 액센트 컬러 등
- 📋 **콘솔** — 실시간 로그 보기, 전체 복사 및 파일 내보내기 지원
- 🖥️ **네이티브 성능** — Rust + egui + OpenGL, 최소 리소스 사용 (실행 후 ~35MB)
- ⚙️ **전체 매개변수 지원** — GPU 레이어, 컨텍스트 크기, Flash Attention 등
