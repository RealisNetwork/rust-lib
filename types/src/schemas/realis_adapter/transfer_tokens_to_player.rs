#[rustfmt::skip]
use crate::{
    requests::AuthInfo,
    schemas::realis_orchestrator::adapter_request::transfer_tokens_to_player::TransferTokensToPlayerSchema as OrchestratorTransferTokensToPlayerSchema,
};
use runtime::{realis_game_api::Call as RealisGameApiCall, AccountId, Call};
use rust_lib::json::u128::{u128_from_string, u128_to_string};
use serde::{Deserialize, Serialize};

pub type Amount = u128;

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

    pub fn into_call(&self) -> Call {
        Call::RealisGameApi(RealisGameApiCall::transfer_from_ptp(
            self.params.dest.clone(),
            self.params.account_id.clone(),
            self.params.amount,
        ))
    }
}
