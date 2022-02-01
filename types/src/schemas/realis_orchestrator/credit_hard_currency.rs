use rust_lib::json::u128::{u128_to_string, u128_from_string};
use serde::{Serialize, Deserialize};
use crate::requests::AuthInfo;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreditHardCurrencySchema {
    pub id: String,
    #[serde(rename = "topicResponse", alias = "topicRes")]
    pub topic_res: String,
    pub params: CreditHardCurrencyParams,
    #[serde(rename = "authInfo")]
    pub auth_info: AuthInfo,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreditHardCurrencyParams {
    #[serde(serialize_with = "u128_to_string")]
    #[serde(deserialize_with = "u128_from_string")]
    pub amount: u128
}
