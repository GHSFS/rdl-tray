//! Clipboard listener — receives `WM_CLIPBOARDUPDATE` for every change and
//! extracts the first URL-looking string from the current text contents.

use crate::error::{Error, Result};
use regex::Regex;
use std::sync::OnceLock;
use windows::Win32::Foundation::{HGLOBAL, HWND};
use windows::Win32::System::DataExchange::{
    AddClipboardFormatListener, CloseClipboard, GetClipboardData, OpenClipboard,
    RemoveClipboardFormatListener,
};
use windows::Win32::System::Memory::{GlobalLock, GlobalUnlock};
use windows::Win32::System::Ole::CF_UNICODETEXT;

pub const WM_CLIPBOARDUPDATE: u32 = 0x031D;

pub fn start_listening(hwnd: HWND) -> Result<()> {
    unsafe { AddClipboardFormatListener(hwnd) }
        .map_err(|e| Error::Win32(format!("AddClipboardFormatListener: {e}")))?;
    Ok(())
}

pub fn stop_listening(hwnd: HWND) {
    unsafe {
        let _ = RemoveClipboardFormatListener(hwnd);
    }
}

/// Reads the current clipboard contents as text and returns the first
/// http(s) URL it contains, if any.
pub fn read_url() -> Option<String> {
    let text = read_clipboard_text()?;
    extract_url(&text)
}

fn read_clipboard_text() -> Option<String> {
    unsafe {
        if OpenClipboard(None).is_err() {
            return None;
        }
        let h = match GetClipboardData(CF_UNICODETEXT.0 as u32) {
            Ok(h) => h,
            Err(_) => {
                let _ = CloseClipboard();
                return None;
            }
        };
        let hglobal = HGLOBAL(h.0);
        let p = GlobalLock(hglobal) as *const u16;
        if p.is_null() {
            let _ = CloseClipboard();
            return None;
        }

        // Find length up to NUL terminator.
        let mut len = 0usize;
        while *p.add(len) != 0 {
            len += 1;
            if len > 8192 {
                break;
            }
        }
        let slice = std::slice::from_raw_parts(p, len);
        let s = String::from_utf16_lossy(slice);

        let _ = GlobalUnlock(hglobal);
        let _ = CloseClipboard();
        Some(s)
    }
}

fn url_regex() -> &'static Regex {
    static R: OnceLock<Regex> = OnceLock::new();
    R.get_or_init(|| {
        // Conservative: match http(s) URLs with at least a host, no whitespace.
        Regex::new(r#"\bhttps?://[^\s<>"']{4,2048}"#).expect("static regex")
    })
}

pub fn extract_url(text: &str) -> Option<String> {
    let trimmed = text.trim();
    let m = url_regex().find(trimmed)?;
    Some(strip_trailing_punct(m.as_str()).to_string())
}

fn strip_trailing_punct(s: &str) -> &str {
    let mut end = s.len();
    let bytes = s.as_bytes();
    while end > 0 {
        match bytes[end - 1] {
            b'.' | b',' | b';' | b':' | b'!' | b'?' | b')' | b']' | b'}' => end -= 1,
            _ => break,
        }
    }
    &s[..end]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn extracts_simple_url() {
        let s = "look here https://example.com/file.mp4 thanks";
        assert_eq!(
            extract_url(s),
            Some("https://example.com/file.mp4".to_string())
        );
    }

    #[test]
    fn strips_trailing_punctuation() {
        assert_eq!(
            extract_url("see https://example.com/p?x=1)."),
            Some("https://example.com/p?x=1".to_string())
        );
    }

    #[test]
    fn ignores_non_urls() {
        assert!(extract_url("just text, no link").is_none());
    }

    #[test]
    fn requires_http_scheme() {
        assert!(extract_url("ftp://example.com/file").is_none());
    }
}
