# AkiClick

A lightweight Windows auto-clicker built with Tauri v2, Rust, and React.

[![Release](https://img.shields.io/github/v/release/AkiroMusic/AkiClick)](https://github.com/AkiroMusic/AkiClick/releases/latest)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](LICENSE)
[![Built with Tauri v2](https://img.shields.io/badge/built%20with-Tauri%20v2-24C8D8?logo=tauri&logoColor=black)](https://v2.tauri.app/)
[![Platform](https://img.shields.io/badge/platform-Windows-0078D6?logo=windows)](https://github.com/AkiroMusic/AkiClick/releases/latest)

## Features

- **Three click modes**: Left click, Right click, Double click (one double-click per interval)
- **Customizable interval**: 1 ms to 60,000 ms (effective precision is limited by the Windows timer granularity, roughly 15 ms)
- **Click count**: Set number of clicks or run infinitely (0 = ∞)
- **Global hotkeys**: F9 to start, F10 to stop, F11 to exit (customizable, take effect immediately after saving — no restart needed)
- **Listening gate**: Hotkeys are only active while listening is enabled; turning listening off also stops an in-progress session
- **System tray**: Menu controls (Show / Enable Listening / Quit); closing the window exits the app
- **Always-on top**: Keep the window above other windows
- **Dark/Light theme**: Toggle between themes
- **Interface languages**: English and Chinese (中文)
- **Portable mode**: Place `shubiao.ini` next to the executable — all settings, including theme/language, live in the INI
- **REF compatible**: Uses the same config format as 鼠标连点器

## Installation

### Download

Download the latest release from [Releases](https://github.com/AkiroMusic/AkiClick/releases).

### Build from Source

```bash
# Clone the repository
git clone https://github.com/AkiroMusic/AkiClick.git
cd AkiClick

# Install dependencies
npm install

# Build the application
npm run tauri build
```

The installer will be located at `src-tauri/target/release/bundle/nsis/AkiClick_1.3.0_x64-setup.exe`.

## Usage

1. Run `AkiClick.exe`
2. Configure click mode, interval, and count, then press **Save**
3. Press **Enable Listening**
4. Press **F9** to start clicking
5. Press **F10** to stop clicking
6. Press **F11** to exit the application

Settings are locked while listening is enabled to avoid mid-session changes.

## Configuration

The configuration file `shubiao.ini` is stored at:

- **Standard**: `%APPDATA%\AkiClick\shubiao.ini`
- **Portable**: Same directory as the executable (a `shubiao.ini` next to `AkiClick.exe` is used automatically)

Missing keys and out-of-range values are repaired to defaults on startup.

### Configuration Options

| Option | Description | Default |
|--------|-------------|---------|
| `mode` | Click mode (0=Left, 1=Right, 2=Double) | 0 |
| `freq` | Click interval in milliseconds (1–60000) | 700 |
| `clicktimes` | Number of clicks (0 = infinite, max 999999) | 100 |
| `clickstate` | Unused; kept for 鼠标连点器 compatibility | 1 |
| `showstate` | Unused; kept for 鼠标连点器 compatibility | 0 |
| `left` | **Start** hotkey VK code | 120 (F9) |
| `right` | **Stop** hotkey VK code | 121 (F10) |
| `stop` | **Exit** hotkey VK code | 122 (F11) |
| `theme` | UI theme (`dark` / `light`) | dark |
| `lang` | UI language (`en` / `zh`) | en |
| `always_on_top` | Start with window always on top (0/1) | 1 |

> Note: the legacy key names are inherited from 鼠标连点器 — `left`/`right`/`stop` actually mean start/stop/exit, not mouse buttons.

## Tech Stack

- **Frontend**: React 18, TypeScript, Vite
- **Backend**: Rust, Tauri v2
- **Click Engine**: enigo (SendInput API)

## License

[MIT](LICENSE)

---

# AkiClick

一个轻量级的 Windows 自动点击器，使用 Tauri v2、Rust 和 React 构建。

## 功能特性

- **三种点击模式**：左键点击、右键点击、双击（每个间隔执行一次双击）
- **自定义间隔**：1 毫秒到 60,000 毫秒（实际精度受 Windows 系统定时器粒度限制，约 15 毫秒）
- **点击次数**：设置点击次数或无限运行（0 = ∞）
- **全局热键**：F9 开始，F10 停止，F11 退出（可自定义，保存后立即生效，无需重启）
- **监听开关**：热键仅在启用监听后生效；关闭监听会同时停止正在进行的点击
- **系统托盘**：菜单控制（显示 / 启用监听 / 退出）；关闭窗口即退出程序
- **窗口置顶**：保持窗口在其他窗口上方
- **深色/浅色主题**：切换主题
- **界面语言**：英文和中文
- **便携模式**：在可执行文件旁放置 `shubiao.ini` —— 全部设置（含主题/语言）都保存在 INI 中
- **REF 兼容**：使用与鼠标连点器相同的配置格式

## 安装

### 下载

从 [Releases](https://github.com/AkiroMusic/AkiClick/releases) 下载最新版本。

### 从源码构建

```bash
# 克隆仓库
git clone https://github.com/AkiroMusic/AkiClick.git
cd AkiClick

# 安装依赖
npm install

# 构建应用
npm run tauri build
```

安装包位于 `src-tauri/target/release/bundle/nsis/AkiClick_1.3.0_x64-setup.exe`。

## 使用方法

1. 运行 `AkiClick.exe`
2. 配置点击模式、间隔和次数，然后点击**保存**
3. 点击**启用监听**
4. 按 **F9** 开始点击
5. 按 **F10** 停止点击
6. 按 **F11** 退出应用

启用监听期间设置会被锁定，避免点击过程中途变更。

## 配置

配置文件 `shubiao.ini` 存储位置：

- **标准模式**：`%APPDATA%\AkiClick\shubiao.ini`
- **便携模式**：与可执行文件位于同一目录（若 `AkiClick.exe` 旁存在 `shubiao.ini`，则自动使用）

启动时会自动将缺失的键和超出范围的值修复为默认值。

### 配置选项

| 选项 | 描述 | 默认值 |
|------|------|--------|
| `mode` | 点击模式（0=左键，1=右键，2=双击） | 0 |
| `freq` | 点击间隔（毫秒，1–60000） | 700 |
| `clicktimes` | 点击次数（0=无限，最大 999999） | 100 |
| `clickstate` | 未使用；仅为鼠标连点器兼容保留 | 1 |
| `showstate` | 未使用；仅为鼠标连点器兼容保留 | 0 |
| `left` | **开始**热键 VK 码 | 120 (F9) |
| `right` | **停止**热键 VK 码 | 121 (F10) |
| `stop` | **退出**热键 VK 码 | 122 (F11) |
| `theme` | 界面主题（`dark` / `light`） | dark |
| `lang` | 界面语言（`en` / `zh`） | en |
| `always_on_top` | 启动时窗口置顶（0/1） | 1 |

> 注意：遗留键名继承自鼠标连点器 —— `left`/`right`/`stop` 实际含义是开始/停止/退出，与鼠标按键无关。

## 技术栈

- **前端**：React 18、TypeScript、Vite
- **后端**：Rust、Tauri v2
- **点击引擎**：enigo（SendInput API）

## 许可证

[MIT](LICENSE)
