//! Account management HTTP endpoint methods — mirrors `pybit/_v5_account.py`.
//!
//! All methods require authentication (auth = true).

use crate::endpoints::account;
use crate::error::Result;
use crate::http::{ApiResponse, HttpManager, Method};
use crate::params::account::*;

use super::params_to_map;

impl HttpManager {
    // ── Get Wallet Balance ─────────────────────────────────────────

    /// Obtain wallet balance, query asset information of each currency,
    /// and account risk rate information.
    pub async fn get_wallet_balance(
        &self,
        params: GetWalletBalanceParams,
    ) -> Result<ApiResponse> {
        self.submit_request(
            Method::Get,
            account::GET_WALLET_BALANCE,
            Some(params_to_map(&params)),
            true,
        )
        .await
    }

    // ── Get Transferable Amount ────────────────────────────────────

    /// Query the available amount to transfer of a specific coin in the Unified wallet.
    pub async fn get_transferable_amount(
        &self,
        params: GetTransferableAmountParams,
    ) -> Result<ApiResponse> {
        self.submit_request(
            Method::Get,
            account::GET_TRANSFERABLE_AMOUNT,
            Some(params_to_map(&params)),
            true,
        )
        .await
    }

    // ── Upgrade to Unified Trading Account ────────────────────────

    /// Upgrade to Unified Trading Account.
    pub async fn upgrade_to_unified_trading_account(&self) -> Result<ApiResponse> {
        self.submit_request(
            Method::Post,
            account::UPGRADE_TO_UNIFIED_ACCOUNT,
            None,
            true,
        )
        .await
    }

    // ── Get Borrow History ────────────────────────────────────────

    /// Get interest records, sorted in reverse order of creation time.
    pub async fn get_borrow_history(
        &self,
        params: GetBorrowHistoryParams,
    ) -> Result<ApiResponse> {
        self.submit_request(
            Method::Get,
            account::GET_BORROW_HISTORY,
            Some(params_to_map(&params)),
            true,
        )
        .await
    }

    // ── Repay Liability ───────────────────────────────────────────

    /// Manually repay the liabilities of the Unified account.
    pub async fn repay_liability(
        &self,
        params: RepayLiabilityParams,
    ) -> Result<ApiResponse> {
        self.submit_request(
            Method::Post,
            account::REPAY_LIABILITY,
            Some(params_to_map(&params)),
            true,
        )
        .await
    }

    // ── Get Collateral Info ───────────────────────────────────────

    /// Get the collateral information of the current unified margin account.
    pub async fn get_collateral_info(
        &self,
        params: GetCollateralInfoParams,
    ) -> Result<ApiResponse> {
        self.submit_request(
            Method::Get,
            account::GET_COLLATERAL_INFO,
            Some(params_to_map(&params)),
            true,
        )
        .await
    }

    // ── Set Collateral Coin ───────────────────────────────────────

    /// Decide whether assets in the Unified account need to be collateral coins.
    pub async fn set_collateral_coin(
        &self,
        params: SetCollateralCoinParams,
    ) -> Result<ApiResponse> {
        self.submit_request(
            Method::Post,
            account::SET_COLLATERAL_COIN,
            Some(params_to_map(&params)),
            true,
        )
        .await
    }

    // ── Batch Set Collateral Coin ─────────────────────────────────

    /// Batch set collateral coins for the Unified account.
    pub async fn batch_set_collateral_coin(
        &self,
        params: BatchSetCollateralCoinParams,
    ) -> Result<ApiResponse> {
        self.submit_request(
            Method::Post,
            account::BATCH_SET_COLLATERAL_COIN,
            Some(params_to_map(&params)),
            true,
        )
        .await
    }

    // ── Get Coin Greeks ───────────────────────────────────────────

    /// Get current account Greeks information.
    pub async fn get_coin_greeks(
        &self,
        params: GetCoinGreeksParams,
    ) -> Result<ApiResponse> {
        self.submit_request(
            Method::Get,
            account::GET_COIN_GREEKS,
            Some(params_to_map(&params)),
            true,
        )
        .await
    }

    // ── Get Fee Rates ─────────────────────────────────────────────

    /// Get the trading fee rate.
    pub async fn get_fee_rates(
        &self,
        params: GetFeeRatesParams,
    ) -> Result<ApiResponse> {
        self.submit_request(
            Method::Get,
            account::GET_FEE_RATE,
            Some(params_to_map(&params)),
            true,
        )
        .await
    }

    // ── Get Account Info ──────────────────────────────────────────

    /// Query the margin mode configuration of the account.
    pub async fn get_account_info(&self) -> Result<ApiResponse> {
        self.submit_request(
            Method::Get,
            account::GET_ACCOUNT_INFO,
            None,
            true,
        )
        .await
    }

    // ── Get Transaction Log ───────────────────────────────────────

    /// Query transaction logs in Unified account.
    pub async fn get_transaction_log(
        &self,
        params: GetTransactionLogParams,
    ) -> Result<ApiResponse> {
        self.submit_request(
            Method::Get,
            account::GET_TRANSACTION_LOG,
            Some(params_to_map(&params)),
            true,
        )
        .await
    }

    // ── Get Contract Transaction Log ──────────────────────────────

    /// Query transaction logs in Classic account.
    pub async fn get_contract_transaction_log(
        &self,
        params: GetContractTransactionLogParams,
    ) -> Result<ApiResponse> {
        self.submit_request(
            Method::Get,
            account::GET_CONTRACT_TRANSACTION_LOG,
            Some(params_to_map(&params)),
            true,
        )
        .await
    }

    // ── Set Margin Mode ───────────────────────────────────────────

    /// Set margin mode (REGULAR_MARGIN or PORTFOLIO_MARGIN).
    pub async fn set_margin_mode(
        &self,
        params: SetMarginModeParams,
    ) -> Result<ApiResponse> {
        self.submit_request(
            Method::Post,
            account::SET_MARGIN_MODE,
            Some(params_to_map(&params)),
            true,
        )
        .await
    }

    // ── Set MMP ───────────────────────────────────────────────────

    /// Set Market Maker Protection (MMP) configuration.
    pub async fn set_mmp(&self, params: SetMmpParams) -> Result<ApiResponse> {
        self.submit_request(
            Method::Post,
            account::SET_MMP,
            Some(params_to_map(&params)),
            true,
        )
        .await
    }

    // ── Reset MMP ─────────────────────────────────────────────────

    /// Unfreeze the account after MMP is triggered.
    pub async fn reset_mmp(&self, params: ResetMmpParams) -> Result<ApiResponse> {
        self.submit_request(
            Method::Post,
            account::RESET_MMP,
            Some(params_to_map(&params)),
            true,
        )
        .await
    }

    // ── Get MMP State ─────────────────────────────────────────────

    /// Get MMP state.
    pub async fn get_mmp_state(
        &self,
        params: GetMmpStateParams,
    ) -> Result<ApiResponse> {
        self.submit_request(
            Method::Get,
            account::GET_MMP_STATE,
            Some(params_to_map(&params)),
            true,
        )
        .await
    }

    // ── Set No Convert Repay ──────────────────────────────────────

    /// Turn off auto repayment using liability (UTA 2.0 only).
    pub async fn set_no_convert_repay(
        &self,
        params: SetNoConvertRepayParams,
    ) -> Result<ApiResponse> {
        self.submit_request(
            Method::Post,
            account::SET_NO_CONVERT_REPAY,
            Some(params_to_map(&params)),
            true,
        )
        .await
    }

    // ── Borrow ────────────────────────────────────────────────────

    /// Borrow a certain amount of a coin.
    pub async fn borrow(&self, params: BorrowParams) -> Result<ApiResponse> {
        self.submit_request(
            Method::Post,
            account::BORROW,
            Some(params_to_map(&params)),
            true,
        )
        .await
    }

    // ── Get Instruments Info (Account) ────────────────────────────

    /// Get available instruments info for unified account.
    pub async fn get_account_instruments_info(&self) -> Result<ApiResponse> {
        self.submit_request(
            Method::Get,
            account::GET_INSTRUMENTS_INFO,
            None,
            true,
        )
        .await
    }

    // ── Repay ─────────────────────────────────────────────────────

    /// Repay a certain amount of a coin.
    pub async fn repay(&self, params: RepayParams) -> Result<ApiResponse> {
        self.submit_request(
            Method::Post,
            account::REPAY,
            Some(params_to_map(&params)),
            true,
        )
        .await
    }

    // ── Query DCP Info ────────────────────────────────────────────

    /// Query the DCP (Disconnect Cancel All) configuration of the account.
    pub async fn query_dcp_info(&self) -> Result<ApiResponse> {
        self.submit_request(
            Method::Get,
            account::QUERY_DCP_INFO,
            None,
            true,
        )
        .await
    }

    // ── Set Hedging Mode ──────────────────────────────────────────

    /// Set hedging mode for the account.
    pub async fn set_hedging_mode(
        &self,
        params: SetHedgingModeParams,
    ) -> Result<ApiResponse> {
        self.submit_request(
            Method::Post,
            account::SET_HEDGING_MODE,
            Some(params_to_map(&params)),
            true,
        )
        .await
    }

    // ── Get SMP Group ─────────────────────────────────────────────

    /// Get the SMP group ID of the account.
    pub async fn get_smp_group(&self) -> Result<ApiResponse> {
        self.submit_request(
            Method::Get,
            account::GET_SMP_GROUP,
            None,
            true,
        )
        .await
    }

    // ── Get User Setting Config ───────────────────────────────────

    /// Get user setting configuration.
    pub async fn get_user_setting_config(&self) -> Result<ApiResponse> {
        self.submit_request(
            Method::Get,
            account::GET_USER_SETTING_CONFIG,
            None,
            true,
        )
        .await
    }

    // ── Set Limit Price Action ────────────────────────────────────

    /// Set the price limit action behavior.
    pub async fn set_limit_price_action(
        &self,
        params: SetLimitPriceActionParams,
    ) -> Result<ApiResponse> {
        self.submit_request(
            Method::Post,
            account::SET_LIMIT_PRICE_ACTION,
            Some(params_to_map(&params)),
            true,
        )
        .await
    }
}
