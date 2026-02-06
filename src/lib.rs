//! An async Rust client for the Fyers trading API.
pub mod auth;
pub mod client;

mod error;
mod models;

pub use self::client::Fyers;
pub use self::error::FyersError;
