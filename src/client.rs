//! Fyers Client.

use crate::FyersError;
use crate::models::api_response::{ApiResponse, ApiStatus};

/// Asynchronous Fyers client.
///
/// Implements endpoints as associated methods.
pub struct Fyers {
    client_id: String,
    access_token: String,
    http: reqwest::Client,
}

impl Fyers {
    /// Create a new Fyers client.
    pub fn new(client_id: &str, access_token: &str) -> Self {
        let http = reqwest::Client::builder()
            .user_agent("fyers/0.1 (https://github.com/fushinori/fyers)")
            .build()
            .unwrap();

        Self {
            client_id: client_id.to_string(),
            access_token: access_token.to_string(),
            http,
        }
    }

    fn auth_header(&self) -> String {
        format!("{}:{}", self.client_id, self.access_token)
    }

    // Send requests and validate the response
    async fn send_and_validate(
        &self,
        req: reqwest::RequestBuilder,
    ) -> Result<serde_json::Value, FyersError> {
        let response = req.send().await?;
        let status = response.status();
        let body = response.text().await?;

        dbg!(&body);

        // Try to parse API envelope FIRST (even if HTTP failed)
        if let Ok(ApiResponse {
            s: ApiStatus::Error,
            code,
            message,
            ..
        }) = serde_json::from_str::<ApiResponse>(&body)
        {
            return Err(FyersError::Api { code, message });
        }

        // If HTTP failed but API didn't give structured error
        if !status.is_success() {
            return Err(FyersError::HttpStatus { status, body });
        }

        // Finally parse the actual payload
        let raw_response = serde_json::from_str(&body)?;
        Ok(raw_response)
    }

    // GET request helper
    pub(crate) async fn get(&self, url: &str) -> Result<serde_json::Value, FyersError> {
        self.send_and_validate(
            self.http
                .get(url)
                .header("Authorization", self.auth_header()),
        )
        .await
    }

    // GET request with query params helper
    pub(crate) async fn get_query<Q>(
        &self,
        url: &str,
        query: &Q,
    ) -> Result<serde_json::Value, FyersError>
    where
        Q: serde::Serialize + ?Sized,
    {
        self.send_and_validate(
            self.http
                .get(url)
                .query(query)
                .header("Authorization", self.auth_header()),
        )
        .await
    }

    // POST request helper
    pub(crate) async fn post<B>(&self, url: &str, body: &B) -> Result<serde_json::Value, FyersError>
    where
        B: serde::Serialize,
    {
        self.send_and_validate(
            self.http
                .post(url)
                .header("Authorization", self.auth_header())
                .json(body),
        )
        .await
    }
}
