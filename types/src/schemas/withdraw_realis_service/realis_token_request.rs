use runtime::AccountId;
use serde::{Deserialize, Serialize};

use json::u128::{u128_from_string, u128_to_string};

use crate::{schemas::realis_orchestrator::withdraw_request::realis_withdraw_tokens::RealisWithdrawTokensSchema, Amount};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct RealisTokenRequestSchema {
    #[serde(rename = "from")]
    pub from_account_id: AccountId,
    pub id: String,
    #[serde(rename = "topicResponse", alias = "topicRes")]
    pub topic_res: String,
    pub params: RealisTokenRequestSchemaParams,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct RealisTokenRequestSchemaParams {
    #[serde(rename = "accountId")]
    pub account_id: AccountId,
    #[serde(serialize_with = "u128_to_string")]
    #[serde(deserialize_with = "u128_from_string")]
    pub amount: Amount,
}

impl RealisTokenRequestSchema {
    pub fn new(other: &RealisWithdrawTokensSchema) -> Self {
        let params = RealisTokenRequestSchemaParams {
            account_id: other.params.account_id.clone(),
            amount: other.params.amount,
        };
        Self {
            from_account_id: other.params.account_id.clone(),
            id: other.id.clone(),
            topic_res: other.topic_res.clone(),
            params,
        }
    }
}
