mod common;
use fyers::{OrderRequest, OrderType, ProductType, Side, Validity};
use httpmock::prelude::*;

const ORDER_SUCCESS: &str = include_str!("fixtures/order_success.json");

#[tokio::test]
async fn place_order_success() {
    let ctx = common::setup().await;

    let order = OrderRequest::builder(
        "NSE:SBIN-EQ",
        1,
        OrderType::Market,
        Side::Buy,
        ProductType::Intraday,
        Validity::Day,
    )
    .build();

    let mock = ctx
        .server
        .mock_async(|when, then| {
            when.method(POST)
                .path("/orders/sync")
                .header("Authorization", "TEST_CLIENT_ID:TEST_ACCESS_TOKEN")
                .header("content-type", "application/json")
                .json_body_obj(&order);

            then.status(200)
                .header("content-type", "application/json")
                .body(ORDER_SUCCESS);
        })
        .await;

    let response = ctx.fyers.place_order(&order).await.unwrap();

    assert_eq!(response.id, "808058117761");

    mock.assert();
}
