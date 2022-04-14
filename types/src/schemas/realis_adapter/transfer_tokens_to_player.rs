use runtime::AccountId;
use crate::{
    requests::AuthInfo,
    schemas::realis_orchestrator::transfer_tokens_to_player::TransferTokensToPlayerSchema as OrchestratorTransferTokensToPlayerSchema,
};
use rust_lib::json::u128::{u128_from_string, u128_to_string};
use serde::{Deserialize, Serialize};
use rust_lib::blockchain::cold_wallets::RealisGameApi;
use crate::schemas::realis_adapter::transfer_nft_to_player::TransferNftToPlayerSchema;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransferTokensToPlayerSchema {
    pub id: String,
    #[serde(rename = "topicResponse", alias = "topicRes")]
    pub topic_res: String,
    pub params: TransferTokensToPlayerParams,
    #[serde(rename = "authInfo")]
    pub auth_info: AuthInfo,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransferTokensToPlayerParams {
    pub dest: AccountId,
    #[serde(serialize_with = "u128_to_string")]
    #[serde(deserialize_with = "u128_from_string")]
    pub amount: Amount,
    #[serde(rename = "accountId")]
    pub account_id: AccountId,
}

impl TransferTokensToPlayerSchema {
    pub fn new(other: OrchestratorTransferTokensToPlayerSchema, account_id: AccountId) -> Self {
        TransferTokensToPlayerSchema {
            id: other.id,
            topic_res: other.topic_res,
            params: TransferTokensToPlayerParams {
                dest: other.params.dest,
                amount: other.params.amount,
                account_id,
            },
            auth_info: other.auth_info,
        }
    }
}
