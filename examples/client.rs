use std::{env, error::Error};

use fyers::{Fyers, HistoryRequest, OrderRequest, OrderType, ProductType, Side, Validity};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenvy::dotenv().ok();
    let client_id = env::var("FYERS_CLIENT_ID").expect("FYERS_CLIENT_ID not set");
    let access_token = env::var("FYERS_ACCESS_TOKEN").expect("FYERS_ACCESS_TOKEN not set");

    // Create a fyers client.
    let fyers = Fyers::new(&client_id, &access_token);

    // Get profile info.
    let profile = fyers.profile().await?;
    println!("{profile:?}");

    // Construct an order
    let order = OrderRequest::builder(
        "NSE:JIOFIN-EQ",
        1,
        OrderType::Market,
        Side::Buy,
        ProductType::Intraday,
        Validity::Day,
    )
    .offline_order(true) // Optional args here
    .build(); // build the request

    // Pass it in here
    let order = fyers.place_order(&order).await?;
    println!("{order:?}");

    // Set from and to dates according to IST easily
    let from = fyers::ist_datetime(2026, 2, 5, 9, 30);
    let to = fyers::ist_datetime(2026, 2, 5, 15, 15);

    // Create a history request
    //
    // defaults to 5 min candles
    let history_request = HistoryRequest::builder("NSE:JIOFIN-EQ", from, to).build();

    // Returns a Vec of candles
    let history = fyers.history(&history_request).await?;
    for candle in &history {
        println!("{}", candle.open)
    }

    Ok(())
}
