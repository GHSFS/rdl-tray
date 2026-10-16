# rdl-tray

> System tray companion for [remote-dl](https://github.com/GHSFS/remote-dl) — sits in the notification area, watches the clipboard for download URLs, and queues them with one click.

[![Build](https://github.com/GHSFS/rdl-tray/actions/workflows/build.yml/badge.svg)](https://github.com/GHSFS/rdl-tray/actions/workflows/build.yml)
[![Tests](https://github.com/GHSFS/rdl-tray/actions/workflows/test.yml/badge.svg)](https://github.com/GHSFS/rdl-tray/actions/workflows/test.yml)
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

- **클립보드 URL 및 마그넷 감지** — `AddClipboardFormatListener`로 변경 이벤트를 받고
  새 내용에서 첫 `http(s)://` 또는 `magnet:?xt=...` 링크를 즉시 추출
- **대화형 벌룬 클릭 지원** — 알림창 클릭(`NIN_BALLOONUSERCLICK`)으로 다운로드 즉시 실행
- **비동기 논블로킹 큐잉** — 백그라운드 워커 스레드로 네트워크 I/O를 격리하여
  서버 지연 중에도 트레이 및 메시지 루프 프리징 0%
- **단일 인스턴스 보호 (Named Mutex)** — 중복 실행을 방지하여 트레이 아이콘과
  클립보드 리스너의 충돌 원천 차단
- **Windows 시작 시 자동 실행** — 트레이 메뉴에서 간편하게 시작프로그램 등록/해제
  (`HKCU\Software\Microsoft\Windows\CurrentVersion\Run`)
- **HTTP 커넥션 풀링** — `OnceLock` 캐싱으로 TLS 핸드셰이크 지연시간 최소화
- **네이티브 Win32 트레이 아이콘** — `Shell_NotifyIconW` + 우클릭 메뉴.
  별도의 무거운 프레임워크나 런타임 의존성 0
- **벌룬 알림** — 확인 프롬프트, 작업 접수 번호, 오류 안내를 Windows 액션 센터 벌룬으로 표시
- **일시정지 / 재개** — 트레이 메뉴에서 프로그램 종료 없이 클립보드 감시 토글
- **마지막 URL 재전송** — 직전에 복사한 URL을 한 번 클릭으로 다시 큐잉
- **단일 바이너리** — 정적 링크, 약 1.5 MB, DLL 의존성 0
- **Per-Monitor DPI v2 매니페스트** — 혼합 DPI 다중 모니터 환경에서 선명하게 렌더링

### 프로젝트 구조

```
rdl-tray/
├── Cargo.toml              패키지 매니페스트, 의존성, 릴리스 프로파일
├── rust-toolchain.toml     툴체인 고정 (stable / x86_64-pc-windows-msvc)
├── .cargo/config.toml      빌드 타겟 + 링커 플래그 (정적 CRT)
├── build.rs                Win32 매니페스트 + 버전 정보 임베드
├── src/
│   ├── main.rs             히든 윈도우 + Win32 메시지 루프, 알림 클릭 및 비동기 디스패치
│   ├── autostart.rs        Windows 시작프로그램 레지스트리 관리
│   ├── tray.rs             Shell_NotifyIconW 라이프사이클 관리
│   ├── clipboard.rs        AddClipboardFormatListener + 메모리 안전 URL/마그넷 추출
│   ├── notify.rs           NIM_MODIFY 기반 정보/에러 벌룬 알림
│   ├── client.rs           풀링된 reqwest 클라이언트, POST /api/dl
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
                                   │  + 벌룬 클릭 즉시실행│
                                   └──────────────────────┘
```

### 호환성

| 항목 | 지원 사양 |
|---|---|
| 운영체제 | Windows 10 1809+ 및 Windows 11 (PerMonitorV2 매니페스트 적용) |
| 아키텍처 | x86_64 전용 |
| Rust 툴체인 | 1.75+ (`rust-toolchain.toml`에서 stable 고정) |
| 링커 | MSVC (Visual Studio Build Tools 2022) |

배포 바이너리는 MSVC 런타임을 정적 링크(`+crt-static`)하므로 Windows 기본 시스템 DLL(`KERNEL32.DLL`, `USER32.DLL`, `SHELL32.DLL`, `ADVAPI32.DLL`, `CRYPT32.DLL`, `WS2_32.DLL`) 외에 별도의 외부 DLL 의존성이 없습니다.

### 보안 고려사항

- **저장 시 토큰 암호화** — Bearer 토큰은 Windows DPAPI(`CryptProtectData`)로 암호화되어 저장되며, 생성한 Windows 사용자 세션에서만 복호화할 수 있습니다.
- **전송 구간 암호화** — `reqwest`는 `https_only=true`로 설정되어 평문 HTTP 리다이렉트를 거부하며 토큰은 TLS `Authorization` 헤더로만 전송됩니다.
- **텔레메트리 및 자동 업데이트 없음** — 설정된 워커 URL 외의 외부 통신이나 백그라운드 업데이터가 일절 없습니다.
- **콘솔 창 비노출** — `windows_subsystem = "windows"`로 빌드되어 콘솔 창을 통한 정보 노출이 없습니다.

### 문제 해결

| 증상 | 주요 원인 | 해결 방법 |
|---|---|---|
| `config not found` 벌룬 알림 | `rdl auth login` 미실행 | 터미널에서 `rdl auth login --token <토큰>` 1회 실행 |
| `server rejected credentials (401)` | 토큰 폐기 또는 만료 | 텔레그램 봇에서 새 토큰 발급 후 재로그인 |
| 트레이 아이콘 미표시 | 동일 `uID`를 점유한 기존 프로세스 존재 | 작업 관리자에서 잔류 `rdl-tray.exe` 종료 후 재실행 |
| 클립보드 복사 시 알림 미표시 | 클립보드 리스너 등록 실패 | 트레이 메뉴에서 Pause / Resume 토글 또는 앱 재시작 |

### 기여하기

PR 제출 전 아래 세 가지 검사를 통과해야 합니다:

```bash
cargo fmt
cargo clippy --all-targets -- -D warnings
cargo test --all-targets
```

### 감사의 글

- [`windows`](https://crates.io/crates/windows) — 공식 Win32 바인딩 (`Shell_NotifyIconW`, `AddClipboardFormatListener`, `CryptProtectData`)
- [`reqwest`](https://crates.io/crates/reqwest) + [`rustls`](https://crates.io/crates/rustls) — TLS 전용 HTTP 클라이언트
- [`directories`](https://crates.io/crates/directories) — `%APPDATA%\rdl\config.json` 경로 탐색
- [`embed-resource`](https://crates.io/crates/embed-resource) — Win32 매니페스트 PE 임베딩

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

- **クリップボード URL & Magnet 検出** — `AddClipboardFormatListener` で変更を購読し、
  最初の `http(s)://` または `magnet:?xt=...` リンクを抽出
- **対話型バルーンクリック** — 通知バルーンのクリック(`NIN_BALLOONUSERCLICK`)で即座にキューへ送信
- **非同期ノンブロッキングキューイング** — バックグラウンドスレッドで通信し、UIのフリーズを完全排除
- **多重起動防止 (Named Mutex)** — トレイアイコンの重複やクリップボードイベント競合を防止
- **Windows 起動時自動実行** — トレイメニューからスタートアップ登録/解除を切り替え
- **HTTP コネクションプーリング** — `OnceLock` による TLS ハンドシェイクオーバーヘッド削減
- **ネイティブ Win32 トレイアイコン** — `Shell_NotifyIconW` + 右クリックメニュー
- **バルーン通知** — 確認プロンプト、ジョブ受付、エラー通知をアクションセンターに表示
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
│   ├── main.rs             非表示ウィンドウ + Win32 メッセージループ、クリック＆非同期キュー
│   ├── autostart.rs        Windows スタートアップレジストリ管理
│   ├── tray.rs             Shell_NotifyIconW のライフサイクル管理
│   ├── clipboard.rs        AddClipboardFormatListener + 安全な URL/Magnet 抽出
│   ├── notify.rs           NIM_MODIFY による情報/エラーバルーン通知
│   ├── client.rs           プールされた reqwest クライアント、POST /api/dl
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

- **剪贴板 URL 及 Magnet 链接检测** — 使用 `AddClipboardFormatListener` 监听变化,提取第一个
  `http(s)://` 或 `magnet:?xt=...` 链接
- **交互式气泡点击** — 直接点击气泡通知(`NIN_BALLOONUSERCLICK`)即可立即派发下载任务
- **异步非阻塞排队** — 后台工作线程处理网络请求,彻底杜绝界面卡顿与无响应
- **防多开保护 (Named Mutex)** — 命名互斥体防止重复运行,杜绝多图标与重复剪贴板监听
- **随 Windows 开机自启** — 托盘右键菜单直接开关自启动注册表项
- **HTTP 连接池** — `OnceLock` 缓存客户端,消除重复建立 TLS 握手的开销
- **原生 Win32 托盘图标** — `Shell_NotifyIconW` + 右键菜单,无厚重 GUI 框架
- **气泡通知** — 在 Windows 操作中心显示确认提示、任务编号及错误通知
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
│   ├── main.rs             隐藏窗口 + Win32 消息循环,支持通知点击与异步排队
│   ├── autostart.rs        Windows 自启注册表管理
│   ├── tray.rs             Shell_NotifyIconW 生命周期管理
│   ├── clipboard.rs        AddClipboardFormatListener + 内存安全 URL/Magnet 提取
│   ├── notify.rs           基于 NIM_MODIFY 的信息/错误气泡通知
│   ├── client.rs           复用连接池的 reqwest 客户端,POST /api/dl
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

- **Phát hiện URL & Magnet từ clipboard** — lắng nghe qua `AddClipboardFormatListener`
  và trích xuất link `http(s)://` hoặc `magnet:?xt=...` đầu tiên
- **Nhấp vào thông báo bóng tương tác** — nhấp vào bóng (`NIN_BALLOONUSERCLICK`)
  để ngay lập tức đưa tác vụ vào hàng đợi
- **Hàng đợi bất đồng bộ không đơ giao diện** — luồng nền xử lý mạng giúp tray
  không bao giờ bị giật lag
- **Chống chạy trùng (Named Mutex)** — bảo đảm chỉ một tiến trình chạy, loại bỏ
  biểu tượng rác và trùng lặp sự kiện
- **Khởi động cùng Windows** — bật/tắt tự khởi động ngay trong menu chuột phải
  (`HKCU\Software\Microsoft\Windows\CurrentVersion\Run`)
- **Tái sử dụng kết nối HTTP** — `OnceLock` giảm thiểu độ trễ bắt tay TLS
- **Biểu tượng tray Win32 native** — `Shell_NotifyIconW` + menu chuột phải
- **Thông báo bóng** — lời nhắc xác nhận, mã công việc và thông báo lỗi
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
│   ├── main.rs             Cửa sổ ẩn + vòng lặp Win32, nhấp thông báo & hàng đợi async
│   ├── autostart.rs        Quản lý tự khởi động qua registry Windows
│   ├── tray.rs             Quản lý vòng đời Shell_NotifyIconW
│   ├── clipboard.rs        AddClipboardFormatListener + trích xuất an toàn URL/Magnet
│   ├── notify.rs           Thông báo bóng qua NIM_MODIFY (thông tin / lỗi)
│   ├── client.rs           Client reqwest tái sử dụng kết nối, POST /api/dl
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

- **Pano URL & Magnet algılama** — `AddClipboardFormatListener` ile değişiklikleri
  dinler ve ilk `http(s)://` veya `magnet:?xt=...` bağlantısını çıkarır
- **Etkileşimli balon tıklaması** — bildirime tıklayarak (`NIN_BALLOONUSERCLICK`)
  indirme görevini anında kuyruğa gönderin
- **Arayüzü dondurmayan asenkron kuyruk** — arka plan iş parçacığı sayesinde
  ağ gecikmelerinde tepsi asla donmaz
- **Çoklu başlatma koruması (Named Mutex)** — yinelenen simgeleri ve çakışan
  pano dinleyicilerini engeller
- **Windows ile başlatma** — tepsi sağ tık menüsünden başlangıç kaydını
  kolayca açıp kapatın (`HKCU\Software\Microsoft\Windows\CurrentVersion\Run`)
- **HTTP bağlantı havuzu** — `OnceLock` ile TLS el sıkışma gecikmesini sıfıra indirir
- **Yerel Win32 sistem tepsisi simgesi** — `Shell_NotifyIconW` + sağ tık menüsü
- **Balon bildirimleri** — onay istemleri, iş kimlikleri ve hata bildirimleri
  Windows Eylem Merkezi'nde gösterilir
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
│   ├── main.rs             Gizli pencere + Win32 mesaj döngüsü, bildirim tıklama ve asenkron kuyruk
│   ├── autostart.rs        Windows başlangıç kayıt defteri yönetimi
│   ├── tray.rs             Shell_NotifyIconW yaşam döngüsü yönetimi
│   ├── clipboard.rs        AddClipboardFormatListener + güvenli URL/Magnet çıkarma
│   ├── notify.rs           NIM_MODIFY ile bilgi/hata balon bildirimleri
│   ├── client.rs           Havuzlanmış reqwest istemcisi, POST /api/dl
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

- **Detecção de URL e links Magnet** — escuta via `AddClipboardFormatListener`
  e extrai o primeiro link `http(s)://` ou `magnet:?xt=...`
- **Clique interativo no balão** — clique na notificação (`NIN_BALLOONUSERCLICK`)
  para despachar imediatamente o download
- **Enfileiramento assíncrono sem travamentos** — thread em segundo plano garante
  zero congelamentos na interface do Windows
- **Proteção contra múltiplas instâncias (Named Mutex)** — impede processos
  duplicados e conflitos de listeners
- **Iniciar com o Windows** — alterne o início automático no logon diretamente pelo menu
  (`HKCU\Software\Microsoft\Windows\CurrentVersion\Run`)
- **Pool de conexões HTTP** — cliente em cache (`OnceLock`) elimina o overhead de handshake TLS
- **Ícone nativo Win32 na bandeja** — `Shell_NotifyIconW` + menu de clique direito
- **Notificações em balão** — solicitações de confirmação, IDs de trabalho e alertas de erro
- **Pausar / retomar** — alternar o listener pelo menu da bandeja sem sair
- **Reenviar a última URL** — recolocar a URL copiada anteriormente na fila com um clique
- **Binário único** — linkado estaticamente, ~1.5 MB, sem dependências de DLL

### Estrutura do projeto

```
rdl-tray/
├── Cargo.toml              Manifesto do pacote, dependências, perfil release
├── rust-toolchain.toml     Fixação do toolchain (stable / x86_64-pc-windows-msvc)
├── .cargo/config.toml      Alvo de build + flags do linker (CRT estática)
├── build.rs                Embute o manifesto Win32 + informação de versão
├── src/
│   ├── main.rs             Janela oculta + loop de mensagens Win32, clique no balão e filas async
│   ├── autostart.rs        Gerenciamento do registro de inicialização do Windows
│   ├── tray.rs             Gerenciamento do ciclo de vida do Shell_NotifyIconW
│   ├── clipboard.rs        AddClipboardFormatListener + extração segura de URL/Magnet
│   ├── notify.rs           Notificações em balão via NIM_MODIFY (info / erro)
│   ├── client.rs           Cliente reqwest com pool de conexões, POST /api/dl
│   ├── config.rs           Carga de configuração + descriptografia DPAPI do token
│   └── error.rs            Tipo de error do crate
├── tests/integration.rs    Testes smoke baseados em assert_cmd
├── resources/              Script de recursos Win32 + manifesto
└── .github/workflows/      CI para cargo build + fmt/clippy/test
```

Para instruções completas de instalação, configuração e referência da CLI,
consulte a seção [English](#english).

### Licença

MIT. Veja [LICENSE](./LICENSE).
