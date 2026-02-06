//! Fyers Client.

use crate::FyersError;
use crate::models::api_response::{ApiResponse, ApiStatus};

const BASE_URL: &str = "https://api-t1.fyers.in/api/v3";

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
        Self {
            client_id: client_id.to_string(),
            access_token: access_token.to_string(),
            http: reqwest::Client::new(),
        }
    }

    fn auth_header(&self) -> String {
        format!("{}:{}", self.client_id, self.access_token)
    }

    // Send requests and parse the response.
    async fn send_and_parse<T>(&self, req: reqwest::RequestBuilder) -> Result<T, FyersError>
    where
        T: serde::de::DeserializeOwned,
    {
        let response = req.send().await?;
        let status = response.status();
        let body = response.text().await?;
        dbg!(&body);

        if !status.is_success() {
            return Err(FyersError::HttpStatus { status, body });
        }

        let api_response: ApiResponse<T> = serde_json::from_str(&body)?;

        if let ApiStatus::Error = api_response.s {
            return Err(FyersError::Api {
                code: api_response.code,
                message: api_response.message,
            });
        }

        let data = api_response.data.ok_or(FyersError::Api {
            code: api_response.code,
            message: "missing data in success response".into(),
        })?;

        Ok(data)
    }

    // GET request helper
    pub(crate) async fn get<T>(&self, path: &str) -> Result<T, FyersError>
    where
        T: serde::de::DeserializeOwned,
    {
        let url = format!("{BASE_URL}/{path}");

        self.send_and_parse(
            self.http
                .get(url)
                .header("Authorization", self.auth_header()),
        )
        .await
    }

    // POST request helper
    pub(crate) async fn post<T, B>(&self, path: &str, body: &B) -> Result<T, FyersError>
    where
        T: serde::de::DeserializeOwned,
        B: serde::Serialize,
    {
        let url = format!("{BASE_URL}/{path}");

        self.send_and_parse(
            self.http
                .post(url)
                .header("Authorization", self.auth_header())
                .json(body),
        )
        .await
    }
}
