use crate::requests::AuthInfo;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use runtime::AccountId;
use rust_lib::json::u128::{u128_from_string, u128_to_string};
use crate::Amount;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RealisWithDrawTokensSchema {
    pub id: String,
    #[serde(rename = "topicRes", alias = "topicResponse")]
    pub topic_res: String,
    pub params: HashMap<String, RealisWithdrawSchemaParams>,
    #[serde(rename = "authInfo")]
    pub auth_info: AuthInfo,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RealisWithdrawSchemaParams {
    #[serde(rename = "accountId")]
    pub account_id: AccountId,
    #[serde(serialize_with = "u128_to_string")]
    #[serde(deserialize_with = "u128_from_string")]
    pub amount: Amount,
}