//! User endpoint paths — mirrors `pybit/user.py`.

pub const CREATE_SUB_UID: &str = "/v5/user/create-sub-member";
pub const CREATE_SUB_API_KEY: &str = "/v5/user/create-sub-api";
pub const GET_SUB_UID_LIST: &str = "/v5/user/query-sub-members";
pub const GET_SUB_UID_LIST_UNLIMITED: &str = "/v5/user/submembers";
pub const FREEZE_SUB_UID: &str = "/v5/user/frozen-sub-member";
pub const GET_API_KEY_INFORMATION: &str = "/v5/user/query-api";
pub const MODIFY_MASTER_API_KEY: &str = "/v5/user/update-api";
pub const MODIFY_SUB_API_KEY: &str = "/v5/user/update-sub-api";
pub const DELETE_MASTER_API_KEY: &str = "/v5/user/delete-api";
pub const DELETE_SUB_API_KEY: &str = "/v5/user/delete-sub-api";
pub const GET_AFFILIATE_USER_INFO: &str = "/v5/user/aff-customer-info";
pub const GET_AFFILIATE_USER_LIST: &str = "/v5/affiliate/aff-user-list";
pub const GET_UID_WALLET_TYPE: &str = "/v5/user/get-member-type";
pub const DELETE_SUB_UID: &str = "/v5/user/del-submember";
pub const GET_ALL_SUB_API_KEYS: &str = "/v5/user/sub-apikeys";
pub const GET_ESCROW_SUB_MEMBERS: &str = "/v5/user/escrow_sub_members";

#[cfg(test)]
pub(crate) const ALL: &[&str] = &[
    CREATE_SUB_UID, CREATE_SUB_API_KEY, GET_SUB_UID_LIST, GET_SUB_UID_LIST_UNLIMITED,
    FREEZE_SUB_UID, GET_API_KEY_INFORMATION, MODIFY_MASTER_API_KEY, MODIFY_SUB_API_KEY,
    DELETE_MASTER_API_KEY, DELETE_SUB_API_KEY, GET_AFFILIATE_USER_INFO,
    GET_AFFILIATE_USER_LIST, GET_UID_WALLET_TYPE, DELETE_SUB_UID, GET_ALL_SUB_API_KEYS,
    GET_ESCROW_SUB_MEMBERS,
];
