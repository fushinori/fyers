use serde::Serialize;

/// Product type for an order.
///
/// This determines how the position is held and margined by the broker.
///
/// Values map directly to the strings expected by Fyers.
#[derive(Debug, Clone, Copy, Serialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "UPPERCASE")]
pub enum ProductType {
    /// **CNC (Cash and Carry)**
    ///
    /// Delivery-based equity trading.
    /// Positions are carried overnight until manually closed.
    Cnc,

    /// **Intraday**
    ///
    /// Positions must be closed within the same trading day.
    /// Lower margin requirements compared to CNC.
    Intraday,

    /// **Margin**
    ///
    /// Margin trading for derivative segments (F&O).
    Margin,

    /// **Cover Order (CO)**
    ///
    /// Intraday order that **requires a stop-loss**.
    /// Provides higher leverage due to mandatory risk control.
    Co,

    /// **Bracket Order (BO)**
    ///
    /// Order with both **stop-loss and take-profit**.
    /// Automatically exits the position when either level is hit.
    Bo,

    /// **Margin Trading Facility (MTF)**
    ///
    /// Broker-funded leveraged delivery positions for approved stocks.
    Mtf,
}
