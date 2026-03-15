//! Spot leverage token endpoint response types.

use serde::Deserialize;
use serde_json::Value;

use super::common::BybitResponse;

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LeveragedTokenInfo {
    pub lt_coin: String,
    #[serde(default)]
    pub lt_name: Option<String>,
    #[serde(default)]
    pub max_purchase: Option<String>,
    #[serde(default)]
    pub min_purchase: Option<String>,
    #[serde(default)]
    pub max_purchase_daily: Option<String>,
    #[serde(default)]
    pub max_redeem: Option<String>,
    #[serde(default)]
    pub min_redeem: Option<String>,
    #[serde(default)]
    pub max_redeem_daily: Option<String>,
    #[serde(default)]
    pub purchase_fee_rate: Option<String>,
    #[serde(default)]
    pub redeem_fee_rate: Option<String>,
    #[serde(default)]
    pub status: Option<String>,
    #[serde(default)]
    pub fund_fee: Option<String>,
    #[serde(default)]
    pub fund_fee_time: Option<u64>,
    #[serde(default)]
    pub management_fee_rate: Option<String>,
    #[serde(default)]
    pub net_value: Option<String>,
    #[serde(default)]
    pub total_amount: Option<String>,
    #[serde(flatten)]
    pub extra: Value,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LeveragedTokenInfoResult {
    pub list: Vec<LeveragedTokenInfo>,
}

pub type LeveragedTokenInfoResponse = BybitResponse<LeveragedTokenInfoResult>;
pub type LeveragedTokenMarketResponse = BybitResponse<Value>;
pub type PurchaseLeveragedTokenResponse = BybitResponse<Value>;
pub type RedeemLeveragedTokenResponse = BybitResponse<Value>;
pub type PurchaseRedemptionRecordsResponse = BybitResponse<Value>;
