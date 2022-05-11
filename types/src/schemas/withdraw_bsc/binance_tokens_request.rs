use runtime::AccountId;
use serde::{Deserialize, Serialize};

use rust_lib::json::u128::{u128_from_string, u128_to_string};

use crate::{
    requests::AuthInfo, schemas::realis_orchestrator::withdraw_request::binance_withdraw_tokens::BinanceWithdrawTokensSchema, Amount,
};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct BinanceTokensSchema {
    pub id: String,
    pub params: BinanceTokensSchemaParams,
    #[serde(rename = "topicResponse", alias = "topicRes")]
    pub topic_res: String,
    #[serde(rename = "authInfo")]
    pub auth_info: AuthInfo,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct BinanceTokensSchemaParams {
    #[serde(rename = "accountId")]
    pub account_id: String,
    #[serde(serialize_with = "u128_to_string")]
    #[serde(deserialize_with = "u128_from_string")]
    pub amount: Amount,
    #[serde(rename = "from")]
    pub from_account_id: AccountId,
}

impl BinanceTokensSchema {
    pub fn new(other: BinanceWithdrawTokensSchema, account_id: AccountId) -> Self {
        let params = BinanceTokensSchemaParams {
            account_id: other.params.account_id.clone(),
            amount: other.params.amount,
            from_account_id: account_id,
        };
        Self {
            id: other.id.clone(),
            params,
            topic_res: other.topic_res.clone(),
            auth_info: other.auth_info.clone(),
        }
    }
}
