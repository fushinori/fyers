use crate::models::history::history_response::{Candle, HistoryResponse};
use crate::urls;
use crate::{Fyers, FyersError, HistoryRequest};

impl Fyers {
    /// Get historical data
    pub async fn history(
        &self,
        history_request: &HistoryRequest,
    ) -> Result<Vec<Candle>, FyersError> {
        let url = format!("{}/history", urls::DATA);
        let response = self.get_query(&url, &history_request.as_api()).await?;
        let api_response: HistoryResponse = serde_json::from_value(response)?;
        Ok(api_response.candles)
    }
}
