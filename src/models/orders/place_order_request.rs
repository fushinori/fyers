use serde::Serialize;

use crate::{OrderType, ProductType, Side, Validity};

#[cfg(doc)]
use crate::Fyers;

/// The request type sent to the Fyers place order API.
///
/// This type is typically constructed using [`OrderRequest::builder`]
/// rather than instantiated directly.
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct OrderRequest {
    symbol: String,
    qty: u32,
    r#type: OrderType,
    side: Side,
    product_type: ProductType,
    limit_price: f64,
    stop_price: f64,
    disclosed_qty: u32,
    validity: Validity,
    offline_order: bool,
    stop_loss: f64,
    take_profit: f64,
    order_tag: Option<String>,
    is_slice_order: bool,
}

/// Builder for creating an [`OrderRequest`] used with [`Fyers::place_order`].
///
/// This provides a safe and ergonomic way to construct orders without
/// accidentally sending invalid or incomplete fields to the Fyers API.
///
/// Required fields are provided via [`OrderBuilder::new`], while optional
/// parameters can be configured using the setter methods.
///
/// # Example
///
/// ```
/// use fyers::{OrderBuilder, OrderType, Side, ProductType, Validity};
///
/// let order = OrderBuilder::new(
///     "NSE:SBIN-EQ",
///     1,
///     OrderType::Market,
///     Side::Buy,
///     ProductType::Intraday,
///     Validity::Day,
/// )
/// .order_tag("testing")
/// .build();
/// ```
#[must_use = "builders must be finalized with .build()"]
#[derive(Debug)]
pub struct OrderBuilder {
    symbol: String,
    qty: u32,
    r#type: OrderType,
    side: Side,
    product_type: ProductType,
    limit_price: f64,
    stop_price: f64,
    disclosed_qty: u32,
    validity: Validity,
    offline_order: bool,
    stop_loss: f64,
    take_profit: f64,
    order_tag: Option<String>,
    is_slice_order: bool,
}

impl OrderBuilder {
    /// Create a new order builder with the required parameters.
    ///
    /// # Parameters
    /// - `symbol` — Trading symbol (e.g. `NSE:SBIN-EQ`)
    /// - `qty` — Quantity to trade
    /// - `order_type` — Market, Limit, Stop, etc.
    /// - `side` — Buy or Sell
    /// - `product_type` — CNC, Intraday, Margin, etc.
    /// - `validity` — DAY or IOC
    pub fn new(
        symbol: impl Into<String>,
        qty: u32,
        order_type: OrderType,
        side: Side,
        product_type: ProductType,
        validity: Validity,
    ) -> Self {
        Self {
            symbol: symbol.into(),
            qty,
            r#type: order_type,
            side,
            product_type,
            validity,

            // defaults
            limit_price: 0.0,
            stop_price: 0.0,
            disclosed_qty: 0,
            offline_order: false,
            stop_loss: 0.0,
            take_profit: 0.0,
            order_tag: None,
            is_slice_order: false,
        }
    }

    /// Set the limit price for the order.
    ///
    /// Required for **Limit** and **Stop-Limit** orders.
    pub fn limit_price(mut self, price: f64) -> Self {
        self.limit_price = price;
        self
    }

    /// Set the stop price for the order.
    ///
    /// Required for **Stop (SL-M)** and **Stop-Limit (SL-L)** orders.
    pub fn stop_price(mut self, price: f64) -> Self {
        self.stop_price = price;
        self
    }

    /// Set the disclosed quantity.
    ///
    /// This is only applicable for **equity orders**.  
    /// Defaults to `0` (no disclosed quantity).
    pub fn disclosed_qty(mut self, qty: u32) -> Self {
        self.disclosed_qty = qty;
        self
    }

    /// Mark the order as an AMO (After Market Order).
    ///
    /// Set to `true` when placing orders outside market hours.  
    /// Defaults to `false`.
    pub fn offline_order(mut self, value: bool) -> Self {
        self.offline_order = value;
        self
    }

    /// Set the stop-loss price.
    ///
    /// Required for **Cover Orders (CO)** and **Bracket Orders (BO)**.
    pub fn stop_loss(mut self, price: f64) -> Self {
        self.stop_loss = price;
        self
    }

    /// Set the take-profit price.
    ///
    /// Required for **Bracket Orders (BO)**.
    pub fn take_profit(mut self, price: f64) -> Self {
        self.take_profit = price;
        self
    }

    /// Attach a custom tag to the order.
    ///
    /// This can be used to identify orders created by a strategy.
    pub fn order_tag(mut self, tag: impl Into<String>) -> Self {
        self.order_tag = Some(tag.into());
        self
    }

    /// Enable slice orders.
    ///
    /// When enabled, large quantities may be split into multiple smaller orders
    /// if they exceed the exchange freeze quantity.
    pub fn slice_order(mut self, value: bool) -> Self {
        self.is_slice_order = value;
        self
    }

    /// Return an [`OrderRequest`] with the desired configuration.
    pub fn build(self) -> OrderRequest {
        OrderRequest {
            symbol: self.symbol,
            qty: self.qty,
            r#type: self.r#type,
            side: self.side,
            product_type: self.product_type,
            limit_price: self.limit_price,
            stop_price: self.stop_price,
            disclosed_qty: self.disclosed_qty,
            validity: self.validity,
            offline_order: self.offline_order,
            stop_loss: self.stop_loss,
            take_profit: self.take_profit,
            order_tag: self.order_tag,
            is_slice_order: self.is_slice_order,
        }
    }
}

impl OrderRequest {
    /// Creates an [`OrderBuilder`] to construct an [`OrderRequest`].
    /// This is the same as [`OrderBuilder::new()`].
    pub fn builder(
        symbol: impl Into<String>,
        qty: u32,
        order_type: OrderType,
        side: Side,
        product_type: ProductType,
        validity: Validity,
    ) -> OrderBuilder {
        OrderBuilder::new(symbol, qty, order_type, side, product_type, validity)
    }
}
