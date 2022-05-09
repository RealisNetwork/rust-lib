use crate::{requests::AuthInfo, schemas::realis_adapter::increase_balance::IncreaseBalanceSchema};
use rust_lib::json::u128::{u128_from_string, u128_to_string};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrchestratorIncreaseBalanceSchema {
    pub id: String,
    #[serde(rename = "topicResponse", alias = "topicRes")]
    pub topic_res: String,
    pub params: OrchestratorIncreaseBalanceSchemaParams,
    #[serde(rename = "authInfo")]
    pub auth_info: AuthInfo,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrchestratorIncreaseBalanceSchemaParams {
    #[serde(serialize_with = "u128_to_string")]
    #[serde(deserialize_with = "u128_from_string")]
    pub amount: u128,
}

impl From<IncreaseBalanceSchema> for OrchestratorIncreaseBalanceSchema {
    fn from(other: IncreaseBalanceSchema) -> Self {
        OrchestratorIncreaseBalanceSchema {
            id: other.id,
            topic_res: other.topic_res,
            params: OrchestratorIncreaseBalanceSchemaParams {
                amount: other.params.amount,
            },
            auth_info: other.auth_info,
        }
    }
}
