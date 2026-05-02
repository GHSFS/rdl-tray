//! Manage the notification-area icon via `Shell_NotifyIconW`.

use crate::error::Result;
use windows::core::PCWSTR;
use windows::Win32::Foundation::HWND;
use windows::Win32::UI::Shell::*;
use windows::Win32::UI::WindowsAndMessaging::{LoadIconW, IDI_APPLICATION};

const TRAY_UID: u32 = 0xC0FFEE; // arbitrary, must be stable for the same hwnd

unsafe fn make_tray_data(hwnd: HWND, callback_msg: u32) -> NOTIFYICONDATAW {
    let icon = LoadIconW(None, IDI_APPLICATION).unwrap_or_default();

    let mut data = NOTIFYICONDATAW {
        cbSize: std::mem::size_of::<NOTIFYICONDATAW>() as u32,
        hWnd: hwnd,
        uID: TRAY_UID,
        uFlags: NIF_ICON | NIF_MESSAGE | NIF_TIP,
        uCallbackMessage: callback_msg,
        hIcon: icon,
        ..Default::default()
    };
    let tip = wide_buf("rdl-tray — clipboard URL watcher");
    let n = tip.len().min(data.szTip.len());
    data.szTip[..n].copy_from_slice(&tip[..n]);
    data
}

pub fn install(hwnd: HWND, callback_msg: u32) -> Result<()> {
    unsafe {
        let mut data = make_tray_data(hwnd, callback_msg);
        if !Shell_NotifyIconW(NIM_ADD, &mut data).as_bool() {
            return Err(crate::error::Error::Win32(
                "Shell_NotifyIconW(NIM_ADD) failed".into(),
            ));
        }
        // Request modern (Vista+) version semantics.
        data.Anonymous.uVersion = NOTIFYICON_VERSION_4;
        let _ = Shell_NotifyIconW(NIM_SETVERSION, &mut data);
    }
    Ok(())
}

pub fn remove(hwnd: HWND) {
    unsafe {
        let mut data = NOTIFYICONDATAW {
            cbSize: std::mem::size_of::<NOTIFYICONDATAW>() as u32,
            hWnd: hwnd,
            uID: TRAY_UID,
            ..Default::default()
        };
        let _ = Shell_NotifyIconW(NIM_DELETE, &mut data);
    }
}

fn wide_buf(s: &str) -> Vec<u16> {
    s.encode_utf16().chain(std::iter::once(0)).collect()
}

// PCWSTR helper kept for convenience if other modules want it.
#[allow(dead_code)]
fn pcwstr(buf: &[u16]) -> PCWSTR {
    PCWSTR(buf.as_ptr())
}
