use crate::primitives::listener::types::{Amount, BscAccount, Hash};
use realis_primitives::TokenId;
use runtime::AccountId;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransferTokens {
    pub from: AccountId,
    pub to: AccountId,
    pub amount: Amount,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransferTokensFromBsc {
    pub from: BscAccount,
    pub to: AccountId,
    pub amount: Amount,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NftMinted {
    pub to: AccountId,
    #[serde(serialize_with = "token_id_to_string")]
    #[serde(deserialize_with = "token_id_from_string")]
    pub token_id: TokenId,
    pub id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransferNft {
    pub from: AccountId,
    pub to: AccountId,
    #[serde(serialize_with = "token_id_to_string")]
    #[serde(deserialize_with = "token_id_from_string")]
    pub token_id: TokenId,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransferNftFromBsc {
    pub from: BscAccount,
    pub to: AccountId,
    #[serde(serialize_with = "token_id_to_string")]
    #[serde(deserialize_with = "token_id_from_string")]
    pub token_id: TokenId,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransferNftToBsc {
    pub from: AccountId,
    pub to: BscAccount,
    #[serde(serialize_with = "token_id_to_string")]
    #[serde(deserialize_with = "token_id_from_string")]
    pub token_id: TokenId,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EventType {
    TransferTokens(TransferTokens, Hash),
    TransferTokensFromBsc(TransferTokensFromBsc, Hash),

    NftMinted(NftMinted),
    NftMintedError(String),

    TransferNft(TransferNft, Hash),
    TransferNftFromBsc(TransferNftFromBsc, Hash),
    TransferNftToBsc(TransferNftToBsc, Hash),
}

fn token_id_to_string<S>(token_id: &TokenId, serializer: S) -> Result<S::Ok, S::Error>
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
            "Cannot convert to u8 with error: {:?}",
            error
        ))),
    }
}
