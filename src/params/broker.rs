//! Broker endpoint request parameters — mirrors `pybit/_v5_broker.py`.

use serde::Serialize;

#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetBrokerEarningsParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub biz_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub begin: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uid: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
}

#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetExchangeBrokerEarningsParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub biz_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub begin: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uid: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
}

// get_exchange_broker_account_info — no parameters needed.

#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetSubaccountDepositRecordsParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sub_member_id: Option<String>,
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
pub struct GetVoucherSpecParams {
    pub id: String,
}

#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct IssueVoucherParams {
    pub account_id: String,
    pub award_id: String,
    pub spec_code: String,
    pub amount: String,
    pub broker_id: String,
}

#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetIssuedVoucherParams {
    pub account_id: String,
    pub award_id: String,
    pub spec_code: String,
}

#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetBrokerAllRateLimitsParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uids: Option<String>,
}

// get_broker_rate_limit_cap — no required parameters.

#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SetBrokerRateLimitParams {
    /// JSON array of { uids, bizType, rate } objects.
    pub list: serde_json::Value,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_broker_earnings_serialization() {
        let params = GetBrokerEarningsParams {
            biz_type: Some("SPOT".into()),
            limit: Some(20),
            ..Default::default()
        };
        let json = serde_json::to_value(&params).unwrap();
        assert_eq!(json["bizType"], "SPOT");
        assert_eq!(json["limit"], 20);
        assert!(json.get("begin").is_none());
    }
}
