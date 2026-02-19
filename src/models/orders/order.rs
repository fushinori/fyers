use serde::Deserialize;

/// Returned on successfully placing an order.
#[derive(Debug, Deserialize, Clone)]
pub struct Order {
    /// Order ID
    pub id: String,
}
