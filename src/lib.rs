//! An async Rust client for the Fyers trading API.
pub mod auth;
pub mod client;

mod datetime;
mod endpoints;
mod error;
mod models;
mod urls;
mod utils;

pub use client::Fyers;
pub use error::FyersError;

// Profile
pub use models::Profile;

// History
pub use models::{Candle, CandleResolution, HistoryRequestBuilder};

// Orders
pub use models::{Order, OrderType, PlaceOrderBuilder, ProductType, Side, Validity};

pub use datetime::ist_datetime;
