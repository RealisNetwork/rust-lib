use crate::{
    requests::AuthInfo, schemas::realis_orchestrator::marketplace::sell_nft::SellNftSchema as OrchestratorSellNftSchema, Amount,
};
use json::{
    token_id::{token_id_from_string, token_id_to_string},
    u128::{u128_from_string, u128_to_string},
};
use realis_primitives::TokenId;
use runtime::AccountId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SellNftSchema {
    pub id: String,
    #[serde(rename = "topicRes", alias = "topicResponse")]
    pub topic_res: String,
    pub method: String,
    pub params: SellNftSchemaParams,
    #[serde(rename = "authInfo")]
    pub auth_info: AuthInfo,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SellNftSchemaParams {
    #[serde(serialize_with = "token_id_to_string")]
    #[serde(deserialize_with = "token_id_from_string")]
    #[serde(rename = "tokenId")]
    pub token_id: TokenId,
    #[serde(serialize_with = "u128_to_string")]
    #[serde(deserialize_with = "u128_from_string")]
    pub amount: Amount,
    #[serde(rename = "accountId")]
    pub account_id: AccountId,
}

impl SellNftSchema {
    pub fn new(other: OrchestratorSellNftSchema, account_id: AccountId) -> Self {
        Self {
            id: other.id,
            topic_res: other.topic_res,
            method: other.method,
            auth_info: other.auth_info,
            params: SellNftSchemaParams {
                token_id: other.params.token_id,
                amount: other.params.amount,
                account_id,
            },
        }
    }
}
