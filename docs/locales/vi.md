<div align="center">

<p align="center">
  <a href="../../README.md">🇺🇸 English</a> · <a href="./ko.md">🇰🇷 한국어</a> · <a href="./zh-CN.md">🇨🇳 中文</a> · <a href="./ja.md">🇯🇵 日本語</a> · <a href="./es.md">🇪🇸 Español</a><br />
  <a href="./ru.md">🇷🇺 Русский</a> · <strong>🇻🇳 Tiếng Việt</strong> · <a href="./tr.md">🇹🇷 Türkçe</a> · <a href="./de.md">🇩🇪 Deutsch</a> · <a href="./pt-BR.md">🇧🇷 Português</a>
</p>

# rdl-tray

[![Build](https://github.com/GHSFS/rdl-tray/actions/workflows/build.yml/badge.svg)](https://github.com/GHSFS/rdl-tray/actions/workflows/build.yml)
[![Tests](https://github.com/GHSFS/rdl-tray/actions/workflows/test.yml/badge.svg)](https://github.com/GHSFS/rdl-tray/actions/workflows/test.yml)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](../../LICENSE)
[![Platform](https://img.shields.io/badge/platform-Windows%20x64-blue)](#installation)

</div>

---

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
