use thiserror::Error;

#[derive(Debug, Error)]
pub enum AuthError {
    #[error("invalid URL")]
    InvalidUrl(#[from] url::ParseError),

    #[error("auth_code not found in redirect URL")]
    MissingAuthCode,

    #[error("http error")]
    Http(#[from] reqwest::Error),

    #[error("fyers auth error (code={code}): {message}")]
    Api { code: i32, message: String },
}
