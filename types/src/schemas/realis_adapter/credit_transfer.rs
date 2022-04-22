use crate::{requests::AuthInfo, schemas::realis_orchestrator::adapter_request::credit_hard_currency::CreditHardCurrencySchema};
use runtime::{realis_game_api::Call as RealisGameApiCall, AccountId, Call};
use rust_lib::json::u128::{u128_from_string, u128_to_string};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreditTransferSchema {
    pub id: String,
    #[serde(rename = "topicResponse", alias = "topicRes")]
    pub topic_res: String,
    pub params: CreditTransferParams,
    #[serde(rename = "authInfo")]
    pub auth_info: AuthInfo,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreditTransferParams {
    #[serde(serialize_with = "u128_to_string")]
    #[serde(deserialize_with = "u128_from_string")]
    pub amount: u128,
    #[serde(rename = "accountId")]
    pub account_id: AccountId,
}

impl CreditTransferSchema {
    pub fn new(other: CreditHardCurrencySchema, account_id: AccountId) -> Self {
        CreditTransferSchema {
            id: other.id,
            topic_res: other.topic_res,
            params: CreditTransferParams {
                amount: other.params.amount,
                account_id,
            },
            auth_info: other.auth_info,
        }
    }

    pub fn into_call(&self) -> Call {
        Call::RealisGameApi(RealisGameApiCall::transfer_from_pallet(
            self.params.account_id.clone(),
            self.params.amount,
        ))
    }
}
