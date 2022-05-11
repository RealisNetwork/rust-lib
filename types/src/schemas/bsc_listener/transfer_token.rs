use runtime::AccountId;
use rust_lib::json::u128::{u128_from_string, u128_to_string};
use serde::{Deserialize, Serialize};
use web3::types::{H256, U64};

type Amount = u128;
type Block = Option<U64>;

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct TransferTokenSchema {
    #[serde(rename = "topicResponse")]
    pub topic_res: String,
    pub id: String,
    pub params: TransferTokenSchemaParams,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct TransferTokenSchemaParams {
    pub block: Block,
    pub hash: H256,
    pub to: H256,
    #[serde(rename = "accountId")]
    pub account_id: AccountId,
    #[serde(serialize_with = "u128_to_string")]
    #[serde(deserialize_with = "u128_from_string")]
    pub amount: Amount,
}
