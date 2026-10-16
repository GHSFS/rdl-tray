//! Windows autostart management via the `HKCU\Software\Microsoft\Windows\CurrentVersion\Run` registry key.

use crate::error::{Error, Result};
use std::env;
use windows::core::PCWSTR;
use windows::Win32::Foundation::{ERROR_FILE_NOT_FOUND, ERROR_SUCCESS};
use windows::Win32::System::Registry::{
    RegCloseKey, RegDeleteValueW, RegOpenKeyExW, RegQueryValueExW, RegSetValueExW, HKEY,
    HKEY_CURRENT_USER, KEY_READ, KEY_WRITE, REG_SZ, REG_VALUE_TYPE,
};

const RUN_KEY_PATH: &str = r"Software\Microsoft\Windows\CurrentVersion\Run";
const APP_REG_NAME: &str = "rdl-tray";

fn wide(s: &str) -> Vec<u16> {
    s.encode_utf16().chain(std::iter::once(0)).collect()
}

/// Checks whether `rdl-tray` is configured to start automatically on Windows logon.
pub fn is_enabled() -> bool {
    unsafe {
        let subkey = wide(RUN_KEY_PATH);
        let mut hkey = HKEY::default();
        let status = RegOpenKeyExW(
            HKEY_CURRENT_USER,
            PCWSTR(subkey.as_ptr()),
            0,
            KEY_READ,
            &mut hkey,
        );
        if status != ERROR_SUCCESS {
            return false;
        }

        let value_name = wide(APP_REG_NAME);
        let mut r#type = REG_VALUE_TYPE::default();
        let mut cb_data = 0u32;
        let res = RegQueryValueExW(
            hkey,
            PCWSTR(value_name.as_ptr()),
            None,
            Some(&mut r#type),
            None,
            Some(&mut cb_data),
        );
        let _ = RegCloseKey(hkey);

        res == ERROR_SUCCESS && cb_data > 0
    }
}

/// Enables or disables autostart on Windows logon by writing or removing the registry value.
pub fn set_enabled(enable: bool) -> Result<()> {
    unsafe {
        let subkey = wide(RUN_KEY_PATH);
        let mut hkey = HKEY::default();
        let open_status = RegOpenKeyExW(
            HKEY_CURRENT_USER,
            PCWSTR(subkey.as_ptr()),
            0,
            KEY_WRITE,
            &mut hkey,
        );
        if open_status != ERROR_SUCCESS {
            return Err(Error::Win32(format!(
                "RegOpenKeyExW failed: error {}",
                open_status.0
            )));
        }

        let value_name = wide(APP_REG_NAME);
        let result = if enable {
            let exe_path = env::current_exe().map_err(Error::Io)?;
            let quoted = format!("\"{}\"", exe_path.display());
            let wide_val = wide(&quoted);
            let byte_len = (wide_val.len() * std::mem::size_of::<u16>()) as u32;

            let set_status = RegSetValueExW(
                hkey,
                PCWSTR(value_name.as_ptr()),
                0,
                REG_SZ,
                Some(std::slice::from_raw_parts(
                    wide_val.as_ptr() as *const u8,
                    byte_len as usize,
                )),
            );
            if set_status != ERROR_SUCCESS {
                Err(Error::Win32(format!(
                    "RegSetValueExW failed: error {}",
                    set_status.0
                )))
            } else {
                Ok(())
            }
        } else {
            let del_status = RegDeleteValueW(hkey, PCWSTR(value_name.as_ptr()));
            if del_status != ERROR_SUCCESS && del_status != ERROR_FILE_NOT_FOUND {
                Err(Error::Win32(format!(
                    "RegDeleteValueW failed: error {}",
                    del_status.0
                )))
            } else {
                Ok(())
            }
        };

        let _ = RegCloseKey(hkey);
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn autostart_query_does_not_panic() {
        // Querying the registry must be infallible and return a valid boolean without panics.
        let _ = is_enabled();
    }
}
