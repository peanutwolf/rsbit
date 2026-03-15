//! Spot leverage token endpoint request parameters — mirrors `pybit/_v5_spot_leverage_token.py`.

use serde::Serialize;

#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetLeveragedTokenInfoParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lt_coin: Option<String>,
}

#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetLeveragedTokenMarketParams {
    pub lt_coin: String,
}

#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PurchaseLeveragedTokenParams {
    pub lt_coin: String,
    pub lt_amount: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub serial_no: Option<String>,
}

#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RedeemLeveragedTokenParams {
    pub lt_coin: String,
    pub quantity: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub serial_no: Option<String>,
}

#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetPurchaseRedemptionRecordsParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lt_coin: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lt_order_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub serial_no: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_purchase_leveraged_token() {
        let params = PurchaseLeveragedTokenParams {
            lt_coin: "BTC3L".into(),
            lt_amount: "100".into(),
            serial_no: None,
        };
        let json = serde_json::to_value(&params).unwrap();
        assert_eq!(json["ltCoin"], "BTC3L");
        assert_eq!(json["ltAmount"], "100");
    }
}
