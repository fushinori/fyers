use chrono::{DateTime, Utc};
use serde::Deserialize;

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
