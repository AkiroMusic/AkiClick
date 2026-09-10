# AkiClick

A lightweight Windows auto-clicker built with Tauri v2, Rust, and React.

## Features

- **Three click modes**: Left click, Right click, Double click
- **Customizable interval**: Set click interval from 1ms to 60,000ms
- **Click count**: Set number of clicks or run infinitely (0 = ∞)
- **Global hotkeys**: F9 to start, F10 to stop, F11 to exit (customizable)
- **System tray**: Minimize to tray with menu controls
- **Always-on-top**: Keep window above other windows
- **Dark/Light theme**: Toggle between themes
- **Portable mode**: Place `shubiao.ini` next to the executable
- **REF compatible**: Uses same config format as 鼠标连点器

## Installation

### Download

Download the latest release from [Releases](https://github.com/AkiroDev/AkiClick/releases).

### Build from Source

```bash
# Clone the repository
git clone https://github.com/AkiroDev/AkiClick.git
cd AkiClick

# Install dependencies
npm install

# Build the application
npm run tauri build
```

The installer will be located at `src-tauri/target/release/bundle/nsis/AkiClick_1.0.0_x64-setup.exe`.

## Usage

1. Run `AkiClick.exe`
2. Configure click mode, interval, and count
3. Press **F9** to start clicking
4. Press **F10** to stop clicking
5. Press **F11** to exit the application

### Portable Mode

Create a `shubiao.ini` file next to `AkiClick.exe` with the following content:

```ini
mode=0
freq=700
clicktimes=100
clickstate=1
showstate=0
left=120
right=121
stop=122
```

## Configuration

The configuration file `shubiao.ini` is stored at:

- **Standard**: `%APPDATA%\AkiClick\shubiao.ini`
- **Portable**: Same directory as the executable

### Configuration Options

| Option | Description | Default |
|--------|-------------|---------|
| `mode` | Click mode (0=Left, 1=Right, 2=Double) | 0 |
| `freq` | Click interval in milliseconds | 700 |
| `clicktimes` | Number of clicks (0 = infinite) | 100 |
| `clickstate` | Click state (1=enabled) | 1 |
| `showstate` | Show state (0=hidden) | 0 |
| `left` | Start hotkey VK code | 120 (F9) |
| `right` | Stop hotkey VK code | 121 (F10) |
| `stop` | Exit hotkey VK code | 122 (F11) |

## Tech Stack

- **Frontend**: React 18, TypeScript, Tailwind CSS
- **Backend**: Rust, Tauri v2
- **Click Engine**: enigo (SendInput API)

## License

MIT License

---

# AkiClick

一个轻量级的 Windows 自动点击器，使用 Tauri v2、Rust 和 React 构建。

## 功能特性

- **三种点击模式**：左键点击、右键点击、双击
- **自定义间隔**：设置 1ms 到 60,000ms 的点击间隔
- **点击次数**：设置点击次数或无限运行（0 = ∞）
- **全局热键**：F9 开始，F10 停止，F11 退出（可自定义）
- **系统托盘**：最小化到托盘，支持菜单控制
- **窗口置顶**：保持窗口在其他窗口上方
- **深色/浅色主题**：切换主题
- **便携模式**：在可执行文件旁放置 `shubiao.ini`
- **REF 兼容**：使用与鼠标连点器相同的配置格式

## 安装

### 下载

从 [Releases](https://github.com/AkiroDev/AkiClick/releases) 下载最新版本。

### 从源码构建

```bash
# 克隆仓库
git clone https://github.com/AkiroDev/AkiClick.git
cd AkiClick

# 安装依赖
npm install

# 构建应用
npm run tauri build
```

安装包位于 `src-tauri/target/release/bundle/nsis/AkiClick_1.0.0_x64-setup.exe`。

## 使用方法

1. 运行 `AkiClick.exe`
2. 配置点击模式、间隔和次数
3. 按 **F9** 开始点击
4. 按 **F10** 停止点击
5. 按 **F11** 退出应用

### 便携模式

在 `AkiClick.exe` 旁边创建 `shubiao.ini` 文件，内容如下：

```ini
mode=0
freq=700
clicktimes=100
clickstate=1
showstate=0
left=120
right=121
stop=122
```

## 配置

配置文件 `shubiao.ini` 存储位置：

- **标准模式**：`%APPDATA%\AkiClick\shubiao.ini`
- **便携模式**：与可执行文件相同目录

### 配置选项

| 选项 | 描述 | 默认值 |
|------|------|--------|
| `mode` | 点击模式（0=左键，1=右键，2=双击） | 0 |
| `freq` | 点击间隔（毫秒） | 700 |
| `clicktimes` | 点击次数（0=无限） | 100 |
| `clickstate` | 点击状态（1=启用） | 1 |
| `showstate` | 显示状态（0=隐藏） | 0 |
| `left` | 开始热键 VK 码 | 120 (F9) |
| `right` | 停止热键 VK 码 | 121 (F10) |
| `stop` | 退出热键 VK 码 | 122 (F11) |

## 技术栈

- **前端**：React 18、TypeScript、Tailwind CSS
- **后端**：Rust、Tauri v2
- **点击引擎**：enigo（SendInput API）

## 许可证

MIT 许可证