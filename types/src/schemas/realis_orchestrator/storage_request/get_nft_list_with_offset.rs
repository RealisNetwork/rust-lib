use crate::{
    requests::AuthInfo,
    schemas::storage_service::user_nft_list_with_offset::GetNftListWithOffsetSchema as StorageGetNftListWithOffsetSchema,
};
use json::u128::{option_u128_from_string, option_u128_to_string};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetNftListWithOffsetSchema {
    pub id: String,
    #[serde(rename = "topicRes", alias = "topicResponse")]
    pub topic_res: String,
    pub params: Option<GetNftListWithOffsetSchemaParams>,
    #[serde(rename = "authInfo")]
    pub auth_info: AuthInfo,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetNftListWithOffsetSchemaParams {
    #[serde(default)]
    #[serde(serialize_with = "option_u128_to_string")]
    #[serde(deserialize_with = "option_u128_from_string")]
    pub size: Option<u128>,
    #[serde(default)]
    #[serde(serialize_with = "option_u128_to_string")]
    #[serde(deserialize_with = "option_u128_from_string")]
    pub offset: Option<u128>,
}

impl From<StorageGetNftListWithOffsetSchema> for GetNftListWithOffsetSchema {
    fn from(other: StorageGetNftListWithOffsetSchema) -> Self {
        let params = Some(GetNftListWithOffsetSchemaParams {
            size: Option::from(other.params.size),
            offset: Option::from(other.params.offset),
        });
        Self {
            id: other.id,
            topic_res: other.topic_res,
            params: params,
            auth_info: other.auth_info,
        }
    }
}
