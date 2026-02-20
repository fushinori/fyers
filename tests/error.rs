mod common;
use fyers::FyersError;
use httpmock::prelude::*;

const RANDOM_ERROR: &str = include_str!("fixtures/error/random_error.json");
const INVALID_JSON: &str = include_str!("fixtures/error/invalid.json");
const MISSING_FIELD: &str = include_str!("fixtures/error/missing_field.json");
const INVALID_TOKEN: &str = include_str!("fixtures/error/invalid_token.json");

#[tokio::test]
async fn http_status_error() {
    let ctx = common::setup().await;

    let mock = ctx
        .server
        .mock_async(|when, then| {
            when.method(GET).path("/profile");

            then.status(403).body("Forbidden");
        })
        .await;

    let result = ctx.fyers.profile().await;

    match result {
        Err(FyersError::HttpStatus { status, body }) => {
            assert_eq!(status.as_u16(), 403);
            assert_eq!(body, "Forbidden")
        }
        other => panic!("Expected HttpStatus error, got {other:?}"),
    }

    mock.assert();
}

#[tokio::test]
async fn api_envelope_error() {
    let ctx = common::setup().await;

    let mock = ctx
        .server
        .mock_async(|when, then| {
            when.method(GET).path("/profile");

            then.status(200)
                .header("content-type", "application/json")
                .body(RANDOM_ERROR);
        })
        .await;

    let result = ctx.fyers.profile().await;

    match result {
        Err(FyersError::Api { code, message }) => {
            assert_eq!(code, 9999);
            assert_eq!(message, "This is a random error".to_string());
        }
        other => panic!("Expected Api error, got {other:?}"),
    }

    mock.assert();
}

#[tokio::test]
async fn json_parse_error() {
    let ctx = common::setup().await;

    let mock = ctx
        .server
        .mock_async(|when, then| {
            when.method(GET).path("/profile");

            then.status(200)
                .header("content-type", "application/json")
                .body(INVALID_JSON);
        })
        .await;

    let result = ctx.fyers.profile().await;

    match result {
        Err(FyersError::Json(_)) => {}
        other => panic!("Expected Json error, got {other:?}"),
    }

    mock.assert();
}

#[tokio::test]
async fn missing_field_on_success_error() {
    let ctx = common::setup().await;

    let mock = ctx
        .server
        .mock_async(|when, then| {
            when.method(GET).path("/profile");

            then.status(200)
                .header("content-type", "application/json")
                .body(MISSING_FIELD);
        })
        .await;

    let result = ctx.fyers.profile().await;

    match result {
        Err(FyersError::MissingField(_)) => {}
        other => panic!("Expected MissingField error, got {other:?}"),
    }

    mock.assert();
}

// Test one of the error variants to make sure our mapping
// function works as intended
#[tokio::test]
async fn variant_error() {
    let ctx = common::setup().await;

    let mock = ctx
        .server
        .mock_async(|when, then| {
            when.method(GET).path("/profile");

            then.status(200)
                .header("content-type", "application/json")
                .body(INVALID_TOKEN);
        })
        .await;

    let result = ctx.fyers.profile().await;

    match result {
        Err(FyersError::InvalidToken) => {}
        other => panic!("Expected InvalidToken error, got {other:?}"),
    }

    mock.assert();
}
