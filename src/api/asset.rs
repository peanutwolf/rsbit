//! Asset management HTTP endpoint methods — mirrors `pybit/_v5_asset.py`.
//!
//! All methods require authentication (auth = true).

use crate::endpoints::asset;
use crate::error::Result;
use crate::http::{ApiResponse, HttpManager, Method};
use crate::params::asset::*;

use super::params_to_map;

impl HttpManager {
    // ── Get Coin Exchange Records ──────────────────────────────────

    /// Query the coin exchange records.
    pub async fn get_coin_exchange_records(
        &self,
        params: GetCoinExchangeRecordsParams,
    ) -> Result<ApiResponse> {
        self.submit_request(
            Method::Get,
            asset::GET_COIN_EXCHANGE_RECORDS,
            Some(params_to_map(&params)),
            true,
        )
        .await
    }

    // ── Get Option Delivery Record ─────────────────────────────────

    /// Query option delivery records, sorted by deliveryTime in descending order.
    pub async fn get_option_delivery_record(
        &self,
        params: GetOptionDeliveryRecordParams,
    ) -> Result<ApiResponse> {
        self.submit_request(
            Method::Get,
            asset::GET_OPTION_DELIVERY_RECORD,
            Some(params_to_map(&params)),
            true,
        )
        .await
    }

    // ── Get USDC Contract Settlement ───────────────────────────────

    /// Query session settlement records of USDC perpetual and futures.
    pub async fn get_usdc_contract_settlement(
        &self,
        params: GetUsdcContractSettlementParams,
    ) -> Result<ApiResponse> {
        self.submit_request(
            Method::Get,
            asset::GET_USDC_CONTRACT_SETTLEMENT,
            Some(params_to_map(&params)),
            true,
        )
        .await
    }

    // ── Get Spot Asset Info ────────────────────────────────────────

    /// Query asset information.
    pub async fn get_spot_asset_info(
        &self,
        params: GetSpotAssetInfoParams,
    ) -> Result<ApiResponse> {
        self.submit_request(
            Method::Get,
            asset::GET_SPOT_ASSET_INFO,
            Some(params_to_map(&params)),
            true,
        )
        .await
    }

    // ── Get All Coins Balance ──────────────────────────────────────

    /// Get all coin balance of all account types under the master account.
    pub async fn get_coins_balance(
        &self,
        params: GetCoinsBalanceParams,
    ) -> Result<ApiResponse> {
        self.submit_request(
            Method::Get,
            asset::GET_ALL_COINS_BALANCE,
            Some(params_to_map(&params)),
            true,
        )
        .await
    }

    // ── Get Single Coin Balance ────────────────────────────────────

    /// Query the balance of a specific coin in a specific account type.
    pub async fn get_coin_balance(
        &self,
        params: GetCoinBalanceParams,
    ) -> Result<ApiResponse> {
        self.submit_request(
            Method::Get,
            asset::GET_SINGLE_COIN_BALANCE,
            Some(params_to_map(&params)),
            true,
        )
        .await
    }

    // ── Get Transferable Coin ──────────────────────────────────────

    /// Query the transferable coin list between each account type.
    pub async fn get_transferable_coin(
        &self,
        params: GetTransferableCoinParams,
    ) -> Result<ApiResponse> {
        self.submit_request(
            Method::Get,
            asset::GET_TRANSFERABLE_COIN,
            Some(params_to_map(&params)),
            true,
        )
        .await
    }

    // ── Create Internal Transfer ───────────────────────────────────

    /// Create internal transfer between different account types under the same UID.
    pub async fn create_internal_transfer(
        &self,
        params: CreateInternalTransferParams,
    ) -> Result<ApiResponse> {
        self.submit_request(
            Method::Post,
            asset::CREATE_INTERNAL_TRANSFER,
            Some(params_to_map(&params)),
            true,
        )
        .await
    }

    // ── Get Internal Transfer Records ──────────────────────────────

    /// Query internal transfer records between different account types under the same UID.
    pub async fn get_internal_transfer_records(
        &self,
        params: GetInternalTransferRecordsParams,
    ) -> Result<ApiResponse> {
        self.submit_request(
            Method::Get,
            asset::GET_INTERNAL_TRANSFER_RECORDS,
            Some(params_to_map(&params)),
            true,
        )
        .await
    }

    // ── Get Sub UIDs ───────────────────────────────────────────────

    /// Query the sub UIDs under a main UID.
    pub async fn get_sub_uid(&self) -> Result<ApiResponse> {
        self.submit_request(
            Method::Get,
            asset::GET_SUB_UID,
            None,
            true,
        )
        .await
    }

    // ── Enable Universal Transfer for Sub UID ──────────────────────

    /// Enable universal transfer for sub UIDs (deprecated).
    pub async fn enable_universal_transfer_for_sub_uid(
        &self,
        params: EnableUniversalTransferForSubUidParams,
    ) -> Result<ApiResponse> {
        self.submit_request(
            Method::Post,
            asset::ENABLE_UT_FOR_SUB_UID,
            Some(params_to_map(&params)),
            true,
        )
        .await
    }

    // ── Create Universal Transfer ──────────────────────────────────

    /// Transfer between sub-sub or main-sub UIDs.
    pub async fn create_universal_transfer(
        &self,
        params: CreateUniversalTransferParams,
    ) -> Result<ApiResponse> {
        self.submit_request(
            Method::Post,
            asset::CREATE_UNIVERSAL_TRANSFER,
            Some(params_to_map(&params)),
            true,
        )
        .await
    }

    // ── Get Universal Transfer Records ─────────────────────────────

    /// Query universal transfer records.
    pub async fn get_universal_transfer_records(
        &self,
        params: GetUniversalTransferRecordsParams,
    ) -> Result<ApiResponse> {
        self.submit_request(
            Method::Get,
            asset::GET_UNIVERSAL_TRANSFER_RECORDS,
            Some(params_to_map(&params)),
            true,
        )
        .await
    }

    // ── Get Allowed Deposit Coin Info ──────────────────────────────

    /// Query allowed deposit coin information.
    pub async fn get_allowed_deposit_coin_info(
        &self,
        params: GetAllowedDepositCoinInfoParams,
    ) -> Result<ApiResponse> {
        self.submit_request(
            Method::Get,
            asset::GET_ALLOWED_DEPOSIT_COIN_INFO,
            Some(params_to_map(&params)),
            true,
        )
        .await
    }

    // ── Set Deposit Account ────────────────────────────────────────

    /// Set auto transfer account after deposit.
    pub async fn set_deposit_account(
        &self,
        params: SetDepositAccountParams,
    ) -> Result<ApiResponse> {
        self.submit_request(
            Method::Post,
            asset::SET_DEPOSIT_ACCOUNT,
            Some(params_to_map(&params)),
            true,
        )
        .await
    }

    // ── Get Deposit Records ────────────────────────────────────────

    /// Query deposit records.
    pub async fn get_deposit_records(
        &self,
        params: GetDepositRecordsParams,
    ) -> Result<ApiResponse> {
        self.submit_request(
            Method::Get,
            asset::GET_DEPOSIT_RECORDS,
            Some(params_to_map(&params)),
            true,
        )
        .await
    }

    // ── Get Sub Deposit Records ────────────────────────────────────

    /// Query subaccount's deposit records by main UID's API key.
    pub async fn get_sub_deposit_records(
        &self,
        params: GetSubDepositRecordsParams,
    ) -> Result<ApiResponse> {
        self.submit_request(
            Method::Get,
            asset::GET_SUB_ACCOUNT_DEPOSIT_RECORDS,
            Some(params_to_map(&params)),
            true,
        )
        .await
    }

    // ── Get Internal Deposit Records ───────────────────────────────

    /// Query deposit records within the Bybit platform (not on blockchain).
    pub async fn get_internal_deposit_records(
        &self,
        params: GetInternalDepositRecordsParams,
    ) -> Result<ApiResponse> {
        self.submit_request(
            Method::Get,
            asset::GET_INTERNAL_DEPOSIT_RECORDS,
            Some(params_to_map(&params)),
            true,
        )
        .await
    }

    // ── Get Master Deposit Address ─────────────────────────────────

    /// Query the deposit address information of the master account.
    pub async fn get_master_deposit_address(
        &self,
        params: GetMasterDepositAddressParams,
    ) -> Result<ApiResponse> {
        self.submit_request(
            Method::Get,
            asset::GET_MASTER_DEPOSIT_ADDRESS,
            Some(params_to_map(&params)),
            true,
        )
        .await
    }

    // ── Get Sub Deposit Address ────────────────────────────────────

    /// Query the deposit address information of a sub account.
    pub async fn get_sub_deposit_address(
        &self,
        params: GetSubDepositAddressParams,
    ) -> Result<ApiResponse> {
        self.submit_request(
            Method::Get,
            asset::GET_SUB_DEPOSIT_ADDRESS,
            Some(params_to_map(&params)),
            true,
        )
        .await
    }

    // ── Get Coin Info ──────────────────────────────────────────────

    /// Query coin information, including chain info, withdraw and deposit status.
    pub async fn get_coin_info(
        &self,
        params: GetCoinInfoParams,
    ) -> Result<ApiResponse> {
        self.submit_request(
            Method::Get,
            asset::GET_COIN_INFO,
            Some(params_to_map(&params)),
            true,
        )
        .await
    }

    // ── Get Withdrawal Address List ────────────────────────────────

    /// Query withdrawal address list.
    pub async fn get_withdrawal_address_list(
        &self,
        params: GetWithdrawalAddressListParams,
    ) -> Result<ApiResponse> {
        self.submit_request(
            Method::Get,
            asset::GET_WITHDRAWAL_ADDRESS_LIST,
            Some(params_to_map(&params)),
            true,
        )
        .await
    }

    // ── Get Withdrawal Records ─────────────────────────────────────

    /// Query withdrawal records.
    pub async fn get_withdrawal_records(
        &self,
        params: GetWithdrawalRecordsParams,
    ) -> Result<ApiResponse> {
        self.submit_request(
            Method::Get,
            asset::GET_WITHDRAWAL_RECORDS,
            Some(params_to_map(&params)),
            true,
        )
        .await
    }

    // ── Get Withdrawable Amount ────────────────────────────────────

    /// Get withdrawable amount.
    pub async fn get_withdrawable_amount(
        &self,
        params: GetWithdrawableAmountParams,
    ) -> Result<ApiResponse> {
        self.submit_request(
            Method::Get,
            asset::GET_WITHDRAWABLE_AMOUNT,
            Some(params_to_map(&params)),
            true,
        )
        .await
    }

    // ── Get Exchange Entity List ───────────────────────────────────

    /// Get the list of exchange entities (VASPs) supported for withdrawals.
    pub async fn get_exchange_entity_list(&self) -> Result<ApiResponse> {
        self.submit_request(
            Method::Get,
            asset::GET_EXCHANGE_ENTITY_LIST,
            None,
            true,
        )
        .await
    }

    // ── Withdraw ───────────────────────────────────────────────────

    /// Withdraw assets from your Bybit account.
    pub async fn withdraw(
        &self,
        params: WithdrawParams,
    ) -> Result<ApiResponse> {
        self.submit_request(
            Method::Post,
            asset::WITHDRAW,
            Some(params_to_map(&params)),
            true,
        )
        .await
    }

    // ── Cancel Withdrawal ──────────────────────────────────────────

    /// Cancel a withdrawal.
    pub async fn cancel_withdrawal(
        &self,
        params: CancelWithdrawalParams,
    ) -> Result<ApiResponse> {
        self.submit_request(
            Method::Post,
            asset::CANCEL_WITHDRAWAL,
            Some(params_to_map(&params)),
            true,
        )
        .await
    }

    // ── Get Convert Coin List ──────────────────────────────────────

    /// Query the list of coins you can convert to/from.
    pub async fn get_convert_coin_list(
        &self,
        params: GetConvertCoinListParams,
    ) -> Result<ApiResponse> {
        self.submit_request(
            Method::Get,
            asset::GET_CONVERT_COIN_LIST,
            Some(params_to_map(&params)),
            true,
        )
        .await
    }

    // ── Request a Quote ────────────────────────────────────────────

    /// Request a quote for coin conversion.
    pub async fn request_a_quote(
        &self,
        params: RequestAQuoteParams,
    ) -> Result<ApiResponse> {
        self.submit_request(
            Method::Post,
            asset::REQUEST_A_QUOTE,
            Some(params_to_map(&params)),
            true,
        )
        .await
    }

    // ── Confirm a Quote ────────────────────────────────────────────

    /// Confirm a coin conversion quote.
    pub async fn confirm_a_quote(
        &self,
        params: ConfirmAQuoteParams,
    ) -> Result<ApiResponse> {
        self.submit_request(
            Method::Post,
            asset::CONFIRM_A_QUOTE,
            Some(params_to_map(&params)),
            true,
        )
        .await
    }

    // ── Get Convert Status ─────────────────────────────────────────

    /// Get the status of a coin conversion.
    pub async fn get_convert_status(
        &self,
        params: GetConvertStatusParams,
    ) -> Result<ApiResponse> {
        self.submit_request(
            Method::Get,
            asset::GET_CONVERT_STATUS,
            Some(params_to_map(&params)),
            true,
        )
        .await
    }

    // ── Get Convert History ────────────────────────────────────────

    /// Get coin conversion history.
    pub async fn get_convert_history(
        &self,
        params: GetConvertHistoryParams,
    ) -> Result<ApiResponse> {
        self.submit_request(
            Method::Get,
            asset::GET_CONVERT_HISTORY,
            Some(params_to_map(&params)),
            true,
        )
        .await
    }

    // ── Get Small Balance Coins ────────────────────────────────────

    /// Get the list of small balance coins eligible for conversion.
    pub async fn get_small_balance_coins(
        &self,
        params: GetSmallBalanceCoinsParams,
    ) -> Result<ApiResponse> {
        self.submit_request(
            Method::Get,
            asset::GET_SMALL_BALANCE_COINS,
            Some(params_to_map(&params)),
            true,
        )
        .await
    }

    // ── Request a Quote (Small Balance) ────────────────────────────

    /// Request a quote for small balance conversion.
    pub async fn request_a_quote_small_balance(
        &self,
        params: RequestAQuoteSmallBalanceParams,
    ) -> Result<ApiResponse> {
        self.submit_request(
            Method::Post,
            asset::REQUEST_A_QUOTE_SMALL_BALANCE,
            Some(params_to_map(&params)),
            true,
        )
        .await
    }

    // ── Confirm a Quote (Small Balance) ────────────────────────────

    /// Confirm a small balance conversion quote.
    pub async fn confirm_a_quote_small_balance(
        &self,
        params: ConfirmAQuoteSmallBalanceParams,
    ) -> Result<ApiResponse> {
        self.submit_request(
            Method::Post,
            asset::CONFIRM_A_QUOTE_SMALL_BALANCE,
            Some(params_to_map(&params)),
            true,
        )
        .await
    }

    // ── Get Exchange History (Small Balance) ───────────────────────

    /// Get small balance conversion history.
    pub async fn get_exchange_history_small_balance(&self) -> Result<ApiResponse> {
        self.submit_request(
            Method::Get,
            asset::GET_EXCHANGE_HISTORY_SMALL_BALANCE,
            None,
            true,
        )
        .await
    }
}
