<div align="center">

<p align="center">
  <a href="../../README.md">🇺🇸 English</a> · <strong>🇰🇷 한국어</strong> · <a href="./zh-CN.md">🇨🇳 中文</a> · <a href="./ja.md">🇯🇵 日本語</a> · <a href="./es.md">🇪🇸 Español</a><br />
  <a href="./ru.md">🇷🇺 Русский</a> · <a href="./vi.md">🇻🇳 Tiếng Việt</a> · <a href="./tr.md">🇹🇷 Türkçe</a> · <a href="./de.md">🇩🇪 Deutsch</a> · <a href="./pt-BR.md">🇧🇷 Português</a>
</p>

# rdl-tray

[![Build](https://github.com/GHSFS/rdl-tray/actions/workflows/build.yml/badge.svg)](https://github.com/GHSFS/rdl-tray/actions/workflows/build.yml)
[![Tests](https://github.com/GHSFS/rdl-tray/actions/workflows/test.yml/badge.svg)](https://github.com/GHSFS/rdl-tray/actions/workflows/test.yml)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](../../LICENSE)
[![Platform](https://img.shields.io/badge/platform-Windows%20x64-blue)](#installation)

</div>

---

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
