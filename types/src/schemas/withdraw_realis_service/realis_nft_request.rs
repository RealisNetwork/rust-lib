use realis_primitives::TokenId;
use runtime::AccountId;
use serde::{Deserialize, Serialize};

use json::token_id::{token_id_from_string, token_id_to_string};

use crate::schemas::realis_orchestrator::withdraw_request::realis_withdraw_nft::RealisWithdrawNftSchema;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct RealisNftRequestSchema {
    #[serde(rename = "from")]
    pub from_account_id: AccountId,
    #[serde(rename = "topicResponse", alias = "topicRes")]
    pub topic_res: String,
    pub id: String,
    pub params: RealisNftRequestSchemaParams,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct RealisNftRequestSchemaParams {
    #[serde(rename = "accountId")]
    pub account_id: AccountId,
    #[serde(rename = "tokenId")]
    #[serde(serialize_with = "token_id_to_string")]
    #[serde(deserialize_with = "token_id_from_string")]
    pub token_id: TokenId,
}

impl RealisNftRequestSchema {
    pub fn new(other: RealisWithdrawNftSchema) -> Self {
        let params = RealisNftRequestSchemaParams {
            account_id: other.params.account_id.clone(),
            token_id: other.params.token_id,
        };
        Self {
            from_account_id: other.params.account_id.clone(),
            topic_res: other.topic_res.clone(),
            id: other.id.clone(),
            params,
        }
    }
}
