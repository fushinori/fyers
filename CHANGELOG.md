# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.2.0] - 2026-02-22

### Added
- Async `Fyers` client for interacting with Fyers REST endpoints.
- `Fyers::place_order` for placing orders.
- `Fyers::cancel_order` for canceling a single order.
- `Fyers::cancel_pending_orders` for canceling all pending orders for a given position.
- `Fyers::exit_all_positions` for closing all open positions.
- `Fyers::history` for retrieving historical market data.
- `Fyers::profile` for fetching account profile information.
- Strongly-typed request builders (`OrderRequest`, `HistoryRequest`).
- Typed response models for endpoints.
- Unified error type: `FyersError`.

### Changed
- Crate expanded beyond authentication to include core trading and market data APIs.

### Notes
- The crate now provides a type-safe interface to commonly used Fyers REST endpoints.
- WebSocket APIs and less commonly used endpoints are not yet implemented.

## [0.1.3] - 2026-02-06

### Fixed
- Add missing query param to `generate_url`.

## [0.1.2] - 2026-02-06

### Fixed
- Re-export `Tokens` from `auth`.

## [0.1.1] - 2026-01-21

### Fixed
- `auth::error` module is now public.
- `AuthError` is also re-exported from `auth`.

## [0.1.0] - 2026-01-21

### Added
- Authentication helpers for the Fyers API.
- Support for generating an authorization URL for the interactive login flow.
- Support for exchanging an authorization code for access and refresh tokens.
- Support for refreshing an access token using a refresh token.
- Error handling for authentication-related failures.
- Example demonstrating the complete authentication flow.

### Notes
- This release focuses exclusively on authentication.
- Token persistence, refresh scheduling, and lifecycle management are expected to be handled by the application.
- Trading, orders, positions, and market data APIs are not yet implemented.
