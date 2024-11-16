[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](https://opensource.org/licenses/MIT)


> 🇬🇧 English | | [🇪🇸 Spanish](README-ES.md)

# SysUpdater

**SysUpdater** is a lightweight and efficient Rust-based tool designed for automating system and Flatpak updates on Fedora Linux. It ensures your system is always up-to-date by handling updates, unused package cleanup, and reboots seamlessly.

---

## Features

- Automates both system (`dnf5`) and Flatpak updates.
- Provides real-time output for update processes.
- Handles cleanup of unused packages.
- Detects if a reboot is required and prompts the user for action.
- Designed specifically for Fedora but may work on other Linux distributions with minor tweaks.

---

## Requirements

- **Operating System**: Fedora Linux (or compatible distributions).
- **Rust**: SysUpdater is written in Rust. You'll need Rust installed for building the binary from source.

---

## Installation

### Precompiled Binary

1. Download the latest release binary from the [Releases](https://github.com/overdox/SysUpdater/releases) page.
2. Place the binary in a directory in your `$PATH`, e.g.:
   ```bash
   mv sysupdater /usr/local/bin/
   chmod +x /usr/local/bin/sysupdater


### Build from Source
1. Clone the repository:
 ```
 git clone https://github.com/overdox/SysUpdater.git 
 cd SysUpdater
 ```
2. Build the project:
```
cargo build --release
```

3. The binary will be located at target/release/sysupdater. Move it to a directory in your $PATH:
```
mv target/release/sysupdater /usr/local/bin/
chmod +x /usr/local/bin/sysupdater
```

### Usage

1. Run the tool with sudo:
sudo sysupdater

2. SysUpdater will:

- Perform system updates using dnf5.
- Perform Flatpak updates.
- Clean up unused packages.
- Check if a system reboot is required and prompt the user for action.

### Contributing

We welcome contributions! Here's how you can help:

1. Fork the repository on GitHub.
2. Create a new branch for your feature or fix.
3. Make your changes and commit them.
4. Open a Pull Request on the original repository.