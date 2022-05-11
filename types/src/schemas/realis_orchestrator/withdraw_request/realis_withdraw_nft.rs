use realis_primitives::TokenId;
use runtime::AccountId;
use serde::{Deserialize, Serialize};

use rust_lib::json::token_id::{token_id_from_string, token_id_to_string};

use crate::schemas::withdraw_realis_service::realis_nft_request::RealisNftRequestSchema;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RealisWithdrawNftSchema {
    pub id: String,
    #[serde(rename = "topicRes", alias = "topicResponse")]
    pub topic_res: String,
    #[serde(rename = "from")]
    pub from_account_id: AccountId,
    pub params: RealisWithdrawNftSchemaParams,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RealisWithdrawNftSchemaParams {
    #[serde(rename = "accountId")]
    pub account_id: AccountId,
    #[serde(rename = "tokenId")]
    #[serde(serialize_with = "token_id_to_string")]
    #[serde(deserialize_with = "token_id_from_string")]
    pub token_id: TokenId,
}

impl From<RealisNftRequestSchema> for RealisWithdrawNftSchema {
    fn from(other: RealisNftRequestSchema) -> Self {
        RealisWithdrawNftSchema {
            id: other.id.clone(),
            topic_res: other.topic_res.clone(),
            from_account_id: other.from_account_id.clone(),
            params: RealisWithdrawNftSchemaParams {
                account_id: other.params.account_id.clone(),
                token_id: other.params.token_id,
            },
        }
    }
}
