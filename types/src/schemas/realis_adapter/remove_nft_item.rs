use crate::{
    requests::AuthInfo,
    schemas::realis_orchestrator::remove_nft_item::RemoveNftItemSchema as OrchestratorRemoveNftItemSchema,
};
use realis_primitives::TokenId;
use runtime::AccountId;
use rust_lib::json::token_id::{token_id_from_string, token_id_to_string};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RemoveNftItemSchema {
    pub id: String,
    #[serde(rename = "topicResponse", alias = "topicRes")]
    pub topic_res: String,
    pub params: RemoveNftItemParams,
    #[serde(rename = "authInfo")]
    pub auth_info: AuthInfo,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RemoveNftItemParams {
    #[serde(serialize_with = "token_id_to_string")]
    #[serde(deserialize_with = "token_id_from_string")]
    #[serde(rename = "tokenId")]
    pub token_id: TokenId,
    #[serde(rename = "accountId")]
    pub account_id: AccountId,
}

impl RemoveNftItemSchema {
    pub fn new(other: OrchestratorRemoveNftItemSchema, account_id: AccountId) -> Self {
        RemoveNftItemSchema {
            id: other.id,
            topic_res: other.topic_res,
            params: RemoveNftItemParams {
                token_id: other.params.token_id,
                account_id,
            },
            auth_info: other.auth_info,
        }
    }
}
