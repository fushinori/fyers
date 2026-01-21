use std::{env, error::Error, io};

use fyers::auth;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenvy::dotenv().ok();
    let client_id = env::var("FYERS_CLIENT_ID").expect("FYERS_CLIENT_ID not set");
    let secret_key = env::var("FYERS_SECRET_KEY").expect("FYERS_SECRET_KEY not set");
    let redirect_uri = env::var("FYERS_REDIRECT_URI").expect("FYERS_REDIRECT_URI not set");
    let pin = env::var("FYERS_PIN").expect("FYERS_PIN not set");

    // Get the login URL
    let url = auth::generate_url(&client_id, &redirect_uri, "yourrandomstate")?;
    println!("{url}");

    let mut response_url = String::new();
    println!("Paste the response URL:");
    io::stdin().read_line(&mut response_url)?;

    // Generate your access and refresh tokens
    let tokens = auth::generate_tokens(&client_id, &secret_key, &response_url).await?;
    println!("Your tokens: {tokens:?}");

    // Using refresh token to generate a new access_token
    let tokens = auth::refresh_tokens(&client_id, &secret_key, &tokens.refresh_token, &pin).await?;
    println!("Your refreshed tokens: {tokens:?}");

    Ok(())
}
