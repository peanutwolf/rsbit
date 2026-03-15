//! Pre-upgrade endpoint response types.

use serde_json::Value;

use super::common::BybitResponse;

pub type PreUpgradeOrderHistoryResponse = BybitResponse<Value>;
pub type PreUpgradeTradeHistoryResponse = BybitResponse<Value>;
pub type PreUpgradeClosedPnlResponse = BybitResponse<Value>;
pub type PreUpgradeTransactionLogResponse = BybitResponse<Value>;
pub type PreUpgradeOptionDeliveryRecordResponse = BybitResponse<Value>;
pub type PreUpgradeUsdcSessionSettlementResponse = BybitResponse<Value>;
