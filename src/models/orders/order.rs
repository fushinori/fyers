use serde::Deserialize;

/// Returned on successfully placing an order.
#[derive(Debug, Deserialize, Clone, PartialEq, Eq)]
pub struct Order {
    /// Order ID
    pub id: String,
}
