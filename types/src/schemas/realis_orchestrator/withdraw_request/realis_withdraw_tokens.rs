use crate::{requests::AuthInfo, Amount};
use runtime::AccountId;
use rust_lib::json::u128::{u128_from_string, u128_to_string};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RealisWithdrawTokensSchema {
    pub id: String,
    #[serde(rename = "topicRes", alias = "topicResponse")]
    pub topic_res: String,
    #[serde(rename = "from")]
    pub from_account_id: AccountId,
    pub params: RealisWithdrawSchemaParams,
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
