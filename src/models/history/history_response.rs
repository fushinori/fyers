use chrono::{DateTime, Utc};
use serde::Deserialize;

/// A single OHLCV candle returned by the Fyers History API.
///
/// Timestamps are in **UTC**. You can convert them to IST using your own
/// timezone utilities if required.
///
/// # Fields
///
/// - `time` — Candle open time (start of the interval)
/// - `open` — Opening price
/// - `high` — Highest traded price during the interval
/// - `low` — Lowest traded price during the interval
/// - `close` — Closing price (last traded price in the interval)
/// - `volume` — Total traded volume during the interval
/// - `open_interest` — Open interest for derivative instruments (if requested)
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
#[derive(Debug, Clone)]
pub struct Candle {
    pub time: DateTime<Utc>,
    pub open: f64,
    pub high: f64,
    pub low: f64,
    pub close: f64,
    pub volume: f64,
    pub open_interest: Option<f64>,
}

impl<'de> Deserialize<'de> for Candle {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let raw: Vec<f64> = Vec::deserialize(deserializer)?;

        if raw.len() < 6 {
            return Err(serde::de::Error::custom("invalid candle length"));
        }

        let timestamp = raw[0] as i64;

        Ok(Candle {
            time: chrono::DateTime::from_timestamp(timestamp, 0)
                .ok_or_else(|| serde::de::Error::custom("invalid timestamp"))?,
            open: raw[1],
            high: raw[2],
            low: raw[3],
            close: raw[4],
            volume: raw[5],
            open_interest: raw.get(6).copied(),
        })
    }
}

#[derive(Debug, Deserialize)]
pub(crate) struct HistoryResponse {
    pub(crate) candles: Vec<Candle>,
}
