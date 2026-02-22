# fyers

An idiomatic, strongly-typed, async Rust client for the Fyers Trading API.

[![Crates.io](https://img.shields.io/crates/v/fyers.svg)](https://crates.io/crates/fyers)
[![Docs.rs](https://docs.rs/fyers/badge.svg)](https://docs.rs/fyers)
[![CI](https://github.com/fushinori/fyers/actions/workflows/ci.yml/badge.svg)](https://github.com/fushinori/fyers/actions)
[![License](https://img.shields.io/crates/l/fyers.svg)](https://crates.io/crates/fyers)

## Features

- Fully async (`reqwest` + `tokio`)
- Typed request builders and response models - no raw JSON responses or dynamic dictionaries
- Single, ergonomic error type (`FyersError`)
- Authentication helpers

## Examples

```rust
use fyers::{Fyers, FyersError, OrderRequest, OrderType, ProductType, Side, Validity};

#[tokio::main]
async fn main() -> Result<(), FyersError> {
    // Create a fyers client.
    let fyers = Fyers::new("CLIENT_ID", "ACCESS_TOKEN");

    // Get profile info.
    let profile = fyers.profile().await?;
    println!("{profile:?}");

    // Construct an order request.
    let order = OrderRequest::builder(
        "NSE:JIOFIN-EQ",
        1,
        OrderType::Market,
        Side::Buy,
        ProductType::Intraday,
        Validity::Day,
    )
    .offline_order(true) // Chain optional args
    .build(); // build the request

    // Pass it to the appropriate method.
    let order = fyers.place_order(&order).await?;
    println!("{order:?}");

    Ok(())
}
```

You can find more examples in the [examples] directory.

Also, check the [docs] for more information on all the endpoints and return types.

## Scope

This crate does not aim to mirror the entire Fyers API one-to-one. Only commonly used and practically necessary endpoints are implemented for now.

If you need support for a specific endpoint, please feel free to open an issue.

## License

Licensed under either of:

- Apache License, Version 2.0 ([LICENSE-APACHE](./LICENSE-APACHE) or <http://www.apache.org/licenses/LICENSE-2.0>)
- MIT license ([LICENSE-MIT](./LICENSE-MIT) or <http://opensource.org/licenses/MIT>)

at your option.

[examples]: https://github.com/fushinori/fyers/tree/master/examples
[docs]: https://docs.rs/fyers
