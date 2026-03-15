//! Asset endpoint response types — mirrors `pybit/_v5_asset.py` responses.

use serde::Deserialize;
use serde_json::Value;

use super::common::{BybitResponse, PaginatedList};

// ── Coin Exchange Records ───────────────────────────────────────────────

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CoinExchangeRecord {
    #[serde(default)]
    pub from_coin: Option<String>,
    #[serde(default)]
    pub from_amount: Option<String>,
    #[serde(default)]
    pub to_coin: Option<String>,
    #[serde(default)]
    pub to_amount: Option<String>,
    #[serde(default)]
    pub exchange_rate: Option<String>,
    #[serde(default)]
    pub created_time: Option<String>,
    #[serde(default)]
    pub exchange_tx_id: Option<String>,
    #[serde(flatten)]
    pub extra: Value,
}

pub type CoinExchangeRecordsResponse = BybitResponse<PaginatedList<CoinExchangeRecord>>;

// ── Coins Balance ───────────────────────────────────────────────────────

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AssetCoinBalance {
    pub coin: String,
    #[serde(default)]
    pub wallet_balance: Option<String>,
    #[serde(default)]
    pub transfer_balance: Option<String>,
    #[serde(default)]
    pub bonus: Option<String>,
    #[serde(flatten)]
    pub extra: Value,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CoinsBalanceResult {
    #[serde(default)]
    pub account_type: Option<String>,
    #[serde(default)]
    pub member_id: Option<String>,
    pub balance: Vec<AssetCoinBalance>,
}

pub type CoinsBalanceResponse = BybitResponse<CoinsBalanceResult>;
pub type CoinBalanceResponse = BybitResponse<Value>;

// ── Transfer ────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TransferResult {
    pub transfer_id: String,
    #[serde(default)]
    pub status: Option<String>,
}

pub type CreateInternalTransferResponse = BybitResponse<TransferResult>;
pub type CreateUniversalTransferResponse = BybitResponse<TransferResult>;

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TransferRecord {
    #[serde(default)]
    pub transfer_id: Option<String>,
    #[serde(default)]
    pub coin: Option<String>,
    #[serde(default)]
    pub amount: Option<String>,
    #[serde(default)]
    pub status: Option<String>,
    #[serde(default)]
    pub timestamp: Option<String>,
    #[serde(flatten)]
    pub extra: Value,
}

pub type InternalTransferRecordsResponse = BybitResponse<PaginatedList<TransferRecord>>;
pub type UniversalTransferRecordsResponse = BybitResponse<PaginatedList<TransferRecord>>;

// ── Deposit ─────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DepositRecord {
    #[serde(default)]
    pub coin: Option<String>,
    #[serde(default)]
    pub chain: Option<String>,
    #[serde(default)]
    pub amount: Option<String>,
    #[serde(default)]
    pub tx_id: Option<String>,
    #[serde(default)]
    pub status: Option<u8>,
    #[serde(default)]
    pub to_address: Option<String>,
    #[serde(default)]
    pub tag: Option<String>,
    #[serde(default)]
    pub deposit_fee: Option<String>,
    #[serde(default)]
    pub success_at: Option<String>,
    #[serde(default)]
    pub confirmations: Option<String>,
    #[serde(default)]
    pub tx_index: Option<String>,
    #[serde(default)]
    pub block_hash: Option<String>,
    #[serde(flatten)]
    pub extra: Value,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DepositRecordsResult {
    pub rows: Vec<DepositRecord>,
    #[serde(default)]
    pub next_page_cursor: Option<String>,
}

pub type DepositRecordsResponse = BybitResponse<DepositRecordsResult>;
pub type SubDepositRecordsResponse = BybitResponse<DepositRecordsResult>;
pub type InternalDepositRecordsResponse = BybitResponse<DepositRecordsResult>;

// ── Deposit Address ─────────────────────────────────────────────────────

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DepositAddressChain {
    #[serde(default)]
    pub chain_type: Option<String>,
    #[serde(default)]
    pub address_deposit: Option<String>,
    #[serde(default)]
    pub tag_deposit: Option<String>,
    #[serde(default)]
    pub chain: Option<String>,
    #[serde(flatten)]
    pub extra: Value,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DepositAddress {
    pub coin: String,
    pub chains: Vec<DepositAddressChain>,
}

pub type MasterDepositAddressResponse = BybitResponse<DepositAddress>;
pub type SubDepositAddressResponse = BybitResponse<DepositAddress>;

// ── Withdrawal ──────────────────────────────────────────────────────────

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WithdrawalRecord {
    #[serde(default)]
    pub coin: Option<String>,
    #[serde(default)]
    pub chain: Option<String>,
    #[serde(default)]
    pub amount: Option<String>,
    #[serde(default)]
    pub tx_id: Option<String>,
    #[serde(default)]
    pub status: Option<String>,
    #[serde(default)]
    pub to_address: Option<String>,
    #[serde(default)]
    pub tag: Option<String>,
    #[serde(default)]
    pub withdraw_fee: Option<String>,
    #[serde(default)]
    pub create_time: Option<String>,
    #[serde(default)]
    pub update_time: Option<String>,
    #[serde(default, rename = "withdrawId")]
    pub withdraw_id: Option<String>,
    #[serde(flatten)]
    pub extra: Value,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WithdrawalRecordsResult {
    pub rows: Vec<WithdrawalRecord>,
    #[serde(default)]
    pub next_page_cursor: Option<String>,
}

pub type WithdrawalRecordsResponse = BybitResponse<WithdrawalRecordsResult>;

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WithdrawResult {
    pub id: String,
}

pub type WithdrawResponse = BybitResponse<WithdrawResult>;
pub type CancelWithdrawalResponse = BybitResponse<Value>;

// ── Coin Info ───────────────────────────────────────────────────────────

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CoinChainInfo {
    #[serde(default)]
    pub chain: Option<String>,
    #[serde(default)]
    pub chain_type: Option<String>,
    #[serde(default)]
    pub confirmation: Option<String>,
    #[serde(default)]
    pub withdraw_fee: Option<String>,
    #[serde(default)]
    pub deposit_min: Option<String>,
    #[serde(default)]
    pub withdraw_min: Option<String>,
    #[serde(default)]
    pub min_accuracy: Option<String>,
    #[serde(default)]
    pub chain_deposit: Option<String>,
    #[serde(default)]
    pub chain_withdraw: Option<String>,
    #[serde(flatten)]
    pub extra: Value,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CoinInfo {
    pub coin: String,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub remain_amount: Option<String>,
    pub chains: Vec<CoinChainInfo>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CoinInfoResult {
    pub rows: Vec<CoinInfo>,
}

pub type CoinInfoResponse = BybitResponse<CoinInfoResult>;

// ── Convert ─────────────────────────────────────────────────────────────

pub type ConvertCoinListResponse = BybitResponse<Value>;
pub type RequestAQuoteResponse = BybitResponse<Value>;
pub type ConfirmAQuoteResponse = BybitResponse<Value>;
pub type ConvertStatusResponse = BybitResponse<Value>;
pub type ConvertHistoryResponse = BybitResponse<Value>;

// ── Small Balance ───────────────────────────────────────────────────────

pub type SmallBalanceCoinsResponse = BybitResponse<Value>;
pub type RequestQuoteSmallBalanceResponse = BybitResponse<Value>;
pub type ConfirmQuoteSmallBalanceResponse = BybitResponse<Value>;
pub type ExchangeHistorySmallBalanceResponse = BybitResponse<Value>;

// ── Remaining simple types ──────────────────────────────────────────────

pub type SpotAssetInfoResponse = BybitResponse<Value>;
pub type TransferableCoinResponse = BybitResponse<Value>;
pub type SubUidResponse = BybitResponse<Value>;
pub type EnableUniversalTransferResponse = BybitResponse<Value>;
pub type AllowedDepositCoinInfoResponse = BybitResponse<Value>;
pub type SetDepositAccountResponse = BybitResponse<Value>;
pub type WithdrawableAmountResponse = BybitResponse<Value>;
pub type WithdrawalAddressListResponse = BybitResponse<Value>;
pub type ExchangeEntityListResponse = BybitResponse<Value>;
pub type OptionDeliveryRecordResponse = BybitResponse<Value>;
pub type UsdcContractSettlementResponse = BybitResponse<Value>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_coins_balance_deserialize() {
        let json = r#"{
            "retCode": 0, "retMsg": "OK",
            "result": {
                "accountType": "FUND",
                "memberId": "123",
                "balance": [
                    {"coin": "USDT", "walletBalance": "1000", "transferBalance": "900"}
                ]
            },
            "retExtInfo": {}, "time": 1234567890
        }"#;
        let resp: CoinsBalanceResponse = serde_json::from_str(json).unwrap();
        assert_eq!(resp.result.account_type.as_deref(), Some("FUND"));
        assert_eq!(resp.result.balance[0].coin, "USDT");
    }

    #[test]
    fn test_transfer_result_deserialize() {
        let json = r#"{
            "retCode": 0, "retMsg": "OK",
            "result": {"transferId": "tx-abc-123", "status": "SUCCESS"},
            "retExtInfo": {}, "time": 1234567890
        }"#;
        let resp: CreateInternalTransferResponse = serde_json::from_str(json).unwrap();
        assert_eq!(resp.result.transfer_id, "tx-abc-123");
    }

    #[test]
    fn test_withdraw_result_deserialize() {
        let json = r#"{
            "retCode": 0, "retMsg": "OK",
            "result": {"id": "w-123"},
            "retExtInfo": {}, "time": 1234567890
        }"#;
        let resp: WithdrawResponse = serde_json::from_str(json).unwrap();
        assert_eq!(resp.result.id, "w-123");
    }
}
