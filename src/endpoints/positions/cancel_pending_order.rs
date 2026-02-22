use crate::{Fyers, FyersError};

impl Fyers {
    /// Cancel all pending orders for the given position.
    ///
    /// Note: Although the Fyers documentation marks `id` as optional,
    /// the API requires a position ID (e.g. NSE:SBIN-EQ-INTRADAY).
    pub async fn cancel_pending_orders(&self, position_id: &str) -> Result<(), FyersError> {
        let url = format!("{}/positions", self.base_urls.api_v3);
        self.delete(
            &url,
            &serde_json::json!({"pending_orders_cancel": 1, "id": position_id}),
        )
        .await?;

        Ok(())
    }
}
