use crate::requests::AuthInfo;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use runtime::AccountId;
use rust_lib::json::u128::u128_from_string;
use crate::Amount;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BinanceWithdrawTokensSchema{
    pub id: String,
    #[serde(rename = "topicRes", alias = "topicResponse")]
    pub topic_res: String,
    pub params: HashMap<String, BinanceWithdrawTokensSchemaParams>,
    #[serde(rename = "authInfo")]
    pub auth_info: AuthInfo,
}
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct BinanceWithdrawTokensSchemaParams{
    #[serde(rename = "accountId")]
    pub account_id: String,
    #[serde(serialize_with = "u128_to_string")]
    #[serde(deserialize_with = "u128_from_string")]
    pub amount: Amount,
    #[serde(rename = "from")]
    pub from_account_id: AccountId,
}