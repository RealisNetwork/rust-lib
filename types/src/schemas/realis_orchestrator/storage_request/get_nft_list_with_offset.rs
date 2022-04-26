use crate::{
    requests::AuthInfo,
    schemas::storage_service::{option_u128_from_string, option_u128_to_string},
};
use serde::{Deserialize, Serialize};
use runtime::AccountId;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetNftListWithOffsetSchema {
    pub id: String,
    #[serde(rename = "topicRes", alias = "topicResponse")]
    pub topic_res: String,
    pub params: Option<GetNftListWithOffsetSchemaParams>,
    #[serde(rename = "accountId")]
    pub account_id: AccountId,
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
