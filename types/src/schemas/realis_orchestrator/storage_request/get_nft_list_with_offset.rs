use crate::schemas::storage_service::{option_u128_from_string, option_u128_to_string};
use crate::{requests::AuthInfo, schemas::storage_service::user_nft_list_with_offset::GetNftListWithOffsetSchema};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrchestratorGetNftListWithOffsetSchema {
    pub id: String,
    #[serde(rename = "topicRes", alias = "topicResponse")]
    pub topic_res: String,
    pub params: Option<OrchestratorGetNftListWithOffsetSchemaParams>,
    #[serde(rename = "authInfo")]
    pub auth_info: AuthInfo,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrchestratorGetNftListWithOffsetSchemaParams {
    #[serde(default)]
    #[serde(serialize_with = "option_u128_to_string")]
    #[serde(deserialize_with = "option_u128_from_string")]
    pub size: Option<u128>,
    #[serde(default)]
    #[serde(serialize_with = "option_u128_to_string")]
    #[serde(deserialize_with = "option_u128_from_string")]
    pub offset: Option<u128>,
}

impl From<GetNftListWithOffsetSchema> for OrchestratorGetNftListWithOffsetSchema {
    fn from(other: GetNftListWithOffsetSchema) -> Self {
        OrchestratorGetNftListWithOffsetSchema {
            id: other.id,
            topic_res: other.topic_res,
            params: OrchestratorGetNftListWithOffsetSchemaParams {
                size: other.params.size,
                offset: other.params.offset,
            },
            auth_info: other.auth_info,
        }
    }
}