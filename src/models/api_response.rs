use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct ApiResponse<T> {
    pub s: ApiStatus,
    pub code: i32,
    pub message: String,
    pub data: Option<T>,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "lowercase")]
pub enum ApiStatus {
    Ok,
    Error,
}
