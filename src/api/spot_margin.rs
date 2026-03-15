//! Spot margin trade HTTP endpoint methods — mirrors `pybit/_v5_spot_margin_trade.py`.

use crate::endpoints::spot_margin;
use crate::error::Result;
use crate::http::{ApiResponse, HttpManager, Method};
use crate::params::spot_margin::*;

use super::params_to_map;

impl HttpManager {
    // ── UTA Spot Margin ─────────────────────────────────────────────

    pub async fn spot_margin_trade_get_vip_margin_data(&self, params: GetVipMarginDataParams) -> Result<ApiResponse> {
        self.submit_request(Method::Get, spot_margin::VIP_MARGIN_DATA, Some(params_to_map(&params)), false).await
    }

    pub async fn spot_margin_trade_toggle_margin_trade(&self, params: ToggleMarginTradeParams) -> Result<ApiResponse> {
        self.submit_request(Method::Post, spot_margin::TOGGLE_MARGIN_TRADE, Some(params_to_map(&params)), true).await
    }

    pub async fn spot_margin_trade_set_leverage(&self, params: SetLeverageParams) -> Result<ApiResponse> {
        self.submit_request(Method::Post, spot_margin::SET_LEVERAGE, Some(params_to_map(&params)), true).await
    }

    pub async fn spot_margin_trade_get_status_and_leverage(&self) -> Result<ApiResponse> {
        self.submit_request(Method::Get, spot_margin::STATUS_AND_LEVERAGE, None, true).await
    }

    pub async fn spot_margin_trade_get_historical_interest_rate(&self, params: GetHistoricalInterestRateParams) -> Result<ApiResponse> {
        self.submit_request(Method::Get, spot_margin::HISTORICAL_INTEREST, Some(params_to_map(&params)), true).await
    }

    pub async fn get_tiered_collateral_ratio(&self, params: GetTieredCollateralRatioParams) -> Result<ApiResponse> {
        self.submit_request(Method::Get, spot_margin::GET_TIERED_COLLATERAL_RATIO, Some(params_to_map(&params)), false).await
    }

    pub async fn get_auto_repay_mode(&self) -> Result<ApiResponse> {
        self.submit_request(Method::Get, spot_margin::GET_AUTO_REPAY_MODE, None, true).await
    }

    pub async fn set_auto_repay_mode(&self, params: SetAutoRepayModeParams) -> Result<ApiResponse> {
        self.submit_request(Method::Post, spot_margin::SET_AUTO_REPAY_MODE, Some(params_to_map(&params)), true).await
    }

    pub async fn spot_margin_trade_get_max_borrowable(&self, params: GetMaxBorrowableParams) -> Result<ApiResponse> {
        self.submit_request(Method::Get, spot_margin::GET_MAX_BORROWABLE, Some(params_to_map(&params)), true).await
    }

    pub async fn spot_margin_trade_get_position_tiers(&self, params: GetPositionTiersParams) -> Result<ApiResponse> {
        self.submit_request(Method::Get, spot_margin::GET_POSITION_TIERS, Some(params_to_map(&params)), false).await
    }

    pub async fn spot_margin_trade_get_coin_state(&self, params: GetCoinStateParams) -> Result<ApiResponse> {
        self.submit_request(Method::Get, spot_margin::GET_COIN_STATE, Some(params_to_map(&params)), false).await
    }

    pub async fn spot_margin_trade_get_repayment_available_amount(&self, params: GetRepaymentAvailableAmountParams) -> Result<ApiResponse> {
        self.submit_request(Method::Get, spot_margin::GET_REPAYMENT_AVAILABLE_AMOUNT, Some(params_to_map(&params)), true).await
    }

    // ── Normal (non-UTA) Spot Margin ────────────────────────────────

    pub async fn spot_margin_trade_normal_get_vip_margin_data(&self, params: GetNormalVipMarginDataParams) -> Result<ApiResponse> {
        self.submit_request(Method::Get, spot_margin::NORMAL_GET_VIP_MARGIN_DATA, Some(params_to_map(&params)), false).await
    }

    pub async fn spot_margin_trade_normal_get_margin_coin_info(&self, params: GetNormalMarginCoinInfoParams) -> Result<ApiResponse> {
        self.submit_request(Method::Get, spot_margin::NORMAL_GET_MARGIN_COIN_INFO, Some(params_to_map(&params)), false).await
    }

    pub async fn spot_margin_trade_normal_get_borrowable_coin_info(&self, params: GetNormalBorrowableCoinInfoParams) -> Result<ApiResponse> {
        self.submit_request(Method::Get, spot_margin::NORMAL_GET_BORROWABLE_COIN_INFO, Some(params_to_map(&params)), false).await
    }

    pub async fn spot_margin_trade_normal_get_interest_quota(&self, params: GetNormalInterestQuotaParams) -> Result<ApiResponse> {
        self.submit_request(Method::Get, spot_margin::NORMAL_GET_INTEREST_QUOTA, Some(params_to_map(&params)), true).await
    }

    pub async fn spot_margin_trade_normal_get_loan_account_info(&self) -> Result<ApiResponse> {
        self.submit_request(Method::Get, spot_margin::NORMAL_GET_LOAN_ACCOUNT_INFO, None, true).await
    }

    pub async fn spot_margin_trade_normal_borrow(&self, params: NormalBorrowParams) -> Result<ApiResponse> {
        self.submit_request(Method::Post, spot_margin::NORMAL_BORROW, Some(params_to_map(&params)), true).await
    }

    pub async fn spot_margin_trade_normal_repay(&self, params: NormalRepayParams) -> Result<ApiResponse> {
        self.submit_request(Method::Post, spot_margin::NORMAL_REPAY, Some(params_to_map(&params)), true).await
    }

    pub async fn spot_margin_trade_normal_get_borrow_order_detail(&self, params: GetNormalBorrowOrderDetailParams) -> Result<ApiResponse> {
        self.submit_request(Method::Get, spot_margin::NORMAL_GET_BORROW_ORDER_DETAIL, Some(params_to_map(&params)), true).await
    }

    pub async fn spot_margin_trade_normal_get_repayment_order_detail(&self, params: GetNormalRepaymentOrderDetailParams) -> Result<ApiResponse> {
        self.submit_request(Method::Get, spot_margin::NORMAL_GET_REPAYMENT_ORDER_DETAIL, Some(params_to_map(&params)), true).await
    }

    pub async fn spot_margin_trade_normal_toggle_margin_trade(&self, params: NormalToggleMarginTradeParams) -> Result<ApiResponse> {
        self.submit_request(Method::Post, spot_margin::NORMAL_TOGGLE_MARGIN_TRADE, Some(params_to_map(&params)), true).await
    }
}
