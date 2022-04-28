use runtime::AccountId;
use serde::{Deserialize, Serialize};
use realis_primitives::TokenId;
use rust_lib::json::token_id::{token_id_from_string,token_id_to_string};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct RealisNftRequestSchema {
    #[serde(rename = "from")]
    pub from_account_id: AccountId,
    pub id: String,
    pub params: RealisNftRequestSchemaParams,

}
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct RealisNftRequestSchemaParams {
    #[serde(rename = "accountId")]
    pub account_id: AccountId,
    #[serde(rename = "tokenId")]
    #[serde(serialize_with = "token_id_to_string")]
    #[serde(deserialize_with = "token_id_from_string")]
    pub token_id: TokenId,

}