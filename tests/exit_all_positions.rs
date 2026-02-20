mod common;

use fyers::ExitPositionResult;
use httpmock::prelude::*;

const SUCCESS: &str = include_str!("fixtures/positions/positions_closed_success.json");
const PENDING: &str = include_str!("fixtures/positions/positions_pending_counter_order.json");

#[tokio::test]
async fn exit_all_positions_success() {
    let ctx = common::setup().await;

    let mock = ctx
        .server
        .mock_async(|when, then| {
            when.method(DELETE).path("/positions");

            then.status(200)
                .header("content-type", "application/json")
                .body(SUCCESS);
        })
        .await;

    let result = ctx.fyers.exit_all_positions().await.unwrap();

    assert_eq!(result, ExitPositionResult::Closed);

    mock.assert();
}

#[tokio::test]
async fn exit_all_positions_pending() {
    let ctx = common::setup().await;

    let mock = ctx
        .server
        .mock_async(|when, then| {
            when.method(DELETE).path("/positions");

            then.status(200)
                .header("content-type", "application/json")
                .body(PENDING);
        })
        .await;

    let result = ctx.fyers.exit_all_positions().await.unwrap();

    assert_eq!(result, ExitPositionResult::PendingCounterOrder);

    mock.assert();
}
