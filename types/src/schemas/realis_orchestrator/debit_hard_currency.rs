use crate::{requests::AuthInfo, schemas::realis_adapter::debit_transfer::DebitTransferSchema};
use rust_lib::json::u128::{u128_from_string, u128_to_string};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DebitHardCurrencySchema {
    pub id: String,
    #[serde(rename = "topicResponse", alias = "topicRes")]
    pub topic_res: String,
    pub params: DebitHardCurrencyParams,
    #[serde(rename = "authInfo")]
    pub auth_info: AuthInfo,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DebitHardCurrencyParams {
    #[serde(serialize_with = "u128_to_string")]
    #[serde(deserialize_with = "u128_from_string")]
    pub amount: u128,
}

impl From<DebitTransferSchema> for DebitHardCurrencySchema {
    fn from(other: DebitTransferSchema) -> DebitHardCurrencySchema {
        DebitHardCurrencySchema {
            id: other.id,
            topic_res: other.topic_res,
            params: DebitHardCurrencyParams {
                amount: other.params.amount,
            },
            auth_info: other.auth_info,
        }
    }
}
