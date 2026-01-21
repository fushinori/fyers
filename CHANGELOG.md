# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

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
