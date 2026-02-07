use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct ApiResponse {
    pub s: ApiStatus,
    pub code: i32,
    pub message: String,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "lowercase")]
pub enum ApiStatus {
    Ok,
    Error,
}
