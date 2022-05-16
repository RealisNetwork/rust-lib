use runtime::AccountId;
use json::u128::{u128_from_string, u128_to_string};
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::Amount;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct RealisListenerIncreaseBalanceSchema {
    pub id: String,
    pub from: AccountId,
    pub to: AccountId,
    #[serde(serialize_with = "u128_to_string")]
    #[serde(deserialize_with = "u128_from_string")]
    pub amount: Amount,
    pub hash: Value,
}
