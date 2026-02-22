mod common;

use httpmock::prelude::*;

const SUCCESS: &str = include_str!("fixtures/order_cancel_success.json");

#[tokio::test]
async fn cancel_order_success() {
    let ctx = common::setup().await;

    let order_id = "808058117761";

    let mock = ctx
        .server
        .mock_async(|when, then| {
            when.method(DELETE)
                .path("/orders/sync")
                .json_body_obj(&serde_json::json!({
                    "id": order_id
                }));

            then.status(200)
                .header("content-type", "application/json")
                .body(SUCCESS);
        })
        .await;

    let result = ctx.fyers.cancel_order(order_id).await;

    assert!(result.is_ok());

    mock.assert();
}
