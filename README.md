# rdl-tray

> System tray companion for [remote-dl](https://github.com/GHSFS/remote-dl) — sits in the notification area, watches the clipboard for download URLs, and queues them with one click.

[![Build](https://github.com/GHSFS/rdl-tray/actions/workflows/build.yml/badge.svg)](https://github.com/GHSFS/rdl-tray/actions/workflows/build.yml)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](./LICENSE)
[![Platform](https://img.shields.io/badge/platform-Windows%20x64-blue)](#installation)

[English](#english) · [한국어](#한국어)

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
