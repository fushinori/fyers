use chrono::{DateTime, Utc};
use serde::Serialize;

use crate::CandleResolution;

#[derive(Debug, Clone)]
pub struct HistoryRequest {
    pub symbol: String,
    pub resolution: CandleResolution,
    pub from: DateTime<Utc>,
    pub to: DateTime<Utc>,
    pub include_oi: bool,
}

#[derive(Serialize)]
pub(crate) struct ApiHistoryRequest<'a> {
    symbol: &'a str,
    resolution: CandleResolution,
    date_format: &'static str,
    range_from: String,
    range_to: String,
    cont_flag: &'static str,

    #[serde(skip_serializing_if = "Option::is_none")]
    oi_flag: Option<&'static str>,
}

impl HistoryRequest {
    pub(crate) fn as_api(&self) -> ApiHistoryRequest<'_> {
        ApiHistoryRequest {
            symbol: &self.symbol,
            resolution: self.resolution,
            date_format: "0",
            range_from: self.from.timestamp().to_string(),
            range_to: self.to.timestamp().to_string(),
            cont_flag: "1",
            oi_flag: if self.include_oi { Some("1") } else { None },
        }
    }

    /// Create a new HistoryRequest with sane defaults.
    ///
    /// 5 minute candle resolution
    /// include_oi = false
    pub fn new(symbol: impl Into<String>, from: DateTime<Utc>, to: DateTime<Utc>) -> Self {
        Self {
            symbol: symbol.into(),

            // default to 5 minutes
            resolution: CandleResolution::Minute5,
            from,
            to,

            // don't include OI by default
            include_oi: false,
        }
    }

    pub fn resolution(mut self, resolution: CandleResolution) -> Self {
        self.resolution = resolution;
        self
    }

    pub fn include_oi(mut self, include: bool) -> Self {
        self.include_oi = include;
        self
    }
}
