//! P2P endpoint request parameters — mirrors `pybit/_v5_p2p.py`.

use serde::Serialize;
use serde_json::Value;

// get_account_information — no parameters needed.
// get_ads_list — no parameters needed.
// get_user_payment_types — no parameters needed.

#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetAdDetailsParams {
    pub item_id: String,
}

#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateAdParams {
    pub id: String,
    pub price_type: String,
    pub premium: String,
    pub price: String,
    pub min_amount: String,
    pub max_amount: String,
    pub remark: String,
    pub action_type: String,
    pub quantity: String,
    pub payment_period: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_ids: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trading_preference_set: Option<Value>,
}

#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RemoveAdParams {
    pub item_id: String,
}

#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetOrdersParams {
    pub page: String,
    pub size: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub token_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub side: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub begin_time: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<String>,
}

#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetPendingOrdersParams {
    pub page: String,
    pub size: String,
}

#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetCounterpartyInfoParams {
    pub original_uid: String,
    pub order_id: String,
}

#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetOrderDetailsParams {
    pub order_id: String,
}

#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ReleaseAssetsParams {
    pub order_id: String,
}

#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MarkAsPaidParams {
    pub order_id: String,
    pub payment_type: String,
    pub payment_id: String,
}

#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetChatMessagesParams {
    pub order_id: String,
    pub size: String,
}

#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SendChatMessageParams {
    pub message: String,
    pub content_type: String,
    pub order_id: String,
}

#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PostNewAdParams {
    pub token_id: String,
    pub currency_id: String,
    pub side: String,
    pub price_type: String,
    pub premium: String,
    pub price: String,
    pub min_amount: String,
    pub max_amount: String,
    pub remark: String,
    pub quantity: String,
    pub payment_period: String,
    pub item_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_ids: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trading_preference_set: Option<Value>,
}

#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetOnlineAdsParams {
    pub token_id: String,
    pub currency_id: String,
    pub side: String,
}

#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct QueryChatSessionListParams {
    pub size: String,
}

#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SendMessageParams {
    pub message: String,
    pub content_type: String,
    pub session_id: String,
}

#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetMessageListParams {
    pub limit: String,
    pub session_id: String,
}
