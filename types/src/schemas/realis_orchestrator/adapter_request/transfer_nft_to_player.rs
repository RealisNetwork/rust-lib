use crate::{
    requests::AuthInfo, schemas::realis_adapter::transfer_nft_to_player::TransferNftToPlayerSchema as AdapterTransferNftToPlayerSchema,
};
use realis_primitives::TokenId;
use runtime::AccountId;
use rust_lib::json::token_id::{token_id_from_string, token_id_to_string};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransferNftToPlayerSchema {
    pub id: String,
    #[serde(rename = "topicResponse", alias = "topicRes")]
    pub topic_res: String,
    pub params: TransferNftToPlayerSchemaParams,
    #[serde(rename = "authInfo")]
    pub auth_info: AuthInfo,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransferNftToPlayerSchemaParams {
    pub dest: AccountId,
    #[serde(serialize_with = "token_id_to_string")]
    #[serde(deserialize_with = "token_id_from_string")]
    #[serde(rename = "tokenId")]
    pub token_id: TokenId,
}

impl From<AdapterTransferNftToPlayerSchema> for TransferNftToPlayerSchema {
    fn from(other: AdapterTransferNftToPlayerSchema) -> Self {
        Self {
            id: other.id,
            topic_res: other.topic_res,
            params: TransferNftToPlayerSchemaParams {
                dest: other.params.dest,
                token_id: other.params.token_id,
            },
            auth_info: other.auth_info,
        }
    }
}
