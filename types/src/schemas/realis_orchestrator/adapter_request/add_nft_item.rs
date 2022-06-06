use crate::{requests::AuthInfo, schemas::realis_adapter::add_nft_item::AddNftItemSchema as AdapterAddNftItemSchema};
use json::token_id::{token_id_from_string, token_id_to_string};
use realis_primitives::{Rarity, TokenId};
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
}

impl From<AdapterAddNftItemSchema> for AddNftItemSchema {
    fn from(other: AdapterAddNftItemSchema) -> Self {
        Self {
            id: other.id,
            topic_res: other.topic_res,
            params: AddNftItemSchemaParams {
                token_id: other.params.token_id,
                mint_id: other.params.mint_id,
                name: other.params.name,
                link: other.params.link,
                rarity: other.params.rarity,
            },
            auth_info: other.auth_info,
        }
    }
}
