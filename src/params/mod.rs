//! Typed request parameter structs for all Bybit V5 API endpoints.
//!
//! Each struct derives `Serialize` so it can be converted to query strings (GET)
//! or JSON bodies (POST). Optional fields use `skip_serializing_if = "Option::is_none"`.

pub mod account;
pub mod asset;
pub mod broker;
pub mod crypto_loan;
pub mod earn;
pub mod fiat;
pub mod institutional_loan;
pub mod market;
pub mod misc;
pub mod position;
pub mod pre_upgrade;
pub mod rate_limit;
pub mod rfq;
pub mod spot_leverage;
pub mod spot_margin;
pub mod spread;
pub mod trade;
pub mod user;
