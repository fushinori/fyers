use serde::Deserialize;

use crate::{Fyers, FyersError, Order, PlaceOrderRequest};

impl Fyers {
    /// Place a single order
    pub async fn place_order(&self, order: PlaceOrderRequest) -> Result<Order, FyersError> {
        let response = self.post("orders/sync", &order).await?;
        Ok(Order::deserialize(response)?)
    }
}
