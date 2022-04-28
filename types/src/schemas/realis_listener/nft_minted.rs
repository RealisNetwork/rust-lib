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
    #[serde(rename = "blockHash")]
    pub block_hash: Hash,
    #[serde(rename = "txHash")]
    pub tx_hash: Hash,
}
