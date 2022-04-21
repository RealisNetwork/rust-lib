use crate::requests::AuthInfo;
use runtime::AccountId;
use realis_primitives::TokenId;
use serde::{Deserialize, Serialize};
use rust_lib::json::token_id::{token_id_from_string, token_id_to_string};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RealisWithdrawNftSchema{
    pub id: String,
    #[serde(rename = "topicRes", alias = "topicResponse")]
    pub topic_res: String,
    #[serde(rename = "from")]
    pub from_account_id: AccountId,
    pub params: RealisWithdrawNftSchemaParams,
    #[serde(rename = "authInfo")]
    pub auth_info: AuthInfo,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RealisWithdrawNftSchemaParams{
    #[serde(rename = "accountId")]
    pub account_id: AccountId,
    #[serde(rename = "tokenId")]
    #[serde(serialize_with = "token_id_to_string")]
    #[serde(deserialize_with = "token_id_from_string")]
    pub token_id: TokenId,
}
