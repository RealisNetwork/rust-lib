pub mod u128;

use realis_primitives::TokenId;

use serde::{Serializer, Deserializer, Deserialize};

/// # Errors
pub fn u128_from_string<'de, D>(deserializer: D) -> Result<u128, D::Error>
    where
        D: Deserializer<'de>,
{
    match String::deserialize(deserializer)?.parse::<u128>() {
        Ok(value) => Ok(value),
        Err(error) => Err(serde::de::Error::custom(format!(
            "Cannot convert to u128 with error: {:?}",
            error
        ))),
    }
}

fn u128_to_string<S>(number: &u128, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
{
    serializer.serialize_str(&number.to_string())
}

pub fn token_id_to_string<S>(token_id: &TokenId, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
{
    serializer.serialize_str(&format!("{:?}", token_id))
}

/// # Errors
pub fn token_id_from_string<'de, D>(deserializer: D) -> Result<TokenId, D::Error>
    where
        D: Deserializer<'de>,
{
    match String::deserialize(deserializer) {
        Ok(value) => TokenId::from_dec_str(&value)
            .map_err(|error| serde::de::Error::custom(error.to_string())),
        Err(error) => Err(serde::de::Error::custom(format!(
            "Cannot convert to token_id(U256) with error: {:?}",
            error
        ))),
    }
}