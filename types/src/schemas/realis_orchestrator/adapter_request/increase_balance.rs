use crate::{requests::AuthInfo, schemas::realis_adapter::increase_balance::IncreaseBalanceSchema as AdapterIncreaseBalanceSchema};
use json::u128::{u128_from_string, u128_to_string};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IncreaseBalanceSchema {
    pub id: String,
    #[serde(rename = "topicResponse", alias = "topicRes")]
    pub topic_res: String,
    pub params: IncreaseBalanceSchemaParams,
    #[serde(rename = "authInfo")]
    pub auth_info: AuthInfo,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IncreaseBalanceSchemaParams {
    #[serde(serialize_with = "u128_to_string")]
    #[serde(deserialize_with = "u128_from_string")]
    pub amount: u128,
}

impl From<AdapterIncreaseBalanceSchema> for IncreaseBalanceSchema {
    fn from(other: AdapterIncreaseBalanceSchema) -> Self {
        Self {
            id: other.id,
            topic_res: other.topic_res,
            params: IncreaseBalanceSchemaParams {
                amount: other.params.amount,
            },
            auth_info: other.auth_info,
        }
    }
}
