use crate::requests::{AuthInfo, Params};
use serde::{Deserialize, Serialize, Deserializer, Serializer};
use rust_lib::json::u128::{blockchain_number_to_string, u128_to_string, u128_from_string, blockchain_number_from_string};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetOrCreateWalletWithTokensSchema {
    pub id: String,
    #[serde(rename = "topicResponse", alias = "topicRes")]
    pub topic_res: String,
    pub params: GetOrCreateWalletWithTokensParams,
    #[serde(rename = "authInfo")]
    pub auth_info: AuthInfo,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetOrCreateWalletWithTokensParams {
    #[serde(serialize_with = "option_u128_to_string")]
    #[serde(deserialize_with = "option_u128_from_string")]
    pub amount: Option<u128>
}

pub fn option_u128_from_string<'de, D>(deserializer: D) -> Result<Option<u128>, D::Error>
    where
        D: Deserializer<'de>,
{
    match Option::deserialize(deserializer)? {
        Some(number) => Ok(Some(blockchain_number_from_string(number)?)),
        None => Ok(None)
    }
}

pub fn option_u128_to_string<S>(option: &Option<u128>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
{
    match option.map(|number| blockchain_number_to_string(&number)) {
        None => serializer.serialize_none(),
        Some(number) => serializer.serialize_some(&number)
    }
}
