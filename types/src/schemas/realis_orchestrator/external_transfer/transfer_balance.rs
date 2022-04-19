use primitives::RequestId;
use runtime::AccountId;
use serde::{Deserialize, Serialize};
use substrate_api_client::Hash;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransferBalanceSchema {
    pub id: RequestId,
    pub agent: String,
    pub method: String,
    pub params: TransferBalanceParams,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransferBalanceParams {
    pub from: AccoundId,
    pub to: AccountId,
    pub amount: String,
    #[serde(rename = "blockHash")]
    pub block_hash: Hash,
    #[serde(rename = "txHash")]
    pub tx_hash: Hash,
}
