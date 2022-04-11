use crate::{requests::AuthInfo, schemas::realis_adapter::credit_transfer::CreditTransferSchema};
use rust_lib::json::u128::{u128_from_string, u128_to_string};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreditHardCurrencySchema {
    pub id: String,
    #[serde(rename = "topicResponse", alias = "topicRes")]
    pub topic_res: String,
    pub params: CreditHardCurrencyParams,
    #[serde(rename = "authInfo")]
    pub auth_info: AuthInfo,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreditHardCurrencyParams {
    #[serde(serialize_with = "u128_to_string")]
    #[serde(deserialize_with = "u128_from_string")]
    pub amount: u128,
}

impl From<CreditTransferSchema> for CreditHardCurrencySchema {
    fn from(other: CreditTransferSchema) -> CreditHardCurrencySchema {
        CreditHardCurrencySchema {
            id: other.id,
            topic_res: other.topic_res,
            params: CreditHardCurrencyParams {
                amount: other.params.amount,
            },
            auth_info: other.auth_info,
        }
    }
}
