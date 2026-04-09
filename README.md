<a id="english"></a>

# 🦙 DeveLlamaGUI Pro

**A lightweight, native GUI for [llama.cpp](https://github.com/ggerganov/llama.cpp) server — written in Rust.**

[![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)
[![Platform](https://img.shields.io/badge/platform-Windows-green.svg)](#)
[![Size](https://img.shields.io/badge/size-~3MB-orange.svg)](#)

<p align="center">
  <img src="screenshot_cyberpunk.png" width="480" alt="Cyberpunk Theme">
  <img src="screenshot_sakura_pink.png" width="480" alt="Sakura Pink Theme">
</p>

<p align="center"><em>Cyberpunk (default) & Sakura Pink themes</em></p>

## 🌐 Languages / 语言

[English](#english) | [简体中文](#简体中文) | [繁體中文](#繁體中文) | [日本語](#日本語) | [한국어](#한국어)

---

## Why DeveLlamaGUI Pro?

Running local LLMs with `llama.cpp` is powerful, but the command-line interface is cumbersome — you have to memorize dozens of flags, type long commands, and there's no way to adjust parameters at runtime.

Third-party GUIs like LM Studio bundle their own engine, resulting in **massive downloads (several GB)** and **high memory usage (1GB+)** just for the wrapper.

**DeveLlamaGUI Pro** takes a different approach:

| | DeveLlamaGUI Pro | LM Studio | llama.cpp CLI |
|---|---|---|---|
| **Size** | ~3 MB | ~2 GB+ | ~50 MB |
| **Memory** | ~100 MB | ~1 GB+ | ~50 MB |
| **GUI** | ✅ Native | ✅ Electron | ❌ CLI only |
| **Runtime tuning** | ✅ | ❌ | ❌ |
| **Preset management** | ✅ | ✅ | ❌ |
| **Uses your llama.cpp** | ✅ | ❌ (bundled) | ✅ |

### Key Features

- 🚀 **Launch & manage** llama-server with a clean GUI — no more typing commands
- ⚡ **Runtime parameter tuning** — adjust temperature, top-k, top-p, repeat penalty on the fly
- 💾 **Preset system** — save, load, and switch between model configurations instantly
- 🌍 **5 languages** — English, 简体中文, 繁體中文, 日本語, 한국어
- 🎨 **6 themes** — including dark, light, and custom accent colors
- 📋 **Console** — real-time log viewer with copy-all and export-to-file
- 🖥️ **Native performance** — built with Rust + egui, minimal resource usage
- ⚙️ **Full parameter support** — GPU layers, context size, batch size, Flash Attention, KV cache type, continuous batching, and more

## Quick Start

1. **Download** the latest release from [Releases](https://github.com/deveuper/DeveLlamaGUI-Pro/releases)
2. **Place** `DeveLlamaGUI-Pro.exe` anywhere on your system
3. **Configure** the path to your `llama-server.exe` and GGUF model file
4. **Click Start** — that's it!

### Requirements

- Windows 10/11
- [llama.cpp](https://github.com/ggerganov/llama.cpp) (just the `llama-server.exe` binary)
- A GGUF model file
- NVIDIA GPU (optional, for GPU acceleration)

## Configuration

All settings are automatically saved to `%APPDATA%/DeveLlamaGUI/config.json` and restored on next launch.

### Supported Parameters

| Parameter | Flag | Description |
|-----------|------|-------------|
| Model Path | `--model` | Path to GGUF model file |
| Host | `--host` | Server host (default: 127.0.0.1) |
| Port | `--port` | Server port (default: 8080) |
| GPU Layers | `--n-gpu-layers` | Number of layers to offload to GPU |
| Context Size | `--ctx-size` | Prompt context size |
| Threads | `--threads` | Number of threads |
| Batch Size | `--batch-size` | Logical batch size |
| Micro Batch | `--ubatch-size` | Physical batch size |
| Flash Attention | `--flash-attn` | Enable Flash Attention |
| KV Cache Type | `--cache-type-k/v` | KV cache quantization |
| Parallel Slots | `--parallel` | Number of parallel sequences |
| Continuous Batching | `--no-cont-batching` | Disable continuous batching |

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
- **rfd** — Native file dialogs

## License

MIT License — feel free to use, modify, and distribute.

---

<a id="简体中文"></a>

## 🀄 简体中文

**一个轻量级的 [llama.cpp](https://github.com/ggerganov/llama.cpp) 服务器原生GUI——用Rust编写。**

用 `llama.cpp` 运行本地大模型很强大，但命令行界面太麻烦了——你得记住几十个参数，输入长命令，而且无法在运行时调整参数。

第三方GUI如LM Studio自带引擎，导致**巨大的下载量（几个GB）**和**高内存占用（1GB+）**。

**DeveLlamaGUI Pro** 采用了不同的方式：

| | DeveLlamaGUI Pro | LM Studio | llama.cpp CLI |
|---|---|---|---|
| **大小** | ~3 MB | ~2 GB+ | ~50 MB |
| **内存** | ~100 MB | ~1 GB+ | ~50 MB |
| **图形界面** | ✅ 原生 | ✅ Electron | ❌ 仅命令行 |
| **运行时调参** | ✅ | ❌ | ❌ |
| **预设管理** | ✅ | ✅ | ❌ |
| **使用自己的llama.cpp** | ✅ | ❌ (内置) | ✅ |

### 核心功能

- 🚀 **启动和管理** llama-server，不再输入命令
- ⚡ **运行时调参** — 实时调整 temperature、top-k、top-p、repeat penalty
- 💾 **预设系统** — 保存、加载、切换模型配置
- 🌍 **5种语言** — English, 简体中文, 繁體中文, 日本語, 한국어
- 🎨 **6种主题** — 包括暗色、亮色和自定义强调色
- 📋 **控制台** — 实时日志查看，支持一键复制和导出文件
- 🖥️ **原生性能** — Rust + egui 构建，极低资源占用
- ⚙️ **全参数支持** — GPU层数、上下文大小、批处理、Flash Attention、KV缓存类型、连续批处理等

---

<a id="繁體中文"></a>

## 🹽 繁體中文

**一個輕量級的 [llama.cpp](https://github.com/ggerganov/llama.cpp) 伺服器原生GUI——用Rust編寫。**

用 `llama.cpp` 運行本地大模型很強大，但命令列介面太麻煩了——你得記住幾十個參數，輸入長命令，而且無法在運行時調整參數。

第三方GUI如LM Studio自帶引擎，導致**巨大的下載量（幾個GB）**和**高記憶體佔用（1GB+）**。

**DeveLlamaGUI Pro** 採用了不同的方式：

### 核心功能

- 🚀 **啟動和管理** llama-server，不再輸入命令
- ⚡ **運行時調參** — 即時調整 temperature、top-k、top-p、repeat penalty
- 💾 **預設系統** — 儲存、載入、切換模型設定
- 🌍 **5種語言** — English, 簡體中文, 繁體中文, 日本語, 한국어
- 🎨 **6種主題** — 包括暗色、亮色和自訂強調色
- 📋 **控制台** — 即時日誌查看，支援一鍵複製和匯出檔案
- 🖥️ **原生效能** — Rust + egui 建構，極低資源佔用
- ⚙️ **全參數支援** — GPU層數、上下文大小、批次處理、Flash Attention、KV快取類型、連續批次處理等

---

<a id="日本語"></a>

## 🇯🇵 日本語

**[llama.cpp](https://github.com/ggerganov/llama.cpp) サーバーのための軽量ネイティブGUI — Rustで作成。**

`llama.cpp`でローカルLLMを実行するのは強力ですが、コマンドラインインターフェースは面倒です——数十のフラグを覚え、長いコマンドを入力する必要があり、実行時にパラメータを調整することもできません。

LM StudioなどのサードパーティGUIは独自のエンジンを同梱しており、**巨大なダウンロード（数GB）**と**高いメモリ使用量（1GB+）**をもたらします。

**DeveLlamaGUI Pro**は別のアプローチを取ります：

### 主な機能

- 🚀 **起動と管理** — llama-serverをGUIで操作、コマンド入力不要
- ⚡ **ランタイムパラメータ調整** — temperature、top-k、top-p、repeat penaltyをリアルタイムで変更
- 💾 **プリセットシステム** — モデル設定の保存、読み込み、切り替え
- 🌍 **5言語対応** — English, 简体中文, 繁體中文, 日本語, 한국어
- 🎨 **6テーマ** — ダーク、ライト、カスタムアクセントカラーなど
- 📋 **コンソール** — リアルタイムログ表示、全コピー＆ファイルエクスポート対応
- 🖥️ **ネイティブパフォーマンス** — Rust + eguiで構築、最小リソース使用
- ⚙️ **全パラメータ対応** — GPUレイヤー、コンテキストサイズ、バッチサイズ、Flash Attention、KVキャッシュタイプ、継続バッチ処理など

---

<a id="한국어"></a>

## 🇰🇷 한국어

**[llama.cpp](https://github.com/ggerganov/llama.cpp) 서버를 위한 가벼운 네이티브 GUI — Rust로 작성됨.**

`llama.cpp`로 로컬 LLM을 실행하는 것은 강력하지만, 명령줄 인터페이스는 번거롭습니다 — 수십 개의 플래그를 외우고, 긴 명령을 입력해야 하며, 실행 중에 매개변수를 조정할 수도 없습니다.

LM Studio 같은 서드파티 GUI는 자체 엔진을 번들로 제공하여 **거대한 다운로드(수 GB)**와 **높은 메모리 사용량(1GB+)**을 초래합니다.

**DeveLlamaGUI Pro**는 다른 접근 방식을 취합니다:

| | DeveLlamaGUI Pro | LM Studio | llama.cpp CLI |
|---|---|---|---|
| **크기** | ~3 MB | ~2 GB+ | ~50 MB |
| **메모리** | ~100 MB | ~1 GB+ | ~50 MB |
| **GUI** | ✅ 네이티브 | ✅ Electron | ❌ CLI만 |
| **런타임 조정** | ✅ | ❌ | ❌ |
| **프리셋 관리** | ✅ | ✅ | ❌ |
| **자체 llama.cpp 사용** | ✅ | ❌ (번들) | ✅ |

### 주요 기능

- 🚀 **시작 및 관리** — GUI로 llama-server 조작, 명령 입력 불필요
- ⚡ **런타임 매개변수 조정** — temperature, top-k, top-p, repeat penalty 실시간 변경
- 💾 **프리셋 시스템** — 모델 설정 저장, 로드, 전환
- 🌍 **5개 언어** — English, 简体中文, 繁體中文, 日本語, 한국어
- 🎨 **6개 테마** — 다크, 라이트, 커스텀 액센트 컬러 등
- 📋 **콘솔** — 실시간 로그 보기, 전체 복사 및 파일 내보내기 지원
- 🖥️ **네이티브 성능** — Rust + egui로 구축, 최소 리소스 사용
- ⚙️ **전체 매개변수 지원** — GPU 레이어, 컨텍스트 크기, 배치 크기, Flash Attention, KV 캐시 유형, 연속 배치 처리 등
