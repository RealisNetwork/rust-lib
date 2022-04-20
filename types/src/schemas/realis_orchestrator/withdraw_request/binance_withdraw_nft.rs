use std::collections::HashMap;
use realis_primitives::TokenId;
use crate::requests::AuthInfo;
use serde::{Deserialize, Serialize};
use rust_lib::json::
    token_id::{token_id_from_string, token_id_to_string};
use runtime::AccountId;


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BinanceWithdrawNftSchema {
    pub id: String,
    #[serde(rename = "topicRes", alias = "topicResponse")]
    pub topic_res: String,
    pub params: HashMap<String, BinanceWithdrawNftSchemaParams>,
    #[serde(rename = "authInfo")]
    pub auth_info: AuthInfo,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct BinanceWithdrawNftSchemaParams {
    #[serde(rename = "accountId")]
    pub account_id: String,
    #[serde(rename = "tokenId")]
    #[serde(serialize_with = "token_id_to_string")]
    #[serde(deserialize_with = "token_id_from_string")]
    pub token_id: TokenId,
    #[serde(rename = "from")]
    pub from_account_id: AccountId,
}