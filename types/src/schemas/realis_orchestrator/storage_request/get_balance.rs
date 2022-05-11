use crate::{
    requests::AuthInfo,
    schemas::storage_service::balance::GetBalanceSchema as StorageGetBalanceSchema,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetBalanceSchema {
    pub id: String,
    #[serde(rename = "topicResponse")]
    pub topic_res: String,
    #[serde(rename = "authInfo")]
    pub auth_info: AuthInfo,
}

impl From<StorageGetBalanceSchema> for GetBalanceSchema {
    fn from(other: StorageGetBalanceSchema) -> Self {
        Self {
            id: other.id,
            topic_res: other.topic_res,
            auth_info: other.auth_info,
        }
    }
}
