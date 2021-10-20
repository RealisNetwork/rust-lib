use serde::{Serializer, Deserializer, Deserialize};
use serde::de::Error;

const DECIMALS: u8 = 12;

/// # Errors
pub fn u128_from_string<'de, D>(deserializer: D) -> Result<u128, D::Error>
    where
        D: Deserializer<'de>,
{
    let number = String::deserialize(deserializer)?;
    if number.contains('.') {
        let splitted = number.split('.').map(String::from).collect::<Vec<String>>();
        let integer = splitted
            .first()
            .ok_or(Error::custom(String::from("Missing integer part!")))?
            .parse::<u128>()
            .map_err(|error|
                Error::custom(format!("Cannot convert integer part to u128: {:?}", error)))?;
        let fractional_length = splitted
            .get(1)
            .ok_or(Error::custom(String::from("Missing fractional part!")))?
            .len();
        let fractional = splitted
            .get(1)
            .ok_or(Error::custom(String::from("Missing fractional part!")))?
            .parse::<u128>()
            .map_err(|error|
                Error::custom(format!("Cannot convert fractional part to u128: {:?}", error)))?;

        let integer = integer * 10_u128.pow(DECIMALS.into());
        let fractional = fractional * 10_u128.pow(DECIMALS as u32 - fractional_length as u32);

        Ok(integer + fractional)
    } else {
        // If number is integer
        let number = number
            .parse::<u128>()
            .map_err(|error| Error::custom(format!("Cannot convert to u128 with error: {}", error)))?;
        Ok(
            number * 10_u128.pow(DECIMALS.into()))
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