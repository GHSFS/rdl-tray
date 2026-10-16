<div align="center">

<p align="center">
  <a href="../../README.md">🇺🇸 English</a> · <a href="./ko.md">🇰🇷 한국어</a> · <a href="./zh-CN.md">🇨🇳 中文</a> · <a href="./ja.md">🇯🇵 日本語</a> · <a href="./es.md">🇪🇸 Español</a><br />
  <strong>🇷🇺 Русский</strong> · <a href="./vi.md">🇻🇳 Tiếng Việt</a> · <a href="./tr.md">🇹🇷 Türkçe</a> · <a href="./de.md">🇩🇪 Deutsch</a> · <a href="./pt-BR.md">🇧🇷 Português</a>
</p>

# rdl-tray

[![Build](https://github.com/GHSFS/rdl-tray/actions/workflows/build.yml/badge.svg)](https://github.com/GHSFS/rdl-tray/actions/workflows/build.yml)
[![Tests](https://github.com/GHSFS/rdl-tray/actions/workflows/test.yml/badge.svg)](https://github.com/GHSFS/rdl-tray/actions/workflows/test.yml)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](../../LICENSE)
[![Platform](https://img.shields.io/badge/platform-Windows%20x64-blue)](#installation)

</div>

---

### Обзор

`rdl-tray.exe` — это небольшое нативное Windows-приложение, работающее в паре
с CLI-клиентом [`rdl`](https://github.com/GHSFS/remote-dl). Если CLI
обрабатывает явные вызовы `rdl <url>` из терминала, то `rdl-tray` покрывает
фоновый сценарий: при каждом копировании URL в буфер обмена уведомление
спрашивает, нужно ли загрузить его в ваше облачное хранилище.

Оба бинарных файла используют один и тот же файл конфигурации на диске
(`%APPDATA%\rdl\config.json`), поэтому однократной аутентификации через CLI
достаточно для работы трея.

### Возможности

- **Обнаружение URL и Magnet-ссылок** — слушатель `AddClipboardFormatListener`
  и извлечение первого `http(s)://` или `magnet:?xt=...`
- **Интерактивный клик по уведомлению** — клик по всплывающему балуну
  (`NIN_BALLOONUSERCLICK`) для мгновенной отправки в очередь
- **Асинхронная неблокирующая отправка** — фоновый поток полностью исключает
  зависания интерфейса и значка трея
- **Защита от повторного запуска (Named Mutex)** — предотвращает появление
  лишних иконок и дублирование событий буфера
- **Автозапуск с Windows** — переключение автозапуска в меню трея
  (`HKCU\Software\Microsoft\Windows\CurrentVersion\Run`)
- **Пул HTTP-соединений** — кэширование клиента (`OnceLock`) устраняет задержки TLS
- **Нативная иконка в области уведомлений Win32** — `Shell_NotifyIconW`
  с контекстным меню по правому клику
- **Всплывающие уведомления** — подтверждения, номера задач и ошибки в Центре
  уведомлений Windows
- **Пауза / возобновление** — переключение слушателя из меню трея без выхода
- **Повторная отправка последнего URL** — повторная постановка в очередь
  предыдущего скопированного URL одним кликом
- **Единый бинарный файл** — статически слинкован, ~1.5 МБ, без зависимостей DLL

### Структура проекта

```
rdl-tray/
├── Cargo.toml              Манифест пакета, зависимости, профиль release
├── rust-toolchain.toml     Закрепление toolchain (stable / x86_64-pc-windows-msvc)
├── .cargo/config.toml      Цель сборки + флаги компоновщика (статическая CRT)
├── build.rs                Встраивание манифеста Win32 + информации о версии
├── src/
│   ├── main.rs             Скрытое окно + цикл сообщений Win32, клики по уведомлениям и асинхрон
│   ├── autostart.rs        Управление автозапуском через реестр Windows
│   ├── tray.rs             Управление жизненным циклом Shell_NotifyIconW
│   ├── clipboard.rs        AddClipboardFormatListener + безопасное извлечение URL/Magnet
│   ├── notify.rs           Всплывающие уведомления через NIM_MODIFY (инфо / ошибки)
│   ├── client.rs           Пул reqwest клиентов, POST /api/dl
│   ├── config.rs           Загрузка конфигурации + расшифровка токена через DPAPI
│   └── error.rs            Тип ошибки уровня крейта
├── tests/integration.rs    Smoke-тесты на основе assert_cmd
├── resources/              Win32 resource script + манифест
└── .github/workflows/      CI для cargo build + fmt/clippy/test
```

Подробные инструкции по установке, настройке и CLI см. в разделе
[English](#english).

### Лицензия

MIT. См. [LICENSE](./LICENSE).
