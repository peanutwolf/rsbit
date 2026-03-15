//! User/sub-account management HTTP endpoint methods — mirrors `pybit/_v5_user.py`.

use crate::endpoints::user;
use crate::error::Result;
use crate::http::{ApiResponse, HttpManager, Method};
use crate::params::user::*;

use super::params_to_map;

impl HttpManager {
    pub async fn create_sub_uid(&self, params: CreateSubUidParams) -> Result<ApiResponse> {
        self.submit_request(Method::Post, user::CREATE_SUB_UID, Some(params_to_map(&params)), true).await
    }

    pub async fn create_sub_api_key(&self, params: CreateSubApiKeyParams) -> Result<ApiResponse> {
        self.submit_request(Method::Post, user::CREATE_SUB_API_KEY, Some(params_to_map(&params)), true).await
    }

    pub async fn get_sub_uid_list(&self) -> Result<ApiResponse> {
        self.submit_request(Method::Get, user::GET_SUB_UID_LIST, None, true).await
    }

    pub async fn get_sub_uid_list_unlimited(&self, params: GetSubUidListUnlimitedParams) -> Result<ApiResponse> {
        self.submit_request(Method::Get, user::GET_SUB_UID_LIST_UNLIMITED, Some(params_to_map(&params)), true).await
    }

    pub async fn freeze_sub_uid(&self, params: FreezeSubUidParams) -> Result<ApiResponse> {
        self.submit_request(Method::Post, user::FREEZE_SUB_UID, Some(params_to_map(&params)), true).await
    }

    pub async fn get_api_key_information(&self) -> Result<ApiResponse> {
        self.submit_request(Method::Get, user::GET_API_KEY_INFORMATION, None, true).await
    }

    pub async fn modify_master_api_key(&self, params: ModifyMasterApiKeyParams) -> Result<ApiResponse> {
        self.submit_request(Method::Post, user::MODIFY_MASTER_API_KEY, Some(params_to_map(&params)), true).await
    }

    pub async fn modify_sub_api_key(&self, params: ModifySubApiKeyParams) -> Result<ApiResponse> {
        self.submit_request(Method::Post, user::MODIFY_SUB_API_KEY, Some(params_to_map(&params)), true).await
    }

    pub async fn delete_master_api_key(&self) -> Result<ApiResponse> {
        self.submit_request(Method::Post, user::DELETE_MASTER_API_KEY, None, true).await
    }

    pub async fn delete_sub_api_key(&self, params: DeleteSubApiKeyParams) -> Result<ApiResponse> {
        self.submit_request(Method::Post, user::DELETE_SUB_API_KEY, Some(params_to_map(&params)), true).await
    }

    pub async fn delete_sub_uid(&self, params: DeleteSubUidParams) -> Result<ApiResponse> {
        self.submit_request(Method::Post, user::DELETE_SUB_UID, Some(params_to_map(&params)), true).await
    }

    pub async fn get_all_sub_api_keys(&self, params: GetAllSubApiKeysParams) -> Result<ApiResponse> {
        self.submit_request(Method::Get, user::GET_ALL_SUB_API_KEYS, Some(params_to_map(&params)), true).await
    }

    pub async fn get_affiliate_user_info(&self, params: GetAffiliateUserInfoParams) -> Result<ApiResponse> {
        self.submit_request(Method::Get, user::GET_AFFILIATE_USER_INFO, Some(params_to_map(&params)), true).await
    }

    pub async fn get_affiliate_user_list(&self, params: GetAffiliateUserListParams) -> Result<ApiResponse> {
        self.submit_request(Method::Get, user::GET_AFFILIATE_USER_LIST, Some(params_to_map(&params)), true).await
    }

    pub async fn get_uid_wallet_type(&self, params: GetUidWalletTypeParams) -> Result<ApiResponse> {
        self.submit_request(Method::Get, user::GET_UID_WALLET_TYPE, Some(params_to_map(&params)), true).await
    }

    pub async fn get_escrow_sub_members(&self) -> Result<ApiResponse> {
        self.submit_request(Method::Get, user::GET_ESCROW_SUB_MEMBERS, None, true).await
    }
}
