use runtime::AccountId;
use serde::{Deserialize, Serialize};
use rust_lib::json::u128::{u128_from_string, u128_to_string};

use crate::Amount;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct RealisTokenRequestSchema {
    #[serde(rename = "from")]
    pub from_account_id: AccountId,
    pub id: String,
    pub params: RealisTokenRequestSchemaParams,

}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct RealisTokenRequestSchemaParams {
    #[serde(rename = "accountId")]
    pub account_id: AccountId,
    #[serde(serialize_with = "u128_to_string")]
    #[serde(deserialize_with = "u128_from_string")]
    pub amount: Amount,

}