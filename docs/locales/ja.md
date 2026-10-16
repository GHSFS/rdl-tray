<div align="center">

<p align="center">
  <a href="../../README.md">🇺🇸 English</a> · <a href="./ko.md">🇰🇷 한국어</a> · <a href="./zh-CN.md">🇨🇳 中文</a> · <strong>🇯🇵 日本語</strong> · <a href="./es.md">🇪🇸 Español</a><br />
  <a href="./ru.md">🇷🇺 Русский</a> · <a href="./vi.md">🇻🇳 Tiếng Việt</a> · <a href="./tr.md">🇹🇷 Türkçe</a> · <a href="./de.md">🇩🇪 Deutsch</a> · <a href="./pt-BR.md">🇧🇷 Português</a>
</p>

# rdl-tray

[![Build](https://github.com/GHSFS/rdl-tray/actions/workflows/build.yml/badge.svg)](https://github.com/GHSFS/rdl-tray/actions/workflows/build.yml)
[![Tests](https://github.com/GHSFS/rdl-tray/actions/workflows/test.yml/badge.svg)](https://github.com/GHSFS/rdl-tray/actions/workflows/test.yml)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](../../LICENSE)
[![Platform](https://img.shields.io/badge/platform-Windows%20x64-blue)](#installation)

</div>

---

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
