use std::{env, error::Error};

use fyers::Fyers;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenvy::dotenv().ok();
    let client_id = env::var("FYERS_CLIENT_ID").expect("FYERS_CLIENT_ID not set");
    let access_token = env::var("FYERS_ACCESS_TOKEN").expect("FYERS_ACCESS_TOKEN not set");

    // Create a fyers client.
    let fyers = Fyers::new(&client_id, &access_token);

    // Call methods.
    let profile = fyers.profile().await?;
    println!("{profile:?}");

    Ok(())
}
