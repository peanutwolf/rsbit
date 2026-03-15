# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.1.0] - 2025-01-01

### Added

- **HTTP client** (`HttpManager`, `HTTP`) with full Bybit V5 API coverage across 19 endpoint categories: market, trade, position, account, asset, user, spot margin, spot leverage, broker, earn, fiat, crypto loan, institutional loan, pre-upgrade, spread, rate limit, RFQ, and misc.
- **WebSocket market data streams** (`WsManager`) with automatic delta processing for orderbook and ticker streams. 18 convenience stream methods covering public and private channels.
- **WebSocket trading** (`WsTradingManager`) for order operations over WebSocket with `reqId`-based request/response correlation. Supports place, amend, cancel (single and batch).
- **Authentication**: HMAC-SHA256 (default) and RSA PKCS1v15 signing for both HTTP and WebSocket.
- **Automatic retry** with exponential backoff for transient errors (recv_window, rate limits, network failures).
- **Rate-limit handling**: automatic sleep until `X-Bapi-Limit-Reset-Timestamp` on 10006 errors.
- **Configuration**: testnet, demo, custom TLD (`.com.hk`, `.nl`, `.kz`, `.eu`), custom domains (`bytick`, `bybit-tr`).
- **Typed request/response**: strongly-typed parameter structs and response deserialization for all endpoints.
- **Feature flags**: `rustls-tls` (default) and `native-tls` for TLS backend selection.
- **Examples**: `http_basic`, `ws_orderbook`, `ws_trading`.
