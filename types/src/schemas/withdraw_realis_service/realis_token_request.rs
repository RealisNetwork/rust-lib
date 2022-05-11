use runtime::AccountId;
use serde::{Deserialize, Serialize};

use rust_lib::json::u128::{u128_from_string, u128_to_string};

use crate::Amount;
use crate::schemas::realis_orchestrator::withdraw_request::realis_withdraw_tokens::RealisWithdrawTokensSchema;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct RealisTokenRequestSchema {
    #[serde(rename = "from")]
    pub from: AccountId,
    pub id: String,
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
        RealisTokenRequestSchema {
            from: other.params.account_id.clone(),
            id: other.id.clone(),
            params,
        }
    }
}
