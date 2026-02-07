//! An async Rust client for the Fyers trading API.
pub mod auth;
pub mod client;

mod endpoints;
mod error;
mod models;
mod utils;

pub use client::Fyers;
pub use error::FyersError;

pub use models::{Order, OrderType, PlaceOrderRequest, ProductType, Profile, Side, Validity};
