use crate::{
    requests::AuthInfo, schemas::realis_adapter::remove_nft_item::RemoveNftItemSchema as AdapterRemoveNftItemSchema,
};
use realis_primitives::TokenId;
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
}

impl From<AdapterRemoveNftItemSchema> for RemoveNftItemSchema {
    fn from(other: AdapterRemoveNftItemSchema) -> Self {
        RemoveNftItemSchema {
            id: other.id,
            topic_res: other.topic_res,
            params: RemoveNftItemParams {
                token_id: other.params.token_id,
            },
            auth_info: other.auth_info,
        }
    }
}
