//! An async Rust client for the Fyers trading API.
pub mod auth;

mod error;
mod models;

pub use self::error::FyersError;
