use realis_primitives::TokenId;
use runtime::AccountId;
use serde::{Deserialize, Serialize};
use substrate_api_client::Hash;

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct NftTransferedSchema {
    #[serde(rename = "topicResponse")]
    pub topic_res: String,
    pub id: String,
    pub params: NftTransferedParams,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct NftTransferedParams {
    pub from: AccountId,
    pub to: AccountId,
    #[serde(rename = "tokenId")]
    pub token_id: TokenId,
    #[serde(rename = "blockHash")]
    pub block_hash: Hash,
    #[serde(rename = "txHash")]
    pub tx_hash: Hash,
}
