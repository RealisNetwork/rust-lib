use crate::schemas::storage_service::{option_u128_from_string, option_u128_to_string};
use crate::{
    requests::AuthInfo, 
    schemas::realis_orchestrator::storage_request::get_nft_list_with_offset::OrchestratorGetNftListWithOffsetSchema
};
use runtime::AccountId;
use serde::{Deserialize, Serialize};

#[allow(clippy::pedantic)]
#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct GetNftListWithOffsetSchema {
    #[serde(rename = "topicResponse")]
    pub topic_res: String,
    pub id: String,
    pub params: Option<GetNftListWithOffsetSchemaParams>,
    #[serde(rename = "authInfo")]
    pub auth_info: AuthInfo,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct GetNftListWithOffsetSchemaParams {
    #[serde(default)]
    #[serde(serialize_with = "option_u128_to_string")]
    #[serde(deserialize_with = "option_u128_from_string")]
    pub size: Option<u128>,
    #[serde(default)]
    #[serde(serialize_with = "option_u128_to_string")]
    #[serde(deserialize_with = "option_u128_from_string")]
    pub offset: Option<u128>,
    #[serde(rename = "accountId")]
    pub account_id: AccountId,
}

impl GetNftListWithOffsetSchema {
    pub fn new(other: OrchestratorGetNftListWithOffsetSchema, account_id: AccountId) -> Self {
        Self {
            id: other.id,
            topic_res: other.topic_res,
            params: GetNftListWithOffsetSchemaParams {
                size: other.params.size,
                offset: other.params.offset,
                account_id,
            },
            auth_info: other.auth_info,
        }
    }
}
