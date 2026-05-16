//! Typed response structs for all Bybit V5 API responses.
//!
//! All responses share the [`common::BybitResponse<T>`] envelope.
//! Core categories (market, trade, account, position, asset) have fully
//! typed inner structs. Other categories use `BybitResponse<Value>` type
//! aliases that can be incrementally replaced with typed structs.

pub mod common;

pub mod account;
pub mod asset;
pub mod broker;
pub mod crypto_loan;
pub mod earn;
pub mod fiat;
pub mod institutional_loan;
pub mod market;
pub mod misc;
pub mod p2p;
pub mod position;
pub mod pre_upgrade;
pub mod rate_limit;
pub mod rfq;
pub mod spot_leverage;
pub mod spot_margin;
pub mod spread;
pub mod trade;
pub mod user;

// Re-export the common envelope types for convenience.
pub use common::{BybitResponse, CategorizedList, PaginatedList, SymbolList};
