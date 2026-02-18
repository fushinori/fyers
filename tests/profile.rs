mod common;
use httpmock::prelude::*;

const PROFILE_SUCCESS: &str = include_str!("fixtures/profile_success.json");

#[tokio::test]
async fn profile_success() {
    let ctx = common::setup();

    let mock = ctx.server.mock(|when, then| {
        when.method(GET)
            .path("/profile")
            .header("Authorization", "TEST_CLIENT_ID:TEST_ACCESS_TOKEN");

        then.status(200)
            .header("content-type", "application/json")
            .body(PROFILE_SUCCESS);
    });

    let profile = ctx.fyers.profile().await.unwrap();

    assert_eq!(profile.name, "XASHXX G H");
    assert!(profile.email_id.contains("@"));
    assert_eq!(profile.pwd_to_expire, 42);
    assert!(!profile.ddpi_enabled);

    mock.assert();
}
