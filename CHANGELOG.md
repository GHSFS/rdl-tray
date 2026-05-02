# Changelog

All notable changes to `rdl-tray` are documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/).

## [Unreleased]

### Added
- Initial system tray companion for [remote-dl](https://github.com/GHSFS/remote-dl).
- Native Win32 hidden window + `Shell_NotifyIconW` tray icon.
- `AddClipboardFormatListener` listener that detects URLs on clipboard change.
- Right-click menu: open web UI, send last URL, pause/resume watch, quit.
- Balloon notifications via `NIF_INFO`.
- Shares config storage with the `rdl` CLI (`%APPDATA%\rdl\config.json`).
- Embedded application manifest (PerMonitorV2 DPI, Common Controls v6, UTF-8).

### Planned
- Custom tray icon (currently uses `IDI_APPLICATION`).
- Settings dialog for editing worker URL / token in place.
- Optional WinRT toast notifications with action buttons.
