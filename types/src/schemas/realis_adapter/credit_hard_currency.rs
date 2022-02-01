use runtime::AccountId;
use rust_lib::json::u128::{u128_to_string, u128_from_string};

pub struct CreditHardCurrencySchema {
    pub id: String,
    pub account_id: AccountId,
    pub params: CreditHardCurrencyParams
}

pub struct CreditHardCurrencyParams {
    #[serde(serialize_with = "u128_to_string")]
    #[serde(deserialize_with = "u128_from_string")]
    pub amount: u128
}
