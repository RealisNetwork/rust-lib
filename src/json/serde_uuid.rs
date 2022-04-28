use serde::{Deserialize, Deserializer, Serializer};
use uuid::Uuid;

pub fn uuid_to_string<S>(uuid_: &Uuid, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    serializer.serialize_str(&uuid_.hyphenated().to_string())
}

/// # Errors
pub fn uuid_from_string<'de, D>(deserializer: D) -> Result<Uuid, D::Error>
where
    D: Deserializer<'de>,
{
    match String::deserialize(deserializer) {
        Ok(value) => Uuid::parse_str(&value).map_err(|error| serde::de::Error::custom(error.to_string())),
        Err(error) => Err(serde::de::Error::custom(format!(
            "Cannot convert to token_id(U256) with error: {:?}",
            error
        ))),
    }
}