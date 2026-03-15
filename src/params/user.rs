//! User endpoint request parameters — mirrors `pybit/_v5_user.py`.

use serde::Serialize;
use serde_json::Value;

#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateSubUidParams {
    pub username: String,
    pub member_type: u8,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub switch: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_uta: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub note: Option<String>,
}

#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateSubApiKeyParams {
    pub subuid: u64,
    pub read_only: u8,
    pub permissions: Value,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub note: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ips: Option<String>,
}

// get_sub_uid_list — no parameters needed.

#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetSubUidListUnlimitedParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_cursor: Option<String>,
}

#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FreezeSubUidParams {
    pub subuid: u64,
    pub frozen: u8,
}

// get_api_key_information — no parameters needed.

#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ModifyMasterApiKeyParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub read_only: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ips: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permissions: Option<Value>,
}

#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ModifySubApiKeyParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub read_only: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ips: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permissions: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub apikey: Option<String>,
}

// delete_master_api_key — no parameters needed.

#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DeleteSubApiKeyParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub apikey: Option<String>,
}

#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DeleteSubUidParams {
    pub sub_member_id: String,
}

#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetAllSubApiKeysParams {
    pub sub_member_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
}

#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetAffiliateUserInfoParams {
    pub uid: String,
}

#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetAffiliateUserListParams {
    pub uid: String,
}

#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetUidWalletTypeParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member_ids: Option<String>,
}

// get_escrow_sub_members — no parameters needed.

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_sub_uid_serialization() {
        let params = CreateSubUidParams {
            username: "testuser".into(),
            member_type: 1,
            is_uta: Some(true),
            ..Default::default()
        };
        let json = serde_json::to_value(&params).unwrap();
        assert_eq!(json["username"], "testuser");
        assert_eq!(json["memberType"], 1);
        assert_eq!(json["isUta"], true);
        assert!(json.get("switch").is_none());
    }
}
