use serde_repr::*;

#[derive(Serialize_repr, Debug, Default, Clone, Copy)]
#[repr(u8)]
pub enum OrderType {
    Limit = 1,
    #[default]
    Market = 2,
    Stop = 3,
    StopLimit = 4,
}
