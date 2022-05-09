use realis_primitives::TokenId;
use runtime::AccountId;
use rust_lib::json::token_id::{token_id_to_string, token_id_from_string};
use serde::{Deserialize, Serialize};
use web3::types::{H160, H256, U64};

type Block = Option<U64>;

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct TransferNftSchema {
    #[serde(rename = "topicResponse")]
    pub topic_res: String,
    pub id: String,
    pub params: TransferNftSchemaParams,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct TransferNftSchemaParams {
    pub block: Block,
    pub hash: H256,
    pub to: H160,
    #[serde(rename = "accountId")]
    pub account_id: AccountId,
    #[serde(serialize_with = "token_id_to_string")]
    #[serde(deserialize_with = "token_id_from_string")]
    #[serde(rename = "tokenId")]
    pub token_id: TokenId,
}