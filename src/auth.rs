//! Authentication helpers for the Fyers API.
//!
//! This module implements the authentication flows required by Fyers:
//!
//! 1. Generate an authorization URL using [`generate_url`].
//! 2. Exchange the resulting authorization code for access and refresh tokens
//!    using [`generate_tokens`].
//! 3. Refresh an expired access token using an existing refresh token with
//!    [`refresh_tokens`].
//!
//! The initial authorization flow is interactive: the user must open the
//! generated URL in a browser and complete the login process before tokens
//! can be issued.
//!
//! Refresh tokens have a limited validity period (usually 15 days), during which new access
//! tokens can be generated without requiring the user to authenticate again.
//!
//! This module is intentionally lightweight and only provides the
//! building blocks for authentication.
//! Token storage, refresh scheduling, and lifecycle management are
//! expected to be handled by the application.

use reqwest::Client;
use url::Url;

pub mod error;
pub use error::AuthError;
mod helpers;
mod types;

use self::types::{
    ApiStatus, GenerateTokenRequest, GenerateTokenResponse, RefreshTokenRequest, Tokens,
};

/// Generates the Fyers authentication URL for the first step of the auth flow.
///
/// This URL must be opened by the user in a browser. After successful login,
/// Fyers will redirect to the provided `redirect_uri` with an
/// `auth_code` query parameter, which can then be exchanged for access tokens.
///
/// Pass in that URL to the [`generate_tokens`] function.
pub fn generate_url(client_id: &str, redirect_uri: &str, state: &str) -> Result<Url, AuthError> {
    let mut url = Url::parse("https://api-t1.fyers.in/api/v3/generate-authcode")?;
    url.query_pairs_mut()
        .append_pair("client_id", client_id)
        .append_pair("redirect_uri", redirect_uri)
        .append_pair("state", state);

    Ok(url)
}

/// Exchanges an authorization code for access and refresh tokens.
///
/// This function implements the second step of the Fyers authentication flow.
/// It takes the redirect URL obtained after user login, extracts the
/// `auth_code` query parameter, and exchanges it with the Fyers API for
/// access and refresh tokens.
///
/// # Example
///
/// ```no_run
/// use fyers::auth::generate_tokens;
///
/// let tokens = generate_tokens(
///     "your_client_id",
///     "your_secret_key",
///     "https://example.com/?s=ok&code=200&auth_code=XYZ",
/// ).await?;
///
/// println!("Access token: {}", tokens.access_token);
/// ```
pub async fn generate_tokens(
    client_id: &str,
    secret_key: &str,
    url: &str,
) -> Result<Tokens, AuthError> {
    let app_id_hash = helpers::compute_app_id_hash(client_id, secret_key);

    let parsed_url = Url::parse(url)?;

    let auth_code =
        helpers::get_query_param(&parsed_url, "auth_code").ok_or(AuthError::MissingAuthCode)?;

    let client = Client::new();

    let response = client
        .post("https://api-t1.fyers.in/api/v3/validate-authcode")
        .json(&GenerateTokenRequest::new(&app_id_hash, &auth_code))
        .send()
        .await?;

    let api_response = response.json::<GenerateTokenResponse>().await?;

    match api_response.s {
        ApiStatus::Error => Err(AuthError::Api {
            code: api_response.code,
            message: api_response.message,
        }),
        ApiStatus::Ok => {
            let access_token = api_response.access_token.ok_or_else(|| AuthError::Api {
                code: api_response.code,
                message: "missing access_token in success response".into(),
            })?;

            let refresh_token = api_response.refresh_token.ok_or_else(|| AuthError::Api {
                code: api_response.code,
                message: "missing refresh_token in success response".into(),
            })?;

            Ok(Tokens {
                access_token,
                refresh_token,
            })
        }
    }
}

/// Generates a new access token using an existing refresh token.
///
/// This function implements the refresh-token flow provided by Fyers.
/// As long as the refresh token is valid (usually for 15 days), it can be used to obtain a new
/// access token without requiring you to authenticate again.
///
/// # Example
///
/// ```no_run
/// use fyers::auth::refresh_tokens;
///
/// let tokens = refresh_tokens(
///     "your_client_id",
///     "your_secret_key",
///     "existing_refresh_token",
///     "your_pin",
/// ).await?;
///
/// println!("New access token: {}", tokens.access_token);
/// ```
pub async fn refresh_tokens(
    client_id: &str,
    secret_key: &str,
    refresh_token: &str,
    pin: &str,
) -> Result<Tokens, AuthError> {
    let app_id_hash = helpers::compute_app_id_hash(client_id, secret_key);

    let client = Client::new();

    let response = client
        .post("https://api-t1.fyers.in/api/v3/validate-refresh-token")
        .json(&RefreshTokenRequest::new(&app_id_hash, refresh_token, pin))
        .send()
        .await?;

    let api_response = response.json::<GenerateTokenResponse>().await?;

    match api_response.s {
        ApiStatus::Error => Err(AuthError::Api {
            code: api_response.code,
            message: api_response.message,
        }),
        ApiStatus::Ok => {
            let access_token = api_response.access_token.ok_or_else(|| AuthError::Api {
                code: api_response.code,
                message: "missing access_token in refresh response".into(),
            })?;

            Ok(Tokens {
                access_token,
                // Refresh token isn't returned usually so just return the existing one
                refresh_token: api_response
                    .refresh_token
                    .unwrap_or_else(|| refresh_token.to_string()),
            })
        }
    }
}
