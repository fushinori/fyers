//! An async Rust client for the Fyers trading API.
pub mod auth;
pub mod client;

mod endpoints;
mod error;
mod models;
mod utils;

pub use self::client::Fyers;
pub use self::error::FyersError;

pub use self::models::profile::Profile;
