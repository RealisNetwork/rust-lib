use crate::{requests::AuthInfo, schemas::realis_adapter::decrease_balance::DecreaseBalanceSchema};
use rust_lib::json::u128::{u128_from_string, u128_to_string};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrchestratorDecreaseBalanceSchema {
    pub id: String,
    #[serde(rename = "topicResponse", alias = "topicRes")]
    pub topic_res: String,
    pub params: OrchestratorDecreaseBalanceSchemaParams,
    #[serde(rename = "authInfo")]
    pub auth_info: AuthInfo,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrchestratorDecreaseBalanceSchemaParams {
    #[serde(serialize_with = "u128_to_string")]
    #[serde(deserialize_with = "u128_from_string")]
    pub amount: u128,
}

impl From<DecreaseBalanceSchema> for OrchestratorDecreaseBalanceSchema {
    fn from(other: DecreaseBalanceSchema) -> Self {
        OrchestratorDecreaseBalanceSchema {
            id: other.id,
            topic_res: other.topic_res,
            params: OrchestratorDecreaseBalanceSchemaParams {
                amount: other.params.amount,
            },
            auth_info: other.auth_info,
        }
    }
}
