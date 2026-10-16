<div align="center">

<p align="center">
  <strong>🇺🇸 English</strong> · <a href="./docs/locales/ko.md">🇰🇷 한국어</a> · <a href="./docs/locales/zh-CN.md">🇨🇳 中文</a> · <a href="./docs/locales/ja.md">🇯🇵 日本語</a> · <a href="./docs/locales/es.md">🇪🇸 Español</a><br />
  <a href="./docs/locales/ru.md">🇷🇺 Русский</a> · <a href="./docs/locales/vi.md">🇻🇳 Tiếng Việt</a> · <a href="./docs/locales/tr.md">🇹🇷 Türkçe</a> · <a href="./docs/locales/de.md">🇩🇪 Deutsch</a> · <a href="./docs/locales/pt-BR.md">🇧🇷 Português</a>
</p>

# rdl-tray

> System tray companion for [remote-dl](https://github.com/GHSFS/remote-dl) — sits in the notification area, watches the clipboard for download URLs, and queues them with one click.

[![Build](https://github.com/GHSFS/rdl-tray/actions/workflows/build.yml/badge.svg)](https://github.com/GHSFS/rdl-tray/actions/workflows/build.yml)
[![Tests](https://github.com/GHSFS/rdl-tray/actions/workflows/test.yml/badge.svg)](https://github.com/GHSFS/rdl-tray/actions/workflows/test.yml)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](./LICENSE)
[![Platform](https://img.shields.io/badge/platform-Windows%20x64-blue)](#installation)

</div>

---

### Overview

`rdl-tray.exe` is a small native Windows application that pairs with the
[`rdl`](https://github.com/GHSFS/remote-dl) CLI client. While `rdl` handles
explicit `rdl <url>` invocations from the terminal, `rdl-tray` provides the
ambient case: any time you copy a URL to the clipboard, a notification asks
whether you want to fetch it into your cloud storage.

The two binaries share the same on-disk configuration
(`%APPDATA%\rdl\config.json`), so authenticating once with the CLI is enough
for the tray to work too.

### Features

- **Clipboard URL & magnet detection** — listens with `AddClipboardFormatListener` and
  matches the first `http(s)://` or `magnet:?xt=...` link in the new contents.
- **Interactive balloon click** — click the balloon notification (`NIN_BALLOONUSERCLICK`)
  to immediately queue the detected download target.
- **Async non-blocking queueing** — background worker thread dispatches requests
  without ever freezing the Win32 message loop or tray icon.
- **Single-instance protection** — named Win32 mutex prevents duplicate processes,
  orphaned icons, and duplicate clipboard events.
- **Start with Windows** — toggle automatic startup on logon directly from the tray
  context menu (`HKCU\Software\Microsoft\Windows\CurrentVersion\Run`).
- **Connection pooling** — cached HTTP client eliminates per-request TLS handshake latency.
- **Native Win32 tray icon** — `Shell_NotifyIconW` with a right-click menu;
  no heavy GUI framework, no extra runtime dependencies.
- **Balloon notifications** — confirmation prompts, job receipts, and error alerts surface
  as native Windows action-center balloons.
- **Pause / resume** — toggle the clipboard listener from the tray menu without exiting.
- **Send last URL** — re-queue the previously copied URL with one click.
- **Single binary** — statically linked, ~1.5 MB, no DLLs to ship.
- **Per-Monitor DPI v2 manifest** — sharp rendering on mixed-DPI setups.

### How it works

```
┌────────────────────────┐         ┌──────────────────────┐         ┌──────────────────┐
│  Windows clipboard     │ change  │  rdl-tray.exe        │  HTTPS  │  Edge worker     │
│  (any source app)      │ ───────▶│  (hidden window)     │ ───────▶│  /api/dl         │
└────────────────────────┘         │  Shell_NotifyIconW   │         └──────────────────┘
                                   │  + balloon click     │
                                   └──────────────────────┘
```

### Repository layout

```
rdl-tray/
├── Cargo.toml                 Package manifest, dependencies, release profile
├── Cargo.lock                 Pinned transitive dependency versions
├── rust-toolchain.toml        Toolchain pin (stable, x86_64-pc-windows-msvc)
├── .cargo/
│   └── config.toml            Build target + linker flags (static CRT,
│                              /SUBSYSTEM:WINDOWS so no console window appears)
├── .gitignore                 Excludes target/, secrets, IDE state
├── build.rs                   Embeds the Win32 manifest + version info via
│                              embed-resource
├── docs/
│   └── locales/               Localized documentation (ko, ja, zh-CN, ru, vi, tr, de, es, pt-BR)
├── src/
│   ├── main.rs                Hidden top-level window + Win32 message loop;
│   │                          dispatches WM_TRAY_ICON, WM_CLIPBOARDUPDATE,
│   │                          WM_COMMAND, NIN_BALLOONUSERCLICK, async queues
│   ├── autostart.rs           Windows logon autostart registry management
│   ├── tray.rs                Shell_NotifyIconW lifecycle (NIM_ADD /
│   │                          NIM_SETVERSION / NIM_DELETE)
│   ├── clipboard.rs           AddClipboardFormatListener subscription, bounded
│   │                          CF_UNICODETEXT read, URL/magnet regex extraction
│   ├── notify.rs              Balloon notifications via NIM_MODIFY + NIF_INFO / NIF_ERROR
│   ├── client.rs              Pooled reqwest client; POST /api/dl with
│   │                          DPAPI-decrypted bearer token
│   ├── config.rs              Config load + DPAPI unprotect for the persisted
│   │                          token (shared with rdl CLI)
│   └── error.rs               Crate-wide thiserror enum
├── tests/
│   └── integration.rs         assert_cmd-based smoke check on the built binary
├── resources/
│   ├── app.rc                 Win32 resource script (manifest reference,
│   │                          VERSIONINFO block)
│   └── app.manifest           PerMonitorV2 DPI, Common Controls v6, UTF-8,
│                              asInvoker
└── .github/workflows/
    ├── build.yml              CI: cargo build --release on windows-latest;
    │                          uploads rdl-tray.exe artifact; cuts a Release on
    │                          tag push
    └── test.yml               CI: cargo fmt --check, cargo clippy -D warnings,
                               cargo test --all-targets
```

### Compatibility

| Axis | Supported |
|---|---|
| Operating system | Windows 10 1809+ and Windows 11 (PerMonitorV2 manifest) |
| Architecture | x86_64 only (single-target by design) |
| Rust toolchain | 1.75+ (`rust-toolchain.toml` pins stable) |
| Linker | MSVC (Visual Studio Build Tools 2022) |

The shipped binary is statically linked against the MSVC runtime
(`+crt-static`), so it has no DLL dependencies beyond what Windows itself
guarantees (`KERNEL32.DLL`, `USER32.DLL`, `SHELL32.DLL`, `ADVAPI32.DLL`,
`CRYPT32.DLL` from DPAPI, `WS2_32.DLL` for sockets).

### Security considerations

- **Token at rest** — the bearer token is wrapped with the Windows DPAPI
  (`CryptProtectData`) before it touches disk. Only the originating Windows
  user account can decrypt it; copying `config.json` to another machine or
  another user's session yields nothing useful.
- **Token in transit** — `reqwest` is built with `https_only=true` so the
  client refuses plaintext HTTP redirects. The bearer token is sent in the
  `Authorization` header over TLS, never in the URL.
- **No telemetry** — the binary makes no network calls other than to the
  configured worker URL.
- **No auto-update** — there is no background updater, no phone-home, no
  crash reporter.
- **No console output** — the binary is built with `windows_subsystem =
  "windows"`, so even with a debug build there is no console window from
  which a screen reader could leak credentials.

### Troubleshooting

| Symptom | Likely cause | Resolution |
|---|---|---|
| "config not found" balloon | `rdl auth login` was never run | Run `rdl auth login --token <token>` from any terminal once |
| "server rejected credentials (401)" | token revoked or expired | Issue a new token from the Telegram bot, then `rdl auth login --token …` |
| Tray icon never appears | another process owns the same `uID` (collision) | Open Task Manager → end any stale `rdl-tray.exe`, then relaunch |
| Clipboard balloon never appears | listener registration failed | Check `Get-EventLog -LogName Application -Source rdl-tray` for errors |
| Process exits silently on launch | missing MSVC runtime (rare; we link statically) | Reinstall Microsoft Visual C++ Redistributable 2015-2022 |

### Contributing

This is a personal-use companion tool, but PRs that improve clipboard
handling robustness, add WinRT toast notifications with action buttons, or
trim the binary further are welcome.

Before opening a PR:

```bash
cargo fmt
cargo clippy --all-targets -- -D warnings
cargo test --all-targets
```

The CI runs the same three checks; PRs that break any of them will be
flagged.

### Acknowledgements

- [`windows`](https://crates.io/crates/windows) — the official Microsoft Win32
  bindings for Rust. Does the heavy lifting of Shell_NotifyIconW,
  AddClipboardFormatListener, and CryptProtectData.
- [`reqwest`](https://crates.io/crates/reqwest) +
  [`rustls`](https://crates.io/crates/rustls) — TLS-only HTTP client.
- [`directories`](https://crates.io/crates/directories) — cross-platform path
  resolution for `%APPDATA%\rdl\config.json`.
- [`embed-resource`](https://crates.io/crates/embed-resource) — compiles the
  Win32 manifest into the PE.

### Installation

#### Pre-built binary (Windows x64)

1. Download `rdl-tray-v0.2.0-windows-x64.zip` from the [Releases](https://github.com/GHSFS/rdl-tray/releases) page.
2. Extract it into a stable location, e.g. `C:\Tools\rdl-tray.exe`.
3. Right-click the tray icon and check **Start with Windows** to enable automatic startup at logon.
4. First-time setup: the tray reads its worker URL and credentials from the
   `rdl` CLI config. Run `rdl auth login --token <token>` once to populate it.

#### Build from source

Requires Rust 1.75+ and the MSVC toolchain.

```bash
git clone https://github.com/GHSFS/rdl-tray.git
cd rdl-tray
cargo build --release
# Output: target/x86_64-pc-windows-msvc/release/rdl-tray.exe
```

### Tray menu

| Item | Action |
|---|---|
| Open web UI | Launches the configured worker URL in the default browser. |
| Send last URL again | Re-queues the most recent URL. |
| Pause / Resume clipboard watch | Toggles the clipboard listener. |
| Start with Windows | Toggles automatic logon startup in Windows registry. |
| Quit | Removes the tray icon and exits. |

### Configuration

There is no `rdl-tray`-specific configuration. The tray reads the same
`%APPDATA%\rdl\config.json` written by the `rdl` CLI. Authenticate once with
the CLI and both tools work.

### License

MIT. See [LICENSE](./LICENSE).

### Disclaimer

Personal-use companion app. The operator is responsible for complying with the
terms of service of any source website and applicable copyright law.
