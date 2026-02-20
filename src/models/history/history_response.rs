use chrono::{DateTime, Utc};
use serde::Deserialize;

/// A single OHLCV candle returned by the Fyers History API.
///
/// Timestamps are in **UTC**. You can convert them to IST using your own
/// timezone utilities if required.
///
/// # Notes
///
/// * The Fyers API returns candles as an array of numeric values.
///   This struct provides a strongly-typed representation.
/// * `open_interest` will be `None` unless the history request enables OI.
///
/// # Example
///
/// ```no_run
/// # use fyers::Fyers;
/// # use fyers::HistoryRequest;
///
/// # async fn run() -> Result<(), Box<dyn std::error::Error>> {
/// # let fyers = Fyers::new("id", "token");
/// # let from = fyers::ist_datetime(2026, 2, 5, 9, 30);
/// # let to = fyers::ist_datetime(2026, 2, 5, 15, 15);
///
/// # let request = HistoryRequest::builder("NSE:JIOFIN-EQ", from, to).build();
/// let candles = fyers.history(&request).await?;
///
/// let last = candles.last().unwrap();
/// println!("Close price: {}", last.close);
/// # Ok(())
/// # }
/// ```
#[derive(Debug, Clone, PartialEq)]
pub struct Candle {
    /// Candle open time (start of the interval)
    pub time: DateTime<Utc>,
    /// Opening price
    pub open: f64,
    /// Highest traded price during the interval
    pub high: f64,
    /// Lowest traded price during the interval
    pub low: f64,
    /// Closing price (last traded price in the interval)
    pub close: f64,
    /// Total traded volume during the interval
    pub volume: u64,
    /// Open interest for derivative instruments (if requested)
    pub open_interest: Option<f64>,
}

#[derive(serde::Deserialize)]
struct RawCandle(
    i64,                           // timestamp
    f64,                           // open
    f64,                           // high
    f64,                           // low
    f64,                           // close
    u64,                           // volume
    #[serde(default)] Option<f64>, // open interest
);

impl<'de> serde::Deserialize<'de> for Candle {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let raw = RawCandle::deserialize(deserializer)?;

        Ok(Candle {
            time: chrono::DateTime::from_timestamp(raw.0, 0)
                .ok_or_else(|| serde::de::Error::custom("invalid timestamp"))?,
            open: raw.1,
            high: raw.2,
            low: raw.3,
            close: raw.4,
            volume: raw.5,
            open_interest: raw.6,
        })
    }
}

#[derive(Debug, Deserialize)]
pub(crate) struct HistoryResponse {
    pub(crate) candles: Vec<Candle>,
}
