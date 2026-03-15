//! User endpoint response types — mirrors `pybit/_v5_user.py` responses.

use serde::Deserialize;
use serde_json::Value;

use super::common::BybitResponse;

// ── Create Sub UID ──────────────────────────────────────────────────────

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateSubUidResult {
    pub uid: String,
    pub username: String,
    #[serde(default)]
    pub member_type: Option<u8>,
    #[serde(default)]
    pub status: Option<u8>,
    #[serde(default)]
    pub remark: Option<String>,
}

pub type CreateSubUidResponse = BybitResponse<CreateSubUidResult>;

// ── API Key ─────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ApiKeyInfo {
    #[serde(default)]
    pub id: Option<String>,
    #[serde(default)]
    pub note: Option<String>,
    #[serde(default)]
    pub apikey: Option<String>,
    #[serde(default)]
    pub read_only: Option<u8>,
    #[serde(default)]
    pub secret: Option<String>,
    #[serde(default)]
    pub permissions: Option<Value>,
    #[serde(default)]
    pub ips: Option<Vec<String>>,
    #[serde(default, rename = "type")]
    pub key_type: Option<u8>,
    #[serde(default)]
    pub created_at: Option<String>,
    #[serde(default)]
    pub expired_at: Option<String>,
    #[serde(default)]
    pub unified_margin_status: Option<u8>,
    #[serde(flatten)]
    pub extra: Value,
}

pub type CreateSubApiKeyResponse = BybitResponse<ApiKeyInfo>;
pub type GetApiKeyInformationResponse = BybitResponse<ApiKeyInfo>;
pub type ModifyMasterApiKeyResponse = BybitResponse<ApiKeyInfo>;
pub type ModifySubApiKeyResponse = BybitResponse<ApiKeyInfo>;

// ── Sub UID List ────────────────────────────────────────────────────────

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SubUidEntry {
    pub uid: String,
    pub username: String,
    #[serde(default)]
    pub member_type: Option<u8>,
    #[serde(default)]
    pub status: Option<String>,
    #[serde(default)]
    pub remark: Option<String>,
    #[serde(flatten)]
    pub extra: Value,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SubUidListResult {
    #[serde(default)]
    pub sub_members: Option<Vec<SubUidEntry>>,
}

pub type SubUidListResponse = BybitResponse<SubUidListResult>;

// ── Simple passthrough ──────────────────────────────────────────────────

pub type SubUidListUnlimitedResponse = BybitResponse<Value>;
pub type FreezeSubUidResponse = BybitResponse<Value>;
pub type DeleteMasterApiKeyResponse = BybitResponse<Value>;
pub type DeleteSubApiKeyResponse = BybitResponse<Value>;
pub type DeleteSubUidResponse = BybitResponse<Value>;
pub type GetAllSubApiKeysResponse = BybitResponse<Value>;
pub type GetAffiliateUserInfoResponse = BybitResponse<Value>;
pub type GetAffiliateUserListResponse = BybitResponse<Value>;
pub type GetUidWalletTypeResponse = BybitResponse<Value>;
pub type GetEscrowSubMembersResponse = BybitResponse<Value>;
