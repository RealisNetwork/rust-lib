use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IncreaseWalletBalanceResponseParams {
    #[serde(rename = "blockId")]
    block_id: String,
    #[serde(rename = "txId")]
    hash_id: String,
}
