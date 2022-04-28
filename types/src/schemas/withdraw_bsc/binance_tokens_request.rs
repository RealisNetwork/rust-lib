use rust_lib::json::u128::{u128_from_string,u128_to_string};
use runtime::AccountId;
use serde::{Deserialize, Serialize};
use crate::Amount;


#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct BinanceTokensSchema {
    pub id: String,
    pub params: BinanceTokensSchemaParams,
}
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct BinanceTokensSchemaParams {
    #[serde(rename = "accountId")]
    pub account_id: String,
    #[serde(serialize_with = "u128_to_string")]
    #[serde(deserialize_with = "u128_from_string")]
    pub amount: Amount,
    #[serde(rename = "from")]
    pub from_account_id: AccountId,
}
