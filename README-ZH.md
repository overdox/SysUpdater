[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](https://opensource.org/licenses/MIT)
[![Rust](https://img.shields.io/badge/Rust-1.70%2B-orange.svg)](https://www.rust-lang.org/)
[![Fedora](https://img.shields.io/badge/Fedora-39%2B-blue.svg)](https://fedoraproject.org/)

> [🇬🇧 English](README.md) | [🇪🇸 Español](README-ES.md) | [🇫🇷 Français](README-FR.md) | [🇩🇪 Deutsch](README-DE.md) | [🇺🇦 Українська](README-UK.md) | 🇨🇳 中文

# Fedora Utility Patchworker - FUP

**Fedora Utility Patchworker - FUP** 是一个基于 Rust 的生产级工具，专为 Fedora Linux 设计，用于自动化系统、Flatpak 和固件更新。它具有安全的默认设置、全面的日志记录和现代化的 CLI 体验。

---

## ✨ 功能特性

| 功能 | 描述 |
|------|------|
| **安全默认值** | 无参数运行时显示帮助 — 需要明确的操作指令 |
| **更新预览** | 使用 `--refresh` 在安装前检查可用更新 |
| **系统更新** | 自动化 dnf5 软件包更新，带元数据刷新 |
| **Flatpak 更新** | 保持所有 Flatpak 应用程序最新 |
| **固件更新** | 可选的 fwupd 集成，用于 UEFI/设备固件 |
| **智能重启检测** | 仅在内核或关键更新需要时提示重启 |
| **网络验证** | 在开始更新前确认网络连接 |
| **优雅关闭** | 干净地处理 CTRL+C，不会导致数据损坏 |
| **全面日志** | 带时间戳的日志记录到 `/var/log/fup.log` |
| **可配置** | 支持 TOML 配置文件，带有合理的默认值 |
| **进度指示器** | 旋转动画和实时输出 |
| **试运行模式** | 预览操作而不执行 |

---

## 📋 系统要求

| 要求 | 详情 |
|------|------|
| **操作系统** | Fedora Linux 39+（或兼容发行版） |
| **包管理器** | `dnf5` 用于系统更新 |
| **可选** | `flatpak` 用于 Flatpak 更新 |
| **可选** | `fwupdmgr` 用于固件更新 |
| **构建** | Rust 1.70+（仅从源码构建时需要） |

---

## 📦 安装

### 预编译二进制文件

1. 从 [Releases](https://github.com/overdox/fedora-utility-patchworker/releases) 页面下载最新的二进制文件
2. 安装：

```bash
sudo mv fup /usr/local/bin/
sudo chmod +x /usr/local/bin/fup
```

### 从源码构建

```bash
# 克隆仓库
git clone https://github.com/overdox/SysUpdater.git
cd SysUpdater

# 优化构建
cargo build --release

# 安装
sudo mv target/release/fup /usr/local/bin/
```

---

## 🚀 使用方法

不带参数运行 `fup` 将显示帮助：

```
╔═══════════════════════════════════════════╗
║               FUP v2.1.2                  ║
║     Fedora Utility Patchworker            ║
╚═══════════════════════════════════════════╝

USAGE

    sudo fup [OPTIONS]

COMMANDS

    -r, --refresh         检查并显示可用更新
    -u, --update-all      更新所有内容（系统 + flatpak）
        --update-system   仅更新系统软件包 (dnf5)
        --update-flatpak  仅更新 Flatpak 应用程序
        --update-firmware 仅更新固件
```

### 快速示例

```bash
# 显示可用更新
sudo fup --refresh

# 更新系统和 Flatpak 软件包
sudo fup --update-all

# 更新所有内容（包括固件）
sudo fup --update-all --firmware

# 仅更新系统软件包
sudo fup --update-system

# 预览将要发生的操作（试运行）
sudo fup --update-all --dry-run

# 静默模式，用于脚本/cron
sudo fup --update-all --quiet --no-reboot-prompt
```

---

## 📖 命令

| 命令 | 简写 | 描述 |
|------|------|------|
| `--refresh` | `-r` | 检查并显示可用更新，不安装 |
| `--update-all` | `-u` | 更新系统软件包和 Flatpak 应用程序 |
| `--update-system` | | 仅更新 dnf5 系统软件包 |
| `--update-flatpak` | | 仅更新 Flatpak 应用程序 |
| `--update-firmware` | | 仅更新固件 |

---

## ⚙️ 选项

| 选项 | 简写 | 描述 |
|------|------|------|
| `--firmware` | `-f` | 在 `--update-all` 时包含固件更新 |
| `--dry-run` | `-n` | 预览操作而不执行 |
| `--no-reboot-prompt` | | 更新后跳过重启提示 |
| `--no-network-check` | | 跳过网络连接验证 |
| `--parallel` | | 并行运行更新 |
| `--config <文件>` | `-c` | 使用自定义配置文件 |
| `--verbose` | `-v` | 增加详细程度（使用 `-vv` 或 `-vvv` 获取更多信息） |
| `--quiet` | `-q` | 最少输出 |

---

## 🔧 配置

SysUpdater 按以下顺序查找配置文件：

1. 通过 `--config` 指定的路径
2. `/etc/fup.toml`
3. `~/.config/fup/config.toml`

### 配置示例

```toml
[system]
enabled = true       # 启用 dnf5 系统更新
auto_remove = true   # 自动删除未使用的软件包
refresh = true       # 更新前刷新元数据

[flatpak]
enabled = true       # 启用 Flatpak 更新
remove_unused = true # 删除未使用的 Flatpak 运行时

[firmware]
enabled = false      # 固件更新默认禁用

[logging]
file = "/var/log/fup.log"
level = "info"       # 选项：error, warn, info, debug, trace

[network]
check_url = "https://fedoraproject.org"
timeout_secs = 10
```

---

## 📤 退出代码

| 代码 | 含义 |
|------|------|
| `0` | 成功 |
| `1` | 一般错误 |
| `130` | 用户取消 (CTRL+C) |

---

## 🤝 贡献

欢迎贡献！以下是您可以提供帮助的方式：

1. 在 GitHub 上 **Fork** 仓库
2. 为您的功能或修复 **创建** 新分支
3. **进行** 更改并提交
4. 在原始仓库上 **打开** Pull Request

请确保您的代码遵循 Rust 最佳实践并包含适当的测试。

---

## 📄 许可证

本项目采用 MIT 许可证 — 详情请参阅 [LICENSE](LICENSE) 文件。

---

<p align="center">
  用 ❤️ 为 Fedora 社区制作
</p>