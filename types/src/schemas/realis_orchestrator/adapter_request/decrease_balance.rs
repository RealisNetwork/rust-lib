use crate::{requests::AuthInfo, schemas::realis_adapter::decrease_balance::DecreaseBalanceSchema as AdapterDecreaseBalanceSchema};
use rust_lib::json::u128::{u128_from_string, u128_to_string};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DecreaseBalanceSchema {
    pub id: String,
    #[serde(rename = "topicResponse", alias = "topicRes")]
    pub topic_res: String,
    pub params: DecreaseBalanceSchemaParams,
    #[serde(rename = "authInfo")]
    pub auth_info: AuthInfo,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DecreaseBalanceSchemaParams {
    #[serde(serialize_with = "u128_to_string")]
    #[serde(deserialize_with = "u128_from_string")]
    pub amount: u128,
}

impl From<AdapterDecreaseBalanceSchema> for DecreaseBalanceSchema {
    fn from(other: AdapterDecreaseBalanceSchema) -> Self {
        Self {
            id: other.id,
            topic_res: other.topic_res,
            params: DecreaseBalanceSchemaParams {
                amount: other.params.amount,
            },
            auth_info: other.auth_info,
        }
    }
}
