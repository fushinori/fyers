use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub(crate) struct GenerateTokenRequest<'a> {
    grant_type: &'a str,

    #[serde(rename = "appIdHash")]
    app_id_hash: &'a str,

    code: &'a str,
}

impl<'a> GenerateTokenRequest<'a> {
    pub(crate) fn new(app_id_hash: &'a str, code: &'a str) -> Self {
        Self {
            grant_type: "authorization_code",
            app_id_hash,
            code,
        }
    }
}

#[derive(Serialize)]
pub(crate) struct RefreshTokenRequest<'a> {
    grant_type: &'a str,

    #[serde(rename = "appIdHash")]
    app_id_hash: &'a str,

    refresh_token: &'a str,
    pin: &'a str,
}

impl<'a> RefreshTokenRequest<'a> {
    pub(crate) fn new(app_id_hash: &'a str, refresh_token: &'a str, pin: &'a str) -> Self {
        Self {
            grant_type: "refresh_token",
            app_id_hash,
            refresh_token,
            pin,
        }
    }
}
// s is either "ok" or "error"
#[derive(Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
pub(crate) enum ApiStatus {
    Ok,
    Error,
}

#[derive(Debug, Deserialize)]
pub(crate) struct GenerateTokenResponse {
    pub(crate) s: ApiStatus,
    pub(crate) code: i32,
    pub(crate) message: String,
    pub(crate) access_token: Option<String>,
    pub(crate) refresh_token: Option<String>,
}

/// Tokens obtained after successful authentication.
#[derive(Debug, Deserialize)]
pub struct Tokens {
    /// Your access token
    pub access_token: String,

    /// Your refresh token (valid for 15 days)
    pub refresh_token: String,
}
