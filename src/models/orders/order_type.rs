use serde_repr::*;

/// Type of order to place.
///
/// This maps directly to the numeric values expected by Fyers.
#[derive(Serialize_repr, Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum OrderType {
    /// Limit order -> 1
    ///
    /// A limit order allows you to buy or sell an asset at a specific price (`limit_price`) or better.
    /// It will only be executed at the specified price or lower for a buy order,
    /// and at the specified price or higher for a sell order.
    Limit = 1,

    /// Market order -> 2
    ///
    /// A market order allows you to buy or sell an asset at the current market price.
    Market = 2,

    /// Stop order (SL-M) -> 3
    ///
    /// A stop order is designed to limit losses on a position. It becomes a market order when the `stop_price` is reached.
    /// The `stop_price` is the trigger price at which the market order will be placed.
    Stop = 3,

    /// Stop limit order (SL-L) -> 4
    ///
    /// A stop-limit order combines the features of a stop order and a limit order.
    /// It triggers a limit order once the `stop_price` is reached.
    StopLimit = 4,
}
