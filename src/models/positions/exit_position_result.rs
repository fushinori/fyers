use serde::Deserialize;

/// Result of an exit position operation.
///
/// Fyers may return different success codes depending on whether
/// the counter order was immediately filled or not.
#[derive(Debug, Deserialize, Clone, Copy, PartialEq, Eq)]
pub enum ExitPositionResult {
    /// The position was successfully closed.
    ///
    /// Corresponds to API code `200`.
    Closed,

    /// A counter order was placed to close the position
    /// but it has not yet been filled.
    ///
    /// Corresponds to API code `201`.
    PendingCounterOrder,
}
