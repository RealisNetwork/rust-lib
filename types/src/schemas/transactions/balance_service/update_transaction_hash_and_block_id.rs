use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct UpdateTransactionHashAndBlockSchema {
    #[serde(rename = "oldTxId")]
    pub old_tx_id: String,
    #[serde(rename = "newTxId")]
    pub new_tx_id: String,
    #[serde(rename = "newBlockId")]
    pub new_block_id: String,
}
