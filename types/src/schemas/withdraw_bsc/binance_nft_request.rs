use realis_primitives::TokenId;
use runtime::AccountId;
use serde::{Deserialize, Serialize};

use rust_lib::json::token_id::{token_id_from_string, token_id_to_string};
use crate::requests::AuthInfo;
use crate::schemas::realis_orchestrator::withdraw_request::binance_withdraw_nft::BinanceWithdrawNftSchema;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct BinanceNftRequestSchema {
    pub id: String,
    #[serde(rename = "topicResponse", alias = "topicRes")]
    pub topic_res: String,
    pub params: BinanceNftSchemaParams,
    #[serde(rename = "authInfo")]
    pub auth_info: AuthInfo,
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

impl BinanceNftRequestSchema {
    pub fn new(other: &BinanceWithdrawNftSchema, account_id: AccountId) -> Self {
        let params = BinanceNftSchemaParams {
            account_id: other.params.account_id.clone(),
            token_id: other.params.token_id,
            from_account_id: account_id,
        };
        BinanceNftRequestSchema {
            id: other.id.clone(),
            topic_res: other.topic_res.clone(),
            params,
            auth_info: other.auth_info.clone()
        }
    }
}

