use serde::Serialize;

#[derive(Debug, Default, Serialize, Clone, Copy)]
#[serde(rename_all = "UPPERCASE")]
pub enum Validity {
    /// Immediate or Cancel
    Ioc,

    /// Valid til the end of the day
    #[default]
    Day,
}
