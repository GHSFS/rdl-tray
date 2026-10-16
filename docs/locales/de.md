<div align="center">

<p align="center">
  <a href="../../README.md">🇺🇸 English</a> · <a href="./ko.md">🇰🇷 한국어</a> · <a href="./zh-CN.md">🇨🇳 中文</a> · <a href="./ja.md">🇯🇵 日本語</a> · <a href="./es.md">🇪🇸 Español</a><br />
  <a href="./ru.md">🇷🇺 Русский</a> · <a href="./vi.md">🇻🇳 Tiếng Việt</a> · <a href="./tr.md">🇹🇷 Türkçe</a> · <strong>🇩🇪 Deutsch</strong> · <a href="./pt-BR.md">🇧🇷 Português</a>
</p>

# rdl-tray

[![Build](https://github.com/GHSFS/rdl-tray/actions/workflows/build.yml/badge.svg)](https://github.com/GHSFS/rdl-tray/actions/workflows/build.yml)
[![Tests](https://github.com/GHSFS/rdl-tray/actions/workflows/test.yml/badge.svg)](https://github.com/GHSFS/rdl-tray/actions/workflows/test.yml)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](../../LICENSE)
[![Platform](https://img.shields.io/badge/platform-Windows%20x64-blue)](#installation)

</div>

---

### Überblick

`rdl-tray.exe` ist eine kleine native Windows-Anwendung, die mit dem
CLI-Client [`rdl`](https://github.com/GHSFS/remote-dl) zusammenarbeitet.
Während die CLI explizite `rdl <url>`-Aufrufe aus dem Terminal verarbeitet,
übernimmt `rdl-tray` den Hintergrundfall: Sobald du eine URL in die
Zwischenablage kopierst, fragt eine Benachrichtigung, ob du sie in deinen
Cloud-Speicher abrufen möchtest.

Beide Binärdateien teilen sich dieselbe Konfigurationsdatei
(`%APPDATA%\rdl\config.json`), sodass eine einmalige Authentifizierung über
die CLI ausreicht, damit auch das Tray funktioniert.

### Funktionen

- **Zwischenablage-URL- & Magnet-Erkennung** — hört über
  `AddClipboardFormatListener` mit und extrahiert die erste `http(s)://`- oder `magnet:?xt=...`-URL
- **Interaktiver Sprechblasen-Klick** — Klick auf die Benachrichtigung
  (`NIN_BALLOONUSERCLICK`) reiht den Download sofort ein
- **Asynchrones, blockierungsfreies Einreihen** — Hintergrund-Worker verhindert
  jegliches Einfrieren der Windows-Nachrichtenschleife
- **Schutz vor Mehrfachinstanzen (Named Mutex)** — verhindert doppelte Icons und
  Zwischenablage-Konflikte
- **Mit Windows starten** — Autostart direkt über das Tray-Kontextmenü umschalten
  (`HKCU\Software\Microsoft\Windows\CurrentVersion\Run`)
- **HTTP-Verbindungspooling** — `OnceLock`-Caching minimiert TLS-Verbindungsverzögerungen
- **Natives Win32-Tray-Symbol** — `Shell_NotifyIconW` + Rechtsklick-Menü
- **Sprechblasen-Benachrichtigungen** — Bestätigungen, Job-IDs und Fehler im Windows Action Center
- **Pause / Fortsetzen** — Listener über das Tray-Menü umschalten, ohne zu beenden
- **Letzte URL erneut senden** — die zuletzt kopierte URL mit einem Klick erneut einreihen
- **Einzelne Binärdatei** — statisch gelinkt, ~1.5 MB, keine DLL-Abhängigkeiten

### Projektstruktur

```
rdl-tray/
├── Cargo.toml              Paketmanifest, Abhängigkeiten, Release-Profil
├── rust-toolchain.toml     Toolchain-Pin (stable / x86_64-pc-windows-msvc)
├── .cargo/config.toml      Build-Ziel + Linker-Flags (statische CRT)
├── build.rs                Bettet Win32-Manifest + Versionsinfo ein
├── src/
│   ├── main.rs             Verstecktes Fenster + Win32-Nachrichtenschleife, Klick-Aktion & Async
│   ├── autostart.rs        Windows-Autostart-Registrierungsverwaltung
│   ├── tray.rs             Shell_NotifyIconW-Lebenszyklusverwaltung
│   ├── clipboard.rs        AddClipboardFormatListener + speichersichere URL/Magnet-Extraktion
│   ├── notify.rs           Sprechblasen-Benachrichtigungen via NIM_MODIFY (Info / Fehler)
│   ├── client.rs           Gepoolter reqwest-Client, POST /api/dl
│   ├── config.rs           Konfiguration laden + Token via DPAPI entschlüsseln
│   └── error.rs            Crate-weiter Fehlertyp
├── tests/integration.rs    Smoke-Tests basierend auf assert_cmd
├── resources/              Win32-Resource-Script + Manifest
└── .github/workflows/      CI für cargo build + fmt/clippy/test
```

Ausführliche Installations-, Konfigurations- und CLI-Referenzanleitungen
findest du im Abschnitt [English](#english).

### Lizenz

MIT. Siehe [LICENSE](./LICENSE).
