use realis_primitives::TokenId;

use runtime::AccountId;
use serde::{Deserialize, Serialize};
use substrate_api_client::Hash;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransferNftSchema {
    pub id: String,
    pub agent: String,
    pub method: String,
    pub params: TransferNftParams,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransferNftParams {
    pub from: AccountId,
    pub to: AccountId,
    #[serde(rename = "tokenId")]
    pub token_id: TokenId,
    #[serde(rename = "blockHash")]
    pub block_hash: Hash,
    #[serde(rename = "txHash")]
    pub tx_hash: Hash,
}
