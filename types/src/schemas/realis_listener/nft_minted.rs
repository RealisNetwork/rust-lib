use error_registry::RealisErrors;
use rust_lib::json::u128::{u128_from_string, u128_to_string};
use runtime::AccountId;
use serde::{Deserialize, Serialize};
use substrate_api_client::Hash;


#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct NftMintedSuccessSchema {
    #[serde(rename = "topicResponse")]
    pub topic_res: String,
    pub id: String,
    pub params: NftMintedSuccessParams,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct NftMintedErrorSchema {
    #[serde(rename = "topicResponse")]
    pub topic_res: String,
    pub id: String,
    pub params: NftMintedErrorParams,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct NftMintedSuccessParams {
    #[serde(rename = "blockHash")]
    pub block_hash: Hash,
    #[serde(rename = "txHash")]
    pub tx_hash: Hash,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct NftMintedErrorParams {
    pub error: RealisErrors,
}