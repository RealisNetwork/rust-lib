mod nft_minted;
mod temp;
mod transfer_token;
mod transfer_nft;

pub fn option_u64_from_string<'de, D>(deserializer: D) -> Result<Option<u128>, D::Error>
where
    D: Deserializer<'de>,
{
    match Option::deserialize(deserializer) {
        Ok(Some(number)) => Ok(Some(blockchain_number_from_string::<'de, D>(number)?)),
        Ok(None) | Err(_) => Ok(None),
    }
}

pub fn option_u64_to_string<S>(option: &Option<u128>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    match option.map(|number| blockchain_number_to_string(&number)) {
        None => serializer.serialize_none(),
        Some(number) => serializer.serialize_some(&number),
    }
}
