use serde_repr::*;

/// Order side used when placing trades.
///
/// This maps to the numeric values expected by Fyers.
#[derive(Serialize_repr, Debug, Clone, Copy)]
#[repr(i8)]
pub enum Side {
    /// Buy -> 1
    Buy = 1,

    /// Sell -> -1
    Sell = -1,
}
