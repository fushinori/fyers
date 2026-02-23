//! Fyers Client.
use reqwest::header::{AUTHORIZATION, HeaderValue};

use crate::FyersError;
use crate::models::api_response::{ApiResponse, ApiStatus};

/// Asynchronous Fyers client.
///
/// Implements endpoints as associated methods.
#[derive(Clone)]
pub struct Fyers {
    http: reqwest::Client,
    auth_header: HeaderValue,
    pub(crate) base_urls: BaseUrls,
}

// All unique base URLs
//
// Edit this struct and its default implementation
// to add more base URLs.
// The endpoints can then choose the base URL it needs.
#[derive(Clone)]
pub(crate) struct BaseUrls {
    pub api_v3: String,
    pub data: String,
}

impl Default for BaseUrls {
    fn default() -> Self {
        Self {
            api_v3: "https://api-t1.fyers.in/api/v3".into(),
            data: "https://api-t1.fyers.in/data".into(),
        }
    }
}

impl Fyers {
    /// Create a new Fyers client.
    pub fn new(client_id: &str, access_token: &str) -> Self {
        let http = reqwest::Client::builder()
            .user_agent("fyers/0.1 (https://github.com/fushinori/fyers)")
            .build()
            .unwrap();

        let mut auth_header =
            HeaderValue::from_str(&format!("{}:{}", client_id, access_token)).unwrap();
        auth_header.set_sensitive(true);

        Self {
            http,
            auth_header,
            base_urls: BaseUrls::default(),
        }
    }

    // Create a client with custom base URLs.
    //
    // Mainly intended for testing.
    #[doc(hidden)]
    pub fn with_base_urls(
        client_id: &str,
        access_token: &str,
        api_v3: impl Into<String>,
        data: impl Into<String>,
    ) -> Self {
        let mut auth_header =
            HeaderValue::from_str(&format!("{}:{}", client_id, access_token)).unwrap();
        auth_header.set_sensitive(true);

        Self {
            http: reqwest::Client::new(),
            auth_header,
            base_urls: BaseUrls {
                api_v3: api_v3.into(),
                data: data.into(),
            },
        }
    }

    // Send requests and validate the response
    async fn send_and_validate(
        &self,
        req: reqwest::RequestBuilder,
    ) -> Result<serde_json::Value, FyersError> {
        let response = req.send().await?;
        let status = response.status();
        let body = response.text().await?;

        // Try to parse API envelope FIRST (even if HTTP failed)
        if let Ok(ApiResponse {
            s: ApiStatus::Error,
            code,
            message,
            ..
        }) = serde_json::from_str::<ApiResponse>(&body)
        {
            return Err(FyersError::map_api_error(code, message));
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
                .header(AUTHORIZATION, self.auth_header.clone()),
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
                .header(AUTHORIZATION, self.auth_header.clone()),
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
                .header(AUTHORIZATION, self.auth_header.clone())
                .json(body),
        )
        .await
    }

    pub(crate) async fn delete<B>(
        &self,
        url: &str,
        body: &B,
    ) -> Result<serde_json::Value, FyersError>
    where
        B: serde::Serialize,
    {
        self.send_and_validate(
            self.http
                .delete(url)
                .header(AUTHORIZATION, self.auth_header.clone())
                .json(body),
        )
        .await
    }
}
