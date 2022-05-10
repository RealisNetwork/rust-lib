use crate::schemas::storage_service::{option_u128_from_string, option_u128_to_string};

use runtime::AccountId;
use serde::{Deserialize, Serialize};

#[allow(clippy::pedantic)]
#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct GetNftListWithOffsetSchema {
    #[serde(rename = "topicResponse")]
    pub topic_res: String,
    pub id: String,
    #[serde(rename = "authInfo")]
    pub auth_info: AuthInfo,
    pub params: Option<GetNftListWithOffsetSchemaParams>,
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
