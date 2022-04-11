use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlockchainResponseParams {
    #[serde(rename = "blockId")]
    block_id: String,
    #[serde(rename = "txId")]
    hash_id: String,
}