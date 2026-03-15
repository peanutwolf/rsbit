//! Account endpoint response types — mirrors `pybit/_v5_account.py` responses.

use serde::Deserialize;
use serde_json::Value;

use super::common::BybitResponse;

// ── Wallet Balance ──────────────────────────────────────────────────────

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CoinBalance {
    pub coin: String,
    #[serde(default)]
    pub equity: Option<String>,
    #[serde(default)]
    pub usd_value: Option<String>,
    #[serde(default)]
    pub wallet_balance: Option<String>,
    #[serde(default)]
    pub available_to_withdraw: Option<String>,
    #[serde(default)]
    pub available_to_borrow: Option<String>,
    #[serde(default)]
    pub borrow_amount: Option<String>,
    #[serde(default)]
    pub accrued_interest: Option<String>,
    #[serde(default)]
    pub total_order_im: Option<String>,
    #[serde(default)]
    pub total_position_im: Option<String>,
    #[serde(default)]
    pub total_position_mm: Option<String>,
    #[serde(default)]
    pub unrealised_pnl: Option<String>,
    #[serde(default)]
    pub cum_realised_pnl: Option<String>,
    /// Extra fields.
    #[serde(flatten)]
    pub extra: Value,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AccountBalance {
    pub account_type: String,
    #[serde(default)]
    pub account_im_rate: Option<String>,
    #[serde(default)]
    pub account_mm_rate: Option<String>,
    #[serde(default)]
    pub total_equity: Option<String>,
    #[serde(default)]
    pub total_wallet_balance: Option<String>,
    #[serde(default)]
    pub total_margin_balance: Option<String>,
    #[serde(default)]
    pub total_available_balance: Option<String>,
    #[serde(default)]
    pub total_perp_upl: Option<String>,
    #[serde(default)]
    pub total_initial_margin: Option<String>,
    #[serde(default)]
    pub total_maintenance_margin: Option<String>,
    #[serde(default)]
    pub coin: Option<Vec<CoinBalance>>,
    /// Extra fields.
    #[serde(flatten)]
    pub extra: Value,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WalletBalanceResult {
    pub list: Vec<AccountBalance>,
}

pub type WalletBalanceResponse = BybitResponse<WalletBalanceResult>;

// ── Transferable Amount ─────────────────────────────────────────────────

pub type TransferableAmountResponse = BybitResponse<Value>;

// ── Upgrade to UTA ──────────────────────────────────────────────────────

pub type UpgradeToUtaResponse = BybitResponse<Value>;

// ── Borrow History ──────────────────────────────────────────────────────

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BorrowHistoryEntry {
    #[serde(default)]
    pub currency: Option<String>,
    #[serde(default)]
    pub created_time: Option<String>,
    #[serde(default)]
    pub borrow_cost: Option<String>,
    #[serde(default)]
    pub hourly_borrow_rate: Option<String>,
    #[serde(default)]
    pub interest_bearing_borrow_size: Option<String>,
    #[serde(default)]
    pub cost_exemption: Option<String>,
    #[serde(flatten)]
    pub extra: Value,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BorrowHistoryResult {
    pub list: Vec<BorrowHistoryEntry>,
    #[serde(default)]
    pub next_page_cursor: Option<String>,
}

pub type BorrowHistoryResponse = BybitResponse<BorrowHistoryResult>;

// ── Collateral Info ─────────────────────────────────────────────────────

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CollateralInfoEntry {
    #[serde(default)]
    pub currency: Option<String>,
    #[serde(default)]
    pub hourly_borrow_rate: Option<String>,
    #[serde(default)]
    pub max_borrow_amount: Option<String>,
    #[serde(default)]
    pub free_borrow_amount: Option<String>,
    #[serde(default)]
    pub borrow_amount: Option<String>,
    #[serde(default)]
    pub available_to_borrow: Option<String>,
    #[serde(default)]
    pub borrowable: Option<bool>,
    #[serde(default)]
    pub collateral_switch: Option<bool>,
    #[serde(default)]
    pub margin_collateral: Option<bool>,
    #[serde(flatten)]
    pub extra: Value,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CollateralInfoResult {
    pub list: Vec<CollateralInfoEntry>,
}

pub type CollateralInfoResponse = BybitResponse<CollateralInfoResult>;

// ── Fee Rates ───────────────────────────────────────────────────────────

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FeeRateEntry {
    pub symbol: String,
    #[serde(default)]
    pub base_coin: Option<String>,
    #[serde(default)]
    pub taker_fee_rate: Option<String>,
    #[serde(default)]
    pub maker_fee_rate: Option<String>,
    #[serde(flatten)]
    pub extra: Value,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FeeRatesResult {
    #[serde(default)]
    pub category: Option<String>,
    pub list: Vec<FeeRateEntry>,
}

pub type FeeRatesResponse = BybitResponse<FeeRatesResult>;

// ── Account Info ────────────────────────────────────────────────────────

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AccountInfo {
    #[serde(default)]
    pub unified_margin_status: Option<u8>,
    #[serde(default)]
    pub margin_mode: Option<String>,
    #[serde(default)]
    pub dcp_status: Option<String>,
    #[serde(default)]
    pub time_window: Option<u32>,
    #[serde(default)]
    pub smp_group: Option<u32>,
    #[serde(default)]
    pub is_master_trader: Option<bool>,
    #[serde(default)]
    pub spot_hedging_status: Option<String>,
    #[serde(flatten)]
    pub extra: Value,
}

pub type AccountInfoResponse = BybitResponse<AccountInfo>;

// ── Transaction Log ─────────────────────────────────────────────────────

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TransactionLogEntry {
    #[serde(default)]
    pub symbol: Option<String>,
    #[serde(default)]
    pub category: Option<String>,
    #[serde(default)]
    pub side: Option<String>,
    #[serde(default)]
    pub transaction_time: Option<String>,
    #[serde(default, rename = "type")]
    pub log_type: Option<String>,
    #[serde(default)]
    pub qty: Option<String>,
    #[serde(default)]
    pub size: Option<String>,
    #[serde(default)]
    pub currency: Option<String>,
    #[serde(default)]
    pub trade_price: Option<String>,
    #[serde(default)]
    pub funding: Option<String>,
    #[serde(default)]
    pub fee: Option<String>,
    #[serde(default)]
    pub cash_flow: Option<String>,
    #[serde(default)]
    pub change: Option<String>,
    #[serde(default)]
    pub cash_balance: Option<String>,
    #[serde(flatten)]
    pub extra: Value,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TransactionLogResult {
    pub list: Vec<TransactionLogEntry>,
    #[serde(default)]
    pub next_page_cursor: Option<String>,
}

pub type TransactionLogResponse = BybitResponse<TransactionLogResult>;
pub type ContractTransactionLogResponse = TransactionLogResponse;

// ── MMP ─────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MmpState {
    #[serde(default)]
    pub base_coin: Option<String>,
    #[serde(default)]
    pub mmp_enabled: Option<bool>,
    #[serde(default)]
    pub window: Option<String>,
    #[serde(default)]
    pub frozen_period: Option<String>,
    #[serde(default)]
    pub qty_limit: Option<String>,
    #[serde(default)]
    pub delta_limit: Option<String>,
    #[serde(default)]
    pub mmp_frozen: Option<bool>,
    #[serde(default)]
    pub mmp_frozen_until: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MmpStateResult {
    pub result: Vec<MmpState>,
}

pub type GetMmpStateResponse = BybitResponse<Value>;
pub type SetMmpResponse = BybitResponse<Value>;
pub type ResetMmpResponse = BybitResponse<Value>;

// ── Simple passthrough responses ────────────────────────────────────────

pub type RepayLiabilityResponse = BybitResponse<Value>;
pub type SetCollateralCoinResponse = BybitResponse<Value>;
pub type BatchSetCollateralCoinResponse = BybitResponse<Value>;
pub type GetCoinGreeksResponse = BybitResponse<Value>;
pub type SetMarginModeResponse = BybitResponse<Value>;
pub type SetNoConvertRepayResponse = BybitResponse<Value>;
pub type BorrowResponse = BybitResponse<Value>;
pub type RepayResponse = BybitResponse<Value>;
pub type QueryDcpInfoResponse = BybitResponse<Value>;
pub type SetHedgingModeResponse = BybitResponse<Value>;
pub type GetSmpGroupResponse = BybitResponse<Value>;
pub type GetUserSettingConfigResponse = BybitResponse<Value>;
pub type SetLimitPriceActionResponse = BybitResponse<Value>;
pub type GetAccountInstrumentsInfoResponse = BybitResponse<Value>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_wallet_balance_deserialize() {
        let json = r#"{
            "retCode": 0, "retMsg": "OK",
            "result": {
                "list": [{
                    "accountType": "UNIFIED",
                    "totalEquity": "100000",
                    "coin": [{"coin": "USDT", "walletBalance": "50000", "equity": "50000"}]
                }]
            },
            "retExtInfo": {}, "time": 1234567890
        }"#;
        let resp: WalletBalanceResponse = serde_json::from_str(json).unwrap();
        let acct = &resp.result.list[0];
        assert_eq!(acct.account_type, "UNIFIED");
        let coins = acct.coin.as_ref().unwrap();
        assert_eq!(coins[0].coin, "USDT");
        assert_eq!(coins[0].wallet_balance.as_deref(), Some("50000"));
    }

    #[test]
    fn test_account_info_deserialize() {
        let json = r#"{
            "retCode": 0, "retMsg": "OK",
            "result": {"unifiedMarginStatus": 4, "marginMode": "REGULAR_MARGIN"},
            "retExtInfo": {}, "time": 1234567890
        }"#;
        let resp: AccountInfoResponse = serde_json::from_str(json).unwrap();
        assert_eq!(resp.result.unified_margin_status, Some(4));
        assert_eq!(resp.result.margin_mode.as_deref(), Some("REGULAR_MARGIN"));
    }
}
