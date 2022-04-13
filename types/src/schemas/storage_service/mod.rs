use rust_lib::json::u128::{blockchain_number_from_string, blockchain_number_to_string};
use serde::{Deserialize, Deserializer, Serializer};

pub mod balance;
pub mod marketplace_nft_list;
pub mod user_nft_list;
pub mod user_nft_list_with_offset;

pub fn option_u128_from_string<'de, D>(deserializer: D) -> Result<Option<u128>, D::Error>
where
    D: Deserializer<'de>,
{
    match Option::deserialize(deserializer) {
        Ok(Some(number)) => Ok(Some(blockchain_number_from_string::<'de, D>(number)?)),
        Ok(None) | Err(_) => Ok(None),
    }
}

pub fn option_u128_to_string<S>(option: &Option<u128>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    match option.map(|number| blockchain_number_to_string(&number)) {
        None => serializer.serialize_none(),
        Some(number) => serializer.serialize_some(&number),
    }
}
