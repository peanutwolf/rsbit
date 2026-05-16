//! Account endpoint request parameters — mirrors `pybit/_v5_account.py`.

use serde::Serialize;
use serde_json::Value;

#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetWalletBalanceParams {
    pub account_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub coin: Option<String>,
}

#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetTransferableAmountParams {
    pub coin_name: String,
}

// upgrade_to_unified_trading_account — no parameters needed.

#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetBorrowHistoryParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
}

#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RepayLiabilityParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub coin: Option<String>,
}

#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetCollateralInfoParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,
}

#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SetCollateralCoinParams {
    pub coin: String,
    pub collateral_switch: String,
}

#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BatchSetCollateralCoinParams {
    pub request: Vec<Value>,
}

#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetCoinGreeksParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub base_coin: Option<String>,
}

#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetFeeRatesParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub symbol: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub base_coin: Option<String>,
}

// get_account_info — no parameters needed.

#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetTransactionLogParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub base_coin: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "type")]
    pub log_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
}

#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetContractTransactionLogParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub base_coin: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "type")]
    pub log_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
}

#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SetMarginModeParams {
    pub set_margin_mode: String,
}

#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SetMmpParams {
    pub base_coin: String,
    pub window: String,
    pub frozen_period: String,
    pub qty_limit: String,
    pub delta_limit: String,
}

#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ResetMmpParams {
    pub base_coin: String,
}

#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetMmpStateParams {
    pub base_coin: String,
}

#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ManualNoConvertRepayParams {
    pub coin: String,
    pub amount: String,
}

#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BorrowParams {
    pub coin: String,
    pub qty: String,
}

// get_instruments_info (account) — no parameters needed.

#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RepayParams {
    pub coin: String,
    pub qty: String,
}

// query_dcp_info — no parameters needed.

#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SetHedgingModeParams {
    pub hedging_mode: String,
}

// get_smp_group — no parameters needed.
// get_user_setting_config — no parameters needed.

#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SetLimitPriceActionParams {
    pub category: String,
    pub modify_enable: String,
}

#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SetDeltaModeParams {
    /// "1" to enable, "0" to disable delta neutral mode.
    pub delta_enable: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_wallet_balance_serialization() {
        let params = GetWalletBalanceParams {
            account_type: "UNIFIED".into(),
            coin: Some("BTC".into()),
        };
        let json = serde_json::to_value(&params).unwrap();
        assert_eq!(json["accountType"], "UNIFIED");
        assert_eq!(json["coin"], "BTC");
    }

    #[test]
    fn test_transaction_log_type_rename() {
        let params = GetTransactionLogParams {
            log_type: Some("TRANSFER_IN".into()),
            ..Default::default()
        };
        let json = serde_json::to_value(&params).unwrap();
        assert_eq!(json["type"], "TRANSFER_IN");
        assert!(json.get("logType").is_none());
    }
}
