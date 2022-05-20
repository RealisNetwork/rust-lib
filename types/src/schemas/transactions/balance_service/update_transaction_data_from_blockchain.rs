use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct UpdateTransactionDataFromBlockchainSchema {
    #[serde(rename = "oldTxId")]
    pub old_tx_id: String,
    #[serde(rename = "newTxId")]
    pub new_tx_id: String,
    #[serde(rename = "BlockId")]
    pub block_id: String,
}
