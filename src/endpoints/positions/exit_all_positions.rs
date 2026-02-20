use crate::{ExitPositionResult, Fyers, FyersError, models::api_response::ApiResponse};

impl Fyers {
    /// Exit all open positions.
    ///
    /// Returns [`ExitPositionResult`] indicating whether the positions were
    /// fully closed or a counter order was placed but not immediately filled.
    pub async fn exit_all_positions(&self) -> Result<ExitPositionResult, FyersError> {
        let url = format!("{}/positions", self.base_urls.api_v3);
        let response = self
            .delete(&url, &serde_json::json!({"exit_all": 1}))
            .await?;
        let api_response: ApiResponse = serde_json::from_value(response)?;

        match api_response.code {
            200 => Ok(ExitPositionResult::Closed),
            201 => Ok(ExitPositionResult::PendingCounterOrder),
            code => Err(FyersError::Api {
                code,
                message: api_response.message,
            }),
        }
    }
}
