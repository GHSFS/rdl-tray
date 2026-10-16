//! Minimal HTTP client for the remote-dl edge worker. Mirrors the surface
//! used by `rdl` CLI but only exposes what the tray companion needs.

use crate::config::Config;
use crate::error::{Error, Result};
use serde::{Deserialize, Serialize};
use std::process::Command;
use std::sync::OnceLock;
use std::time::Duration;

const USER_AGENT: &str = concat!("rdl-tray/", env!("CARGO_PKG_VERSION"));

static HTTP_CLIENT: OnceLock<reqwest::blocking::Client> = OnceLock::new();

fn get_client() -> &'static reqwest::blocking::Client {
    HTTP_CLIENT.get_or_init(|| {
        reqwest::blocking::Client::builder()
            .user_agent(USER_AGENT)
            .timeout(Duration::from_secs(30))
            .build()
            .unwrap_or_else(|_| reqwest::blocking::Client::new())
    })
}

#[derive(Debug, Deserialize)]
pub struct JobRef {
    pub id: String,
}

#[derive(Debug, Serialize)]
struct QueueRequest<'a> {
    url: &'a str,
    #[serde(skip_serializing_if = "Option::is_none")]
    folder: Option<&'a str>,
}

pub fn queue(cfg: &Config, url: &str) -> Result<JobRef> {
    let http = get_client();

    let body = QueueRequest {
        url,
        folder: cfg.folder.as_deref(),
    };
    let resp = http
        .post(format!("{}/api/dl", cfg.worker()?))
        .header("Authorization", format!("Bearer {}", cfg.token()?))
        .json(&body)
        .send()
        .map_err(|e| Error::Http(e.to_string()))?;

    let status = resp.status();
    if !status.is_success() {
        return Err(match status.as_u16() {
            401 | 403 => Error::Auth(format!("server rejected credentials ({status})")),
            _ => Error::Http(format!("unexpected status {status}")),
        });
    }
    resp.json::<JobRef>()
        .map_err(|e| Error::Http(e.to_string()))
}

/// Open the configured worker's web UI in the default browser.
pub fn open_web_ui() -> Result<()> {
    let cfg = Config::load()?;
    let worker = cfg.worker()?.to_string();
    Command::new("cmd")
        .args(["/C", "start", "", &worker])
        .spawn()
        .map_err(Error::Io)?;
    Ok(())
}
