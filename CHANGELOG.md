# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.2.0] - 2026-05-16

Syncs with pybit v5.15.0rc2 → v5.16.0 (pybit commits `f3dbc69`–`5b4da99`).

### Added

- **P2P module** (`api/p2p`, `params/p2p`, `endpoints/p2p`): 19 authenticated POST methods for P2P trading — account info, ads management, order handling, counterparty lookup, chat messaging, and payment types. Mirrors `pybit/_v5_p2p.py`. Note: `upload_chat_file` is not yet implemented (requires multipart upload support).
- **Account**: `set_delta_mode()` for delta-neutral trading mode (`/v5/account/set-delta-mode`).
- **Asset**: `get_asset_overview()` and `get_funding_acc_history()` for total asset snapshots and funding account transaction logs.
- **User**: `sign_agreement()` and `get_friend_referrals()`.
- **Broker**: `get_broker_all_rate_limits()`, `get_broker_rate_limit_cap()`, `set_broker_rate_limit()` (institutional only).
- **Crypto Loan**: `get_max_loan_amount_new_crypto_loan()`.
- **Position**: `confirm_new_risk_limit()` (confirms acceptance of a new MMR after liquidation).
- **Spot Margin**: `spot_margin_trade_get_currency_data()`.

### Changed

- **Account**: `set_no_convert_repay()` renamed to `manual_no_convert_repay()` with updated params (`coin` + `amount`). Endpoint constant `SET_NO_CONVERT_REPAY` renamed to `NO_CONVERT_REPAY`. Mirrors pybit v5.15.0rc2.
- **HTTP response parsing**: error fields are now detected for both `retCode`/`retMsg` (standard) and `ret_code`/`ret_msg` (P2P API). Mirrors pybit commit `f3dbc69`.

### Notes

- WebSocket shutdown (pybit commits `b58fd4a`, `73966e0`) does not require changes — tokio task + broadcast channel already handles ping-task cancellation and avoids busy-waits.
- The fiat-related methods added to pybit's asset module (`get_fiat_balance`, `get_fiat_trading_pair_list`, etc.) are already covered by rsbit's existing `fiat` module and were not duplicated.

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
