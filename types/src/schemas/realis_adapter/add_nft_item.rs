use crate::{requests::AuthInfo, schemas::realis_orchestrator::add_nft_item::AddNftItemSchema as OrchestratorAddNftItemSchema};
use realis_primitives::{Rarity, TokenId};
use runtime::AccountId;
use rust_lib::{
    blockchain::cold_wallets::RealisGameApi,
    json::token_id::{token_id_from_string, token_id_to_string},
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AddNftItemSchema {
    pub id: String,
    #[serde(rename = "topicResponse", alias = "topicRes")]
    pub topic_res: String,
    pub params: AddNftItemParams,
    #[serde(rename = "authInfo")]
    pub auth_info: AuthInfo,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AddNftItemParams {
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
        AddNftItemSchema {
            id: other.id,
            topic_res: other.topic_res,
            params: AddNftItemParams {
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
}

impl From<AddNftItemSchema> for Call {
    fn from(schema: AddNftItemSchema) -> Call {
        Call::RealisGameApi(RealisGameApiCall::mint_nft(
            schema.params.account_id,
            schema.params.token_id,
            schema.params.mint_id,
            schema.params.name,
            schema.params.rarity,
            schema.params.link,
        ))
    }
}
