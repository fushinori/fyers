mod common;

use httpmock::prelude::*;

const SUCCESS: &str = include_str!("fixtures/positions/positions_closed_success.json");

#[tokio::test]
async fn cancel_pending_orders_success() {
    let ctx = common::setup().await;

    let position_id = "NSE:SBIN-EQ-INTRADAY";

    let mock = ctx
        .server
        .mock_async(|when, then| {
            when.method(DELETE)
                .path("/positions")
                .json_body_obj(&serde_json::json!({
                    "pending_orders_cancel": 1,
                    "id": position_id
                }));

            then.status(200)
                .header("content-type", "application/json")
                .body(SUCCESS);
        })
        .await;

    let result = ctx.fyers.cancel_pending_orders(position_id).await;

    assert!(result.is_ok());

    mock.assert();
}
