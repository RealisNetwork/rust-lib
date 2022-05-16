use crate::{
    requests::AuthInfo, schemas::realis_orchestrator::marketplace::remove_nft::RemoveNftSchema as OrchestratorRemoveNftSchema,
};
use json::token_id::{token_id_from_string, token_id_to_string};
use realis_primitives::TokenId;
use runtime::AccountId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RemoveNftSchema {
    pub id: String,
    #[serde(rename = "topicRes", alias = "topicResponse")]
    pub topic_res: String,
    pub method: String,
    pub params: RemoveNftSchemaParams,
    #[serde(rename = "authInfo")]
    pub auth_info: AuthInfo,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RemoveNftSchemaParams {
    #[serde(serialize_with = "token_id_to_string")]
    #[serde(deserialize_with = "token_id_from_string")]
    #[serde(rename = "tokenId")]
    pub token_id: TokenId,
    #[serde(rename = "accountId")]
    pub account_id: AccountId,
}

impl RemoveNftSchema {
    pub fn new(other: OrchestratorRemoveNftSchema, account_id: AccountId) -> Self {
        Self {
            id: other.id,
            topic_res: other.topic_res,
            method: other.method,
            auth_info: other.auth_info,
            params: RemoveNftSchemaParams {
                token_id: other.params.token_id,
                account_id,
            },
        }
    }
}
