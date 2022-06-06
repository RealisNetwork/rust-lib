use crate::requests::AuthInfo;
use json::u128::{option_u128_from_string, option_u128_to_string};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetOrCreateWalletWithTokensSchema {
    pub id: String,
    #[serde(rename = "topicResponse")]
    pub topic_res: String,
    #[serde(default)]
    pub params: Option<GetOrCreateWalletWithTokensSchemaParams>,
    #[serde(rename = "authInfo")]
    pub auth_info: AuthInfo,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetOrCreateWalletWithTokensSchemaParams {
    #[serde(default)]
    #[serde(serialize_with = "option_u128_to_string")]
    #[serde(deserialize_with = "option_u128_from_string")]
    pub amount: Option<u128>,
}
