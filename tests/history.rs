mod common;
use chrono::{TimeZone, Utc};
use fyers::{CandleResolution, HistoryRequest};
use httpmock::prelude::*;

const HISTORY_SUCCESS: &str = include_str!("fixtures/history_success.json");

#[tokio::test]
async fn history_success() {
    let ctx = common::setup().await;

    let from = fyers::ist_datetime(2021, 1, 1, 9, 15);
    let to = fyers::ist_datetime(2021, 1, 6, 3, 30);
    let history_request = HistoryRequest::builder("NSE:SBIN-EQ", from, to)
        .resolution(CandleResolution::Day)
        .build();
    let mock = ctx
        .server
        .mock_async(|when, then| {
            match_query(when.method(GET).path("/history"), &history_request);

            then.status(200)
                .header("content-type", "application/json")
                .body(HISTORY_SUCCESS);
        })
        .await;

    let candles = ctx.fyers.history(&history_request).await.unwrap();

    assert_eq!(candles.len(), 4);

    let first = candles.first().unwrap();

    assert_eq!(first.time, Utc.timestamp_opt(1609459200, 0).unwrap());
    assert_eq!(first.open, 274.9);
    assert_eq!(first.high, 280.0);
    assert_eq!(first.low, 274.4);
    assert_eq!(first.close, 279.4);
    assert_eq!(first.volume, 24_531_791);

    let last = candles.last().unwrap();

    assert_eq!(last.time, Utc.timestamp_opt(1609891200, 0).unwrap());
    assert_eq!(last.close, 285.05);
    assert_eq!(last.volume, 40_765_708);

    // Ensure candles are in chronological order.
    assert!(
        candles
            .windows(2)
            .all(|candle_pair| candle_pair[0].time < candle_pair[1].time)
    );

    assert!(candles.iter().all(|candle| candle.open_interest.is_none()));

    for c in &candles {
        assert!(c.high >= c.low);
    }

    mock.assert();
}

/// Adds query parameter matchers to an [`httpmock::When`] from a serializable value.
///
/// The value is serialized using `serde_urlencoded` (the same encoder used by
/// `reqwest::RequestBuilder::query`) and then URL-decoded before being converted
/// into `.query_param()` matchers. This keeps tests aligned with the actual
/// request sent by the client and avoids duplicating serialization logic.
///
/// Intended for test code. Panics if serialization fails.
pub fn match_query<T: serde::Serialize>(mut when: httpmock::When, value: &T) -> httpmock::When {
    let params = serde_urlencoded::to_string(value).unwrap();

    for (k, v) in url::form_urlencoded::parse(params.as_bytes()) {
        when = when.query_param(k.as_ref(), v.as_ref());
    }

    when
}
