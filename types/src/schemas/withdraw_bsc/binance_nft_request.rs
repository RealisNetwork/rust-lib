use realis_primitives::TokenId;
use runtime::AccountId;
use rust_lib::json::token_id::{token_id_from_string, token_id_to_string};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct BinanceNftRequestSchema {
    pub id: String,
    pub params: BinanceNftSchemaParams,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct BinanceNftSchemaParams {
    #[serde(rename = "accountId")]
    pub account_id: String,
    #[serde(rename = "tokenId")]
    #[serde(serialize_with = "token_id_to_string")]
    #[serde(deserialize_with = "token_id_from_string")]
    pub token_id: TokenId,
    #[serde(rename = "from")]
    pub from_account_id: AccountId,
}
