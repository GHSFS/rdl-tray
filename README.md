# rdl-tray

> System tray companion for [remote-dl](https://github.com/GHSFS/remote-dl) — sits in the notification area, watches the clipboard for download URLs, and queues them with one click.

[![Build](https://github.com/GHSFS/rdl-tray/actions/workflows/build.yml/badge.svg)](https://github.com/GHSFS/rdl-tray/actions/workflows/build.yml)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](./LICENSE)
[![Platform](https://img.shields.io/badge/platform-Windows%20x64-blue)](#installation)

[English](#english) · [한국어](#한국어) · [日本語](#日本語) · [中文](#中文) · [Русский](#русский) · [Tiếng Việt](#tiếng-việt) · [Türkçe](#türkçe) · [Deutsch](#deutsch) · [Español](#español) · [Português](#português)

---

## English

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

- **Clipboard URL detection** — listens with `AddClipboardFormatListener` and
  matches the first `http(s)://` URL in the new contents.
- **Native Win32 tray icon** — `Shell_NotifyIconW` with a right-click menu;
  no GUI framework, no extra runtime dependencies.
- **Balloon notifications** — confirmation prompts and queue status surface as
  Windows action-center balloons.
- **Pause / resume** — toggle the listener from the tray menu without exiting.
- **Send last URL** — re-queue the previously copied URL with one click.
- **Single binary** — statically linked, ~1.5 MB, no DLLs to ship.
- **Per-Monitor DPI v2 manifest** — sharp rendering on mixed-DPI setups.

### How it works

```
┌────────────────────────┐         ┌──────────────────────┐         ┌──────────────────┐
│  Windows clipboard     │ change  │  rdl-tray.exe        │  HTTPS  │  Edge worker     │
│  (any source app)      │ ───────▶│  (hidden window)     │ ───────▶│  /api/dl         │
└────────────────────────┘         │  Shell_NotifyIconW   │         └──────────────────┘
                                   │  + balloon prompt    │
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
├── src/
│   ├── main.rs                Hidden top-level window + Win32 message loop;
│   │                          dispatches WM_TRAY_ICON, WM_CLIPBOARDUPDATE,
│   │                          WM_COMMAND
│   ├── tray.rs                Shell_NotifyIconW lifecycle (NIM_ADD /
│   │                          NIM_SETVERSION / NIM_DELETE)
│   ├── clipboard.rs           AddClipboardFormatListener subscription, CF_UNICODETEXT
│   │                          read, regex-based URL extraction with unit tests
│   ├── notify.rs              Balloon notifications via NIM_MODIFY + NIF_INFO
│   ├── client.rs              reqwest::blocking client; POST /api/dl with
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

1. Download `rdl-tray-x64.exe` from the [Releases](https://github.com/GHSFS/rdl-tray/releases) page.
2. Drop it in a stable location, e.g. `C:\Tools\rdl-tray.exe`.
3. (Optional) Add a Start-menu shortcut to the Startup folder so it launches
   at login.
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
| Pause / Resume clipboard watch | Toggles the listener. |
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

---

## 한국어

### 개요

`rdl-tray.exe`는 [`rdl`](https://github.com/GHSFS/remote-dl) CLI 클라이언트와
짝을 이루는 작은 네이티브 Windows 앱입니다. CLI가 터미널에서 `rdl <url>`
명령을 처리한다면, `rdl-tray`는 그 외의 경우를 자동화합니다 — 클립보드에
URL을 복사할 때마다 알림으로 "받을까요?" 묻습니다.

두 바이너리는 동일한 디스크 설정 파일(`%APPDATA%\rdl\config.json`)을
공유하므로, CLI로 한 번 인증하면 트레이도 그대로 동작합니다.

### 특징

- **클립보드 URL 감지** — `AddClipboardFormatListener`로 변경 이벤트를 받고
  새 내용에서 첫 `http(s)://` URL을 추출
- **네이티브 Win32 트레이 아이콘** — `Shell_NotifyIconW` + 우클릭 메뉴.
  별도의 GUI 프레임워크 없음
- **벌룬 알림** — 확인 프롬프트와 큐잉 결과를 Windows 액션 센터 벌룬으로 표시
- **일시정지 / 재개** — 트레이 메뉴에서 종료 없이 리스너 토글
- **마지막 URL 재전송** — 직전에 복사한 URL을 한 번 클릭으로 다시 큐잉
- **단일 바이너리** — 정적 링크, 약 1.5 MB, DLL 의존성 0
- **Per-Monitor DPI v2 매니페스트** — 혼합 DPI 환경에서 선명하게 렌더링

### 프로젝트 구조

```
rdl-tray/
├── Cargo.toml              패키지 매니페스트, 의존성, 릴리스 프로파일
├── rust-toolchain.toml     툴체인 고정 (stable / x86_64-pc-windows-msvc)
├── .cargo/config.toml      빌드 타겟 + 링커 플래그 (정적 CRT)
├── build.rs                Win32 매니페스트 + 버전 정보 임베드
├── src/
│   ├── main.rs             히든 윈도우 + Win32 메시지 루프
│   ├── tray.rs             Shell_NotifyIconW 라이프사이클 관리
│   ├── clipboard.rs        AddClipboardFormatListener + URL 추출
│   ├── notify.rs           NIM_MODIFY + NIF_INFO 기반 벌룬 알림
│   ├── client.rs           reqwest::blocking 클라이언트, POST /api/dl
│   ├── config.rs           설정 로드 + DPAPI 토큰 복호화
│   └── error.rs            크레이트 전체 에러 타입
├── tests/integration.rs    assert_cmd 기반 스모크 테스트
├── resources/              Win32 리소스 스크립트 + 매니페스트
└── .github/workflows/      cargo build + fmt/clippy/test CI
```

### 동작 흐름

```
┌────────────────────────┐         ┌──────────────────────┐         ┌──────────────────┐
│  Windows 클립보드      │  변경   │  rdl-tray.exe        │  HTTPS  │  엣지 워커       │
│  (어떤 앱이든)         │ ───────▶│  (히든 윈도우)       │ ───────▶│  /api/dl         │
└────────────────────────┘         │  Shell_NotifyIconW   │         └──────────────────┘
                                   │  + 벌룬 프롬프트     │
                                   └──────────────────────┘
```

### 설치

#### 사전 빌드 바이너리 (Windows x64)

1. [Releases](https://github.com/GHSFS/rdl-tray/releases)에서
   `rdl-tray-x64.exe` 다운로드
2. 적당한 경로에 배치 (예: `C:\Tools\rdl-tray.exe`)
3. (선택) 시작 폴더에 바로가기 추가하면 로그인 시 자동 실행
4. 최초 설정: 트레이는 `rdl` CLI의 설정에서 워커 URL과 인증 정보를 읽습니다.
   CLI에서 `rdl auth login --token <토큰>`을 한 번 실행하세요.

#### 소스 빌드

Rust 1.75+ 및 MSVC 툴체인 필요.

```bash
git clone https://github.com/GHSFS/rdl-tray.git
cd rdl-tray
cargo build --release
# 결과물: target/x86_64-pc-windows-msvc/release/rdl-tray.exe
```

### 트레이 메뉴

| 항목 | 동작 |
|---|---|
| Open web UI | 설정된 워커 URL을 기본 브라우저에서 열기 |
| Send last URL again | 가장 최근에 받은 URL 재큐잉 |
| Pause / Resume clipboard watch | 리스너 토글 |
| Quit | 트레이 아이콘 제거 후 종료 |

### 설정

`rdl-tray` 전용 설정은 없습니다. `rdl` CLI가 작성한
`%APPDATA%\rdl\config.json`을 그대로 사용하므로, CLI에서 한 번 인증하면 두
도구가 모두 동작합니다.

### 라이선스

MIT. [LICENSE](./LICENSE) 참고.

### 면책

개인용 보조 도구입니다. 출처 사이트의 이용약관 및 저작권법 준수는 전적으로
운영자(사용자) 본인의 책임입니다.

---

## 日本語

### 概要

`rdl-tray.exe` は [`rdl`](https://github.com/GHSFS/remote-dl) CLI クライアントと
組になって動く、軽量なネイティブ Windows アプリケーションです。CLI が
ターミナルでの `rdl <url>` 明示的な呼び出しを担当するのに対し、`rdl-tray` は
バックグラウンドの自動化を提供します — クリップボードに URL がコピーされる
たびに、クラウドストレージに取得するかどうかを通知で確認します。

両バイナリは同じディスク設定ファイル(`%APPDATA%\rdl\config.json`)を共有する
ため、CLI で一度認証すればトレイも動作します。

### 特徴

- **クリップボード URL 検出** — `AddClipboardFormatListener` で変更を購読し、
  最初の `http(s)://` URL を抽出
- **ネイティブ Win32 トレイアイコン** — `Shell_NotifyIconW` + 右クリックメニュー
- **バルーン通知** — 確認プロンプトとキュー結果を Windows アクションセンターに表示
- **一時停止 / 再開** — トレイメニューから終了せずにリスナーを切り替え
- **直近 URL の再送信** — 直前にコピーした URL をワンクリックで再キュー
- **単一バイナリ** — 静的リンク、約 1.5 MB、DLL 依存なし

### プロジェクト構成

```
rdl-tray/
├── Cargo.toml              パッケージマニフェスト、依存関係、リリースプロファイル
├── rust-toolchain.toml     ツールチェイン固定 (stable / x86_64-pc-windows-msvc)
├── .cargo/config.toml      ビルドターゲット + リンカフラグ (静的 CRT)
├── build.rs                Win32 マニフェスト + バージョン情報の埋め込み
├── src/
│   ├── main.rs             非表示ウィンドウ + Win32 メッセージループ
│   ├── tray.rs             Shell_NotifyIconW のライフサイクル管理
│   ├── clipboard.rs        AddClipboardFormatListener + URL 抽出
│   ├── notify.rs           NIM_MODIFY + NIF_INFO によるバルーン通知
│   ├── client.rs           reqwest::blocking クライアント、POST /api/dl
│   ├── config.rs           設定読み込み + DPAPI でのトークン復号
│   └── error.rs            クレート全体のエラー型
├── tests/integration.rs    assert_cmd によるスモークテスト
├── resources/              Win32 リソーススクリプト + マニフェスト
└── .github/workflows/      cargo build + fmt/clippy/test の CI
```

詳細なインストール、設定、CLI リファレンスは [English](#english) セクションを
参照してください。

### ライセンス

MIT。[LICENSE](./LICENSE) を参照。

---

## 中文

### 概述

`rdl-tray.exe` 是一个与 [`rdl`](https://github.com/GHSFS/remote-dl) CLI
客户端配套的小型原生 Windows 应用程序。CLI 处理终端中显式的 `rdl <url>`
调用,而 `rdl-tray` 提供后台场景:每当你将 URL 复制到剪贴板时,通知栏会询问
是否要将其下载到你的云存储。

两个二进制文件共享同一份磁盘配置文件(`%APPDATA%\rdl\config.json`),因此只需
通过 CLI 进行一次认证,托盘程序也能正常工作。

### 特性

- **剪贴板 URL 检测** — 使用 `AddClipboardFormatListener` 监听变化,提取第一个
  `http(s)://` URL
- **原生 Win32 托盘图标** — `Shell_NotifyIconW` + 右键菜单
- **气泡通知** — 在 Windows 操作中心显示确认提示和队列状态
- **暂停 / 恢复** — 通过托盘菜单切换监听器,无需退出
- **重新发送上一个 URL** — 一键重新加入最近复制的 URL
- **单一二进制** — 静态链接,约 1.5 MB,无 DLL 依赖

### 项目结构

```
rdl-tray/
├── Cargo.toml              包清单、依赖、发布配置
├── rust-toolchain.toml     工具链锁定 (stable / x86_64-pc-windows-msvc)
├── .cargo/config.toml      构建目标 + 链接器标志 (静态 CRT)
├── build.rs                嵌入 Win32 清单 + 版本信息
├── src/
│   ├── main.rs             隐藏窗口 + Win32 消息循环
│   ├── tray.rs             Shell_NotifyIconW 生命周期管理
│   ├── clipboard.rs        AddClipboardFormatListener + URL 提取
│   ├── notify.rs           通过 NIM_MODIFY + NIF_INFO 的气泡通知
│   ├── client.rs           reqwest::blocking 客户端,POST /api/dl
│   ├── config.rs           配置加载 + DPAPI 解密令牌
│   └── error.rs            crate 范围的错误类型
├── tests/integration.rs    基于 assert_cmd 的冒烟测试
├── resources/              Win32 资源脚本 + 清单
└── .github/workflows/      cargo build + fmt/clippy/test 的 CI
```

完整的安装、配置和 CLI 参考请参见 [English](#english) 部分。

### 许可证

MIT。详见 [LICENSE](./LICENSE)。

---

## Русский

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

- **Обнаружение URL в буфере обмена** — слушатель `AddClipboardFormatListener`
  и извлечение первого `http(s)://` URL
- **Нативная иконка в области уведомлений Win32** — `Shell_NotifyIconW`
  с контекстным меню по правому клику
- **Всплывающие уведомления** — подтверждения и статус очереди в Центре
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
│   ├── main.rs             Скрытое окно + цикл сообщений Win32
│   ├── tray.rs             Управление жизненным циклом Shell_NotifyIconW
│   ├── clipboard.rs        AddClipboardFormatListener + извлечение URL
│   ├── notify.rs           Всплывающие уведомления через NIM_MODIFY + NIF_INFO
│   ├── client.rs           reqwest::blocking клиент, POST /api/dl
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

---

## Tiếng Việt

### Tổng quan

`rdl-tray.exe` là một ứng dụng Windows native nhỏ gọn, đi kèm với client CLI
[`rdl`](https://github.com/GHSFS/remote-dl). Trong khi CLI xử lý các lệnh
`rdl <url>` rõ ràng từ terminal, `rdl-tray` lo phần tự động hóa nền — mỗi khi
bạn sao chép một URL vào clipboard, một thông báo sẽ hỏi liệu bạn có muốn
tải nó vào kho lưu trữ đám mây hay không.

Hai binary chia sẻ cùng một tệp cấu hình trên đĩa
(`%APPDATA%\rdl\config.json`), vì vậy chỉ cần xác thực một lần qua CLI là
tray cũng hoạt động.

### Tính năng

- **Phát hiện URL từ clipboard** — lắng nghe qua `AddClipboardFormatListener`
  và trích xuất URL `http(s)://` đầu tiên
- **Biểu tượng tray Win32 native** — `Shell_NotifyIconW` + menu chuột phải
- **Thông báo bóng** — lời nhắc xác nhận và trạng thái hàng đợi trong Action
  Center của Windows
- **Tạm dừng / tiếp tục** — bật/tắt listener từ menu tray mà không cần thoát
- **Gửi lại URL cuối cùng** — đưa lại URL vừa sao chép vào hàng đợi với một
  cú click
- **Binary đơn** — link tĩnh, khoảng 1.5 MB, không phụ thuộc DLL

### Cấu trúc dự án

```
rdl-tray/
├── Cargo.toml              Manifest gói, dependencies, cấu hình release
├── rust-toolchain.toml     Cố định toolchain (stable / x86_64-pc-windows-msvc)
├── .cargo/config.toml      Mục tiêu build + cờ linker (CRT tĩnh)
├── build.rs                Nhúng manifest Win32 + thông tin phiên bản
├── src/
│   ├── main.rs             Cửa sổ ẩn + vòng lặp message Win32
│   ├── tray.rs             Quản lý vòng đời Shell_NotifyIconW
│   ├── clipboard.rs        AddClipboardFormatListener + trích xuất URL
│   ├── notify.rs           Thông báo bóng qua NIM_MODIFY + NIF_INFO
│   ├── client.rs           Client reqwest::blocking, POST /api/dl
│   ├── config.rs           Tải cấu hình + giải mã token DPAPI
│   └── error.rs            Kiểu lỗi của crate
├── tests/integration.rs    Kiểm thử smoke dựa trên assert_cmd
├── resources/              Resource script Win32 + manifest
└── .github/workflows/      CI cho cargo build + fmt/clippy/test
```

Hướng dẫn cài đặt, cấu hình và tham chiếu CLI đầy đủ có ở phần
[English](#english).

### Giấy phép

MIT. Xem [LICENSE](./LICENSE).

---

## Türkçe

### Genel Bakış

`rdl-tray.exe`, [`rdl`](https://github.com/GHSFS/remote-dl) CLI istemcisiyle
eşleşen küçük yerel bir Windows uygulamasıdır. CLI, terminalden açık `rdl
<url>` çağrılarını işlerken, `rdl-tray` arka plan otomasyonunu sağlar —
panoya bir URL kopyaladığınızda, onu bulut depolamanıza indirmek isteyip
istemediğinizi soran bir bildirim gösterir.

Her iki ikili de aynı disk yapılandırma dosyasını
(`%APPDATA%\rdl\config.json`) paylaşır, bu nedenle CLI üzerinden bir kez
kimlik doğrulaması yapmak tepsi için de yeterlidir.

### Özellikler

- **Pano URL algılama** — `AddClipboardFormatListener` ile değişiklikleri
  dinler ve içerikten ilk `http(s)://` URL'sini çıkarır
- **Yerel Win32 sistem tepsisi simgesi** — `Shell_NotifyIconW` + sağ tık menüsü
- **Balon bildirimleri** — onay istemleri ve kuyruk durumu Windows Eylem
  Merkezi'nde gösterilir
- **Duraklat / devam ettir** — uygulamayı kapatmadan tepsi menüsünden
  dinleyiciyi açıp kapatın
- **Son URL'yi tekrar gönder** — son kopyalanan URL'yi tek tıkla yeniden
  kuyruğa alın
- **Tek ikili dosya** — statik bağlı, ~1.5 MB, DLL bağımlılığı yok

### Proje yapısı

```
rdl-tray/
├── Cargo.toml              Paket manifesti, bağımlılıklar, release profili
├── rust-toolchain.toml     Toolchain sabitleme (stable / x86_64-pc-windows-msvc)
├── .cargo/config.toml      Derleme hedefi + bağlayıcı bayrakları (statik CRT)
├── build.rs                Win32 manifest + sürüm bilgisi gömme
├── src/
│   ├── main.rs             Gizli pencere + Win32 mesaj döngüsü
│   ├── tray.rs             Shell_NotifyIconW yaşam döngüsü yönetimi
│   ├── clipboard.rs        AddClipboardFormatListener + URL çıkarma
│   ├── notify.rs           NIM_MODIFY + NIF_INFO ile balon bildirimleri
│   ├── client.rs           reqwest::blocking istemcisi, POST /api/dl
│   ├── config.rs           Yapılandırma yükleme + DPAPI ile token çözme
│   └── error.rs            Crate genelinde hata türü
├── tests/integration.rs    assert_cmd tabanlı smoke testler
├── resources/              Win32 resource script + manifest
└── .github/workflows/      cargo build + fmt/clippy/test için CI
```

Ayrıntılı kurulum, yapılandırma ve CLI başvurusu için [English](#english)
bölümüne bakın.

### Lisans

MIT. [LICENSE](./LICENSE) dosyasına bakın.

---

## Deutsch

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

- **URL-Erkennung in der Zwischenablage** — hört über
  `AddClipboardFormatListener` mit und extrahiert die erste `http(s)://`-URL
- **Natives Win32-Tray-Symbol** — `Shell_NotifyIconW` + Rechtsklick-Menü
- **Sprechblasen-Benachrichtigungen** — Bestätigungsaufforderungen und
  Warteschlangenstatus im Windows Action Center
- **Pause / Fortsetzen** — Listener über das Tray-Menü umschalten, ohne zu
  beenden
- **Letzte URL erneut senden** — die zuletzt kopierte URL mit einem Klick
  erneut einreihen
- **Einzelne Binärdatei** — statisch gelinkt, ~1.5 MB, keine DLL-Abhängigkeiten

### Projektstruktur

```
rdl-tray/
├── Cargo.toml              Paketmanifest, Abhängigkeiten, Release-Profil
├── rust-toolchain.toml     Toolchain-Pin (stable / x86_64-pc-windows-msvc)
├── .cargo/config.toml      Build-Ziel + Linker-Flags (statische CRT)
├── build.rs                Bettet Win32-Manifest + Versionsinfo ein
├── src/
│   ├── main.rs             Verstecktes Fenster + Win32-Nachrichtenschleife
│   ├── tray.rs             Shell_NotifyIconW-Lebenszyklusverwaltung
│   ├── clipboard.rs        AddClipboardFormatListener + URL-Extraktion
│   ├── notify.rs           Sprechblasen-Benachrichtigungen via NIM_MODIFY + NIF_INFO
│   ├── client.rs           reqwest::blocking-Client, POST /api/dl
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

---

## Español

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

### Características

- **Detección de URL en portapapeles** — escucha mediante
  `AddClipboardFormatListener` y extrae la primera URL `http(s)://`
- **Icono nativo de bandeja Win32** — `Shell_NotifyIconW` + menú contextual
- **Notificaciones tipo globo** — solicitudes de confirmación y estado de la
  cola en el Centro de Acciones de Windows
- **Pausar / reanudar** — alternar el listener desde el menú sin salir
- **Reenviar la última URL** — volver a encolar la URL copiada anteriormente
  con un solo clic
- **Binario único** — enlazado estáticamente, ~1.5 MB, sin dependencias DLL

### Estructura del proyecto

```
rdl-tray/
├── Cargo.toml              Manifiesto del paquete, dependencias, perfil release
├── rust-toolchain.toml     Fijación del toolchain (stable / x86_64-pc-windows-msvc)
├── .cargo/config.toml      Destino de compilación + flags del linker (CRT estático)
├── build.rs                Incrusta el manifiesto Win32 + información de versión
├── src/
│   ├── main.rs             Ventana oculta + bucle de mensajes Win32
│   ├── tray.rs             Gestión del ciclo de vida de Shell_NotifyIconW
│   ├── clipboard.rs        AddClipboardFormatListener + extracción de URL
│   ├── notify.rs           Notificaciones tipo globo vía NIM_MODIFY + NIF_INFO
│   ├── client.rs           Cliente reqwest::blocking, POST /api/dl
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

---

## Português

### Visão geral

`rdl-tray.exe` é um pequeno aplicativo nativo do Windows que faz par com o
cliente CLI [`rdl`](https://github.com/GHSFS/remote-dl). Enquanto a CLI lida
com invocações explícitas `rdl <url>` no terminal, o `rdl-tray` cobre o caso
ambiente: sempre que você copia uma URL para a área de transferência, uma
notificação pergunta se deseja baixá-la para seu armazenamento em nuvem.

Ambos os binários compartilham o mesmo arquivo de configuração em disco
(`%APPDATA%\rdl\config.json`), portanto autenticar-se uma vez pela CLI já é
suficiente para que o tray também funcione.

### Recursos

- **Detecção de URL na área de transferência** — escuta via
  `AddClipboardFormatListener` e extrai a primeira URL `http(s)://`
- **Ícone nativo Win32 na bandeja** — `Shell_NotifyIconW` + menu de clique
  direito
- **Notificações em balão** — solicitações de confirmação e status da fila
  na Central de Ações do Windows
- **Pausar / retomar** — alternar o listener pelo menu da bandeja sem sair
- **Reenviar a última URL** — recolocar a URL copiada anteriormente na fila
  com um clique
- **Binário único** — linkado estaticamente, ~1.5 MB, sem dependências de DLL

### Estrutura do projeto

```
rdl-tray/
├── Cargo.toml              Manifesto do pacote, dependências, perfil release
├── rust-toolchain.toml     Fixação do toolchain (stable / x86_64-pc-windows-msvc)
├── .cargo/config.toml      Alvo de build + flags do linker (CRT estática)
├── build.rs                Embute o manifesto Win32 + informação de versão
├── src/
│   ├── main.rs             Janela oculta + loop de mensagens Win32
│   ├── tray.rs             Gerenciamento do ciclo de vida do Shell_NotifyIconW
│   ├── clipboard.rs        AddClipboardFormatListener + extração de URL
│   ├── notify.rs           Notificações em balão via NIM_MODIFY + NIF_INFO
│   ├── client.rs           Cliente reqwest::blocking, POST /api/dl
│   ├── config.rs           Carga de configuração + descriptografia DPAPI do token
│   └── error.rs            Tipo de erro do crate
├── tests/integration.rs    Testes smoke baseados em assert_cmd
├── resources/              Script de recursos Win32 + manifesto
└── .github/workflows/      CI para cargo build + fmt/clippy/test
```

Para instruções completas de instalação, configuração e referência da CLI,
consulte a seção [English](#english).

### Licença

MIT. Veja [LICENSE](./LICENSE).
