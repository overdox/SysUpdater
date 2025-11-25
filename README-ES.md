[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](https://opensource.org/licenses/MIT)
[![Rust](https://img.shields.io/badge/Rust-1.70%2B-orange.svg)](https://www.rust-lang.org/)
[![Fedora](https://img.shields.io/badge/Fedora-39%2B-blue.svg)](https://fedoraproject.org/)

> [🇬🇧 English](README.md) | 🇪🇸 Español | [🇫🇷 Français](README-FR.md) | [🇩🇪 Deutsch](README-DE.md) | [🇺🇦 Українська](README-UK.md) | [🇨🇳 中文](README-ZH.md)

# Fedora Utility Patchworker - FUP

**Fedora Utility Patchworker - FUP** es una herramienta robusta basada en Rust, diseñada para automatizar las actualizaciones del sistema, Flatpak y firmware en Fedora Linux. Cuenta con valores predeterminados seguros, registro completo y una experiencia CLI moderna.

---

## ✨ Características

| Característica | Descripción |
|----------------|-------------|
| **Valores Seguros** | Muestra ayuda cuando se ejecuta sin flags — requiere acción explícita |
| **Vista Previa** | Verifica actualizaciones disponibles antes de instalar con `--refresh` |
| **Actualizaciones del Sistema** | Actualizaciones automáticas de paquetes dnf5 con refresco de metadatos |
| **Actualizaciones Flatpak** | Mantiene todas las aplicaciones Flatpak actualizadas |
| **Actualizaciones de Firmware** | Integración opcional con fwupd para firmware UEFI/dispositivos |
| **Detección de Reinicio** | Solicita reinicio solo cuando el kernel o actualizaciones críticas lo requieren |
| **Verificación de Red** | Confirma conectividad antes de iniciar actualizaciones |
| **Cierre Seguro** | Maneja CTRL+C limpiamente sin corrupción |
| **Registro Completo** | Logs con marca de tiempo en `/var/log/fup.log` |
| **Configurable** | Soporte para archivo de configuración TOML con valores predeterminados |
| **Indicadores de Progreso** | Spinners y salida en tiempo real |
| **Modo Simulación** | Vista previa de acciones sin ejecutar |

---

## 📋 Requisitos

| Requisito | Detalles |
|-----------|----------|
| **Sistema Operativo** | Fedora Linux 39+ (o distribuciones compatibles) |
| **Gestor de Paquetes** | `dnf5` para actualizaciones del sistema |
| **Opcional** | `flatpak` para actualizaciones de Flatpak |
| **Opcional** | `fwupdmgr` para actualizaciones de firmware |
| **Compilación** | Rust 1.70+ (solo si compilas desde el código fuente) |

---

## 📦 Instalación

### Binario Precompilado

1. Descarga el último binario de la página de [Releases](https://github.com/overdox/fedora-utility-patchworker/releases)
2. Instálalo:

```bash
sudo mv fup /usr/local/bin/
sudo chmod +x /usr/local/bin/fup
```

### Compilar desde el Código Fuente

```bash
# Clona el repositorio
git clone https://github.com/overdox/SysUpdater.git
cd SysUpdater

# Compila con optimizaciones
cargo build --release

# Instala
sudo mv target/release/fup /usr/local/bin/
```

---

## 🚀 Uso

Ejecutar `fup` sin argumentos muestra la ayuda:

```
╔═══════════════════════════════════════════╗
║               FUP v2.1.0                  ║
║     Fedora Utility Patchworker            ║
╚═══════════════════════════════════════════╝

USAGE

    sudo fup [OPTIONS]

COMMANDS

    -r, --refresh         Verificar y mostrar actualizaciones disponibles
    -u, --update-all      Actualizar todo (sistema + flatpak)
        --update-system   Actualizar solo paquetes del sistema (dnf5)
        --update-flatpak  Actualizar solo aplicaciones Flatpak
        --update-firmware Actualizar solo firmware
```

### Ejemplos Rápidos

```bash
# Mostrar actualizaciones disponibles
sudo fup --refresh

# Actualizar sistema y paquetes Flatpak
sudo fup --update-all

# Actualizar todo incluyendo firmware
sudo fup --update-all --firmware

# Actualizar solo paquetes del sistema
sudo fup --update-system

# Vista previa de lo que sucedería (simulación)
sudo fup --update-all --dry-run

# Modo silencioso para scripts/cron
sudo fup --update-all --quiet --no-reboot-prompt
```

---

## 📖 Comandos

| Comando | Corto | Descripción |
|---------|-------|-------------|
| `--refresh` | `-r` | Verificar y mostrar actualizaciones sin instalar |
| `--update-all` | `-u` | Actualizar paquetes del sistema y aplicaciones Flatpak |
| `--update-system` | | Actualizar solo paquetes del sistema dnf5 |
| `--update-flatpak` | | Actualizar solo aplicaciones Flatpak |
| `--update-firmware` | | Actualizar solo firmware |

---

## ⚙️ Opciones

| Opción | Corto | Descripción |
|--------|-------|-------------|
| `--firmware` | `-f` | Incluir actualizaciones de firmware con `--update-all` |
| `--dry-run` | `-n` | Vista previa de acciones sin ejecutar |
| `--no-reboot-prompt` | | Omitir solicitud de reinicio después de actualizar |
| `--no-network-check` | | Omitir verificación de conectividad de red |
| `--parallel` | | Ejecutar actualizaciones concurrentemente |
| `--config <ARCHIVO>` | `-c` | Usar un archivo de configuración personalizado |
| `--verbose` | `-v` | Aumentar verbosidad (usar `-vv` o `-vvv` para más) |
| `--quiet` | `-q` | Salida mínima |

---

## 🔧 Configuración

SysUpdater busca archivos de configuración en este orden:

1. Ruta especificada con `--config`
2. `/etc/fup.toml`
3. `~/.config/fup/config.toml`

### Ejemplo de Configuración

```toml
[system]
enabled = true       # Habilitar actualizaciones del sistema dnf5
auto_remove = true   # Eliminar automáticamente paquetes no utilizados
refresh = true       # Refrescar metadatos antes de actualizar

[flatpak]
enabled = true       # Habilitar actualizaciones de Flatpak
remove_unused = true # Eliminar runtimes de Flatpak no utilizados

[firmware]
enabled = false      # Actualizaciones de firmware deshabilitadas por defecto

[logging]
file = "/var/log/fup.log"
level = "info"       # Opciones: error, warn, info, debug, trace

[network]
check_url = "https://fedoraproject.org"
timeout_secs = 10
```

---

## 📤 Códigos de Salida

| Código | Significado |
|--------|-------------|
| `0` | Éxito |
| `1` | Error general |
| `130` | Cancelado por el usuario (CTRL+C) |

---

## 🤝 Contribuciones

¡Las contribuciones son bienvenidas! Así es cómo puedes ayudar:

1. **Fork** el repositorio en GitHub
2. **Crea** una nueva rama para tu característica o corrección
3. **Realiza** tus cambios y haz commit
4. **Abre** un Pull Request en el repositorio original

Por favor asegúrate de que tu código siga las mejores prácticas de Rust e incluya pruebas apropiadas.

---

## 📄 Licencia

Este proyecto está licenciado bajo la Licencia MIT — consulta el archivo [LICENSE](LICENSE) para más detalles.

---

<p align="center">
  Hecho con ❤️ para la comunidad de Fedora
</p>