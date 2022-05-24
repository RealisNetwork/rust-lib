use crate::requests::AuthInfo;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct UpdateTransactionDataFromBlockchainSchema {
    pub id: String,
    #[serde(rename = "topicResponse", alias = "topicRes")]
    pub topic_res: String,
    pub params: UpdateTransactionDataFromBlockchainSchemaParams,
    #[serde(rename = "authInfo")]
    pub auth_info: AuthInfo,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct UpdateTransactionDataFromBlockchainSchemaParams {
    #[serde(rename = "oldTxId")]
    pub old_tx_id: String,
    #[serde(rename = "newTxId")]
    pub new_tx_id: String,
    #[serde(rename = "BlockId")]
    pub block_id: String,
}
