[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](https://opensource.org/licenses/MIT)
[![Rust](https://img.shields.io/badge/Rust-1.70%2B-orange.svg)](https://www.rust-lang.org/)
[![Fedora](https://img.shields.io/badge/Fedora-39%2B-blue.svg)](https://fedoraproject.org/)

> [🇬🇧 English](README.md) | [🇪🇸 Español](README-ES.md) | 🇫🇷 Français | [🇩🇪 Deutsch](README-DE.md) | [🇺🇦 Українська](README-UK.md) | [🇨🇳 中文](README-ZH.md)

# Fedora Utility Patchworker - FUP

**Fedora Utility Patchworker - FUP** est un outil robuste basé sur Rust, conçu pour automatiser les mises à jour du système, Flatpak et firmware sur Fedora Linux. Il propose des valeurs par défaut sécurisées, une journalisation complète et une expérience CLI moderne.

---

## ✨ Fonctionnalités

| Fonctionnalité | Description |
|----------------|-------------|
| **Valeurs Sécurisées** | Affiche l'aide lorsqu'exécuté sans options — nécessite une action explicite |
| **Aperçu des Mises à Jour** | Vérifiez les mises à jour disponibles avant d'installer avec `--refresh` |
| **Mises à Jour Système** | Mises à jour automatiques des paquets dnf5 avec actualisation des métadonnées |
| **Mises à Jour Flatpak** | Maintient toutes les applications Flatpak à jour |
| **Mises à Jour Firmware** | Intégration optionnelle fwupd pour le firmware UEFI/périphériques |
| **Détection de Redémarrage** | Demande un redémarrage uniquement lorsque le noyau ou les mises à jour critiques l'exigent |
| **Vérification Réseau** | Confirme la connectivité avant de démarrer les mises à jour |
| **Arrêt Propre** | Gère CTRL+C proprement sans corruption |
| **Journalisation Complète** | Logs horodatés dans `/var/log/fup.log` |
| **Configurable** | Support de fichier de configuration TOML avec valeurs par défaut |
| **Indicateurs de Progression** | Spinners et sortie en temps réel |
| **Mode Simulation** | Aperçu des actions sans exécution |

---

## 📋 Prérequis

| Prérequis | Détails |
|-----------|---------|
| **Système d'Exploitation** | Fedora Linux 39+ (ou distributions compatibles) |
| **Gestionnaire de Paquets** | `dnf5` pour les mises à jour système |
| **Optionnel** | `flatpak` pour les mises à jour Flatpak |
| **Optionnel** | `fwupdmgr` pour les mises à jour firmware |
| **Compilation** | Rust 1.70+ (uniquement si vous compilez depuis les sources) |

---

## 📦 Installation

### Binaire Précompilé

1. Téléchargez le dernier binaire depuis la page [Releases](https://github.com/overdox/fedora-utility-patchworker/releases)
2. Installez-le :

```bash
sudo mv fup /usr/local/bin/
sudo chmod +x /usr/local/bin/fup
```

### Compiler depuis les Sources

```bash
# Clonez le dépôt
git clone https://github.com/overdox/SysUpdater.git
cd SysUpdater

# Compilez avec les optimisations
cargo build --release

# Installez
sudo mv target/release/fup /usr/local/bin/
```

---

## 🚀 Utilisation

Exécuter `fup` sans arguments affiche l'aide :

```
╔═══════════════════════════════════════════╗
║               FUP v2.0.0                  ║
║     Fedora Utility Patchworker            ║
╚═══════════════════════════════════════════╝

USAGE

    sudo fup [OPTIONS]

COMMANDS

    -r, --refresh         Vérifier et afficher les mises à jour disponibles
    -u, --update-all      Tout mettre à jour (système + flatpak)
        --update-system   Mettre à jour uniquement les paquets système (dnf5)
        --update-flatpak  Mettre à jour uniquement les applications Flatpak
        --update-firmware Mettre à jour uniquement le firmware
```

### Exemples Rapides

```bash
# Afficher les mises à jour disponibles
sudo fup --refresh

# Mettre à jour le système et les paquets Flatpak
sudo fup --update-all

# Tout mettre à jour y compris le firmware
sudo fup --update-all --firmware

# Mettre à jour uniquement les paquets système
sudo fup --update-system

# Aperçu de ce qui se passerait (simulation)
sudo fup --update-all --dry-run

# Mode silencieux pour scripts/cron
sudo fup --update-all --quiet --no-reboot-prompt
```

---

## 📖 Commandes

| Commande | Court | Description |
|----------|-------|-------------|
| `--refresh` | `-r` | Vérifier et afficher les mises à jour sans installer |
| `--update-all` | `-u` | Mettre à jour les paquets système et les applications Flatpak |
| `--update-system` | | Mettre à jour uniquement les paquets système dnf5 |
| `--update-flatpak` | | Mettre à jour uniquement les applications Flatpak |
| `--update-firmware` | | Mettre à jour uniquement le firmware |

---

## ⚙️ Options

| Option | Court | Description |
|--------|-------|-------------|
| `--firmware` | `-f` | Inclure les mises à jour firmware avec `--update-all` |
| `--dry-run` | `-n` | Aperçu des actions sans exécution |
| `--no-reboot-prompt` | | Ignorer la demande de redémarrage après les mises à jour |
| `--no-network-check` | | Ignorer la vérification de connectivité réseau |
| `--parallel` | | Exécuter les mises à jour simultanément |
| `--config <FICHIER>` | `-c` | Utiliser un fichier de configuration personnalisé |
| `--verbose` | `-v` | Augmenter la verbosité (utiliser `-vv` ou `-vvv` pour plus) |
| `--quiet` | `-q` | Sortie minimale |

---

## 🔧 Configuration

SysUpdater recherche les fichiers de configuration dans cet ordre :

1. Chemin spécifié avec `--config`
2. `/etc/fup.toml`
3. `~/.config/fup/config.toml`

### Exemple de Configuration

```toml
[system]
enabled = true       # Activer les mises à jour système dnf5
auto_remove = true   # Supprimer automatiquement les paquets inutilisés
refresh = true       # Actualiser les métadonnées avant la mise à jour

[flatpak]
enabled = true       # Activer les mises à jour Flatpak
remove_unused = true # Supprimer les runtimes Flatpak inutilisés

[firmware]
enabled = false      # Mises à jour firmware désactivées par défaut

[logging]
file = "/var/log/fup.log"
level = "info"       # Options : error, warn, info, debug, trace

[network]
check_url = "https://fedoraproject.org"
timeout_secs = 10
```

---

## 📤 Codes de Sortie

| Code | Signification |
|------|---------------|
| `0` | Succès |
| `1` | Erreur générale |
| `130` | Annulé par l'utilisateur (CTRL+C) |

---

## 🤝 Contributions

Les contributions sont les bienvenues ! Voici comment vous pouvez aider :

1. **Forkez** le dépôt sur GitHub
2. **Créez** une nouvelle branche pour votre fonctionnalité ou correction
3. **Effectuez** vos modifications et committez-les
4. **Ouvrez** une Pull Request sur le dépôt original

Veuillez vous assurer que votre code suit les meilleures pratiques Rust et inclut des tests appropriés.

---

## 📄 Licence

Ce projet est sous licence MIT — consultez le fichier [LICENSE](LICENSE) pour plus de détails.

---

<p align="center">
  Fait avec ❤️ pour la communauté Fedora
</p>