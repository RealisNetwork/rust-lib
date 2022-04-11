use crate::requests::AuthInfo;
use realis_primitives::{Rarity, TokenId};
use runtime::AccountId;
use rust_lib::json::token_id::{token_id_from_string, token_id_to_string};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AddNftItem {
    pub id: String,
    #[serde(rename = "topicResponse", alias = "topicRes")]
    pub topic_res: String,
    pub params: AddNftItemParams,
    #[serde(rename = "authInfo")]
    pub auth_info: AuthInfo,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AddNftItemParams {
    #[serde(serialize_with = "token_id_to_string")]
    #[serde(deserialize_with = "token_id_from_string")]
    #[serde(rename = "tokenId")]
    pub token_id: TokenId,
    #[serde(rename = "mintId")]
    pub mint_id: u32,
    pub name: String,
    pub link: String,
    pub rarity: Rarity,
    #[serde(rename = "accountId")]
    pub account_id: AccountId,
}
