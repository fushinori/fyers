use serde::Deserialize;

/// Returned on successful order
#[derive(Debug, Deserialize, Clone)]
pub struct Order {
    pub id: String,
}
