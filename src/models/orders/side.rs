use serde_repr::*;

#[derive(Serialize_repr, Debug, Default, Clone, Copy)]
#[repr(i8)]
pub enum Side {
    #[default]
    Buy = 1,
    Sell = -1,
}
