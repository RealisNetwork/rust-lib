use crate::{
    requests::AuthInfo,
    schemas::realis_orchestrator::adapter_request::decrease_balance::DecreaseBalanceSchema as OrchestratorDecreaseBalanceSchema,
};
use runtime::{realis_game_api::Call as RealisGameApiCall, AccountId, Call};
use rust_lib::json::u128::{u128_from_string, u128_to_string};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DecreaseBalanceSchema {
    pub id: String,
    #[serde(rename = "topicResponse", alias = "topicRes")]
    pub topic_res: String,
    pub params: DecreaseBalanceSchemaParams,
    #[serde(rename = "authInfo")]
    pub auth_info: AuthInfo,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DecreaseBalanceSchemaParams {
    #[serde(serialize_with = "u128_to_string")]
    #[serde(deserialize_with = "u128_from_string")]
    pub amount: u128,
    #[serde(rename = "accountId")]
    pub account_id: AccountId,
}

impl DecreaseBalanceSchema {
    pub fn new(other: OrchestratorDecreaseBalanceSchema, account_id: AccountId) -> Self {
        Self {
            id: other.id,
            topic_res: other.topic_res,
            params: DecreaseBalanceSchemaParams {
                amount: other.params.amount,
                account_id,
            },
            auth_info: other.auth_info,
        }
    }

    pub fn into_call(&self) -> Call {
        Call::RealisGameApi(RealisGameApiCall::spend_in_game(self.params.account_id.clone(), self.params.amount))
    }
}
