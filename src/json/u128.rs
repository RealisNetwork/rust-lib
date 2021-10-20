use serde::{Serializer, Deserializer, Deserialize};

const DECIMALS: u8 = 12;

/// # Errors
pub fn u128_from_string<'de, D>(deserializer: D) -> Result<u128, D::Error>
    where
        D: Deserializer<'de>,
{
    // f64 max value = 17 976 931 348 623 156.0
    match String::deserialize(deserializer)?.parse::<f64>().map(|value| {
        (value * (10_u64.pow(DECIMALS.into()) as f64)).round() as u128
    }) {
        Ok(value) => Ok(value),
        Err(error) => Err(serde::de::Error::custom(format!(
            "Cannot convert to f64 with error: {:?}",
            error
        ))),
    }
}

fn u128_to_string<S>(number: &u128, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
{
    let mut number = number.to_string();
    number.insert(number.len() - usize::from(DECIMALS), '.');
    serializer.serialize_str(&number)
}