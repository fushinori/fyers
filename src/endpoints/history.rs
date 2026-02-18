use crate::models::history::history_request::HistoryRequest;
use crate::models::history::history_response::{Candle, HistoryResponse};
use crate::{Fyers, FyersError};

impl Fyers {
    /// Fetch historical candle data for a symbol.
    ///
    /// Returns a vector of [`Candle`] values ordered from oldest to newest.
    ///
    /// Use [`HistoryRequest::builder`] to construct the request.
    ///
    /// # Example
    /// ```no_run
    /// let from = fyers::ist_datetime(2026, 2, 5, 9, 30);
    /// let to = fyers::ist_datetime(2026, 2, 5, 15, 15);
    ///
    /// let history_request = HistoryRequest::builder("NSE:JIOFIN-EQ", from, to).build();
    ///
    /// let candles = fyers.history(&history_request).await?;
    /// ```
    pub async fn history(
        &self,
        history_request: &HistoryRequest,
    ) -> Result<Vec<Candle>, FyersError> {
        let url = format!("{}/history", self.base_urls.data);
        let response = self.get_query(&url, &history_request).await?;
        let api_response: HistoryResponse = serde_json::from_value(response)?;
        Ok(api_response.candles)
    }
}
