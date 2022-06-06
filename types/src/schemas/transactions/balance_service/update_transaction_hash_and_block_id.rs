use crate::requests::AuthInfo;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct UpdateTransactionHashAndBlockIdSchema {
    pub id: String,
    #[serde(rename = "topicResponse", alias = "topicRes")]
    pub topic_res: String,
    pub params: UpdateTransactionHashAndBlockIdSchemaParams,
    #[serde(rename = "authInfo")]
    pub auth_info: AuthInfo,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct UpdateTransactionHashAndBlockIdSchemaParams {
    #[serde(rename = "oldTxId")]
    pub old_tx_id: String,
    #[serde(rename = "newTxId")]
    pub new_tx_id: String,
    #[serde(rename = "newBlockId")]
    pub new_block_id: String,
}
