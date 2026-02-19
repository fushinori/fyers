use serde::Serialize;

/// Order validity specifying how long the order remains active.
#[derive(Debug, Serialize, Clone, Copy)]
#[serde(rename_all = "UPPERCASE")]
pub enum Validity {
    /// Immediate or Cancel
    ///
    /// The order is executed immediately. Any remaining unfilled quantity
    /// is cancelled.
    Ioc,

    /// Valid for the entire trading day
    Day,
}
