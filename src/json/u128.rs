use serde::{de::Error, Deserialize, Deserializer, Serializer};

const DECIMALS: u8 = 12;

/// # Errors
pub fn u128_from_string<'de, D>(deserializer: D) -> Result<u128, D::Error>
where
    D: Deserializer<'de>,
{
    let number = String::deserialize(deserializer)?;
    if number.contains('.') {
        // If number is float
        let splitted = number.split('.').map(String::from).collect::<Vec<String>>();
        // Get integer part
        let integer = splitted
            .first()
            .ok_or(Error::custom(String::from("Missing integer part!")))?
            .parse::<u128>()
            .map_err(|error| Error::custom(format!("Cannot convert integer part to u128: {:?}", error)))?;
        // Get fractional len
        let fractional_length = splitted
            .get(1)
            .ok_or(Error::custom(String::from("Missing fractional part!")))?
            .len();
        // Get fractional part
        let fractional = splitted
            .get(1)
            .ok_or(Error::custom(String::from("Missing fractional part!")))?
            .parse::<u128>()
            .map_err(|error| Error::custom(format!("Cannot convert fractional part to u128: {:?}", error)))?;

        let integer = integer * 10_u128.pow(DECIMALS.into());
        let fractional = fractional * 10_u128.pow(DECIMALS as u32 - fractional_length as u32);

        Ok(integer + fractional)
    } else {
        // If number is integer
        let number = number
            .parse::<u128>()
            .map_err(|error| Error::custom(format!("Cannot convert to u128 with error: {}", error)))?;
        Ok(number * 10_u128.pow(DECIMALS.into()))
    }
}

pub fn blockchain_number_to_string(number: &u128) -> String {
    if *number == 0 {
        return String::from("0");
    }
    let mut number_string = number.to_string();
    if *number < 10_u128.pow(DECIMALS as u32) {
        for _i in 0..(DECIMALS as usize - number_string.len() + 1) {
            number_string.insert(0, '0');
        }
    }
    number_string.insert(number_string.len() - usize::from(DECIMALS), '.');
    number_string
}

pub fn u128_to_string<S>(number: &u128, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let number = blockchain_number_to_string(number);
    serializer.serialize_str(&number)
}
