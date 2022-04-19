use crate::requests::AuthInfo;
use realis_primitives::TokenId;
use serde::{Deserialize, Serialize};
use rust_lib::json::token_id::{token_id_from_string, token_id_to_string};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BuyNftSchema {
    pub id: String,
    #[serde(rename = "topicRes", alias = "topicResponse")]
    pub topic_res: String,
    pub method: String,
    pub params: BuyNftParams,
    #[serde(rename = "authInfo")]
    pub auth_info: AuthInfo,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BuyNftParams {
    #[serde(serialize_with = "token_id_to_string")]
    #[serde(deserialize_with = "token_id_from_string")]
    #[serde(rename = "tokenId")]
    token_id: TokenId,
}
