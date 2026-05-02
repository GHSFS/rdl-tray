//! Lightweight balloon notifications via `Shell_NotifyIconW(NIM_MODIFY)`.
//!
//! These are the legacy Win32 notifications. They are simpler than WinRT
//! toasts and require no AppUserModelID registration, which keeps the binary
//! self-contained and portable across Windows versions.

use windows::Win32::Foundation::HWND;
use windows::Win32::UI::Shell::*;

const TRAY_UID: u32 = 0xC0FFEE;

pub fn balloon(hwnd: HWND, title: &str, message: &str) {
    show(hwnd, title, message, NIIF_INFO);
}

pub fn balloon_with_action(hwnd: HWND, title: &str, message: &str) {
    // Reuse the same balloon machinery; the click action is delivered via
    // tray icon callback (NIN_BALLOONUSERCLICK) and handled in the window
    // procedure.
    show(hwnd, title, message, NIIF_INFO);
}

fn show(hwnd: HWND, title: &str, message: &str, icon: NOTIFY_ICON_INFOTIP_FLAGS) {
    unsafe {
        let mut data = NOTIFYICONDATAW {
            cbSize: std::mem::size_of::<NOTIFYICONDATAW>() as u32,
            hWnd: hwnd,
            uID: TRAY_UID,
            uFlags: NIF_INFO,
            ..Default::default()
        };
        let title_w = wide_buf(title);
        let msg_w = wide_buf(message);
        let n_title = title_w.len().min(data.szInfoTitle.len());
        let n_msg = msg_w.len().min(data.szInfo.len());
        data.szInfoTitle[..n_title].copy_from_slice(&title_w[..n_title]);
        data.szInfo[..n_msg].copy_from_slice(&msg_w[..n_msg]);
        data.dwInfoFlags = icon;

        let _ = Shell_NotifyIconW(NIM_MODIFY, &mut data);
    }
}

fn wide_buf(s: &str) -> Vec<u16> {
    s.encode_utf16().chain(std::iter::once(0)).collect()
}
