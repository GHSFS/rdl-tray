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

Para instruções completas de instalação, configuração e referência da CLI,
consulte a seção [English](#english).

### Licença

MIT. Veja [LICENSE](./LICENSE).
