//! An async Rust client for the Fyers trading API.

#![warn(missing_docs)]
#![warn(rustdoc::broken_intra_doc_links)]

pub mod auth;
pub mod client;

mod datetime;
mod endpoints;
mod error;
mod models;
mod utils;

pub use client::Fyers;
pub use error::FyersError;

// Profile
pub use models::Profile;

// History
pub use models::{Candle, CandleResolution, HistoryBuilder, HistoryRequest};

// Orders
pub use models::{Order, OrderBuilder, OrderRequest, OrderType, ProductType, Side, Validity};

pub use datetime::ist_datetime;
