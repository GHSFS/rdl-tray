<div align="center">

<p align="center">
  <a href="../../README.md">🇺🇸 English</a> · <a href="./ko.md">🇰🇷 한국어</a> · <a href="./zh-CN.md">🇨🇳 中文</a> · <a href="./ja.md">🇯🇵 日本語</a> · <strong>🇪🇸 Español</strong><br />
  <a href="./ru.md">🇷🇺 Русский</a> · <a href="./vi.md">🇻🇳 Tiếng Việt</a> · <a href="./tr.md">🇹🇷 Türkçe</a> · <a href="./de.md">🇩🇪 Deutsch</a> · <a href="./pt-BR.md">🇧🇷 Português</a>
</p>

# rdl-tray

[![Build](https://github.com/GHSFS/rdl-tray/actions/workflows/build.yml/badge.svg)](https://github.com/GHSFS/rdl-tray/actions/workflows/build.yml)
[![Tests](https://github.com/GHSFS/rdl-tray/actions/workflows/test.yml/badge.svg)](https://github.com/GHSFS/rdl-tray/actions/workflows/test.yml)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](../../LICENSE)
[![Platform](https://img.shields.io/badge/platform-Windows%20x64-blue)](#installation)

</div>

---

### Descripción general

`rdl-tray.exe` es una pequeña aplicación nativa de Windows que se empareja
con el cliente CLI [`rdl`](https://github.com/GHSFS/remote-dl). Mientras que
la CLI maneja invocaciones explícitas `rdl <url>` desde la terminal,
`rdl-tray` cubre el caso ambiente: cada vez que copias una URL al
portapapeles, una notificación pregunta si quieres descargarla en tu
almacenamiento en la nube.

Ambos binarios comparten la misma configuración en disco
(`%APPDATA%\rdl\config.json`), así que autenticarse una sola vez con la CLI
es suficiente para que el tray también funcione.

#### Características

- **Detección de URL y enlaces Magnet** — escucha mediante `AddClipboardFormatListener`
  y extrae el primer enlace `http(s)://` o `magnet:?xt=...`
- **Clic interactivo en notificación tipo globo** — haz clic en el globo
  (`NIN_BALLOONUSERCLICK`) para enviar inmediatamente la descarga a la cola
- **Encolado asíncrono sin bloqueo** — hilo de trabajo en segundo plano para evitar
  cualquier congelamiento del bucle de mensajes o del icono
- **Protección de instancia única (Named Mutex)** — previene duplicados, iconos huérfanos
  y colisiones de eventos del portapapeles
- **Iniciar con Windows** — conmutador en el menú contextual del tray para autoinicio
  (`HKCU\Software\Microsoft\Windows\CurrentVersion\Run`)
- **Reutilización de conexiones HTTP** — cliente en caché (`OnceLock`) que elimina la
  latencia de negociación TLS
- **Icono nativo de bandeja Win32** — `Shell_NotifyIconW` + menú contextual
- **Notificaciones tipo globo** — confirmaciones, ID de trabajos y errores en el
  Centro de Acciones de Windows
- **Pausar / reanudar** — alternar el listener desde el menú sin salir
- **Reenviar la última URL** — volver a encolar la URL copiada anteriormente con un solo clic
- **Binario único** — enlazado estáticamente, ~1.5 MB, sin dependencias DLL

### Estructura del proyecto

```
rdl-tray/
├── Cargo.toml              Manifiesto del paquete, dependencias, perfil release
├── rust-toolchain.toml     Fijación del toolchain (stable / x86_64-pc-windows-msvc)
├── .cargo/config.toml      Destino de compilación + flags del linker (CRT estático)
├── build.rs                Incrusta el manifiesto Win32 + información de versión
├── src/
│   ├── main.rs             Ventana oculta + bucle de mensajes Win32, clic en notificación y colas async
│   ├── autostart.rs        Gestión del registro de inicio de Windows
│   ├── tray.rs             Gestión del ciclo de vida de Shell_NotifyIconW
│   ├── clipboard.rs        AddClipboardFormatListener + extracción segura de URL/Magnet
│   ├── notify.rs           Notificaciones tipo globo vía NIM_MODIFY (info / error)
│   ├── client.rs           Cliente reqwest con pool de conexiones, POST /api/dl
│   ├── config.rs           Carga de configuración + descifrado del token DPAPI
│   └── error.rs            Tipo de error del crate
├── tests/integration.rs    Pruebas smoke basadas en assert_cmd
├── resources/              Script de recursos Win32 + manifiesto
└── .github/workflows/      CI para cargo build + fmt/clippy/test
```

Para instrucciones completas de instalación, configuración y referencia de
CLI, consulta la sección [English](#english).

### Licencia

MIT. Consulta [LICENSE](./LICENSE).
