use serde::Deserialize;

use crate::models::orders::place_order_request::OrderRequest;
use crate::{Fyers, FyersError, Order};

impl Fyers {
    /// Place a single order.
    ///
    /// Returns the placed [`Order`] on success.
    ///
    /// Use [`OrderRequest::builder`] to construct the order.
    ///
    /// # Example
    /// ```no_run
    /// let order = OrderRequest::builder(
    ///    "NSE:JIOFIN-EQ",
    ///    1,
    ///    OrderType::Market,
    ///    Side::Buy,
    ///    ProductType::Intraday,
    ///    Validity::Day,
    /// )
    /// .offline_order(true) // Optional args here
    /// .build(); // build the request
    ///
    /// let order = fyers.place_order(&order).await?;
    /// println!("{order:?}");
    /// ```
    pub async fn place_order(&self, order: &OrderRequest) -> Result<Order, FyersError> {
        let url = format!("{}/orders/sync", self.base_urls.api_v3);
        let response = self.post(&url, order).await?;
        Ok(Order::deserialize(response)?)
    }
}
