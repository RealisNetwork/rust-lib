use runtime::AccountId;
use serde::{Deserialize, Serialize};

use rust_lib::json::u128::{u128_from_string, u128_to_string};

use crate::{schemas::withdraw_realis_service::realis_token_request::RealisTokenRequestSchema, Amount};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RealisWithdrawTokensSchema {
    pub id: String,
    #[serde(rename = "topicRes", alias = "topicResponse")]
    pub topic_res: String,
    #[serde(rename = "from")]
    pub from_account_id: AccountId,
    pub params: RealisWithdrawTokensSchemaParams,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RealisWithdrawTokensSchemaParams {
    #[serde(rename = "accountId")]
    pub account_id: AccountId,
    #[serde(serialize_with = "u128_to_string")]
    #[serde(deserialize_with = "u128_from_string")]
    pub amount: Amount,
}

impl From<RealisTokenRequestSchema> for RealisWithdrawTokensSchema {
    fn from(other: RealisTokenRequestSchema) -> Self {
        RealisWithdrawTokensSchema {
            id: other.id.clone(),
            topic_res: other.topic_res.clone(),
            from_account_id: other.from_account_id.clone(),
            params: RealisWithdrawTokensSchemaParams {
                account_id: other.params.account_id.clone(),
                amount: other.params.amount,
            },
        }
    }
}
