use serde::Serialize;

use crate::{OrderType, ProductType, Side, Validity};

#[derive(Debug, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct PlaceOrderRequest {
    /// Eg: NSE:SBIN-EQ
    pub symbol: String,

    /// The quantity should be in multiples of lot size for derivatives
    pub qty: u32,

    /// Order type
    pub r#type: OrderType,

    /// Buy or sell
    pub side: Side,

    /// Product type
    pub product_type: ProductType,

    /// Provide valid price for Limit and Stoplimit orders
    pub limit_price: f64,

    /// Provide valid price for Stop and Stoplimit orders
    pub stop_price: f64,

    /// Allowed only for Equity
    pub disclosed_qty: u32,

    /// Validity
    pub validity: Validity,

    /// False => When market is open
    /// True => When placing AMO order
    pub offline_order: bool,

    /// Provide valid price for CO and BO orders
    pub stop_loss: f64,

    /// Provide valid price for BO orders
    pub take_profit: f64,

    /// (Optional) Tag you want to assign to the specific order
    pub order_tag: Option<String>,

    /// False => The full quantity is placed as one single order.
    /// True => The quantity is placed in multiple smaller orders if the total quantity is more than the freeze quantity.
    pub is_slice_order: bool,
}
