use crate::{Fyers, FyersError};

impl Fyers {
    /// Cancel a pending order by its order ID.
    pub async fn cancel_order(&self, order_id: &str) -> Result<(), FyersError> {
        let url = format!("{}/orders/sync", self.base_urls.api_v3);
        self.delete(&url, &serde_json::json!({"id": order_id}))
            .await?;

        Ok(())
    }
}
