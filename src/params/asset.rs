//! Asset endpoint request parameters — mirrors `pybit/_v5_asset.py`.

use serde::Serialize;

#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetCoinExchangeRecordsParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from_coin: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub to_coin: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
}

#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetOptionDeliveryRecordParams {
    pub category: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub symbol: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exp_date: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
}

#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetUsdcContractSettlementParams {
    pub category: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub symbol: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
}

#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetSpotAssetInfoParams {
    pub account_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub coin: Option<String>,
}

#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetCoinsBalanceParams {
    pub account_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub coin: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub with_bonus: Option<String>,
}

#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetCoinBalanceParams {
    pub account_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub coin: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub with_bonus: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub with_transfer_safe_amount: Option<String>,
}

#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetTransferableCoinParams {
    pub from_account_type: String,
    pub to_account_type: String,
}

#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateInternalTransferParams {
    pub transfer_id: String,
    pub coin: String,
    pub amount: String,
    pub from_account_type: String,
    pub to_account_type: String,
}

#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetInternalTransferRecordsParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transfer_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub coin: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
}

// get_sub_uid — no parameters needed.

#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct EnableUniversalTransferForSubUidParams {
    pub sub_member_ids: String,
}

#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateUniversalTransferParams {
    pub transfer_id: String,
    pub coin: String,
    pub amount: String,
    pub from_member_id: String,
    pub to_member_id: String,
    pub from_account_type: String,
    pub to_account_type: String,
}

#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetUniversalTransferRecordsParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transfer_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub coin: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
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
pub struct GetAllowedDepositCoinInfoParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub coin: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chain: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
}

#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SetDepositAccountParams {
    pub account_type: String,
}

#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetDepositRecordsParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub coin: Option<String>,
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
pub struct GetSubDepositRecordsParams {
    pub sub_member_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub coin: Option<String>,
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
pub struct GetInternalDepositRecordsParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub coin: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
}

#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetMasterDepositAddressParams {
    pub coin: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chain_type: Option<String>,
}

#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetSubDepositAddressParams {
    pub coin: String,
    pub chain_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sub_member_id: Option<String>,
}

#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetCoinInfoParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub coin: Option<String>,
}

#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetWithdrawalAddressListParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub coin: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chain: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
}

#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetWithdrawalRecordsParams {
    #[serde(skip_serializing_if = "Option::is_none", rename = "withdrawID")]
    pub withdraw_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub coin: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub withdraw_type: Option<u8>,
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
pub struct GetWithdrawableAmountParams {
    pub coin: String,
}

// get_exchange_entity_list — no parameters needed.

#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct WithdrawParams {
    pub coin: String,
    pub chain: String,
    pub address: String,
    pub tag: String,
    pub amount: String,
    pub timestamp: u64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub force_chain: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fee_type: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub beneficiary: Option<serde_json::Value>,
}

#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CancelWithdrawalParams {
    pub id: String,
}

#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetConvertCoinListParams {
    pub account_type: String,
}

#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RequestAQuoteParams {
    pub from_coin: String,
    pub to_coin: String,
    pub request_coin: String,
    pub request_amount: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub param_type: Option<String>,
}

#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ConfirmAQuoteParams {
    pub quote_tx_id: String,
}

#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetConvertStatusParams {
    pub quote_tx_id: String,
    pub account_type: String,
}

#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetConvertHistoryParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub index: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
}

#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetSmallBalanceCoinsParams {
    pub account_type: String,
}

#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RequestAQuoteSmallBalanceParams {
    pub account_type: String,
    pub from_coin_list: Vec<String>,
    pub to_coin: String,
}

#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ConfirmAQuoteSmallBalanceParams {
    pub quote_id: String,
}

// get_exchange_history_small_balance — no parameters needed.

#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetAssetOverviewParams {
    /// User ID. Required when master API key queries a sub-account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub with_bonus: Option<String>,
}

#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetFundingAccHistoryParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_time_from: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_time_to: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_withdraw_serialization() {
        let params = WithdrawParams {
            coin: "USDT".into(),
            chain: "ETH".into(),
            address: "0xabc".into(),
            tag: "".into(),
            amount: "100".into(),
            timestamp: 1700000000000,
            force_chain: Some(0),
            ..Default::default()
        };
        let json = serde_json::to_value(&params).unwrap();
        assert_eq!(json["coin"], "USDT");
        assert_eq!(json["forceChain"], 0);
        assert!(json.get("accountType").is_none());
    }

    #[test]
    fn test_withdrawal_records_withdraw_id_rename() {
        let params = GetWithdrawalRecordsParams {
            withdraw_id: Some("12345".into()),
            ..Default::default()
        };
        let json = serde_json::to_value(&params).unwrap();
        assert_eq!(json["withdrawID"], "12345");
    }
}
