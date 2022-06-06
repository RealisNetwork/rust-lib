use json::u128::{u128_from_string, u128_to_string};
use runtime::AccountId;
use serde::{Deserialize, Serialize};
use substrate_api_client::Hash;

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct BalanceTransferedSchema {
    #[serde(rename = "topicResponse")]
    pub topic_res: String,
    pub id: String,
    pub params: BalanceTransferedSchemaParams,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct BalanceTransferedSchemaParams {
    pub from: AccountId,
    pub to: AccountId,
    #[serde(serialize_with = "u128_to_string")]
    #[serde(deserialize_with = "u128_from_string")]
    pub amount: u128,
    #[serde(rename = "blockHash")]
    pub block_hash: Hash,
    #[serde(rename = "txHash")]
    pub tx_hash: Hash,
}
