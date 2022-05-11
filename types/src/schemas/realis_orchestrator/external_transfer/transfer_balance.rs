use runtime::AccountId;
use serde::{Deserialize, Serialize};
use substrate_api_client::Hash;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransferBalanceSchema {
    pub id: String,
    pub agent: String,
    pub method: String,
    pub params: TransferBalanceSchemaParams,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransferBalanceSchemaParams {
    pub from: AccountId,
    pub to: AccountId,
    pub amount: String,
    #[serde(rename = "blockHash")]
    pub block_hash: Hash,
    #[serde(rename = "txHash")]
    pub tx_hash: Hash,
}
