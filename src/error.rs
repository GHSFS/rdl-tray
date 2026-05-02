use thiserror::Error;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Error)]
pub enum Error {
    #[error("config: {0}")]
    Config(String),

    #[error("auth: {0}")]
    Auth(String),

    #[error("http: {0}")]
    Http(String),

    #[error("win32: {0}")]
    Win32(String),

    #[error("io: {0}")]
    Io(#[from] std::io::Error),

    #[error("json: {0}")]
    Json(#[from] serde_json::Error),
}

impl From<windows::core::Error> for Error {
    fn from(e: windows::core::Error) -> Self {
        Error::Win32(e.to_string())
    }
}
