use serde_repr::*;

#[derive(Serialize_repr, Debug, Clone, Copy)]
#[repr(u8)]
pub enum OrderType {
    Limit = 1,
    Market = 2,
    Stop = 3,
    StopLimit = 4,
}
