use thiserror::Error;

/// Errors that could occur during authentication.
#[derive(Debug, Error)]
pub enum AuthError {
    /// Invalid URL passed to generate_tokens
    #[error("invalid URL")]
    InvalidUrl(#[from] url::ParseError),

    /// Auth code missing in the URL
    #[error("auth_code not found in URL")]
    MissingAuthCode,

    /// HTTP error during authentication
    #[error("http error")]
    Http(#[from] reqwest::Error),

    /// Fyers API error
    #[error("fyers auth error (code={code}): {message}")]
    Api {
        /// Code associated with the error
        code: i32,
        /// Message associated with the error
        message: String,
    },
}
