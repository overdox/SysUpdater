[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](https://opensource.org/licenses/MIT)
[![Rust](https://img.shields.io/badge/Rust-1.70%2B-orange.svg)](https://www.rust-lang.org/)
[![Fedora](https://img.shields.io/badge/Fedora-39%2B-blue.svg)](https://fedoraproject.org/)

> [🇬🇧 English](README.md) | [🇪🇸 Español](README-ES.md) | [🇫🇷 Français](README-FR.md) | 🇩🇪 Deutsch | [🇺🇦 Українська](README-UK.md) | [🇨🇳 中文](README-ZH.md)

# Fedora Utility Patchworker - FUP

**Fedora Utility Patchworker - FUP** ist ein produktionsreifes, auf Rust basierendes Werkzeug zur Automatisierung von System-, Flatpak- und Firmware-Updates unter Fedora Linux. Es bietet sichere Standardeinstellungen, umfassende Protokollierung und eine moderne CLI-Erfahrung.

---

## ✨ Funktionen

| Funktion | Beschreibung |
|----------|--------------|
| **Sichere Standards** | Zeigt Hilfe an, wenn ohne Flags ausgeführt — erfordert explizite Aktion |
| **Update-Vorschau** | Verfügbare Updates vor der Installation mit `--refresh` prüfen |
| **System-Updates** | Automatische dnf5-Paketaktualisierungen mit Metadaten-Aktualisierung |
| **Flatpak-Updates** | Hält alle Flatpak-Anwendungen aktuell |
| **Firmware-Updates** | Optionale fwupd-Integration für UEFI-/Geräte-Firmware |
| **Intelligente Neustart-Erkennung** | Fordert nur zum Neustart auf, wenn Kernel oder kritische Updates dies erfordern |
| **Netzwerk-Überprüfung** | Bestätigt Konnektivität vor dem Start der Updates |
| **Sauberes Beenden** | Behandelt CTRL+C sauber ohne Korruption |
| **Umfassende Protokollierung** | Zeitgestempelte Logs in `/var/log/fup.log` |
| **Konfigurierbar** | TOML-Konfigurationsdatei-Unterstützung mit sinnvollen Standards |
| **Fortschrittsanzeigen** | Spinner und Echtzeit-Ausgabe |
| **Trockenlauf-Modus** | Vorschau der Aktionen ohne Ausführung |

---

## 📋 Voraussetzungen

| Voraussetzung | Details |
|---------------|---------|
| **Betriebssystem** | Fedora Linux 39+ (oder kompatible Distributionen) |
| **Paketmanager** | `dnf5` für System-Updates |
| **Optional** | `flatpak` für Flatpak-Updates |
| **Optional** | `fwupdmgr` für Firmware-Updates |
| **Kompilierung** | Rust 1.70+ (nur beim Kompilieren aus dem Quellcode) |

---

## 📦 Installation

### Vorkompilierte Binärdatei

1. Laden Sie die neueste Binärdatei von der [Releases](https://github.com/overdox/fedora-utility-patchworker/releases)-Seite herunter
2. Installieren Sie sie:

```bash
sudo mv fup /usr/local/bin/
sudo chmod +x /usr/local/bin/fup
```

### Aus dem Quellcode kompilieren

```bash
# Repository klonen
git clone https://github.com/overdox/SysUpdater.git
cd SysUpdater

# Mit Optimierungen kompilieren
cargo build --release

# Installieren
sudo mv target/release/fup /usr/local/bin/
```

---

## 🚀 Verwendung

Das Ausführen von `fup` ohne Argumente zeigt die Hilfe an:

```
╔═══════════════════════════════════════════╗
║               FUP v2.1.0                  ║
║     Fedora Utility Patchworker            ║
╚═══════════════════════════════════════════╝

USAGE

    sudo fup [OPTIONS]

COMMANDS

    -r, --refresh         Verfügbare Updates prüfen und anzeigen
    -u, --update-all      Alles aktualisieren (System + Flatpak)
        --update-system   Nur Systempakete aktualisieren (dnf5)
        --update-flatpak  Nur Flatpak-Anwendungen aktualisieren
        --update-firmware Nur Firmware aktualisieren
```

### Schnelle Beispiele

```bash
# Verfügbare Updates anzeigen
sudo fup --refresh

# System und Flatpak-Pakete aktualisieren
sudo fup --update-all

# Alles einschließlich Firmware aktualisieren
sudo fup --update-all --firmware

# Nur Systempakete aktualisieren
sudo fup --update-system

# Vorschau was passieren würde (Trockenlauf)
sudo fup --update-all --dry-run

# Stiller Modus für Skripte/Cron
sudo fup --update-all --quiet --no-reboot-prompt
```

---

## 📖 Befehle

| Befehl | Kurz | Beschreibung |
|--------|------|--------------|
| `--refresh` | `-r` | Verfügbare Updates prüfen und anzeigen ohne Installation |
| `--update-all` | `-u` | Systempakete und Flatpak-Anwendungen aktualisieren |
| `--update-system` | | Nur dnf5-Systempakete aktualisieren |
| `--update-flatpak` | | Nur Flatpak-Anwendungen aktualisieren |
| `--update-firmware` | | Nur Firmware aktualisieren |

---

## ⚙️ Optionen

| Option | Kurz | Beschreibung |
|--------|------|--------------|
| `--firmware` | `-f` | Firmware-Updates bei `--update-all` einschließen |
| `--dry-run` | `-n` | Vorschau der Aktionen ohne Ausführung |
| `--no-reboot-prompt` | | Neustart-Aufforderung nach Updates überspringen |
| `--no-network-check` | | Netzwerk-Konnektivitätsprüfung überspringen |
| `--parallel` | | Updates gleichzeitig ausführen |
| `--config <DATEI>` | `-c` | Benutzerdefinierte Konfigurationsdatei verwenden |
| `--verbose` | `-v` | Ausführlichkeit erhöhen (`-vv` oder `-vvv` für mehr) |
| `--quiet` | `-q` | Minimale Ausgabe |

---

## 🔧 Konfiguration

SysUpdater sucht Konfigurationsdateien in dieser Reihenfolge:

1. Mit `--config` angegebener Pfad
2. `/etc/fup.toml`
3. `~/.config/fup/config.toml`

### Beispielkonfiguration

```toml
[system]
enabled = true       # dnf5-System-Updates aktivieren
auto_remove = true   # Unbenutzte Pakete automatisch entfernen
refresh = true       # Metadaten vor dem Update aktualisieren

[flatpak]
enabled = true       # Flatpak-Updates aktivieren
remove_unused = true # Unbenutzte Flatpak-Runtimes entfernen

[firmware]
enabled = false      # Firmware-Updates standardmäßig deaktiviert

[logging]
file = "/var/log/fup.log"
level = "info"       # Optionen: error, warn, info, debug, trace

[network]
check_url = "https://fedoraproject.org"
timeout_secs = 10
```

---

## 📤 Exit-Codes

| Code | Bedeutung |
|------|-----------|
| `0` | Erfolg |
| `1` | Allgemeiner Fehler |
| `130` | Vom Benutzer abgebrochen (CTRL+C) |

---

## 🤝 Beitragen

Beiträge sind willkommen! So können Sie helfen:

1. **Forken** Sie das Repository auf GitHub
2. **Erstellen** Sie einen neuen Branch für Ihre Funktion oder Korrektur
3. **Nehmen** Sie Ihre Änderungen vor und committen Sie sie
4. **Öffnen** Sie einen Pull Request im Original-Repository

Bitte stellen Sie sicher, dass Ihr Code den Rust-Best-Practices folgt und entsprechende Tests enthält.

---

## 📄 Lizenz

Dieses Projekt ist unter der MIT-Lizenz lizenziert — siehe die [LICENSE](LICENSE)-Datei für Details.

---

<p align="center">
  Mit ❤️ für die Fedora-Community gemacht
</p>