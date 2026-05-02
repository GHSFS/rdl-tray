//! Reuses the on-disk config layout from the `rdl` CLI client. Both binaries
//! talk to the same edge worker, so they share `%APPDATA%\rdl\config.json`.

use crate::error::{Error, Result};
use base64::engine::general_purpose::STANDARD as B64;
use directories::ProjectDirs;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Config {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub worker: Option<String>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub token_enc: Option<String>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub folder: Option<String>,
}

impl Config {
    pub fn path() -> Result<PathBuf> {
        let dirs = ProjectDirs::from("dev", "GHSFS", "rdl")
            .ok_or_else(|| Error::Config("could not resolve config directory".into()))?;
        Ok(dirs.config_dir().join("config.json"))
    }

    pub fn load() -> Result<Self> {
        let path = Self::path()?;
        if !path.exists() {
            return Err(Error::Config(format!(
                "config not found at {} — run `rdl auth login` first",
                path.display()
            )));
        }
        let raw = fs::read_to_string(&path)?;
        Ok(serde_json::from_str(&raw)?)
    }

    pub fn worker(&self) -> Result<&str> {
        self.worker
            .as_deref()
            .ok_or_else(|| Error::Config("worker URL not configured".into()))
    }

    pub fn token(&self) -> Result<String> {
        let enc = self
            .token_enc
            .as_ref()
            .ok_or_else(|| Error::Auth("not authenticated".into()))?;
        dpapi::unprotect(enc)
    }
}

#[cfg(windows)]
mod dpapi {
    use super::{Error, Result, B64};
    use base64::Engine as _;
    use windows::Win32::Foundation::{HLOCAL, LocalFree};
    use windows::Win32::Security::Cryptography::{CryptUnprotectData, CRYPT_INTEGER_BLOB};

    pub fn unprotect(encoded: &str) -> Result<String> {
        let mut bytes = B64
            .decode(encoded)
            .map_err(|e| Error::Config(format!("invalid token encoding: {e}")))?;
        let mut input = CRYPT_INTEGER_BLOB {
            cbData: bytes.len() as u32,
            pbData: bytes.as_mut_ptr(),
        };
        let mut output = CRYPT_INTEGER_BLOB::default();

        unsafe {
            CryptUnprotectData(&mut input, None, None, None, None, 0, &mut output)
                .map_err(|e| Error::Config(format!("DPAPI unprotect: {e}")))?;
            let plain = std::slice::from_raw_parts(output.pbData, output.cbData as usize);
            let s = String::from_utf8_lossy(plain).into_owned();
            let _ = LocalFree(HLOCAL(output.pbData as _));
            Ok(s)
        }
    }
}

#[cfg(not(windows))]
mod dpapi {
    use super::{Error, Result, B64};
    use base64::Engine as _;
    pub fn unprotect(encoded: &str) -> Result<String> {
        let bytes = B64
            .decode(encoded)
            .map_err(|e| Error::Config(format!("invalid token encoding: {e}")))?;
        String::from_utf8(bytes).map_err(|e| Error::Config(format!("non-utf8 token: {e}")))
    }
}
