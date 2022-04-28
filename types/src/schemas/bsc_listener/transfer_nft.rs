use crate::schemas::bsc_listener::{
    option_u64_from_string,
    option_u64_to_string
};
use realis_primitives::TokenId;
use rust_lib::json::{
    u128::{u128_from_string, u128_to_string},
    token_id::{token_id_from_string, token_id_to_string}
};
use runtime::AccountId;
use serde::{Deserialize, Serialize};
use substrate_api_client::Hash;

type Amount = u128;
type Block = Option<U64>;

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct TransferNftSchema {
    #[serde(rename = "topicResponse")]
    pub topic_res: String,
    pub id: String,
    pub params: TransferNftParams,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct TransferNftParams {
    #[serde(serialize_with = "option_u64_to_string")]
    #[serde(deserialize_with = "option_u64_from_string")]
    pub block: Block,
    pub hash: Hash,
    pub to: Hash,
    #[serde(rename = "accountId")]
    pub account_id: AccountId,
    #[serde(serialize_with = "token_id_to_string")]
    #[serde(deserialize_with = "token_id_from_string")]
    #[serde(rename = "tokenId")]
    pub token_id: TokenId,
}