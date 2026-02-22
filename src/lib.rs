//! # fyers
//!
//! An idiomatic, strongly-typed, async Rust client for the Fyers Trading API.
//!
//! ## Overview
//!
//! This crate provides a type-safe interface to the Fyers REST API,
//! modeling requests and responses as Rust types rather than exposing
//! raw JSON values.
//!
//! ## Example
//!
//! ```no_run
//! use fyers::{Fyers, FyersError};
//!
//! #[tokio::main]
//! async fn main() -> Result<(), FyersError> {
//!     let fyers = Fyers::new("CLIENT_ID", "ACCESS_TOKEN");
//!
//!     let profile = fyers.profile().await?;
//!     println!("{profile:?}");
//!
//!     Ok(())
//! }
//! ```

#![deny(missing_docs)]
#![deny(rustdoc::broken_intra_doc_links)]

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

// Positions
pub use models::ExitPositionResult;

pub use datetime::ist_datetime;
