[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](https://opensource.org/licenses/MIT)
[![Rust](https://img.shields.io/badge/Rust-1.70%2B-orange.svg)](https://www.rust-lang.org/)
[![Fedora](https://img.shields.io/badge/Fedora-39%2B-blue.svg)](https://fedoraproject.org/)

> 🇬🇧 English | [🇪🇸 Español](README-ES.md) | [🇫🇷 Français](README-FR.md) | [🇩🇪 Deutsch](README-DE.md) | [🇺🇦 Українська](README-UK.md) | [🇨🇳 中文](README-ZH.md)

# Fedora Utility Patchworker - FUP

**Fedora Utility Patchworker - FUP** is a production-ready, Rust-based tool designed for automating system, Flatpak, and firmware updates on Fedora Linux. It features safe defaults, comprehensive logging, and a modern CLI experience.

---

## ✨ Features

| Feature | Description |
|---------|-------------|
| **Safe Defaults** | Shows help when run without flags — requires explicit action |
| **Update Preview** | Check available updates before installing with `--refresh` |
| **System Updates** | Automated dnf5 package updates with metadata refresh |
| **Flatpak Updates** | Keep all Flatpak applications current |
| **Firmware Updates** | Optional fwupd integration for UEFI/device firmware |
| **Smart Reboot Detection** | Prompts only when kernel or critical updates require restart |
| **Network Verification** | Confirms connectivity before starting updates |
| **Graceful Shutdown** | Handles CTRL+C cleanly without corruption |
| **Comprehensive Logging** | Timestamped logs to `/var/log/fup.log` |
| **Configurable** | TOML config file support with sensible defaults |
| **Progress Indicators** | Spinners and real-time output |
| **Dry Run Mode** | Preview actions without executing |

---

## 📋 Requirements

| Requirement | Details |
|-------------|---------|
| **Operating System** | Fedora Linux 39+ (or compatible distributions) |
| **Package Manager** | `dnf5` for system updates |
| **Optional** | `flatpak` for Flatpak updates |
| **Optional** | `fwupdmgr` for firmware updates |
| **Build** | Rust 1.70+ (only if building from source) |

---

## 📦 Installation

### Precompiled Binary

1. Download the latest binary from the [Releases](https://github.com/overdox/fedora-utility-patchworker/releases) page
2. Install it:

```bash
sudo mv fup /usr/local/bin/
sudo chmod +x /usr/local/bin/fup
```

### Build from Source

```bash
# Clone the repository
git clone https://github.com/overdox/SysUpdater.git
cd SysUpdater

# Build with optimizations
cargo build --release

# Install
sudo mv target/release/fup /usr/local/bin/
```

---

## 🚀 Usage

Running `fup` without arguments displays help:

```
╔═══════════════════════════════════════════╗
║               FUP v2.0.0                  ║
║     Fedora Utility Patchworker            ║
╚═══════════════════════════════════════════╝

USAGE

    sudo fup [OPTIONS]

COMMANDS

    -r, --refresh         Check and display available updates
    -u, --update-all      Update everything (system + flatpak)
        --update-system   Update only system packages (dnf5)
        --update-flatpak  Update only Flatpak applications
        --update-firmware Update only firmware
```

### Quick Examples

```bash
# Show available updates
sudo fup --refresh

# Update system and Flatpak packages
sudo fup --update-all

# Update everything including firmware
sudo fup --update-all --firmware

# Update only system packages
sudo fup --update-system

# Preview what would happen (dry run)
sudo fup --update-all --dry-run

# Quiet mode for scripts/cron
sudo fup --update-all --quiet --no-reboot-prompt
```

---

## 📖 Commands

| Command | Short | Description |
|---------|-------|-------------|
| `--refresh` | `-r` | Check and display available updates without installing |
| `--update-all` | `-u` | Update system packages and Flatpak applications |
| `--update-system` | | Update only dnf5 system packages |
| `--update-flatpak` | | Update only Flatpak applications |
| `--update-firmware` | | Update only firmware |

---

## ⚙️ Options

| Option | Short | Description |
|--------|-------|-------------|
| `--firmware` | `-f` | Include firmware updates when using `--update-all` |
| `--dry-run` | `-n` | Preview actions without executing |
| `--no-reboot-prompt` | | Skip the reboot prompt after updates |
| `--no-network-check` | | Skip network connectivity verification |
| `--parallel` | | Run updates concurrently (may interleave output) |
| `--config <FILE>` | `-c` | Use a custom configuration file |
| `--verbose` | `-v` | Increase verbosity (use `-vv` or `-vvv` for more) |
| `--quiet` | `-q` | Minimal output |

---

## 🔧 Configuration

SysUpdater looks for configuration files in this order:

1. Path specified with `--config`
2. `/etc/fup.toml`
3. `~/.config/fup/config.toml`

### Example Configuration

```toml
[system]
enabled = true       # Enable dnf5 system updates
auto_remove = true   # Automatically remove unused packages
refresh = true       # Refresh package metadata before updating

[flatpak]
enabled = true       # Enable Flatpak updates
remove_unused = true # Remove unused Flatpak runtimes

[firmware]
enabled = false      # Firmware updates disabled by default

[logging]
file = "/var/log/fup.log"
level = "info"       # Options: error, warn, info, debug, trace

[network]
check_url = "https://fedoraproject.org"
timeout_secs = 10
```

---

## 📤 Exit Codes

| Code | Meaning |
|------|---------|
| `0` | Success |
| `1` | General error |
| `130` | Cancelled by user (CTRL+C) |

---

## 🤝 Contributing

Contributions are welcome! Here's how you can help:

1. **Fork** the repository on GitHub
2. **Create** a new branch for your feature or fix
3. **Make** your changes and commit them
4. **Open** a Pull Request on the original repository

Please ensure your code follows Rust best practices and includes appropriate tests.

---

## 📄 License

This project is licensed under the MIT License — see the [LICENSE](LICENSE) file for details.

---

<p align="center">
  Made with ❤️ for the Fedora community
</p>