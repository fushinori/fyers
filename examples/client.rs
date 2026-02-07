use std::{env, error::Error};

use fyers::{Fyers, PlaceOrderRequest};

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

    // Place a single order.
    //
    // Note how we can use default values
    let order_request = PlaceOrderRequest {
        symbol: "NSE:JIOFIN-EQ".to_string(),
        qty: 1,
        offline_order: true,
        ..Default::default()
    };
    let order = fyers.place_order(&order_request).await?;

    println!("{order:?}");

    Ok(())
}
