use crate::{requests::AuthInfo, Amount};
use runtime::AccountId;
use rust_lib::json::u128::{u128_from_string, u128_to_string};
use serde::{Deserialize, Serialize};
use crate::schemas::withdraw_bsc::binance_tokens_request::BinanceTokensSchema;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BinanceWithdrawTokensSchema {
    pub id: String,
    #[serde(rename = "topicRes", alias = "topicResponse")]
    pub topic_res: String,
    pub params: BinanceWithdrawTokensSchemaParams,
    #[serde(rename = "authInfo")]
    pub auth_info: AuthInfo,
}
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct BinanceWithdrawTokensSchemaParams {
    #[serde(rename = "accountId")]
    pub account_id: String,
    #[serde(serialize_with = "u128_to_string")]
    #[serde(deserialize_with = "u128_from_string")]
    pub amount: Amount,
    #[serde(rename = "from")]
    pub from_account_id: AccountId,
}

impl From<BinanceTokensSchema> for BinanceWithdrawTokensSchema {
    fn from(other: &BinanceTokensSchema) -> Self {
        BinanceWithdrawTokensSchema {
            id: other.id.clone(),
            topic_res: other.topic_res.clone(),
            params: BinanceWithdrawTokensSchemaParams {
                account_id: other.params.account_id.clone(),
                amount: other.params.amount,
                from_account_id: other.params.from_account_id.clone(),

            },
            auth_info: other.auth_info.clone(),
        }
    }
}
