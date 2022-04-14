use crate::{requests::AuthInfo, schemas::realis_orchestrator::debit_hard_currency::DebitHardCurrencySchema};
use runtime::{realis_game_api::Call as RealisGameApiCall, AccountId, Call};
use rust_lib::{
    json::u128::{u128_from_string, u128_to_string},
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DebitTransferSchema {
    pub id: String,
    #[serde(rename = "topicResponse", alias = "topicRes")]
    pub topic_res: String,
    pub params: DebitTransferParams,
    #[serde(rename = "authInfo")]
    pub auth_info: AuthInfo,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DebitTransferParams {
    #[serde(serialize_with = "u128_to_string")]
    #[serde(deserialize_with = "u128_from_string")]
    pub amount: u128,
    #[serde(rename = "accountId")]
    pub account_id: AccountId,
}

impl DebitTransferSchema {
    pub fn new(other: DebitHardCurrencySchema, account_id: AccountId) -> Self {
        DebitTransferSchema {
            id: other.id,
            topic_res: other.topic_res,
            params: DebitTransferParams {
                amount: other.params.amount,
                account_id,
            },
            auth_info: other.auth_info,
        }
    }

    pub fn into_call(&self) -> Call {
        Call::RealisGameApi(RealisGameApiCall::spend_in_game(
            self.params.account_id.clone(),
            self.params.amount,
        ))
    }
}
