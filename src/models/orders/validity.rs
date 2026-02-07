use serde::Serialize;

#[derive(Debug, Serialize, Clone, Copy)]
#[serde(rename_all = "UPPERCASE")]
pub enum Validity {
    /// Immediate or Cancel
    Ioc,

    /// Valid til the end of the day
    Day,
}
