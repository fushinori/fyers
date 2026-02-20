use serde::Serialize;

/// Candle timeframe used when requesting historical market data.
///
/// This enum maps directly to the resolution values expected by the
/// Fyers History API.
///
/// # Example
///
/// ```
/// use fyers::CandleResolution;
///
/// // 5 minute candles (most common)
/// let resolution = CandleResolution::Minute5;
///
/// // Daily candles
/// let resolution = CandleResolution::Day;
/// ```
///
/// The variant names are intentionally explicit and human-readable,
/// while the serialized values match the raw API requirements.
#[derive(Debug, Clone, Copy, Serialize, PartialEq, Eq, Hash)]
pub enum CandleResolution {
    /// Daily candles.
    #[serde(rename = "D")]
    Day,

    /// 5-second candles.
    #[serde(rename = "5S")]
    Seconds5,

    /// 10-second candles.
    #[serde(rename = "10S")]
    Seconds10,

    /// 15-second candles.
    #[serde(rename = "15S")]
    Seconds15,

    /// 30-second candles.
    #[serde(rename = "30S")]
    Seconds30,

    /// 45-second candles.
    #[serde(rename = "45S")]
    Seconds45,

    /// 1-minute candles.
    #[serde(rename = "1")]
    Minute1,

    /// 2-minute candles.
    #[serde(rename = "2")]
    Minute2,

    /// 3-minute candles.
    #[serde(rename = "3")]
    Minute3,

    /// 5-minute candles (most commonly used).
    #[serde(rename = "5")]
    Minute5,

    /// 10-minute candles.
    #[serde(rename = "10")]
    Minute10,

    /// 15-minute candles.
    #[serde(rename = "15")]
    Minute15,

    /// 20-minute candles.
    #[serde(rename = "20")]
    Minute20,

    /// 30-minute candles.
    #[serde(rename = "30")]
    Minute30,

    /// 60-minute candles.
    #[serde(rename = "60")]
    Minute60,

    /// 120-minute candles.
    #[serde(rename = "120")]
    Minute120,

    /// 240-minute candles.
    #[serde(rename = "240")]
    Minute240,
}
