use serde_repr::*;

#[derive(Serialize_repr, Debug, Clone, Copy)]
#[repr(i8)]
pub enum Side {
    Buy = 1,
    Sell = -1,
}
