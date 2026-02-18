use chrono::{DateTime, Utc};
use serde::Serialize;

use crate::CandleResolution;

#[cfg(doc)]
use crate::Fyers;

/// Builder for creating a [`HistoryRequest`] used with [`Fyers::history`].
///
/// # Example
///
/// ```
/// use chrono::{Utc, Duration};
///
/// let from = Utc::now() - Duration::days(5);
/// let to   = Utc::now();
///
/// let request = HistoryRequest::builder("NSE:SBIN-EQ", from, to)
///     .resolution(CandleResolution::Minute1)
///     .include_oi(true)
///     .build();
/// ```
#[must_use = "builders must be finalized with .build()"]
#[derive(Debug, Clone)]
pub struct HistoryBuilder {
    symbol: String,
    resolution: CandleResolution,
    from: DateTime<Utc>,
    to: DateTime<Utc>,
    include_oi: bool,
}

/// The request type sent to the Fyers history API.
///
/// This type is typically constructed using [`HistoryRequest::builder`]
/// rather than instantiated directly.
#[derive(Serialize)]
pub struct HistoryRequest {
    symbol: String,
    resolution: CandleResolution,
    date_format: &'static str,
    range_from: String,
    range_to: String,
    cont_flag: &'static str,

    #[serde(skip_serializing_if = "Option::is_none")]
    oi_flag: Option<&'static str>,
}

impl HistoryBuilder {
    /// Creates a builder with default configuration:
    ///
    /// - Resolution: **5 minute candles**
    /// - Open Interest: **disabled**
    pub fn new(symbol: impl Into<String>, from: DateTime<Utc>, to: DateTime<Utc>) -> Self {
        Self {
            symbol: symbol.into(),
            resolution: CandleResolution::Minute5,
            from,
            to,
            include_oi: false,
        }
    }

    /// Set the candle resolution.
    ///
    /// Defaults to **5-minute candles**.
    pub fn resolution(mut self, resolution: CandleResolution) -> Self {
        self.resolution = resolution;
        self
    }

    /// Include Open Interest (OI) data in the response.
    ///
    /// This is only available for **futures and options instruments**.
    /// Disabled by default.
    pub fn include_oi(mut self, include: bool) -> Self {
        self.include_oi = include;
        self
    }

    /// Return a [`HistoryRequest`] with the desired configuration.
    pub fn build(self) -> HistoryRequest {
        HistoryRequest {
            symbol: self.symbol,
            resolution: self.resolution,
            date_format: "0",
            range_from: self.from.timestamp().to_string(),
            range_to: self.to.timestamp().to_string(),
            cont_flag: "1",
            oi_flag: if self.include_oi { Some("1") } else { None },
        }
    }
}

impl HistoryRequest {
    /// Creates a [`HistoryBuilder`] to construct a [`HistoryRequest`].
    /// This is the same as [`HistoryBuilder::new()`].
    pub fn builder(
        symbol: impl Into<String>,
        from: DateTime<Utc>,
        to: DateTime<Utc>,
    ) -> HistoryBuilder {
        HistoryBuilder::new(symbol, from, to)
    }
}
