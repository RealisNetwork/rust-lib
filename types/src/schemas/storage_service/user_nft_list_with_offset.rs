#[rustfmt::skip]
use crate::{
    requests::AuthInfo,
    schemas::realis_orchestrator::storage_request::get_nft_list_with_offset::GetNftListWithOffsetSchema as OrchestratorGetNftListWithOffsetSchema,
};
use json::u128::{option_u128_from_string, option_u128_to_string};
use runtime::AccountId;
use serde::{Deserialize, Serialize};

#[allow(clippy::pedantic)]
#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct GetNftListWithOffsetSchema {
    #[serde(rename = "topicResponse")]
    pub topic_res: String,
    pub id: String,
    pub params: GetNftListWithOffsetSchemaParams,
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
        let params = match other.params {
            Some(params_) => GetNftListWithOffsetSchemaParams {
                size: params_.size,
                offset: params_.offset,
                account_id: account_id,
            },
            None => GetNftListWithOffsetSchemaParams {
                size: None,
                offset: None,
                account_id: account_id,
            },
        };
        Self {
            id: other.id,
            topic_res: other.topic_res,
            params: params,
            auth_info: other.auth_info,
        }
    }
}
