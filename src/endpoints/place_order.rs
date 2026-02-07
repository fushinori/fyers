use serde::Deserialize;

use crate::urls;
use crate::{Fyers, FyersError, Order, PlaceOrderRequest};

impl Fyers {
    /// Place a single order
    pub async fn place_order(&self, order: &PlaceOrderRequest) -> Result<Order, FyersError> {
        let url = format!("{}/orders/sync", urls::API_V3);
        let response = self.post(&url, order).await?;
        Ok(Order::deserialize(response)?)
    }
}
