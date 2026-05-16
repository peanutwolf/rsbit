//! P2P HTTP endpoint methods — mirrors `pybit/_v5_p2p.py`.
//!
//! All endpoints are authenticated POST requests.
//! Note: `upload_chat_file` is not implemented — it requires multipart
//! form upload which is not yet supported by `submit_request`.

use crate::endpoints::p2p;
use crate::error::Result;
use crate::http::{ApiResponse, HttpManager, Method};
use crate::params::p2p::*;

use super::params_to_map;

impl HttpManager {
    /// Get P2P account information.
    /// <https://bybit-exchange.github.io/docs/p2p/user/acct-info>
    pub async fn get_p2p_account_information(&self) -> Result<ApiResponse> {
        self.submit_request(Method::Post, p2p::GET_ACCOUNT_INFORMATION, None, true).await
    }

    /// Get your P2P ads list.
    /// <https://bybit-exchange.github.io/docs/p2p/ad/ad-list>
    pub async fn get_p2p_ads_list(&self) -> Result<ApiResponse> {
        self.submit_request(Method::Post, p2p::GET_ADS_LIST, None, true).await
    }

    /// Get your P2P ad details.
    /// <https://bybit-exchange.github.io/docs/p2p/ad/ad-detail>
    pub async fn get_p2p_ad_details(&self, params: GetAdDetailsParams) -> Result<ApiResponse> {
        self.submit_request(Method::Post, p2p::GET_AD_DETAILS, Some(params_to_map(&params)), true).await
    }

    /// Update or relist a P2P ad.
    /// <https://bybit-exchange.github.io/docs/p2p/ad/update-list-ad>
    pub async fn update_p2p_ad(&self, params: UpdateAdParams) -> Result<ApiResponse> {
        self.submit_request(Method::Post, p2p::UPDATE_AD, Some(params_to_map(&params)), true).await
    }

    /// Remove a P2P ad.
    /// <https://bybit-exchange.github.io/docs/p2p/ad/remove-ad>
    pub async fn remove_p2p_ad(&self, params: RemoveAdParams) -> Result<ApiResponse> {
        self.submit_request(Method::Post, p2p::REMOVE_AD, Some(params_to_map(&params)), true).await
    }

    /// Post a new P2P ad.
    /// <https://bybit-exchange.github.io/docs/p2p/ad/post-new-ad>
    pub async fn post_new_p2p_ad(&self, params: PostNewAdParams) -> Result<ApiResponse> {
        self.submit_request(Method::Post, p2p::POST_NEW_AD, Some(params_to_map(&params)), true).await
    }

    /// Get online P2P ads.
    /// <https://bybit-exchange.github.io/docs/p2p/ad/online-ad-list>
    pub async fn get_p2p_online_ads(&self, params: GetOnlineAdsParams) -> Result<ApiResponse> {
        self.submit_request(Method::Post, p2p::GET_ONLINE_ADS, Some(params_to_map(&params)), true).await
    }

    /// Get all P2P orders.
    /// <https://bybit-exchange.github.io/docs/p2p/order/order-list>
    pub async fn get_p2p_orders(&self, params: GetOrdersParams) -> Result<ApiResponse> {
        self.submit_request(Method::Post, p2p::GET_ORDERS, Some(params_to_map(&params)), true).await
    }

    /// Get pending P2P orders.
    /// <https://bybit-exchange.github.io/docs/p2p/order/pending-order>
    pub async fn get_p2p_pending_orders(&self, params: GetPendingOrdersParams) -> Result<ApiResponse> {
        self.submit_request(Method::Post, p2p::GET_PENDING_ORDERS, Some(params_to_map(&params)), true).await
    }

    /// Get P2P order details.
    /// <https://bybit-exchange.github.io/docs/p2p/order/order-detail>
    pub async fn get_p2p_order_details(&self, params: GetOrderDetailsParams) -> Result<ApiResponse> {
        self.submit_request(Method::Post, p2p::GET_ORDER_DETAILS, Some(params_to_map(&params)), true).await
    }

    /// Release digital assets to the buyer.
    /// <https://bybit-exchange.github.io/docs/p2p/order/release-digital-asset>
    pub async fn release_p2p_assets(&self, params: ReleaseAssetsParams) -> Result<ApiResponse> {
        self.submit_request(Method::Post, p2p::RELEASE_ASSETS, Some(params_to_map(&params)), true).await
    }

    /// Mark a P2P order as paid.
    /// <https://bybit-exchange.github.io/docs/p2p/order/mark-order-as-paid>
    pub async fn mark_p2p_order_as_paid(&self, params: MarkAsPaidParams) -> Result<ApiResponse> {
        self.submit_request(Method::Post, p2p::MARK_AS_PAID, Some(params_to_map(&params)), true).await
    }

    /// Get counterparty user information.
    /// <https://bybit-exchange.github.io/docs/p2p/user/counterparty-user-info>
    pub async fn get_p2p_counterparty_info(&self, params: GetCounterpartyInfoParams) -> Result<ApiResponse> {
        self.submit_request(Method::Post, p2p::GET_COUNTERPARTY_INFO, Some(params_to_map(&params)), true).await
    }

    /// Get P2P order chat messages.
    /// <https://bybit-exchange.github.io/docs/p2p/order/chat-msg>
    pub async fn get_p2p_chat_messages(&self, params: GetChatMessagesParams) -> Result<ApiResponse> {
        self.submit_request(Method::Post, p2p::GET_CHAT_MESSAGES, Some(params_to_map(&params)), true).await
    }

    // TODO: upload_p2p_chat_file — requires multipart form upload, not yet supported by submit_request.

    /// Send a P2P order chat message.
    /// <https://bybit-exchange.github.io/docs/p2p/order/send-chat-msg>
    pub async fn send_p2p_chat_message(&self, params: SendChatMessageParams) -> Result<ApiResponse> {
        self.submit_request(Method::Post, p2p::SEND_CHAT_MESSAGE, Some(params_to_map(&params)), true).await
    }

    /// Get user payment methods.
    /// <https://bybit-exchange.github.io/docs/p2p/user/user-payment>
    pub async fn get_p2p_user_payment_types(&self) -> Result<ApiResponse> {
        self.submit_request(Method::Post, p2p::GET_USER_PAYMENT_TYPES, None, true).await
    }

    /// Query P2P chat session list.
    /// <https://bybit-exchange.github.io/docs/p2p/chat/session-list>
    pub async fn query_p2p_chat_session_list(&self, params: QueryChatSessionListParams) -> Result<ApiResponse> {
        self.submit_request(Method::Post, p2p::QUERY_CHAT_SESSION_LIST, Some(params_to_map(&params)), true).await
    }

    /// Send a P2P chat message.
    /// <https://bybit-exchange.github.io/docs/p2p/chat/send-message>
    pub async fn send_p2p_message(&self, params: SendMessageParams) -> Result<ApiResponse> {
        self.submit_request(Method::Post, p2p::SEND_MESSAGE, Some(params_to_map(&params)), true).await
    }

    /// Get P2P chat message list.
    /// <https://bybit-exchange.github.io/docs/p2p/chat/message-list>
    pub async fn get_p2p_message_list(&self, params: GetMessageListParams) -> Result<ApiResponse> {
        self.submit_request(Method::Post, p2p::GET_MESSAGE_LIST, Some(params_to_map(&params)), true).await
    }
}
