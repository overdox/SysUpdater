[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](https://opensource.org/licenses/MIT)

> 🇬🇧 [English](README.md) | 🇪🇸 Español |

# SysUpdater

**SysUpdater** es una herramienta ligera y eficiente basada en Rust, diseñada para automatizar las actualizaciones del sistema y Flatpak en Fedora Linux. Garantiza que tu sistema esté siempre actualizado al gestionar actualizaciones, limpieza de paquetes no utilizados y reinicios de manera sencilla.

---

## Características

- Automatiza tanto las actualizaciones del sistema (`dnf5`) como las de Flatpak.
- Proporciona salida en tiempo real para los procesos de actualización.
- Maneja la limpieza de paquetes no utilizados.
- Detecta si se requiere un reinicio y solicita la acción del usuario.
- Diseñado específicamente para Fedora, pero podría funcionar en otras distribuciones de Linux con ajustes menores.

---

## Requisitos

- **Sistema Operativo**: Fedora Linux (o distribuciones compatibles).
- **Rust**: SysUpdater está escrito en Rust. Necesitarás Rust instalado para compilar el binario desde el código fuente.

---

## Instalación

### Binario Precompilado

1. Descarga el último binario de la página de [Releases](https://github.com/overdox/SysUpdater/releases).
2. Coloca el binario en un directorio dentro de tu `$PATH`, por ejemplo:
   ```bash
   mv sysupdater /usr/local/bin/
   chmod +x /usr/local/bin/sysupdater


### Compilar desde el Código Fuente
1. Clona el repositorio:
 ```
 git clone https://github.com/overdox/SysUpdater.git 
 cd SysUpdater
 ```
2. Compila el proyecto:
```
cargo build --release
```

3. El binario estará ubicado en target/release/sysupdater. Muévelo a un directorio dentro de tu $PATH:
```
mv target/release/sysupdater /usr/local/bin/
chmod +x /usr/local/bin/sysupdater
```

### Uso

1. Ejecuta la herramienta con sudo:
```sudo ./sysupdater
```

2. SysUpdater realizará:

- Actualizaciones del sistema usando dnf5.
- Actualizaciones de Flatpak.
- Limpieza de paquetes no utilizados.
- Verificación de si se requiere un reinicio y te pedirá una acción.

### Contribuciones

¡Aceptamos contribuciones! Así es cómo puedes ayudar:

1. Haz un fork del repositorio en GitHub.
2. Crea una nueva rama para tu característica o corrección.
3. Realiza tus cambios y haz commi.
4. Abre un Pull Request en el repositorio original.