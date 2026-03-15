//! Pre-upgrade HTTP endpoint methods — mirrors `pybit/_v5_pre_upgrade.py`.

use crate::endpoints::pre_upgrade;
use crate::error::Result;
use crate::http::{ApiResponse, HttpManager, Method};
use crate::params::pre_upgrade::*;

use super::params_to_map;

impl HttpManager {
    pub async fn get_pre_upgrade_order_history(&self, params: GetPreUpgradeOrderHistoryParams) -> Result<ApiResponse> {
        self.submit_request(Method::Get, pre_upgrade::GET_PRE_UPGRADE_ORDER_HISTORY, Some(params_to_map(&params)), true).await
    }

    pub async fn get_pre_upgrade_trade_history(&self, params: GetPreUpgradeTradeHistoryParams) -> Result<ApiResponse> {
        self.submit_request(Method::Get, pre_upgrade::GET_PRE_UPGRADE_TRADE_HISTORY, Some(params_to_map(&params)), true).await
    }

    pub async fn get_pre_upgrade_closed_pnl(&self, params: GetPreUpgradeClosedPnlParams) -> Result<ApiResponse> {
        self.submit_request(Method::Get, pre_upgrade::GET_PRE_UPGRADE_CLOSED_PNL, Some(params_to_map(&params)), true).await
    }

    pub async fn get_pre_upgrade_transaction_log(&self, params: GetPreUpgradeTransactionLogParams) -> Result<ApiResponse> {
        self.submit_request(Method::Get, pre_upgrade::GET_PRE_UPGRADE_TRANSACTION_LOG, Some(params_to_map(&params)), true).await
    }

    pub async fn get_pre_upgrade_option_delivery_record(&self, params: GetPreUpgradeOptionDeliveryRecordParams) -> Result<ApiResponse> {
        self.submit_request(Method::Get, pre_upgrade::GET_PRE_UPGRADE_OPTION_DELIVERY_RECORD, Some(params_to_map(&params)), true).await
    }

    pub async fn get_pre_upgrade_usdc_session_settlement(&self, params: GetPreUpgradeUsdcSessionSettlementParams) -> Result<ApiResponse> {
        self.submit_request(Method::Get, pre_upgrade::GET_PRE_UPGRADE_USDC_SESSION_SETTLEMENT, Some(params_to_map(&params)), true).await
    }
}
