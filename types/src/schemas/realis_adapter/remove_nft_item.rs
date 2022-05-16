use crate::{
    requests::AuthInfo,
    schemas::realis_orchestrator::adapter_request::remove_nft_item::RemoveNftItemSchema as OrchestratorRemoveNftItemSchema,
};
use realis_primitives::TokenId;
use runtime::{realis_game_api::Call as RealisGameApiCall, AccountId, Call};
use json::token_id::{token_id_from_string, token_id_to_string};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RemoveNftItemSchema {
    pub id: String,
    #[serde(rename = "topicResponse", alias = "topicRes")]
    pub topic_res: String,
    pub params: RemoveNftItemSchemaParams,
    #[serde(rename = "authInfo")]
    pub auth_info: AuthInfo,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RemoveNftItemSchemaParams {
    #[serde(serialize_with = "token_id_to_string")]
    #[serde(deserialize_with = "token_id_from_string")]
    #[serde(rename = "tokenId")]
    pub token_id: TokenId,
    #[serde(rename = "accountId")]
    pub account_id: AccountId,
}

impl RemoveNftItemSchema {
    pub fn new(other: OrchestratorRemoveNftItemSchema, account_id: AccountId) -> Self {
        Self {
            id: other.id,
            topic_res: other.topic_res,
            params: RemoveNftItemSchemaParams {
                token_id: other.params.token_id,
                account_id,
            },
            auth_info: other.auth_info,
        }
    }

    pub fn into_call(&self) -> Call {
        Call::RealisGameApi(RealisGameApiCall::burn_nft(self.params.account_id.clone(), self.params.token_id))
    }
}
