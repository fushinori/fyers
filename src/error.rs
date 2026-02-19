use reqwest::StatusCode;
use thiserror::Error;

/// Errors that can occur when interacting with the Fyers API.
///
/// This enum represents **all failures that can happen when sending requests
/// or parsing responses** from Fyers.
///
/// Errors fall into three broad categories:
///
/// 1. **Transport errors** — network failures or non-200 HTTP responses.
/// 2. **API errors** — valid responses where Fyers rejected the request.
/// 3. **Client errors** — JSON parsing or unexpected response formats.
///
/// This enum is marked as `#[non_exhaustive]` so new variants may be added
/// in future releases without breaking changes.
#[derive(Debug, Error)]
#[non_exhaustive]
pub enum FyersError {
    /// A network or request error occurred while sending the HTTP request.
    ///
    /// Examples:
    /// - No internet connection
    /// - DNS failure
    /// - TLS handshake failure
    /// - Request timeout
    #[error("request failed: {0}")]
    Request(#[from] reqwest::Error),

    /// The server returned a non-success HTTP status code.
    ///
    /// This usually indicates:
    /// - Wrong endpoint URL
    /// - Cloudflare blocking the request
    /// - Internal server error
    ///
    /// The raw response body is included to help with debugging.
    #[error("http error {status}: {body}")]
    HttpStatus {
        /// Status code returned by the server
        status: StatusCode,
        /// The raw response returned by the server
        body: String,
    },

    /// Authentication token has expired.
    ///
    /// The user must refresh the access token and retry the request.
    #[error("token expired")]
    TokenExpired,

    /// The provided token is invalid.
    #[error("invalid token")]
    InvalidToken,

    /// One or more request parameters were invalid.
    ///
    /// The contained message is provided by the Fyers API.
    #[error("invalid parameters: {0}")]
    InvalidParams(String),

    /// The provided trading symbol is invalid.
    #[error("invalid symbol")]
    InvalidSymbol,

    /// The provided order ID is invalid.
    #[error("invalid order id")]
    InvalidOrderId,

    /// The provided position ID is invalid.
    #[error("invalid position id")]
    InvalidPositionId,

    /// Order was rejected by the exchange or broker.
    ///
    /// The message usually contains the rejection reason.
    #[error("order rejected: {0}")]
    OrderRejected(String),

    /// Invalid Fyers App ID was provided.
    #[error("invalid app id")]
    InvalidAppId,

    /// API rate limits have been exceeded.
    ///
    /// Per second - 10
    /// Per minute - 200
    /// Per day - 1,00,000
    ///
    /// The request should be retried later.
    #[error("rate limit exceeded")]
    RateLimited,

    /// A generic error returned by the Fyers API.
    ///
    /// This is used as a fallback when an error code is not yet mapped
    /// to a specific variant.
    #[error("fyers api error {code}: {message}")]
    Api {
        /// Error code
        code: i32,

        /// Error message
        message: String,
    },

    /// Failed to serialize or deserialize JSON.
    ///
    /// This usually indicates:
    /// - Fyers changed their API response format
    /// - A bug in the SDK models
    #[error("json error: {0}")]
    Json(#[from] serde_json::Error),

    /// A required field was missing from a successful API response.
    ///
    /// This indicates an unexpected API response format.
    #[error("missing field: '{0}' in success response")]
    MissingField(&'static str),
}

impl FyersError {
    pub(crate) fn map_api_error(code: i32, message: String) -> FyersError {
        match code {
            -8 => FyersError::TokenExpired,

            // -15 | -16 | -17
            -17..=-15 => FyersError::InvalidToken,

            -50 | 400 => FyersError::InvalidParams(message),

            -51 => FyersError::InvalidOrderId,

            -53 => FyersError::InvalidPositionId,

            -99 => FyersError::OrderRejected(message),

            -300 => FyersError::InvalidSymbol,

            -352 => FyersError::InvalidAppId,

            -429 => FyersError::RateLimited,

            _ => FyersError::Api { code, message },
        }
    }
}
