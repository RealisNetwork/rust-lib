use crate::{
    requests::AuthInfo, schemas::realis_orchestrator::adapter_request::add_nft_item::AddNftItemSchema as OrchestratorAddNftItemSchema,
};
use json::token_id::{token_id_from_string, token_id_to_string};
use realis_primitives::{Rarity, TokenId};
use runtime::{realis_game_api::Call as RealisGameApiCall, AccountId, Call};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AddNftItemSchema {
    pub id: String,
    #[serde(rename = "topicResponse", alias = "topicRes")]
    pub topic_res: String,
    pub params: AddNftItemSchemaParams,
    #[serde(rename = "authInfo")]
    pub auth_info: AuthInfo,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AddNftItemSchemaParams {
    #[serde(serialize_with = "token_id_to_string")]
    #[serde(deserialize_with = "token_id_from_string")]
    #[serde(rename = "tokenId")]
    pub token_id: TokenId,
    #[serde(rename = "mintId")]
    pub mint_id: u32,
    pub name: String,
    pub link: String,
    pub rarity: Rarity,
    #[serde(rename = "accountId")]
    pub account_id: AccountId,
}

impl AddNftItemSchema {
    pub fn new(other: OrchestratorAddNftItemSchema, account_id: AccountId) -> Self {
        Self {
            id: other.id,
            topic_res: other.topic_res,
            params: AddNftItemSchemaParams {
                token_id: other.params.token_id,
                mint_id: other.params.mint_id,
                name: other.params.name,
                link: other.params.link,
                rarity: other.params.rarity,
                account_id,
            },
            auth_info: other.auth_info,
        }
    }

    pub fn into_call(&self) -> Call {
        Call::RealisGameApi(RealisGameApiCall::mint_nft(
            self.params.account_id.clone(),
            self.params.token_id,
            self.params.mint_id,
            self.params.name.clone().into_bytes(),
            self.params.rarity,
            self.params.link.clone().into_bytes(),
            self.id.clone().into_bytes(),
        ))
    }
}
