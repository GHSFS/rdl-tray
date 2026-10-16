//! `rdl-tray` — system tray companion for remote-dl.
//!
//! Runs as a hidden window that listens for clipboard changes. When a URL
//! that looks like a download target appears, a balloon notification asks
//! whether to queue it via the configured remote-dl backend.

#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod autostart;
mod client;
mod clipboard;
mod config;
mod error;
mod notify;
mod tray;

use error::Result;
use std::cell::RefCell;
use windows::core::PCWSTR;
use windows::Win32::Foundation::{
    CloseHandle, GetLastError, ERROR_ALREADY_EXISTS, HINSTANCE, HWND, LPARAM, LRESULT, WPARAM,
};
use windows::Win32::System::LibraryLoader::GetModuleHandleW;
use windows::Win32::System::Threading::CreateMutexW;
use windows::Win32::UI::Shell::{NIN_BALLOONTIMEOUT, NIN_BALLOONUSERCLICK};
use windows::Win32::UI::WindowsAndMessaging::*;

const WM_TRAY_ICON: u32 = WM_APP + 1;
const WM_CLIPBOARD_URL: u32 = WM_APP + 2;
const WM_QUEUE_SUCCESS: u32 = WM_APP + 3;
const WM_QUEUE_FAILURE: u32 = WM_APP + 4;

const ID_TRAY_OPEN: u32 = 1001;
const ID_TRAY_LAST: u32 = 1002;
const ID_TRAY_TOGGLE_WATCH: u32 = 1003;
const ID_TRAY_AUTOSTART: u32 = 1004;
const ID_TRAY_QUIT: u32 = 1099;

const ID_CONFIRM_YES: u32 = 2001;

/// Per-process state shared between the message loop and the worker callback.
struct AppState {
    #[allow(dead_code)]
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
        // Enforce single instance via named mutex.
        let mutex_name = wide("Global\\rdl_tray_single_instance_mutex");
        let h_mutex = match CreateMutexW(None, true, PCWSTR(mutex_name.as_ptr())) {
            Ok(m) => m,
            Err(_) => {
                // If global namespace fails due to permissions, fallback to local session.
                let local_name = wide("rdl_tray_single_instance_mutex");
                CreateMutexW(None, true, PCWSTR(local_name.as_ptr()))?
            }
        };

        if GetLastError() == ERROR_ALREADY_EXISTS {
            let _ = CloseHandle(h_mutex);
            // Another instance is already running; exit cleanly.
            return Ok(());
        }

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

        notify::balloon(hwnd, "rdl-tray", "Watching clipboard for download URLs.");

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
        let _ = CloseHandle(h_mutex);
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
            } else if event == NIN_BALLOONUSERCLICK {
                handle_command(hwnd, ID_CONFIRM_YES);
            } else if event == NIN_BALLOONTIMEOUT {
                with_state(|s| {
                    s.pending_url = None;
                });
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
                    notify::balloon_with_action(hwnd, "Send to remote-dl?", &truncate(&url, 80));
                });
            }
            LRESULT(0)
        }
        WM_COMMAND => {
            let id = (wparam.0 as u32) & 0xFFFF;
            handle_command(hwnd, id);
            LRESULT(0)
        }
        WM_QUEUE_SUCCESS => {
            if lparam.0 != 0 {
                let job_id = Box::from_raw(lparam.0 as *mut String);
                notify::balloon(hwnd, "Queued", &format!("job {}", *job_id));
            }
            LRESULT(0)
        }
        WM_QUEUE_FAILURE => {
            if lparam.0 != 0 {
                let err_msg = Box::from_raw(lparam.0 as *mut String);
                notify::balloon_error(hwnd, "Queue Failed", &err_msg);
            }
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

    let _ = AppendMenuW(
        menu,
        MF_STRING,
        ID_TRAY_OPEN as usize,
        PCWSTR(wide("Open web UI").as_ptr()),
    );
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

    let autostart_flags = if autostart::is_enabled() {
        MF_STRING | MF_CHECKED
    } else {
        MF_STRING | MF_UNCHECKED
    };
    let _ = AppendMenuW(
        menu,
        autostart_flags,
        ID_TRAY_AUTOSTART as usize,
        PCWSTR(wide("Start with Windows").as_ptr()),
    );

    let _ = AppendMenuW(menu, MF_SEPARATOR, 0, PCWSTR::null());
    let _ = AppendMenuW(
        menu,
        MF_STRING,
        ID_TRAY_QUIT as usize,
        PCWSTR(wide("Quit").as_ptr()),
    );

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
            let url = with_state(|s| s.last_url.clone());
            if let Some(url) = url {
                dispatch_pending(hwnd, url);
            }
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
        ID_TRAY_AUTOSTART => {
            let current = autostart::is_enabled();
            let next = !current;
            if let Err(e) = autostart::set_enabled(next) {
                notify::balloon_error(
                    hwnd,
                    "rdl-tray",
                    &format!("Failed to update autostart: {e}"),
                );
            } else {
                let msg = if next {
                    "Autostart enabled."
                } else {
                    "Autostart disabled."
                };
                notify::balloon(hwnd, "rdl-tray", msg);
            }
        }
        ID_CONFIRM_YES => {
            let url = with_state(|s| s.pending_url.take());
            if let Some(url) = url {
                dispatch_pending(hwnd, url);
            }
        }
        ID_TRAY_QUIT => {
            DestroyWindow(hwnd).ok();
        }
        _ => {}
    }
}

fn dispatch_pending(hwnd: HWND, url: String) {
    let hwnd_raw = hwnd.0 as isize;
    std::thread::spawn(move || {
        let target_hwnd = HWND(hwnd_raw as *mut _);
        let res = (|| -> Result<String> {
            let cfg = config::Config::load()?;
            let job = client::queue(&cfg, &url)?;
            Ok(job.id)
        })();

        unsafe {
            match res {
                Ok(job_id) => {
                    let ptr = Box::into_raw(Box::new(job_id));
                    let _ = PostMessageW(
                        target_hwnd,
                        WM_QUEUE_SUCCESS,
                        WPARAM(0),
                        LPARAM(ptr as isize),
                    );
                }
                Err(err) => {
                    let ptr = Box::into_raw(Box::new(err.to_string()));
                    let _ = PostMessageW(
                        target_hwnd,
                        WM_QUEUE_FAILURE,
                        WPARAM(0),
                        LPARAM(ptr as isize),
                    );
                }
            }
        }
    });
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
