use reqwest::StatusCode;
use thiserror::Error;

/// Errors that can occur when interacting with the Fyers API.
#[derive(Debug, Error)]
pub enum FyersError {
    /// A network or request error occurred.
    #[error("request failed: {0}")]
    Request(#[from] reqwest::Error),

    /// The Fyers API returned a non-success HTTP status code.
    #[error("http error {status}: {body}")]
    HttpStatus {
        /// HTTP status code returned by the server.
        status: StatusCode,

        /// Raw response body returned by the server.
        body: String,
    },

    #[error("token expired")]
    TokenExpired,

    #[error("invalid token")]
    InvalidToken,

    #[error("invalid parameters: {0}")]
    InvalidParams(String),

    #[error("invalid symbol")]
    InvalidSymbol,

    #[error("invalid order id")]
    InvalidOrderId,

    #[error("invalid position id")]
    InvalidPositionId,

    #[error("order rejected: {0}")]
    OrderRejected(String),

    #[error("invalid app id")]
    InvalidAppId,

    #[error("rate limit exceeded")]
    RateLimited,

    /// The Fyers API returned an error.
    #[error("fyers api error {code}: {message}")]
    Api {
        /// Fyers error code identifying the failure.
        code: i32,

        /// Human-readable error message returned by the API.
        message: String,
    },

    /// Failed to deserialize a JSON response from the Fyers API.
    #[error("failed to deserialize response: {0}")]
    Deserialize(#[from] serde_json::Error),

    // Missing field in success response
    #[error("missing field: '{0}' in success response")]
    MissingField(&'static str),
}
