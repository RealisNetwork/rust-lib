use rust_lib::json::u128::{u128_from_string, u128_to_string};
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
pub struct NftMintedSuccessParams {
    #[serde(rename = "txId")]
    pub hash: Hash,
    pub uuid: uuid::Uuid,
}