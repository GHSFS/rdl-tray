<div align="center">

<p align="center">
  <a href="../../README.md">🇺🇸 English</a> · <a href="./ko.md">🇰🇷 한국어</a> · <strong>🇨🇳 中文</strong> · <a href="./ja.md">🇯🇵 日本語</a> · <a href="./es.md">🇪🇸 Español</a><br />
  <a href="./ru.md">🇷🇺 Русский</a> · <a href="./vi.md">🇻🇳 Tiếng Việt</a> · <a href="./tr.md">🇹🇷 Türkçe</a> · <a href="./de.md">🇩🇪 Deutsch</a> · <a href="./pt-BR.md">🇧🇷 Português</a>
</p>

# rdl-tray

[![Build](https://github.com/GHSFS/rdl-tray/actions/workflows/build.yml/badge.svg)](https://github.com/GHSFS/rdl-tray/actions/workflows/build.yml)
[![Tests](https://github.com/GHSFS/rdl-tray/actions/workflows/test.yml/badge.svg)](https://github.com/GHSFS/rdl-tray/actions/workflows/test.yml)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](../../LICENSE)
[![Platform](https://img.shields.io/badge/platform-Windows%20x64-blue)](#installation)

</div>

---

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
