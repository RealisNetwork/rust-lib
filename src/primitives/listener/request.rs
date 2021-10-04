use realis_primitives::{Rarity, TokenId};
use runtime::{realis_game_api::Call as RealisGameApiCall, AccountId, Call};
use serde::{Deserialize, Deserializer, Serialize, Serializer};

pub type Agent = String;
pub type Lang = String;
pub type Id = String;
pub type TopicRes = String;

pub type Amount = u128;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Request {
    pub id: String,
    pub lang: String,
    pub agent: String,
    #[serde(alias = "topicRes")]
    pub topic_res: String,

    #[serde(flatten)]
    pub params: Params,

    #[serde(alias = "authInfo")]
    pub auth_info: AuthInfo,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AuthInfo {
    #[serde(alias = "userId")]
    pub user_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "method", content = "params")]
pub enum Params {
    #[serde(rename = "create_wallet")]
    CreateWallet {},

    #[serde(rename = "credit_hard_currency")]
    CreditTransfer {
        #[serde(serialize_with = "u128_to_string")]
        #[serde(deserialize_with = "u128_from_string")]
        amount: Amount,
        currency: Vec<String>,
    },

    #[serde(rename = "debit_hard_currency")]
    DebitTransfer {
        #[serde(serialize_with = "u128_to_string")]
        #[serde(deserialize_with = "u128_from_string")]
        amount: Amount,
        currency: Vec<String>,
    },

    #[serde(rename = "get_balance")]
    GetBalance {},

    #[serde(rename = "get_nft_item_list")]
    GetNftList {},

    #[serde(rename = "add_nft_item")]
    AddNftItem {
        #[serde(serialize_with = "token_id_to_string")]
        #[serde(deserialize_with = "token_id_from_string")]
        token_id: TokenId,
        rarity: Rarity,
    },

    #[serde(rename = "remove_nft_item")]
    RemoveNftItem { token_id: TokenId },

    #[serde(rename = "withdraw_tokens")]
    WithdrawTokens {
        account_id: AccountId,
        #[serde(serialize_with = "u128_to_string")]
        #[serde(deserialize_with = "u128_from_string")]
        amount: Amount,
        currency: Vec<String>,
    },

    #[serde(rename = "withdraw_nft")]
    WithdrawNft {
        account_id: AccountId,
        #[serde(serialize_with = "token_id_to_string")]
        #[serde(deserialize_with = "token_id_from_string")]
        token_id: TokenId,
    },

    #[serde(rename = "transfer_tokens_from_player")]
    TransferTokensToPlayer {
        #[serde(rename = "userId")]
        user_id: String,
        #[serde(serialize_with = "u128_to_string")]
        #[serde(deserialize_with = "u128_from_string")]
        amount: Amount,
        currency: Vec<String>,
    },

    #[serde(rename = "transfer_nft_from_player")]
    TransferNftToPlayer {
        #[serde(rename = "userId")]
        user_id: String,
        #[serde(serialize_with = "token_id_to_string")]
        #[serde(deserialize_with = "token_id_from_string")]
        token_id: TokenId,
    },

    #[serde(rename = "get_account_id")]
    GetAccountId {},
}

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

/// # Errors
pub fn u8_from_string<'de, D>(deserializer: D) -> Result<u8, D::Error>
    where
        D: Deserializer<'de>,
{
    match String::deserialize(deserializer)?.parse::<u8>() {
        Ok(value) => Ok(value),
        Err(error) => Err(serde::de::Error::custom(format!(
            "Cannot convert to u8 with error: {:?}",
            error
        ))),
    }
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

#[allow(clippy::trivially_copy_pass_by_ref)]
fn u8_to_string<S>(number: &u8, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
{
    serializer.serialize_str(&number.to_string())
}

