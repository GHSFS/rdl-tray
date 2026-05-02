//! `rdl-tray` — system tray companion for remote-dl.
//!
//! Runs as a hidden window that listens for clipboard changes. When a URL
//! that looks like a download target appears, a balloon notification asks
//! whether to queue it via the configured remote-dl backend.

#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod clipboard;
mod client;
mod config;
mod error;
mod notify;
mod tray;

use error::Result;
use std::cell::RefCell;
use windows::core::PCWSTR;
use windows::Win32::Foundation::{HINSTANCE, HWND, LPARAM, LRESULT, WPARAM};
use windows::Win32::System::LibraryLoader::GetModuleHandleW;
use windows::Win32::UI::WindowsAndMessaging::*;

const WM_TRAY_ICON: u32 = WM_APP + 1;
const WM_CLIPBOARD_URL: u32 = WM_APP + 2;

const ID_TRAY_OPEN: u32 = 1001;
const ID_TRAY_LAST: u32 = 1002;
const ID_TRAY_TOGGLE_WATCH: u32 = 1003;
const ID_TRAY_QUIT: u32 = 1099;

const ID_CONFIRM_YES: u32 = 2001;

/// Per-process state shared between the message loop and the worker callback.
struct AppState {
    hwnd: HWND,
    watching: bool,
    pending_url: Option<String>,
    last_url: Option<String>,
}

thread_local! {
    static STATE: RefCell<Option<AppState>> = const { RefCell::new(None) };
}

fn main() -> Result<()> {
    unsafe {
        let hmodule = GetModuleHandleW(None)?;
        let hinst = HINSTANCE(hmodule.0);

        let class_name = wide("rdl_tray_window");
        let wc = WNDCLASSEXW {
            cbSize: std::mem::size_of::<WNDCLASSEXW>() as u32,
            lpfnWndProc: Some(window_proc),
            hInstance: hinst,
            lpszClassName: PCWSTR(class_name.as_ptr()),
            ..Default::default()
        };
        RegisterClassExW(&wc);

        let hwnd = CreateWindowExW(
            WINDOW_EX_STYLE(0),
            PCWSTR(class_name.as_ptr()),
            PCWSTR(wide("rdl-tray").as_ptr()),
            WS_OVERLAPPEDWINDOW,
            CW_USEDEFAULT,
            CW_USEDEFAULT,
            CW_USEDEFAULT,
            CW_USEDEFAULT,
            None,
            None,
            hinst,
            None,
        )?;

        STATE.with(|s| {
            *s.borrow_mut() = Some(AppState {
                hwnd,
                watching: true,
                pending_url: None,
                last_url: None,
            })
        });

        tray::install(hwnd, WM_TRAY_ICON)?;
        clipboard::start_listening(hwnd)?;

        notify::balloon(
            hwnd,
            "rdl-tray",
            "Watching clipboard for download URLs.",
        );

        let mut msg = MSG::default();
        loop {
            let r = GetMessageW(&mut msg, None, 0, 0);
            if r.0 == 0 || r.0 == -1 {
                break;
            }
            let _ = TranslateMessage(&msg);
            DispatchMessageW(&msg);
        }

        clipboard::stop_listening(hwnd);
        tray::remove(hwnd);
    }
    Ok(())
}

unsafe extern "system" fn window_proc(
    hwnd: HWND,
    msg: u32,
    wparam: WPARAM,
    lparam: LPARAM,
) -> LRESULT {
    match msg {
        WM_DESTROY => {
            PostQuitMessage(0);
            LRESULT(0)
        }
        WM_TRAY_ICON => {
            let event = (lparam.0 as u32) & 0xFFFF;
            if event == WM_RBUTTONUP || event == WM_CONTEXTMENU {
                show_tray_menu(hwnd);
            } else if event == WM_LBUTTONDBLCLK {
                let _ = client::open_web_ui();
            }
            LRESULT(0)
        }
        clipboard::WM_CLIPBOARDUPDATE => {
            if let Some(url) = clipboard::read_url() {
                with_state(|s| {
                    if !s.watching || Some(&url) == s.last_url.as_ref() {
                        return;
                    }
                    s.pending_url = Some(url.clone());
                    s.last_url = Some(url.clone());
                    notify::balloon_with_action(
                        hwnd,
                        "Send to remote-dl?",
                        &truncate(&url, 80),
                    );
                });
            }
            LRESULT(0)
        }
        WM_COMMAND => {
            let id = (wparam.0 as u32) & 0xFFFF;
            handle_command(hwnd, id);
            LRESULT(0)
        }
        WM_CLIPBOARD_URL => LRESULT(0),
        _ => DefWindowProcW(hwnd, msg, wparam, lparam),
    }
}

unsafe fn show_tray_menu(hwnd: HWND) {
    let menu = match CreatePopupMenu() {
        Ok(m) => m,
        Err(_) => return,
    };
    let watching_label = with_state(|s| {
        if s.watching {
            wide("Pause clipboard watch")
        } else {
            wide("Resume clipboard watch")
        }
    });
    let _ = AppendMenuW(menu, MF_STRING, ID_TRAY_OPEN as usize, PCWSTR(wide("Open web UI").as_ptr()));
    let _ = AppendMenuW(
        menu,
        MF_STRING,
        ID_TRAY_LAST as usize,
        PCWSTR(wide("Send last URL again").as_ptr()),
    );
    let _ = AppendMenuW(
        menu,
        MF_STRING,
        ID_TRAY_TOGGLE_WATCH as usize,
        PCWSTR(watching_label.as_ptr()),
    );
    let _ = AppendMenuW(menu, MF_SEPARATOR, 0, PCWSTR::null());
    let _ = AppendMenuW(menu, MF_STRING, ID_TRAY_QUIT as usize, PCWSTR(wide("Quit").as_ptr()));

    let mut pt = windows::Win32::Foundation::POINT::default();
    let _ = windows::Win32::UI::WindowsAndMessaging::GetCursorPos(&mut pt);
    let _ = SetForegroundWindow(hwnd);
    let _ = TrackPopupMenu(menu, TPM_RIGHTBUTTON, pt.x, pt.y, 0, hwnd, None);
    let _ = DestroyMenu(menu);
}

unsafe fn handle_command(hwnd: HWND, id: u32) {
    match id {
        ID_TRAY_OPEN => {
            let _ = client::open_web_ui();
        }
        ID_TRAY_LAST => {
            with_state(|s| {
                if let Some(url) = s.last_url.clone() {
                    let _ = dispatch_pending(&url);
                }
            });
        }
        ID_TRAY_TOGGLE_WATCH => {
            with_state(|s| {
                s.watching = !s.watching;
                let msg = if s.watching {
                    "Watching clipboard."
                } else {
                    "Paused."
                };
                notify::balloon(hwnd, "rdl-tray", msg);
            });
        }
        ID_CONFIRM_YES => {
            let url = with_state(|s| s.pending_url.take());
            if let Some(url) = url {
                let _ = dispatch_pending(&url);
            }
        }
        ID_TRAY_QUIT => {
            DestroyWindow(hwnd).ok();
        }
        _ => {}
    }
}

fn dispatch_pending(url: &str) -> Result<()> {
    let cfg = config::Config::load()?;
    let job = client::queue(&cfg, url)?;
    with_state(|s| {
        notify::balloon(s.hwnd, "Queued", &format!("job {}", job.id));
    });
    Ok(())
}

fn with_state<R>(f: impl FnOnce(&mut AppState) -> R) -> R {
    STATE.with(|cell| {
        let mut borrow = cell.borrow_mut();
        let s = borrow.as_mut().expect("state not initialised");
        f(s)
    })
}

fn wide(s: &str) -> Vec<u16> {
    s.encode_utf16().chain(std::iter::once(0)).collect()
}

fn truncate(s: &str, n: usize) -> String {
    if s.chars().count() <= n {
        s.to_string()
    } else {
        let mut out: String = s.chars().take(n).collect();
        out.push('…');
        out
    }
}
