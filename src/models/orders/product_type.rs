use serde::Serialize;

#[derive(Debug, Serialize, Clone, Copy)]
#[serde(rename_all = "UPPERCASE")]
pub enum ProductType {
    /// For equity only
    Cnc,

    /// Applicable for all segments
    Intraday,

    /// Applicable only for derivatives
    Margin,

    ///  Cover Order
    Co,

    /// Bracket Order
    Bo,

    /// Approved Symbols Only
    Mtf,
}
