//! P2P endpoint response types.

use serde_json::Value;

use super::common::BybitResponse;

pub type P2PAccountResponse = BybitResponse<Value>;
pub type P2PAdsListResponse = BybitResponse<Value>;
pub type P2PAdDetailsResponse = BybitResponse<Value>;
pub type P2PUpdateAdResponse = BybitResponse<Value>;
pub type P2PRemoveAdResponse = BybitResponse<Value>;
pub type P2POrdersResponse = BybitResponse<Value>;
pub type P2POrderDetailsResponse = BybitResponse<Value>;
pub type P2PReleaseAssetsResponse = BybitResponse<Value>;
pub type P2PMarkAsPaidResponse = BybitResponse<Value>;
pub type P2PChatMessagesResponse = BybitResponse<Value>;
pub type P2PSendMessageResponse = BybitResponse<Value>;
pub type P2POnlineAdsResponse = BybitResponse<Value>;
pub type P2PPaymentTypesResponse = BybitResponse<Value>;
pub type P2PChatSessionListResponse = BybitResponse<Value>;
pub type P2PMessageListResponse = BybitResponse<Value>;
